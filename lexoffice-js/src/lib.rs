mod utils;

use lexoffice::model::*;
use lexoffice::request::Request;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct Client {
    client: lexoffice::Client,
}

fn to_js_err<T, E>(result: Result<T, E>) -> Result<T, JsValue>
where
    E: std::fmt::Debug,
{
    match result {
        Ok(x) => Ok(x),
        Err(err) => {
            let err_str = format!("{:?}", err);
            // TODO: use proper Error object
            Err(JsValue::from_str(&err_str))
        }
    }
}

#[macro_export]
macro_rules! request {
    ( $x:ident, $y:ident $(, $z:ident)* ) => {
        paste::item! {
            #[wasm_bindgen]
            pub struct [<$x Request>] {
                inner: Request<$x>,
            }
            impl From<Request<$x>> for [<$x Request>] {
                fn from(inner: Request<$x>) -> Self {
                    Self{ inner }
                }
            }
            impl Into<Request<$x>> for [<$x Request>] {
                fn into(self) -> Request<$x> {
                    self.inner
                }
            }
            #[wasm_bindgen]
            impl Client {
                pub fn $y(&self) -> [<$x Request>] {
                    self.client.request::<$x>().into()
                }
            }
        }
    };
}

#[macro_export]
macro_rules! request_impl {
    (by_id for $x:ident) => {
        paste::item! {
            #[wasm_bindgen]
            impl [<$x Request>] {
                pub async fn update(self, obj: JsValue) -> Result<JsValue, JsValue> {
                    let obj = to_js_err(obj.into_serde::<$x>())?;
                    let result = self.inner.update(obj).await;
                    let result = to_js_err(result)?;
                    to_js_err(JsValue::from_serde(&result))
                }
            }
        }
    };
    (paginated for $x:ident) => {
        paste::item! {
            #[wasm_bindgen]
            impl [<$x Request>] {
                pub async fn page(self, page: usize, size: Option<usize>) -> Result<JsValue, JsValue> {
                    let result = if let Some(size) = size {
                        self.inner.page_size(page, size).await
                    } else {
                        self.inner.page(page).await
                    };
                    let result = to_js_err(result)?;
                    to_js_err(JsValue::from_serde(&result))
                }
            }
        }
    };
}

request!(Contact, contact);
request_impl!(by_id for Contact);
request_impl!(paginated for Contact);

#[wasm_bindgen]
pub fn client(api_key: &str) -> Client {
    let client = lexoffice::Client::new(api_key);
    Client { client }
}
