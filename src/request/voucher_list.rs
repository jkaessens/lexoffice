use crate::model::voucher_list::{VoucherStatusEnum, VoucherTypeEnum};
use crate::model::VoucherList;
use crate::request::impls::by_id::ById;
use crate::request::impls::paginated::Paginated;
use crate::request::Endpoint;
use crate::request::StateRequest;
use std::marker::PhantomData;

pub trait Void {}
impl Void for () {}

// Not implementing the into trait here as this must not be public.
fn into<O, T, S>(
    request: StateRequest<VoucherList, O>,
) -> StateRequest<VoucherList, (T, S)> {
    StateRequest {
        client: request.client,
        url: request.url,
        target: request.target,
        state: PhantomData,
    }
}

type Finished = (VoucherTypeEnum, VoucherStatusEnum);

impl Endpoint for StateRequest<VoucherList, Finished> {
    const ENDPOINT: &'static str = "voucherlist";
}

impl StateRequest<VoucherList, ()> {
    pub fn type_(
        self,
        voucher_type: VoucherTypeEnum,
    ) -> StateRequest<VoucherList, (VoucherTypeEnum, ())> {
        into::<_, (), ()>(self).type_(voucher_type)
    }
    pub fn status(
        self,
        voucher_status: VoucherStatusEnum,
    ) -> StateRequest<VoucherList, ((), VoucherStatusEnum)> {
        into::<_, (), ()>(self).status(voucher_status)
    }
}

impl<T, S> StateRequest<VoucherList, (T, S)> {
    pub fn type_(
        mut self,
        voucher_type: VoucherTypeEnum,
    ) -> StateRequest<VoucherList, (VoucherTypeEnum, S)>
    where
        T: Void,
    {
        self.url.query_pairs_mut().append_pair(
            "voucherType",
            &serde_plain::to_string(&voucher_type).unwrap(),
        );
        into(self)
    }
    pub fn status(
        mut self,
        voucher_status: VoucherStatusEnum,
    ) -> StateRequest<VoucherList, (T, VoucherStatusEnum)>
    where
        S: Void,
    {
        self.url.query_pairs_mut().append_pair(
            "voucherStatus",
            &serde_plain::to_string(&voucher_status).unwrap(),
        );
        into(self)
    }
}

impl ById for StateRequest<VoucherList, Finished> {}

impl Paginated for StateRequest<VoucherList, Finished> {}
