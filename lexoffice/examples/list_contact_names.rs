use lexoffice::model::Contact;
use lexoffice::{ApiKey, Client};
use tokio::stream::StreamExt;

use std::error::Error;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new(ApiKey::try_default().await?);
    let mut contacts = client.request::<Contact>().stream();

    while let Some(contact) = contacts.next().await {
        let contact = contact?;
        println!("Person: {:?}", &contact);
        if let Some(company) = contact.company {
            println!("Company: {}", company.name);
        }
        if let Some(person) = contact.person {
            println!("Person: {}", person.last_name);
        }
    }
    Ok(())
}
