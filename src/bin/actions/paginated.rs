use lexoffice::request::Request;
use lexoffice::request::{Endpoint, Paginated};
use lexoffice::model::Page;
use lexoffice::Result;
use serde::de::DeserializeOwned;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct PaginatedOpt {
    #[structopt(short, long)]
    page: Option<usize>,
    #[structopt(short, long)]
    size: Option<usize>,
}

impl PaginatedOpt {
    pub async fn exec<T>(self, request: Request<T, ()>) -> Result<Page<T>>
    where
        Request<T, ()>: Paginated + Endpoint + Send + Sync + Clone,
        T: DeserializeOwned + Send + Sync + 'static,
    {
        let page = self.page.unwrap_or(0);
        if let Some(size) = self.size {
            request.page_size(page, size).await
        } else {
            request.page(page).await
        }
    }
}
