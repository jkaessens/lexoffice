use crate::actions::*;
use crate::ReturnType;
use lexoffice::client::Client;
use lexoffice::model::contacts::*;
use lexoffice::model::Contact;
use lexoffice::Result;
use structopt::StructOpt;

/// contact endpoint
#[derive(Debug, StructOpt)]
pub enum ContactOpt {
    /// retrieves a paginated list of all contacts
    List(PaginatedOpt),
    /// creates new contact and opens it in an editor
    New(StorableOpt),
    /// modifies a contact
    Edit(UpdatableOpt),
    /// queries a specific contact by its id
    Get(ByIdOpt),
}

fn default() -> Contact {
    Contact::builder()
        .roles(
            Roles::builder()
                .customer(Customer::builder().build())
                .build(),
        )
        .build()
}

impl ContactOpt {
    pub async fn exec(&self, client: Client) -> Result<ReturnType<Contact>> {
        let request = client.request::<Contact>();
        let result = match self {
            Self::List(x) => ReturnType::Paged(x.exec(request).await?),
            Self::Get(x) => ReturnType::Obj(x.exec(request).await?),
            Self::New(x) => {
                ReturnType::ResultInfo(x.exec(request, default()).await?)
            }
            Self::Edit(x) => ReturnType::ResultInfo(x.exec(request).await?),
        };
        Ok(result)
    }
}
