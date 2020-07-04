mod resources;
mod actions;

use structopt::StructOpt;
use resources::ContactOpt;
use lexoffice::model::Page;
use lexoffice::client::*;

pub enum ReturnType<T> {
    Paged(Page<T>),
    Obj(T),
}

#[derive(Debug, StructOpt)]
enum Opt {
    Contact(ContactOpt),
    //CreditNotes(CreditNotesOpt),
    //EventSubscription(EventSubscriptionOpt),
    //File(FileOpt),
    //Invoice(InvoiceOpt),
    //OrderConfirmation(OrderConfirmationOpt),
    //Profile(ProfileOpt),
    //Quotation(QuotationOpt),
    //VoucherList(VoucherList),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let client = Client::new(ApiKey::try_default().await?);
    let result = match opt {
        Opt::Contact(x) => x.exec(client).await
    };
    let stdout = std::io::stdout();

    match result? {
        ReturnType::Paged(x) => serde_json::to_writer_pretty(stdout.lock(), &x)?,
        ReturnType::Obj(x) => serde_json::to_writer_pretty(stdout.lock(), &x)?
    }

    Ok(())
}
