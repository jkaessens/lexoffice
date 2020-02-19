use crate::model::voucher_list::{VoucherStatusEnum, VoucherTypeEnum};
use crate::model::VoucherList;
use crate::request::impls::ById;
use crate::request::impls::Paginated;
use crate::request::Endpoint;
use crate::request::Request;
use std::marker::PhantomData;

// Not implementing the into trait here as this mustn't be public.
fn into<O, T, S>(
    request: Request<VoucherList, O>,
) -> Request<VoucherList, (T, S)> {
    Request {
        client: request.client,
        url: request.url,
        target: request.target,
        state: PhantomData,
    }
}

/// This type represents the state of a Request to the VoucherList endpoint
/// that is ready to be sent
pub type VoucherListStateFinished = (VoucherTypeEnum, VoucherStatusEnum);

/// This type represents the state of a Request to the VoucherList endpoint
/// hasn't been started to be configured
pub type VoucherListStateUnstarted = ();

/// This type represents the state of a Request to the VoucherList endpoint
/// that configuration hasn't been finished
pub type VoucherListState<T, S> = (T, S);

impl<S> Endpoint for Request<VoucherList, S> {
    const ENDPOINT: &'static str = "voucherlist";
}

impl Request<VoucherList, VoucherListStateUnstarted> {
    /// Sets the voucher status for this request. Calling this function is mandatory
    pub fn type_(
        self,
        voucher_type: VoucherTypeEnum,
    ) -> Request<VoucherList, VoucherListState<VoucherTypeEnum, ()>> {
        into::<_, (), ()>(self).type_(voucher_type)
    }
    /// Sets the voucher status for this request. Calling this function is mandatory
    pub fn status(
        self,
        voucher_status: VoucherStatusEnum,
    ) -> Request<VoucherList, VoucherListState<(), VoucherStatusEnum>> {
        into::<_, (), ()>(self).status(voucher_status)
    }
}

impl<S> Request<VoucherList, VoucherListState<(), S>> {
    /// Sets the voucher status for this request. Calling this function is mandatory
    pub fn type_(
        mut self,
        voucher_type: VoucherTypeEnum,
    ) -> Request<VoucherList, VoucherListState<VoucherTypeEnum, S>> {
        self.url.query_pairs_mut().append_pair(
            "voucherType",
            &serde_plain::to_string(&voucher_type).unwrap(),
        );
        into(self)
    }
}

impl<T> Request<VoucherList, VoucherListState<T, ()>> {
    /// Sets the voucher status for this request. Calling this function is mandatory
    pub fn status(
        mut self,
        voucher_status: VoucherStatusEnum,
    ) -> Request<VoucherList, VoucherListState<T, VoucherStatusEnum>> {
        self.url.query_pairs_mut().append_pair(
            "voucherStatus",
            &serde_plain::to_string(&voucher_status).unwrap(),
        );
        into(self)
    }
}

/// # Examples
///
/// ```
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::VoucherList;
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let uuid = uuid::Uuid::parse_str("f4add52b-44e3-474a-b718-890885094d9a")?;
/// let invoices = client.request::<VoucherList>().by_id(uuid).await?;
/// println!("{:#?}", invoices);
/// # Ok(())
/// # }
/// ```
///
impl ById for Request<VoucherList, ()> {}

/// # Examples
///
/// ```
/// use lexoffice::client::{ApiKey, Client};
/// use lexoffice::model::VoucherList;
/// use lexoffice::model::voucher_list::{VoucherStatusEnum, VoucherTypeEnum};
///
/// # async fn run() -> Result<(), Box<dyn std::error::Error>> {
/// let client = Client::new(ApiKey::try_default().await?);
/// let voucherlist = client
///        .request::<VoucherList>()
///        .type_(VoucherTypeEnum::Invoice)
///        .status(VoucherStatusEnum::Open)
///        .page(0).await?;
/// println!("{:#?}", voucherlist);
/// # Ok(())
/// # }
/// ```
///
impl Paginated for Request<VoucherList, VoucherListStateFinished> {}
