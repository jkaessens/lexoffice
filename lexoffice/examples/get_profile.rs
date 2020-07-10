use lexoffice::{ApiKey, Client};
use lexoffice::model::Profile;

use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new(ApiKey::try_default().await?);
    let voucher_list = client.request::<Profile>().get().await?;
    println!("{:#?}", voucher_list);
    Ok(())
}
