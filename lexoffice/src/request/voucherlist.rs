use crate::model::voucherlist::{VoucherStatus, VoucherType};
use crate::model::Voucherlist;
use crate::request::impls::ById;
use crate::request::impls::Paginated;
use crate::request::Endpoint;
use crate::request::Request;
use crate::request::RequestWithState;
use crate::types::Date;
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
    ) -> RequestWithState<Voucherlist, VoucherlistState<(), VoucherStatus>>
    {
        into::<_, (), ()>(self).status(voucher_status)
    }
}

impl<T: Clone, S: Clone> RequestWithState<Voucherlist, VoucherlistState<T, S>> {
    /// Filter out all voucher dates before `voucher_date_from`
    pub fn voucher_date_from(
        mut self,
        voucher_date_from: &Date,
    ) -> RequestWithState<Voucherlist, VoucherlistState<T, VoucherStatus>> {
        self.url.query_pairs_mut().append_pair(
            "voucherDateFrom",
            &serde_plain::to_string(voucher_date_from).unwrap(),
        );
        into(self)
    }
    /// Filter out all voucher dates after `voucher_date_to`
    pub fn voucher_date_to(
        mut self,
        voucher_date_to: &Date,
    ) -> RequestWithState<Voucherlist, VoucherlistState<T, VoucherStatus>> {
        self.url.query_pairs_mut().append_pair(
            "voucherDateTo",
            &serde_plain::to_string(voucher_date_to).unwrap(),
        );
        into(self)
    }
    /// Filter out all created dates before `created_date_from`
    pub fn created_date_from(
        mut self,
        created_date_from: &Date,
    ) -> RequestWithState<Voucherlist, VoucherlistState<T, VoucherStatus>> {
        self.url.query_pairs_mut().append_pair(
            "createdDateFrom",
            &serde_plain::to_string(created_date_from).unwrap(),
        );
        into(self)
    }
    /// Filter out all created dates after `created_date_to`
    pub fn created_date_to(
        mut self,
        created_date_to: &Date,
    ) -> RequestWithState<Voucherlist, VoucherlistState<T, VoucherStatus>> {
        self.url.query_pairs_mut().append_pair(
            "createdDateTo",
            &serde_plain::to_string(created_date_to).unwrap(),
        );
        into(self)
    }
    /// Filter out all updated dates before `updated_date_from`
    pub fn updated_date_from(
        mut self,
        updated_date_from: &Date,
    ) -> RequestWithState<Voucherlist, VoucherlistState<T, VoucherStatus>> {
        self.url.query_pairs_mut().append_pair(
            "updatedDateFrom",
            &serde_plain::to_string(updated_date_from).unwrap(),
        );
        into(self)
    }
    /// Filter out all updated dates after `updated_date_to`
    pub fn updated_date_to(
        mut self,
        updated_date_to: &Date,
    ) -> RequestWithState<Voucherlist, VoucherlistState<T, VoucherStatus>> {
        self.url.query_pairs_mut().append_pair(
            "updatedDateTo",
            &serde_plain::to_string(updated_date_to).unwrap(),
        );
        into(self)
    }
    /// Filter by the voucher number
    pub fn voucher_number(
        mut self,
        voucher_number: &str,
    ) -> RequestWithState<Voucherlist, VoucherlistState<T, VoucherStatus>> {
        self.url
            .query_pairs_mut()
            .append_pair("voucherNumber", voucher_number);
        into(self)
    }

    /// Filter by the contact ID
    pub fn contact_id(
        mut self,
        contact_id: &uuid::Uuid,
    ) -> RequestWithState<Voucherlist, VoucherlistState<T, VoucherStatus>> {
        self.url.query_pairs_mut().append_pair(
            "contactId",
            &serde_plain::to_string(contact_id).unwrap(),
        );
        into(self)
    }
    /// Filter by the archived flag
    pub fn archived(
        mut self,
        archived: bool,
    ) -> RequestWithState<Voucherlist, VoucherlistState<T, VoucherStatus>> {
        self.url.query_pairs_mut().append_pair(
            "archived",
            &serde_plain::to_string(&archived).unwrap(),
        );
        into(self)
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
