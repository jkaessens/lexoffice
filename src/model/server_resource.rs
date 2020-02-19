use serde::{Deserialize, Serialize};
use std::marker::PhantomData;
use std::ops::Deref;
use typed_builder::TypedBuilder;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ServerResource<T> {
    pub id: Option<Uuid>,
    pub version: Option<u64>,

    #[serde(flatten)]
    pub inner: T,
}

impl<T> Deref for ServerResource<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> ServerResource<PhantomData<T>> {
    pub fn wrap(self, inner: T) -> ServerResource<T> {
        ServerResource {
            id: self.id,
            version: self.version,
            inner,
        }
    }
}
