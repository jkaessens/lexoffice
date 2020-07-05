use lexoffice::model::Page;
use lexoffice::request::Request;
use lexoffice::request::{Endpoint, Paginated};
use lexoffice::Result;
use serde::de::DeserializeOwned;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct PaginatedOpt {
    /// page to retrieve
    #[structopt(short, long)]
    page: Option<usize>,
    /// number of items in a page
    #[structopt(short, long)]
    size: Option<usize>,
}

impl PaginatedOpt {
    pub async fn exec<T, U>(&self, request: Request<T, U>) -> Result<Page<T>>
    where
        Request<T, U>: Paginated + Endpoint + Send + Sync + Clone,
        T: DeserializeOwned + Send + Sync + Clone + 'static,
        U: Send + Sync + Clone + 'static,
    {
        let page = self.page.unwrap_or(0);
        if let Some(size) = self.size {
            request.page_size(page, size).await
        } else {
            request.page(page).await
        }
    }
}
