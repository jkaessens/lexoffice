mod actions;
mod resources;

use lexoffice::client::*;
use lexoffice::model::Page;
use resources::*;
use structopt::StructOpt;

pub enum ReturnType<T> {
    Paged(Page<T>),
    Obj(T),
    None,
}

#[derive(Debug, StructOpt)]
enum Opt {
    Contact(ContactOpt),
    CreditNote(CreditNoteOpt),
    EventSubscription(EventSubscriptionOpt),
    File(FileOpt),
    Invoice(InvoiceOpt),
    OrderConfirmation(OrderConfirmationOpt),
    Profile(ProfileOpt),
    Quotation(QuotationOpt),
    Voucherlist(VoucherlistOpt),
    Voucher(VoucherOpt),
}

async fn out<T>(result: ReturnType<T>) -> Result<(), Box<dyn std::error::Error>>
where
    T: serde::Serialize,
{
    let stdout = std::io::stdout();
    let out = stdout.lock();

    match result {
        ReturnType::Paged(x) => serde_json::to_writer_pretty(out, &x)?,
        ReturnType::Obj(x) => serde_json::to_writer_pretty(out, &x)?,
        ReturnType::None => return Ok(()),
    }
    println!("");

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let client = Client::new(ApiKey::try_default().await?);
    match opt {
        Opt::Contact(x) => out(x.exec(client).await?).await,
        Opt::CreditNote(x) => out(x.exec(client).await?).await,
        Opt::EventSubscription(x) => out(x.exec(client).await?).await,
        Opt::Invoice(x) => out(x.exec(client).await?).await,
        Opt::OrderConfirmation(x) => out(x.exec(client).await?).await,
        Opt::Quotation(x) => out(x.exec(client).await?).await,
        Opt::Profile(x) => out(x.exec(client).await?).await,
        Opt::File(x) => Ok(x.exec(client).await?),
        Opt::Voucherlist(x) => out(x.exec(client).await?).await,
        Opt::Voucher(x) => out(x.exec(client).await?).await,
    }
}
