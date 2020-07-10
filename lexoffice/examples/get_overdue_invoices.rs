use lexoffice::{ApiKey, Client};
use lexoffice::model::voucherlist::*;
use lexoffice::model::Voucherlist;
use tokio::stream::StreamExt;

use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new(ApiKey::try_default().await?);
    let mut voucher_list = client
        .request::<Voucherlist>()
        .type_(&VoucherType::Invoice)
        .status(&VoucherStatus::Open)
        .stream();

    while let Some(voucher) = voucher_list.next().await {
        let voucher = voucher?;
        println!(
            "{}: {}",
            voucher.contact_name.unwrap_or("Unknown".to_string()),
            voucher.voucher_number.unwrap_or("Unknown".to_string())
        );
    }

    Ok(())
}
