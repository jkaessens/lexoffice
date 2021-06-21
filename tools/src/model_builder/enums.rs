use crate::model_builder::utils::mk_doc;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use scraper::ElementRef;
use scraper::Selector;
use std::collections::BTreeMap;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct ModelEnum {
    variants: BTreeMap<String, Option<String>>,
    name: String,
    pub doc: Option<String>,
}

impl ModelEnum {
    pub fn create(name: String) -> Self {
        Self {
            name,
            variants: BTreeMap::new(),
            doc: None,
        }
    }

    pub fn type_name(&self) -> String {
        string_morph::to_pascal_case(self.name.as_str())
    }

    pub fn parse_extra(&mut self, table: &ElementRef) {
        let tbody_selector = Selector::parse("tbody > tr").unwrap();

        for field in table.select(&tbody_selector) {
            let mut children = field.children().filter_map(ElementRef::wrap);
            let _resource = children.next().unwrap();
            let event_type = children.next().unwrap();
            let description = children.next().unwrap();

            self.variants.insert(
                event_type.text().collect::<String>(),
                Some(description.inner_html()),
            );
        }
    }

    pub fn parse_description(&mut self, description: &ElementRef) {
        let strong_selector = Selector::parse("strong").unwrap();

        self.variants = description
            .select(&strong_selector)
            .map(|x| (x.text().collect::<String>(), None))
            .collect();
    }

    pub fn codegen(&self) -> Option<TokenStream> {
        println!("VAR=>{:?}", self.variants);
        let variants = self.variants.iter().filter_map(|x| {
            let name = string_morph::to_pascal_case(&x.0);
            if name.contains("WillSoonAllowAnyValue") {
                return None;
            }
            let name = format_ident!("{}", name);
            let doc = mk_doc(&x.1.clone().unwrap_or_default());
            let org_name = x.0.clone();
            Some(quote! {
                #doc
                #[serde(rename = #org_name)]
                #name
            })
        });
        let type_name = format_ident!("{}", self.type_name());
        let doc = mk_doc(&self.doc.clone().unwrap_or_default());
        Some(quote! {
            #doc
            #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
            #[serde(deny_unknown_fields)]
            pub enum #type_name {
                #( #variants ),*
            }

            impl std::str::FromStr for #type_name {
                type Err = serde_plain::Error;

                fn from_str(s: &str) -> Result<Self, Self::Err> {
                    serde_plain::from_str::<Self>(s)
                }
            }
        })
    }
}
