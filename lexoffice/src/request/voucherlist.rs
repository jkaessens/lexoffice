use crate::model::voucherlist::{VoucherStatus, VoucherType};
use crate::model::Voucherlist;
use crate::request::impls::ById;
use crate::request::impls::Paginated;
use crate::request::Endpoint;
use crate::request::RequestWithState;
use crate::request::Request;
use std::marker::PhantomData;

// Not implementing the into trait here as this mustn't be public.
fn into<O: Clone, T: Clone, S: Clone>(
    request: RequestWithState<Voucherlist, O>,
) -> RequestWithState<Voucherlist, (T, S)> {
    RequestWithState {
        client: request.client,
        url: request.url,
        target: request.target,
        state: PhantomData,
    }
}

/// This type represents the state of a Request to the Voucherlist endpoint
/// that is ready to be sent
pub type VoucherlistStateFinished = (VoucherType, VoucherStatus);

/// This type represents the state of a Request to the Voucherlist endpoint
/// hasn't been started to be configured
pub type VoucherlistStateUnstarted = ();

/// This type represents the state of a Request to the Voucherlist endpoint
/// that configuration hasn't been finished
pub type VoucherlistState<T, S> = (T, S);

impl<S: Clone> Endpoint for RequestWithState<Voucherlist, S> {
    const ENDPOINT: &'static str = "voucherlist";
}

impl RequestWithState<Voucherlist, VoucherlistStateUnstarted> {
    /// Sets the voucher status for this request. Calling this function is mandatory
    pub fn type_(
        self,
        voucher_type: &VoucherType,
    ) -> RequestWithState<Voucherlist, VoucherlistState<VoucherType, ()>> {
        into::<_, (), ()>(self).type_(voucher_type)
    }
    /// Sets the voucher status for this request. Calling this function is mandatory
    pub fn status(
        self,
        voucher_status: &VoucherStatus,
    ) -> RequestWithState<Voucherlist, VoucherlistState<(), VoucherStatus>> {
        into::<_, (), ()>(self).status(voucher_status)
    }
}

impl<S: Clone> RequestWithState<Voucherlist, VoucherlistState<(), S>> {
    /// Sets the voucher status for this request. Calling this function is mandatory
    pub fn type_(
        mut self,
        voucher_type: &VoucherType,
    ) -> RequestWithState<Voucherlist, VoucherlistState<VoucherType, S>> {
        self.url.query_pairs_mut().append_pair(
            "voucherType",
            &serde_plain::to_string(voucher_type).unwrap(),
        );
        into(self)
    }
}

impl<T: Clone> RequestWithState<Voucherlist, VoucherlistState<T, ()>> {
    /// Sets the voucher status for this request. Calling this function is mandatory
    pub fn status(
        mut self,
        voucher_status: &VoucherStatus,
    ) -> RequestWithState<Voucherlist, VoucherlistState<T, VoucherStatus>> {
        self.url.query_pairs_mut().append_pair(
            "voucherStatus",
            &serde_plain::to_string(voucher_status).unwrap(),
        );
        into(self)
    }
}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::Voucherlist;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let invoices = client.request::<Voucherlist>().by_id(uuid).await?;
/// println!("{:#?}", invoices);
/// # Ok(())
/// # }
/// ```
///
impl ById for Request<Voucherlist> {}

/// # Examples
///
/// ```
/// use lexoffice::{ApiKey, Client};
/// use lexoffice::model::Voucherlist;
/// use lexoffice::model::voucherlist::{VoucherStatus, VoucherType};
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let voucherlist = client
///        .request::<Voucherlist>()
///        .type_(&VoucherType::Invoice)
///        .status(&VoucherStatus::Open)
///        .page(0).await?;
/// println!("{:#?}", voucherlist);
/// # Ok(())
/// # }
/// ```
///
impl Paginated for RequestWithState<Voucherlist, VoucherlistStateFinished> {}
