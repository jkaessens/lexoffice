use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use scraper::element_ref::ElementRef;
use scraper::node::Node;
use scraper::Html;
use scraper::Selector;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::iter;
use string_morph::*;
use ::rustfmt;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord)]
enum FieldDef {
    List(String, Option<String>),
    Object(String, Option<String>),
    Enum(String, Vec<String>),
    Primitive(String, String),
}

impl FieldDef {
    fn handle_primitive(
        label: &str,
        type_def: String,
        description: &ElementRef,
    ) -> FieldDef {
        let strong_selector = Selector::parse("strong").unwrap();

        let label = string_morph::to_upper_first(&label.trim()).to_string();
        if label == "countryCode" {
            FieldDef::Object(
                label,
                Some("County".to_string())
            )
        }
        else if type_def == "enum" {
            FieldDef::Enum(
                label,
                description
                    .select(&strong_selector)
                    .map(|x| x.text().collect::<String>())
                    .collect::<Vec<_>>(),
            )
        } else {
            FieldDef::Primitive(label, type_def.to_string())
        }
    }

    fn handle_complex(
        label: &str,
        type_def: &str,
        definition: &ElementRef,
        description: &ElementRef,
    ) -> Result<FieldDef, Box<dyn Error>> {
        let a_selector = Selector::parse("a").unwrap();

        let label = label.to_string();

        let mut field_type = definition
            .select(&a_selector)
            .chain(description.select(&a_selector))
            .filter_map(|x| x.value().attr("href"))
            .filter(|x| x.starts_with("#"))
            .map(|x| x.to_string())
            .next();

        if field_type.is_none()
            && description
                .text()
                .collect::<String>()
                .contains("type string")
        {
            field_type = Some("string".to_string());
        }

        match type_def {
            "list" => Ok(FieldDef::List(label, field_type)),
            "object" => Ok(FieldDef::Object(label, field_type)),
            _ => Err("Unknown Complex type".into()),
        }
    }

    fn handle(
        definition: &ElementRef,
        description: &ElementRef,
    ) -> Result<FieldDef, Box<dyn Error>> {
        let name_field = definition.first_child().unwrap();
        let type_def = definition
            .children()
            .filter_map(ElementRef::wrap)
            .last()
            .unwrap()
            .text()
            .collect::<String>();

        match name_field.value() {
            Node::Text(label) => {
                if ["object", "list"].contains(&type_def.as_str()) {
                    Self::handle_complex(
                        label.trim(),
                        &type_def,
                        &definition,
                        &description,
                    )
                } else {
                    Ok(Self::handle_primitive(&label.trim(), type_def, description))
                }
            }
            Node::Element(_) => {
                let label = name_field
                    .children()
                    .filter_map(|x| x.value().as_text())
                    .next()
                    .unwrap().trim();
                Self::handle_complex(
                    label,
                    &type_def,
                    &definition,
                    &description,
                )
            }
            _ => Err("Unexpected Node Type".into()),
        }
    }

    fn field_type(&self) -> TokenStream {
        let field_type = match self {
            Self::List(_, Some(field_type)) => field_type.clone(),
            Self::Object(_, Some(field_type)) => field_type.clone(),
            Self::List(name, None) => name.to_owned() + " Details",
            Self::Object(name, None) => name.to_owned() + " Details",
            Self::Enum(name, _) => name.clone(),
            Self::Primitive(_, field_type) => match field_type.as_str() {
                "string" => "String",
                "number" => "f64",
                "integer" => "i64",
                "boolean" => "bool",
                "uuid" => "Uuid",
                _ => panic!("Unknown type: {}", field_type),
            }
            .to_string(),
        };
        let var_type = format_ident!("{}", field_type);
        quote! {}
    }

    fn generate_list(
        &self,
        parser: &ModuleParser,
        name: &String,
        field_type: &String,
    ) -> TokenStream {
        let var_type = if field_type.starts_with("#") {
            let struct_def = parser.modules.iter()
                .flat_map(|x| x.structs_by_id.get(field_type))
                .next()
                .expect(&format!("Can't find field {}", field_type));
            format_ident!("{}", struct_def.source_name())
        } else if field_type == "string" {
            format_ident!("String")
        } else {
            format_ident!("{}", field_type)
        };
        let var_name = format_ident!("{}", string_morph::to_snake_case(name));

        quote! {
            #var_name: Vec<#var_type>
        }
    }

    fn generate_object(
        &self,
        parser: &ModuleParser,
        name: &String,
        field_type: &String,
    ) -> TokenStream {
        //let var_type = format_ident!("{}", field_type);
        let var_type = if field_type.starts_with("#") {
            let struct_def = parser.modules.iter()
                .flat_map(|x| x.structs_by_id.get(field_type))
                .next()
                .expect(&format!("Can't find field {}", field_type));
            
            format_ident!("{}", struct_def.source_name())
        } else {
            format_ident!("{}", field_type)
        };
        let var_name = format_ident!("field_{}", string_morph::to_snake_case(name));
        quote! {
            #var_name: #var_type
        }
    }

