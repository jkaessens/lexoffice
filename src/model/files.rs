use crate::result::Result;
use bytes::Bytes;
use futures::Stream;
use mime::Mime;
use serde::{Deserialize, Serialize};
use std::pin::Pin;
use typed_builder::TypedBuilder;

pub enum FileContent {
    Bytes(Bytes),
    Stream(Pin<Box<dyn Stream<Item = Result<Bytes>> + Send + Sync>>),
}

impl std::fmt::Debug for FileContent {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Bytes(b) => b.fmt(f),
            Self::Stream(_) => write!(f, "Stream"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum TypeEnum {
    Voucher,
}

impl Default for TypeEnum {
    fn default() -> Self {
        Self::Voucher
    }
}

#[derive(Debug, TypedBuilder)]
pub struct File {
    pub file: FileContent,
    #[builder(default)]
    pub type_: TypeEnum,
    pub mime: Mime,
}
