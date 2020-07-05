mod actions;
mod resources;

use lexoffice::client::*;
use lexoffice::model::Page;
use lexoffice::request::ResultInfo;
use resources::*;
use serde::Serialize;
use serde_any::{to_writer_pretty, Format};
use structopt::StructOpt;

pub enum ReturnType<T> {
    Paged(Page<T>),
    ResultInfo(ResultInfo<T>),
    Obj(T),
    Empty,
}

const OUTPUT_VARIANTS: &[&str] = &["yaml", "json", "toml"];

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(flatten)]
    sub_opt: SubOpt,
    #[structopt(short, long, possible_values = OUTPUT_VARIANTS, case_insensitive = true, default_value = "yaml")]
    output: Format,
}

#[derive(Debug, StructOpt)]
enum SubOpt {
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

impl Opt {
    fn serialize<T: Serialize>(
        &self,
        obj: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let stdout = std::io::stdout();
        let out = stdout.lock();

        to_writer_pretty(out, &obj, self.output).unwrap();
        Ok(())
    }

    async fn out<T>(
        &self,
        result: ReturnType<T>,
    ) -> Result<(), Box<dyn std::error::Error>>
    where
        T: serde::Serialize,
    {
        match result {
            ReturnType::Paged(x) => self.serialize(&x)?,
            ReturnType::Obj(x) => self.serialize(&x)?,
            ReturnType::ResultInfo(x) => self.serialize(&x)?,
            ReturnType::Empty => return Ok(()),
        }
        println!();

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let api_key = match ApiKey::try_default().await {
        Err(lexoffice::Error::FailedToLoadApiKey) => {
            eprintln!("Please generate a new API-KEY here:\n");
            eprintln!("    https://app.lexoffice.de/settings/#/public-api\n");
            eprintln!("Then place it in $HOME/.lexoffice:\n");
            eprintln!("    echo API-KEY > ~/.lexoffice\n");
            std::process::exit(1);
        }
        x => x,
    };
    let client = Client::new(api_key?);
    match &opt.sub_opt {
        SubOpt::Contact(x) => opt.out(x.exec(client).await?).await,
        SubOpt::CreditNote(x) => opt.out(x.exec(client).await?).await,
        SubOpt::EventSubscription(x) => opt.out(x.exec(client).await?).await,
        SubOpt::Invoice(x) => opt.out(x.exec(client).await?).await,
        SubOpt::OrderConfirmation(x) => opt.out(x.exec(client).await?).await,
        SubOpt::Quotation(x) => opt.out(x.exec(client).await?).await,
        SubOpt::Profile(x) => opt.out(x.exec(client).await?).await,
        SubOpt::File(x) => Ok(x.exec(client).await?),
        SubOpt::Voucherlist(x) => opt.out(x.exec(client).await?).await,
        SubOpt::Voucher(x) => opt.out(x.exec(client).await?).await,
    }
}
