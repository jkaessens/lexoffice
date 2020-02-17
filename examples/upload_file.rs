use lexoffice::client::{ApiKey, Client};
use lexoffice::fs::ApiKeyFromFile;
use lexoffice::fs::FilesUpload;
use lexoffice::model::File;
use lexoffice::Result;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new(ApiKey::try_default().await?);
    let args = std::env::args().collect::<Vec<_>>();
    let path = Path::new(&args[1]);
    let response = client.request::<File>().upload_path(&path).await?;
    println!("{:#?}", response);
    Ok(())
}
