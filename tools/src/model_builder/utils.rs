use html2md::StructuredPrinter;
use html2md::TagHandler;
use html2md::{Handle, NodeData, TagHandlerFactory};
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use url::Url;

#[derive(Default)]
struct AnchorHandlerFactory {}
impl TagHandlerFactory for AnchorHandlerFactory {
    fn instantiate(
        &self,
    ) -> std::boxed::Box<(dyn html2md::TagHandler + 'static)> {
        Box::new(AnchorHandler::default())
    }
}

#[derive(Default)]
struct AnchorHandler {
    start_pos: usize,
    url: String,
}

impl TagHandler for AnchorHandler {
    fn handle(&mut self, tag: &Handle, printer: &mut StructuredPrinter) {
        self.start_pos = printer.data.len();

        // try to extract a hyperlink
        self.url = match tag.data {
            NodeData::Element { ref attrs, .. } => {
                let attrs = attrs.borrow();
                let href = attrs
                    .iter()
                    .find(|attr| attr.name.local.to_string() == "href");
                match href {
                    Some(link) => link.value.to_string(),
                    None => String::new(),
                }
            }
            _ => String::new(),
        };
    }

    fn after_handle(&mut self, printer: &mut StructuredPrinter) {
        let url = Url::parse(&self.url);

        let url = if let Ok(url) = url {
            url
        } else {
            Url::parse("https://developers.lexoffice.io/docs/")
                .unwrap()
                .join(&self.url)
                .unwrap()
        };

        printer.insert_str(self.start_pos, "[");
        printer.append_str(&format!("]({})", url))
    }
}

pub trait StringUtils {
    fn remove_suffix(&self, suffix: &'static str) -> String;
    fn remove_prefix(&self, prefix: &str) -> String;
}

impl StringUtils for String {
    fn remove_suffix(&self, suffix: &'static str) -> String {
        self.as_str().remove_suffix(suffix)
    }

    fn remove_prefix(&self, prefix: &str) -> String {
        self.as_str().remove_prefix(prefix)
    }
}

impl StringUtils for &str {
    fn remove_suffix(&self, suffix: &'static str) -> String {
        if self.to_lowercase().ends_with(&suffix.to_lowercase()) {
            self.split_at(self.len() - suffix.len()).0
        } else {
            self
        }
        .to_string()
    }

    fn remove_prefix(&self, prefix: &str) -> String {
        if self.to_lowercase().starts_with(&prefix.to_lowercase()) {
            self.split_at(prefix.len()).1
        } else {
            self
        }
        .to_string()
    }
}

pub fn mk_doc(html: &str) -> TokenStream {
    let mut tag_handlers: HashMap<String, Box<dyn TagHandlerFactory>> =
        HashMap::new();
    tag_handlers.insert("a".to_string(), Box::new(AnchorHandlerFactory {}));

    let md = html2md::parse_html_custom(html, &tag_handlers)
        .trim()
        .to_string();
    if md == "" {
        quote! {}
    } else {
        quote! { #[doc = #md] }
    }
}
