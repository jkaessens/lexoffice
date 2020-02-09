use lexoffice::client::{ApiKey, Client};
use lexoffice::model::VoucherList;
use lexoffice::model::voucher_list::*;
use lexoffice::request::Paginated;
use lexoffice::request::voucher_list::*;

use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new(ApiKey::try_default().await?);
    let voucher_list = client.request::<VoucherList>()
        .voucher_type(VoucherTypeEnum::Invoice)
        .voucher_status(VoucherStatusEnum::Open).page(0).await?;
    println!("{:#?}", voucher_list);
    Ok(())
}
