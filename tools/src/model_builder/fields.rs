use crate::model_builder::enums::ModelEnum;
use crate::model_builder::utils::mk_doc;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use scraper::ElementRef;
use scraper::Selector;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum AccessType {
    Mandatory,
    ReadOnly,
    Optional,
    Unsure,
}

#[derive(Debug)]
pub enum ModelType {
    Object(String),
    CountryCode,
    DateTime,
    Date,
    Uuid,
    Integer,
    Number,
    String,
    Bool,
    Currency,
    Enum(ModelEnum),
}

#[derive(Debug)]
pub struct ModelField {
    is_list: bool,
    pub model_type: ModelType,
    pub access_type: AccessType,
    doc: String,
    pub name: String,
    pub assume_mandatory: bool,
}

impl ModelField {
    pub fn create(table_row: ElementRef) -> Self {
        println!("{:?}", table_row.inner_html());
        let mut children = table_row.children().filter_map(ElementRef::wrap);
        let property = children.next().unwrap();
        let description = children.next().unwrap();
        let name = Self::find_name(&property);
        let model_type = Self::find_type(&name, &property, &description);
        Self {
            is_list: Self::find_list(&property),
            access_type: Self::find_access_type(&name, &description),
            name,
            model_type,
            doc: description.inner_html(),
            assume_mandatory: false,
        }
    }

    pub fn type_name(&self) -> String {
        let model_type = match &self.model_type {
            ModelType::Object(x) => {
                if x.starts_with('#') {
                    string_morph::to_pascal_case(&self.name)
                } else {
                    string_morph::to_pascal_case(x)
                }
            }
            ModelType::Uuid => "uuid::Uuid".to_string(),
            ModelType::Integer => "i64".to_string(),
            ModelType::Number => "f64".to_string(),
            ModelType::String => "String".to_string(),
            ModelType::Bool => "bool".to_string(),
            ModelType::CountryCode => "crate::types::CountryCode".to_string(),
            ModelType::DateTime => "crate::types::DateTime".to_string(),
            ModelType::Date => "crate::types::Date".to_string(),
            ModelType::Currency => "crate::types::Currency".to_string(),
            ModelType::Enum(_) => {
                string_morph::to_pascal_case(self.name.as_str())
            }
        };
        if self.is_list {
            format!("Vec<{}>", model_type)
        } else {
            model_type
        }
    }
    pub fn ident(&self) -> String {
        match self.name.as_str() {
            "type" => "_type".to_string(),
            x => string_morph::to_snake_case(&x),
        }
    }
    fn find_name(property: &ElementRef) -> String {
        property
            .text()
            .map(|x| x.trim())
            .next()
            .unwrap()
            .to_string()
    }

    fn find_list(property: &ElementRef) -> bool {
        let code_selector = Selector::parse("code").unwrap();

        if Self::find_name(&property) == "subItems" {
            return true;
        }

        let codes = property.select(&code_selector).collect::<Vec<_>>();

        codes
            .get(0)
            .map(|x| x.text().collect::<String>())
            .unwrap_or(String::new())
            == "list"
    }

