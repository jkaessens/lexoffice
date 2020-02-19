use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::ops::Deref;
use uuid::Uuid;

/// This is a struct that wraps classes returned by the server.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ServerResource<T> {
    pub id: Uuid,
    pub version: u64,

    #[serde(flatten)]
    pub(crate) inner: T,
}

impl<T> Deref for ServerResource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> ServerResource<PhantomData<T>> {
    pub(crate) fn wrap(self, inner: T) -> ServerResource<T> {
        ServerResource {
            id: self.id,
            version: self.version,
            inner,
        }
    }
}
