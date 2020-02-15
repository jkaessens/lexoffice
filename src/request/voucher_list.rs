use crate::client::Client;
use crate::model::voucher_list::{VoucherStatusEnum, VoucherTypeEnum};
use crate::model::VoucherList;
use crate::request::ById;
use crate::request::Endpoint;
use crate::request::Paginated;
use crate::request::Request;
use crate::request::Requestable;
use reqwest::Url;
use std::marker::PhantomData;

pub trait Void {}
impl Void for () {}

pub trait VoucherListRequestTrait<T, S> {
    fn type_(
        self,
        voucher_type: VoucherTypeEnum,
    ) -> VoucherListRequest<VoucherTypeEnum, S>
    where
        T: Void;
    fn status(
        self,
        voucher_status: VoucherStatusEnum,
    ) -> VoucherListRequest<T, VoucherStatusEnum>
    where
        S: Void;
}

#[derive(Clone, Debug)]
pub struct VoucherListRequest<T, S> {
    inner: Request<VoucherList>,
    phantom: PhantomData<(T, S)>,
}

impl<T, S> From<Request<VoucherList>> for VoucherListRequest<T, S> {
    fn from(request: Request<VoucherList>) -> Self {
        Self {
            phantom: PhantomData,
            inner: request,
        }
    }
}

impl VoucherListRequestTrait<(), ()> for Request<VoucherList> {
    fn type_(
        self,
        voucher_type: VoucherTypeEnum,
    ) -> VoucherListRequest<VoucherTypeEnum, ()> {
        VoucherListRequest::<(), ()>::from(self).type_(voucher_type)
    }
    fn status(
        self,
        voucher_status: VoucherStatusEnum,
    ) -> VoucherListRequest<(), VoucherStatusEnum> {
        VoucherListRequest::<(), ()>::from(self).status(voucher_status)
    }
}

impl<T, S> VoucherListRequestTrait<T, S> for VoucherListRequest<T, S> {
    fn type_(
        mut self,
        voucher_type: VoucherTypeEnum,
    ) -> VoucherListRequest<VoucherTypeEnum, S>
    where
        T: Void,
    {
        self.inner.url.query_pairs_mut().append_pair(
            "voucherType",
            &serde_plain::to_string(&voucher_type).unwrap(),
        );
        self.inner.into()
    }
    fn status(
        mut self,
        voucher_status: VoucherStatusEnum,
    ) -> VoucherListRequest<T, VoucherStatusEnum>
    where
        S: Void,
    {
        self.inner.url.query_pairs_mut().append_pair(
            "voucherStatus",
            &serde_plain::to_string(&voucher_status).unwrap(),
        );
        self.inner.into()
    }
}

impl Requestable for VoucherListRequest<VoucherTypeEnum, VoucherStatusEnum> {
    fn client(&self) -> &Client {
        self.inner.client()
    }
    fn url(&self) -> Url {
        self.inner.url()
    }
}

impl Endpoint for VoucherListRequest<VoucherTypeEnum, VoucherStatusEnum> {
    const ENDPOINT: &'static str = "voucherlist";
}
impl Endpoint for Request<VoucherList> {
    const ENDPOINT: &'static str = "voucherlist";
}

impl ById<VoucherList> for Request<VoucherList> {}

impl Paginated<VoucherList>
    for VoucherListRequest<VoucherTypeEnum, VoucherStatusEnum>
{
}
