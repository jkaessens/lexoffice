use lexoffice::client::{ApiKey, Client};
use lexoffice::model::File;
use lexoffice::request::files::FilesRequest;
use lexoffice::Result;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new(ApiKey::try_default().await?);
    let args = std::env::args().collect::<Vec<_>>();
    let file = Path::new(&args[1]);
    let response = client.request::<File>().upload(file).await?;
    println!("{:#?}", response);
    Ok(())
}
