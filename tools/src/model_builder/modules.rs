use crate::model_builder::enums::ModelEnum;
use crate::model_builder::structs::ModelStruct;
use inflector::string::singularize::to_singular;
use proc_macro2::TokenStream;
use quote::quote;
use scraper::ElementRef;
use std::collections::BTreeSet;

#[derive(Debug)]
pub struct ModelModule {
    pub id: String,
    doc: String,
    structs: Vec<ModelStruct>,
    extra_enums: Vec<ModelEnum>,
}

impl ModelModule {
    pub fn create(header: ElementRef) -> Self {
        let id = header.value().id().unwrap().to_string();
        assert!(id.ends_with("-endpoint"));

        let doc = header
            .next_siblings()
            .filter_map(ElementRef::wrap)
            .skip_while(|x| x.value().name() != "p")
            .take_while(|x| x.value().name() == "p")
            .map(|x| x.html())
            .collect::<String>()
            .replace("\n", " ");

        ModelModule {
            id,
            doc,
            structs: vec![],
            extra_enums: vec![],
        }
    }

    pub fn type_name(&self) -> String {
        let name = self.id.clone();
        assert!(name.ends_with("-endpoint"));
        let (name, _) = name.split_at(name.len() - "-endpoint".len());
        string_morph::to_snake_case(&name)
    }

    pub fn parse_extra_enums(
        &mut self,
        name: String,
        section: Vec<ElementRef>,
    ) {
        let mut iter = section.iter();
        let mut extra_enum =
            crate::model_builder::enums::ModelEnum::create(to_singular(&name));
        while let Some(element) = iter.next() {
            if element.value().name() == "p" {
                extra_enum.doc = Some(element.inner_html());
            }
            if element.value().name() != "table" {
                continue;
            }

            extra_enum.parse_extra(element);
            self.extra_enums.push(extra_enum);

            break;
        }
    }

    pub fn parse_sections(&mut self, section: Vec<ElementRef>) {
        let mut iter = section.iter();
        while let Some(element) = iter.next() {
            let html = element.inner_html();
            if element.value().name() == "h2"
                && html.to_lowercase().contains(" properties") == false
                && html.contains("Purpose") == false
            {
                if "Event Types" == element.text().collect::<String>().trim() {
                    self.parse_extra_enums(
                        "EventType".into(),
                        iter.clone().map(|x| x.to_owned()).collect::<Vec<_>>(),
                    );
                } else {
                    break;
                }
            }

            let name = if html.starts_with("<strong>")
                && (html.ends_with(" Details</strong>")
                    || html.ends_with(" Properties</strong>"))
                && html.contains(" Required ") == false
            {
                element.text().collect::<String>()
            } else if element.value().id().is_some()
                && element.value().name() == "h2"
            {
                let parts = element
                    .value()
                    .id()
                    .unwrap()
                    .split('-')
                    .collect::<Vec<_>>();
                if parts.contains(&"properties") || parts.contains(&"details") {
                    element.text().collect()
                } else {
                    continue;
                }
            } else {
                continue;
            };

            let mut model_struct = ModelStruct::create(name, &self);
            let sub_section = iter
                .clone()
                .take_while(|x| x.value().id().is_none())
                .map(|x| x.to_owned())
                .collect::<Vec<_>>();
            model_struct.parse_section(sub_section);

            let create_info = iter
                .clone()
                .skip_while(|x| {
                    x.value().id().unwrap_or("").contains("-create-a-") == false
                })
                .take_while(|x| {
                    x.value().name() != "h2"
                        || x.value().id().unwrap_or("").contains("-create-a-")
                })
                .collect::<Vec<_>>();
            model_struct.parse_creation_info(create_info);
            self.structs.push(model_struct);
        }
    }

    pub fn collect_enums(&self) -> BTreeSet<&ModelEnum> {
        self.structs
            .iter()
            .flat_map(|x| x.collect_enums().into_iter())
            .collect()
    }

    pub fn codegen(&self) -> TokenStream {
        let doc = html2md::parse_html(&self.doc).trim().to_string();

        if self.type_name() == "files" {
            return quote! {
                #![doc = #doc]

                #[derive(Debug, Clone)]
                pub struct File {}
            };
        }

        let types =
            self.structs.iter().map(|x| x.codegen()).collect::<Vec<_>>();
        let enums = self
            .collect_enums()
            .iter()
            .map(|x| x.to_owned())
            .chain(self.extra_enums.iter())
            .map(|x| x.codegen())
            .collect::<Vec<_>>();
        let extra_use = if self.id == "down-payment-invoices-endpoint" {
            quote!(
                use crate::model::invoices::ShippingConditions;
            )
        } else {
            quote!()
        };
        println!("{}", self.id);
        quote! {
            #![doc = #doc]

            use serde::{Deserialize, Serialize};
            use typed_builder::TypedBuilder;
            #extra_use

            #( #enums )*
            #( #types )*
        }
    }
}