    pub fn find_type(
        name: &str,
        property: &ElementRef,
        description: &ElementRef,
    ) -> ModelType {
        let link_selector = Selector::parse("a").unwrap();
        let code_selector = Selector::parse("code").unwrap();

        match name {
            "eventType" => return ModelType::Object("EventType".to_string()),
            "billing" | "shipping" => {
                return ModelType::Object("Address".to_string())
            }
            "countryCode" => return ModelType::CountryCode,
            "currency" => return ModelType::Currency,
            "subItems" => return ModelType::Object("LineItems".to_string()),
            "contactPersons" => {
                return ModelType::Object("CompanyContactPerson".to_string())
            }
            _ => (),
        }

        let codes = property.select(&code_selector).collect::<Vec<_>>();
        if codes.len() == 0 && property.inner_html().ends_with("Date") {
            return ModelType::DateTime;
        }
        assert_eq!(codes.len(), 1, "<code> tags: {:?}", property.inner_html());

        println!(
            "   {}: {}",
            name,
            codes[0].text().collect::<String>().as_str()
        );
        match codes[0].text().collect::<String>().to_lowercase().as_str() {
            "date" => {
                assert!(description.select(&code_selector).any(|x| x
                    .text()
                    .collect::<String>()
                    .as_str()
                    == "yyyy-MM-dd"));
                return ModelType::Date;
            }
            "datetime" => return ModelType::DateTime,
            "uuid" => return ModelType::Uuid,
            "integer" => return ModelType::Integer,
            "number" => return ModelType::Number,
            "string" => return ModelType::String,
            "boolean" => return ModelType::Bool,
            "enum" => {
                let mut model_enum = ModelEnum::create(name.to_string());
                model_enum.parse_description(&description);
                return ModelType::Enum(model_enum);
            }
            _ => {}
        }

        let text = description.text().collect::<String>();
        if text.contains("type string") {
            return ModelType::String;
        } else if text.contains("A list of voucher image file uuids") {
            return ModelType::Uuid;
        }

        let links = property
            .select(&link_selector)
            .chain(description.select(&link_selector))
            .filter(|x| {
                x.value().attr("href").map_or(false, |x| x.starts_with('#'))
            })
            .collect::<Vec<_>>();

        if links.len() == 1 {
            let link = links[0].value().attr("href").unwrap();
            ModelType::Object(link.to_string())
        } else {
            ModelType::Object(name.to_string())
        }
    }

    fn find_access_type(name: &str, description: &ElementRef) -> AccessType {
        match name {
            "id" => return AccessType::ReadOnly,
            "version" => return AccessType::Mandatory,
            _ => (),
        }
        let description =
            description.text().collect::<String>().to_ascii_lowercase();
        // Field is optional if description contains "may be present"
        if description.contains("may be present")
            || description.contains("(optional)")
        {
            AccessType::Optional
        } else if description.contains("read-only")
            || description.contains("created by lexoffice")
        {
            AccessType::ReadOnly
        } else {
            AccessType::Unsure
        }
    }

    pub fn codegen(&self) -> TokenStream {
        let mut annotations = Vec::<TokenStream>::new();
        let name = format_ident!("{}", self.ident());
        let property_type = self.type_name();
        let property_type = TokenStream::from_str(&property_type).unwrap();
        let access_type = if self.access_type == AccessType::Unsure {
            if self.assume_mandatory {
                &AccessType::Mandatory
            } else {
                &AccessType::Optional
            }
        } else {
            &self.access_type
        };

        if matches!(self.model_type, ModelType::Date) {
            match access_type {
                AccessType::Optional | AccessType::Unsure => annotations.push(
                    quote!( #[serde(with = "crate::serde::optional_date")]),
                ),
                _ => annotations
                    .push(quote!( #[serde(with = "crate::serde::date")])),
            }
        }
        let property_type = match access_type {
            AccessType::Mandatory => {
                if name == "version" {
                    annotations
                        .push(quote! {#[builder(default, setter(skip))] });
                } else {
                    annotations.push(quote! {#[builder(setter(into))] });
                }
                property_type
            }
            AccessType::ReadOnly => {
                annotations.push(quote! {#[builder(default, setter(skip))] });
                //annotations.push(quote! {#[serde(skip_serializing)] });
                quote! { crate::marker::ReadOnly< #property_type > }
            }
            AccessType::Optional | AccessType::Unsure => {
                annotations.push(quote! { #[serde(skip_serializing_if = "Option::is_none")] });
                annotations
                    .push(quote! {#[builder(default, setter(strip_option))] });
                quote! { Option< #property_type > }
            }
        };
        let doc = mk_doc(&self.doc);
        quote! {
            #doc
            #( #annotations )*
            pub #name: #property_type
        }
    }
    pub fn codegen_enum(&self) -> TokenStream {
        let mut annotations = Vec::<TokenStream>::new();
        let name = self.ident();
        let property_type = self.type_name();
        let property_type = TokenStream::from_str(&property_type).unwrap();
        let doc = mk_doc(&self.doc);
        annotations.push(quote! {#[serde(rename = #name)] });
        quote! {
            #doc
            #( #annotations )*
            #property_type(#property_type)
        }
    }
}
