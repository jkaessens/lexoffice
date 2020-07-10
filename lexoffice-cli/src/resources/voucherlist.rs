use crate::actions::*;
use crate::ReturnType;
use lexoffice::Client;
use lexoffice::model::voucherlist::{VoucherStatus, VoucherType};
use lexoffice::model::Voucherlist;
use lexoffice::Result;
use structopt::StructOpt;

const STATUS_VALUES: &[&str] = &[
    "draft", "open", "paid", "paidoff", "voided", "overdue", "accepted",
    "rejected",
];
const TYPE_VALUES: &[&str] = &[
    "salesinvoice",
    "salescreditnote",
    "purchaseinvoice",
    "purchasecreditnote",
    "invoice",
    "creditnote",
    "orderconfirmation",
    "quotation",
];

/// voucherlist endpoint
#[derive(Debug, StructOpt)]
pub enum VoucherlistOpt {
    /// retrieves a paginated list of all voucher list items
    List {
        #[structopt(flatten)]
        page: PaginatedOpt,
        #[structopt(short, long, possible_values = TYPE_VALUES)]
        type_: VoucherType,
        #[structopt(short = "S", long, possible_values = STATUS_VALUES)]
        status: VoucherStatus,
    },
    /// queries a specific voucher list item by its id
    Get(ByIdOpt),
}

impl VoucherlistOpt {
    pub async fn exec(
        &self,
        client: Client,
    ) -> Result<ReturnType<Voucherlist>> {
        let request = client.request::<Voucherlist>();
        let result = match self {
            Self::List {
                page,
                type_,
                status,
            } => ReturnType::Paged(
                page.exec(request.type_(type_).status(status)).await?,
            ),
            //Self::New(x) => x.exec(request),
            //Self::Updatable(x) => x.exec(request),
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
        };
        Ok(result)
    }
}
