use crate::account::mail_link::MailLinkAddress;
use crate::account::pages::account::AccountPage;
use fantoccini::Client;
use fantoccini::ClientBuilder;
use fantoccini::Locator;
use mail_link::MailLink;
use mkpasswd::{
    alphabets::{LatinLowerNumbers, Password},
    generate,
};
use pages::public_api::PublicApiPage;
use pages::sign_in::SignInPage;
use pages::sign_up::SignUpPage;

pub mod mail_link;
pub mod pages;

pub async fn new_client() -> Result<Client, Box<dyn std::error::Error>> {
    Ok(ClientBuilder::native()
        .connect("http://localhost:4444")
        .await?)
}

pub async fn new_account(
) -> Result<(MailLinkAddress, String), Box<dyn std::error::Error>> {
    let mail: MailLinkAddress =
        String::from_utf8(generate(&LatinLowerNumbers, 10)?)?.into();
    let password =
        format!("@aA10{}", String::from_utf8(generate(&Password, 10)?)?);
    let mut client = new_client().await?;

    SignUpPage::navigate(client.clone())
        .await?
        .sign_up_as(&mail, &password)
        .await?;

    let link = MailLink::new(&mail, client.clone())
        .wait_for_welcome()
        .await?;

    client.goto(&link).await?;
    client.close().await?;

    Ok((mail, password))
}

pub async fn create_api_key(
    mail: &MailLinkAddress,
    password: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let client = new_client().await?;

    SignInPage::navigate(client.clone())
        .await?
        .sign_in_as(mail, password)
        .await?;

    let api_key = PublicApiPage::navigate(client)
        .await?
        .create_api_key()
        .await?;
    Ok(api_key)
}

pub async fn delete_account(
    mail: &MailLinkAddress,
    password: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut client = new_client().await?;

    SignInPage::navigate(client.clone())
        .await?
        .sign_in_as(mail, password)
        .await?;

    AccountPage::navigate(client.clone())
        .await?
        .delete_account()
        .await?;

    let link = MailLink::new(mail, client.clone())
        .wait_for_delete_confirum()
        .await?;

    client.goto(&link).await?;
    client
        .wait_for_find(Locator::Id("confirm_delete_organization"))
        .await?
        .click()
        .await?;
    client.close().await?;

    Ok(())
}
