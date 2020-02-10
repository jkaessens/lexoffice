use bytes::Bytes;
use futures::Stream;
use serde::{Deserialize, Serialize};
use tokio::io::AsyncRead;
use typed_builder::TypedBuilder;

pub enum FileContent {
    Bytes(Bytes),
    Stream(Box<dyn Stream<Item = reqwest::Result<Bytes>> + Send + Sync>),
    AsyncRead(Box<dyn AsyncRead + Send + Sync>),
}

impl std::fmt::Debug for FileContent {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        match self {
            Self::Bytes(b) => b.fmt(f),
            Self::Stream(_) => write!(f, "Stream"),
            Self::AsyncRead(_) => write!(f, "AsyncRead"),
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
    file: FileContent,
    type_: TypeEnum,
}