    fn generate_enum(
        &self,
        parser: &ModuleParser,
        name: &String,
        values: &Vec<String>,
    ) -> TokenStream {
        let var_type = format_ident!("{}Enum", string_morph::to_upper_first(name));
        let var_name = format_ident!("field_{}", string_morph::to_snake_case(name));
        quote!{
            #var_name: #var_type
        }
    }

    fn generate_primitive(
        &self,
        parser: &ModuleParser,
        name: &String,
        field_type: &String,
    ) -> TokenStream {
        let var_type = match field_type.as_str() {
            "string" => "String",
            "number" => "f64",
            "integer" => "i64",
            "boolean" => "bool",
            "uuid" => "Uuid",
            "dateTime" => "DateTime<Utc>",
            _ => panic!("Unknown type: {}", field_type),
        };
        let var_name = format_ident!("field_{}", string_morph::to_snake_case(name));
        let type_stream: proc_macro2::TokenStream = var_type.parse().unwrap();
        quote! {
            #var_name: #type_stream
        }
    }

    fn get_enum_def(&self) -> TokenStream {
        match self {
            Self::Enum(name, values) => {
                if name.as_str() == "countryCode" {
                    return quote!{};
                }
                let name = format_ident!("{}Enum", name);
                let values = values.iter()
                    .map(|x| format_ident!("{}", string_morph::to_upper_first(x))).collect::<Vec<_>>();

                quote!{
                    #[derive(Debug, Clone, PartialEq)]
                    enum #name {
                        #(#values,)*
                    }
                }
            }
            _ => panic!()
        }
    }

    fn generate(&self, parser: &ModuleParser) -> TokenStream {
        let field = match self {
            Self::List(name, Some(field_type)) => {
                self.generate_list(parser, name, field_type)
            }
            Self::Object(name, Some(field_type)) => {
                self.generate_object(parser, name, field_type)
            }
            Self::Object(name, None) => {
                let mut myType = string_morph::to_upper_first(name);
                myType.push_str("Details");
                self.generate_object(parser, name, &myType)
            }
            Self::List(name, None) => {
                let mut myType = string_morph::to_upper_first(name);
                myType.push_str("Details");
                self.generate_object(parser, name, &myType)
            }
            Self::Enum(name, values) => self.generate_enum(parser, name, values),
            Self::Primitive(name, field_type) => {
                self.generate_primitive(parser, name, field_type)
            }
            _ => panic!("unhandled: {:?}", self)
        };

        quote! {
            #field,
        }
    }
}

#[derive(Debug, Clone)]
struct StructDef {
    name: String,
    id: Option<String>,
    fields: Vec<FieldDef>,
}

impl StructDef {
    fn new(name: String, id: Option<String>) -> Self {
        Self {
            fields: vec![],
            name,
            id,
        }
    }

    fn parse(self: &mut Self, table: ElementRef) {
        let tr_selector = Selector::parse("tr").unwrap();
        let td_selector = Selector::parse("td").unwrap();

        let mut fields = table
            .select(&tr_selector)
            .map(|x| x.select(&td_selector).collect::<Vec<_>>())
            .filter(|x| x.len() != 0)
            .filter(|x| x[0].value().name() == "td")
            .filter_map(|td| FieldDef::handle(&td[0], &td[1]).ok())
            .collect::<Vec<_>>();
        self.fields.append(&mut fields);
    }

    fn source_name(&self) -> String {
        let name = string_morph::to_upper_first(&string_morph::to_camel_case(
            &self.name,
        ));
        let suffix = "_endpoint";
        if name.ends_with(suffix) {
            name[..name.len() - suffix.len()].to_string()
        } else {
            name
        }
    }

    fn generate(&self, parser: &ModuleParser) -> TokenStream {
        let identifier = format_ident!("{}", self.source_name());
        let generates = self
            .fields
            .iter()
            .map(|x| x.generate(&parser))
            .collect::<Vec<_>>();
        quote! {
            #[derive(Debug, Clone, PartialEq)]
            struct #identifier {
                #(#generates)*
            }
        }
    }
}

#[derive(Debug, Clone)]
struct ModuleDef {
    name: String,
    structs: Vec<StructDef>,
    structs_by_id: HashMap<String, StructDef>,
}

impl ModuleDef {
    fn new(name: String) -> Self {
        Self {
            structs_by_id: HashMap::new(),
            structs: vec![],
            name,
        }
    }

