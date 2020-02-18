use lexoffice::client::{ApiKey, Client};
use lexoffice::model::voucher_list::*;
use lexoffice::model::VoucherList;
use tokio::stream::StreamExt;

use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new(ApiKey::try_default().await?);
    let mut voucher_list = client
        .request::<VoucherList>()
        .type_(VoucherTypeEnum::Invoice)
        .status(VoucherStatusEnum::Open)
        .stream();

    while let Some(voucher) = voucher_list.next().await {
        let voucher = voucher?;
        println!("{}: {}", voucher.contact_name, voucher.voucher_number);
    }

    Ok(())
}
