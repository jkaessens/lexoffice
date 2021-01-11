use lexoffice::model::Page;
use lexoffice::request::ResultInfo;
pub mod actions;
pub mod resources;

pub enum ReturnType<T> {
    Paged(Page<T>),
    ResultInfo(ResultInfo<T>),
    Obj(T),
    Empty,
}