    fn parse(self: &mut Self, iter: &mut dyn Iterator<Item = ElementRef>) {
        let thead_selector = Selector::parse("thead").unwrap();
        let mut name = None;
        let mut id = None;
        for elem in iter {
            match elem.value().name() {
                "h1" | "h2" => {
                    let text = elem.text().collect::<String>().trim().to_string();
                    println!("{:?}", text);
                    name = Some(if text.starts_with("Object") {
                        text[6..].to_string()
                    } else {
                        text
                    });
                    if let Some(elem_id) = elem.value().id() {
                        id = Some(elem_id);
                    }
                }
                "div" => {
                    if let Some(elem_id) = elem.value().id() {
                        id = Some(elem_id);
                    }
                }
                "p" => {
                    let children = elem.children().collect::<Vec<_>>();
                    if children.len() == 1 {
                        if let Some(elem) = ElementRef::wrap(children[0]) {
                            if elem.value().name() != "strong" {
                                continue;
                            }
                            let text = elem.text().collect::<String>().trim().to_string();
                            println!("{:?}", text);
                            name = Some(if text.starts_with("Object") {
                                text[6..].to_string()
                            } else {
                                text
                            });
                        }
                    }
                }
                "table" => {
                    let thead = elem.select(&thead_selector).next().unwrap();
                    let title =
                        thead.text().collect::<String>().trim().to_owned();

                    if title == "Property\nDescription" {
                        let mut struct_def = StructDef::new(
                            name.unwrap(),
                            id.map(str::to_string),
                        );
                        struct_def.parse(elem);
                        self.structs
                            .push(struct_def.clone());
                        if let Some(id) = id {
                            let id = format!("#{}", id);
                            self.structs_by_id.insert(id, struct_def);
                        }
                    }
                    id = None;
                    name = None;
                }
                _ => {}
            }
        }
    }

    fn generate(&self, parser: &ModuleParser) -> TokenStream {
        let identifier = format_ident!("{}", self.source_name());
        let generates = self
            .structs
            .iter()
            .map(|x| x.generate(parser))
            .collect::<Vec<_>>();
        let mut enums = self.structs.iter().flat_map(|x| x.fields.iter())
            .collect::<Vec<_>>();
        enums.sort();
        enums.dedup();
        let enums = enums.iter()
            .filter_map(|x| match x {
                FieldDef::Enum(_, _) => Some(x.get_enum_def()),
                _ => None,
            }).collect::<Vec<_>>();
        quote! {
            mod #identifier {
                use uuid::Uuid;
                use chrono::{DateTime,Utc};

                #(#enums)*
                #(#generates)*
            }
        }
    }

    fn source_name(&self) -> String {
        let name = string_morph::to_snake_case(&self.name);
        let suffix = "_endpoint";
        if name.ends_with(suffix) {
            name[..name.len() - suffix.len()].to_string()
        } else {
            name
        }
    }
}

#[derive(Debug, Clone)]
struct ModuleParser {
    modules: Vec<ModuleDef>,
}

impl ModuleParser {
    fn new() -> Self {
        Self { modules: vec![] }
    }

    fn parse(self: &mut Self, document: &Html) -> Result<(), Box<dyn Error>> {
        let h1_selector = Selector::parse("h1").unwrap();
        let headings = document.select(&h1_selector);

        for heading in headings {
            if heading.value().id().unwrap_or("").ends_with("-endpoint")
                == false
            {
                continue;
            }
            let content_iterator = heading
                .next_siblings()
                .filter_map(ElementRef::wrap)
                .take_while(|x| x.value().name() != "h1");
            let name = heading.text().collect::<String>();
            let mut parser = ModuleDef::new(name);
            parser.parse(&mut iter::once(heading).chain(content_iterator));
            self.modules.push(parser);
        }
        Ok(())
    }

    fn generate(self) -> String {
        let generates = self
            .modules
            .iter()
            .map(|x| x.generate(&self))
            .collect::<Vec<_>>();
        let result = quote! {
            mod lexoffice {
                #(#generates)*
            }
        };
        result.to_string()
    }
}

fn load_docs() -> Result<String, Box<dyn std::error::Error>> {
    let url = "https://developers.lexoffice.io/docs/";
    let file_name = "assets/docs.html";
    match File::open(file_name) {
        Ok(mut file) => {
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            Ok(contents)
        }
        Err(_) => {
            let contents = reqwest::blocking::get(url)?.text()?;
            let mut file = File::create(file_name)?;
            file.write_all(contents.as_bytes())?;
            Ok(contents)
        }
    }
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let html = load_docs()?;

    let document = Html::parse_document(&html);
    let mut parser = ModuleParser::new();
    parser.parse(&document)?;
    let source = rustfmt::Input::Text(parser.generate());
    let mut out: Vec<u8> = Vec::new();
    let result = rustfmt::format_input(source, &Default::default(), Some(&mut out));
    let src = &result.unwrap().1[0].1;

    let mut file = File::create("src/lib.rs")?;
    let content = src.chars().map(|(c, _)| c).collect::<String>();
    file.write_all(content.as_bytes())?;




    Ok(())
}
