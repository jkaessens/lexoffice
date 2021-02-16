use inflector::string::singularize::to_singular;
use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use scraper::ElementRef;
use scraper::Html;
use scraper::Selector;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use tools::model_builder::io;
use tools::model_builder::io::write_token_stream;
use tools::model_builder::modules::ModelModule;
use tools::model_builder::result::Result;

fn create_mod_rs(modules: &[ModelModule]) -> TokenStream {
    let reexports = modules
        .iter()
        .map(|x| {
            format!(
                "{}::{}",
                x.type_name(),
                to_singular(&string_morph::to_pascal_case(&x.type_name()))
            )
        })
        .map(|x| TokenStream::from_str(&x).unwrap())
        .collect::<Vec<_>>();
    let pub_mod = modules
        .iter()
        .map(|x| format_ident!("{}", x.type_name()))
        .collect::<Vec<_>>();
    quote! {
        #![allow(missing_docs)]

        //! This model was semi-automaticly generated from The official lexoffice
        //! documentation
        //!
        //! See <https://developers.lexoffice.io/docs/> for more information

        pub mod pages;
        #(pub mod #pub_mod; )*
        pub use pages::Page;
        #(pub use #reexports; )*
    }
}

fn main() -> Result<()> {
    let content_selector = Selector::parse("div.content > h1").unwrap();

    let html = io::load_docs()?;
    let document = Html::parse_document(&html);
    let mut modules = vec![];
    let dir = PathBuf::from_str(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../lexoffice/src/model"
    ))?;
    let _ = fs::create_dir(&dir);

    for header in document.select(&content_selector) {
        if !header.text().collect::<String>().ends_with(" Endpoint") {
            continue;
        }
        println!("   {}", header.text().collect::<String>());
        let section = header
            .next_siblings()
            .filter_map(ElementRef::wrap)
            .take_while(|x| {
                let text = x.text().collect::<String>().trim().to_string();
                x.value().name() != "h1"
                    || text.to_lowercase().ends_with(" properties")
                    || text == "Purpose"
                //if x.value().name() == "h1" {
                //    false
                //} else if x.value().name() == "h2"
                //    && !x.text().collect::<String>().ends_with("Properties")
                //{
                //    false
                //} else {
                //    false
                //}
            })
            .collect::<Vec<_>>();

        let mut module = ModelModule::create(header);
        module.parse_sections(section);
        modules.push(module);
    }

    let mut path = dir.clone();
    path.push("mod.rs");
    write_token_stream(&path, create_mod_rs(&modules))?;

    for module in modules {
        let mut path = dir.clone();
        path.push(format!("{}.rs", module.type_name()));
        write_token_stream(&path, module.codegen())?
    }
    Ok(())
}
