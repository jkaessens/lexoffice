use crate::ReturnType;
use lexoffice::model::PostingCategory;
use lexoffice::Client;
use lexoffice::Result;
use structopt::StructOpt;

/// retrieves the users profile
#[derive(Debug, StructOpt)]
pub struct PostingCategoryOpt {}

impl PostingCategoryOpt {
    pub async fn exec(
        &self,
        client: Client,
    ) -> Result<ReturnType<Vec<PostingCategory>>> {
        let request = client.request::<PostingCategory>();

        Ok(ReturnType::Obj(request.get().await?))
    }
}
