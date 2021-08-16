use crate::model_builder::enums::ModelEnum;
use crate::model_builder::fields::AccessType;
use crate::model_builder::fields::ModelField;
use crate::model_builder::fields::ModelType;
use crate::model_builder::modules::ModelModule;
use crate::model_builder::utils::mk_doc;
use crate::model_builder::utils::StringUtils;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use scraper::ElementRef;
use scraper::Selector;

#[derive(Debug)]
pub struct ModelStruct {
    pub name: String,
    doc: String,
    module_id: String,
    id: Option<String>,
    fields: Vec<ModelField>,
    assume_mandatory: bool,
}

impl ModelStruct {
    pub fn create(name: String, module: &ModelModule) -> Self {
        //println!(" => {}", name);
        Self {
            name,
            module_id: module.id.clone(),
            doc: String::new(),
            id: None,
            fields: vec![],
            assume_mandatory: false,
        }
    }

    pub fn has_id(&self) -> Option<String> {
        self.fields
            .iter()
            .find(|x| x.name == "id" || x.name == "subscriptionId")
            .map(|x| x.ident())
    }

    pub fn type_name(&self) -> String {
        // Fastpath:
        let name = &self.name;
        if let "E-Mail Addresses Details" = name.as_str() {
            return "EmailAddresses".to_string();
        }

        let name = if name.to_ascii_lowercase().ends_with(" properties") {
            // Endpoint properties should be singular
            name.remove_suffix(" properties").to_singular()
        } else if name.ends_with(" Details") {
            name.remove_suffix(" details")
                .remove_prefix("object ")
                .to_string()
        } else {
            panic!("{:?}", name);
        };
        string_morph::to_pascal_case(&name)
    }

    pub fn parse_section(&mut self, section: Vec<ElementRef>) {
        let thead_selector = Selector::parse("thead").unwrap();
        let mut iter = section.iter();
        if self.name.is_empty() {
            self.name = iter.next().unwrap().text().collect();
        }
        self.doc = iter
            .take_while(|x| {
                !x.text()
                    .collect::<String>()
                    .trim()
                    .replace("\n", " ")
                    .starts_with("Property Description")
            })
            .filter(|x| !["blockquote", "pre"].contains(&x.value().name()))
            .map(|x| x.html())
            .collect::<String>()
            .trim()
            .to_string();

        match self.type_name().as_str() {
            "WebhookCallback" | "EventSubscription" | "Country" => {
                self.assume_mandatory = true
            }
            _ => (),
        }
        if let Some(table) = section
            .iter()
            .filter(|x| x.value().name() == "table")
            .find(|x| {
                x.select(&thead_selector)
                    .next()
                    .unwrap()
                    .text()
                    .collect::<String>()
                    .trim()
                    .replace("\n", " ")
                    .starts_with("Property Description")
            })
        {
            self.parse_fields(table);
        }
    }
    fn parse_fields(&mut self, table: &ElementRef) {
        let tbody_selector = Selector::parse("tbody > tr").unwrap();
        for field in table.select(&tbody_selector) {
            let mut model_field = ModelField::create(field);
            model_field.assume_mandatory = self.assume_mandatory;
            self.fields.push(model_field);
        }
    }

    pub fn collect_enums(&self) -> Vec<&ModelEnum> {
        self.fields
            .iter()
            .filter_map(|x| match &x.model_type {
                ModelType::Enum(x) => Some(x.to_owned()),
                _ => None,
            })
            .collect()
    }

    pub fn parse_creation_info(&mut self, section: Vec<&ElementRef>) {
        if section.is_empty() {
            return;
        }

        let mut current_info = string_morph::to_pascal_case(
            &section[0]
                .text()
                .collect::<String>()
                .trim()
                .remove_prefix("Create a "),
        );
        let mut iter = section.iter();

        while let Some(element) = iter.next() {
            if current_info == self.type_name() {
                break;
            }

            if !element.inner_html().trim().starts_with("<strong") {
                continue;
            }

            let text = element.text().collect::<String>();
            if text.ends_with(" Details")
                || text.ends_with(" Required Properties")
            {
                current_info = string_morph::to_pascal_case(
                    &text
                        .remove_suffix(" Details")
                        .remove_suffix(" Required Properties")
                        .remove_prefix("Object "),
                );
                //println!(
                //    ":: {} -> {:?}",
                //    current_info,
                //    text.remove_suffix("Details")
                //);
            }

            if current_info == self.type_name() {
                break;
            }
        }
        let table = iter.find(|x| x.value().name() == "table");
        if table.is_none() {
            return;
        }
        let table = table.unwrap();

        let tr_selector = Selector::parse("tbody > tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();
        for row in table.select(&tr_selector) {
            let mut iter = row.select(&td_selector);
            let property = iter
                .next()
                .unwrap()
                .text()
                .collect::<String>()
                .trim()
                .to_string();
            let required =
                iter.next().unwrap().text().collect::<String>().trim() == "Yes";

            if required {
                println!("===! {:?}\n{:?}", property, self.fields);
                let field = self.fields.iter_mut().find(|x| x.name == property);
                if let Some(field) = field {
                    if field.access_type == AccessType::Unsure {
                        field.access_type = AccessType::Mandatory
                    }
                }
            }
        }
    }

    pub fn codegen(&self) -> TokenStream {
        let name = format_ident!("{}", self.type_name());
        //if name == "Roles" {
        //    return self.codegen_roles();
        //}
        let fields = self.fields.iter().map(|x| x.codegen());
        let doc = mk_doc(&self.doc);
        let implementation = if let Some(id_name) = self.has_id() {
            let id_name = format_ident!("{}", id_name);
            quote! {
                impl crate::request::HasId for #name {
                    fn id(&self) -> &crate::marker::ReadOnly<uuid::Uuid> {
                        &self.#id_name
                    }
                }
            }
        } else {
            quote! {}
        };
        quote! {
            #doc
            #[derive(Debug, Clone, PartialEq, TypedBuilder, Serialize, Deserialize)]
            #[builder(doc)]
            #[serde(deny_unknown_fields, rename_all = "camelCase")]
            pub struct #name {
                #( #fields ),*
            }

            #implementation
        }
    }
    pub fn codegen_roles(&self) -> TokenStream {
        let name = format_ident!("{}", self.type_name());
        let fields = self.fields.iter().map(|x| x.codegen_enum());
        let constructors = self.fields.iter().map(|x| {
            let ident = format_ident!("{}", x.ident());
            let type_name = format_ident!("{}", x.type_name());
            let doc = format!("Generates new {}", x.type_name());

            quote! {
                #[doc = #doc]
                pub fn #ident() -> Self{
                    Self:: #type_name ( #type_name::builder().build() )
                }
            }
        });
        let doc = mk_doc(&self.doc);
        quote! {
            #doc
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            #[serde(deny_unknown_fields)]
            pub enum #name {
                #( #fields ),*
            }

            impl #name {
                #( #constructors )*
            }
        }
    }
}
