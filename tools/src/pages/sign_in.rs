use crate::mail_link::MailLinkAddress;
use fantoccini::error::CmdError;
use fantoccini::Client;
use fantoccini::Element;
use fantoccini::Locator;
use log::info;

const URL: &str = "https://app.lexoffice.de/";

const EMAIL_LOCATOR: Locator = Locator::Id("email");
const PASSWORD_LOCATOR: Locator = Locator::Id("password");
const SUBMIT_LOCATOR: Locator = Locator::Id("login-btn");

pub struct SignInPage {
    email: Element,
    password: Element,
    submit: Element,
}

impl SignInPage {
    pub async fn navigate(client: Client) -> Result<Self, CmdError> {
        info!("Navigate to signin...");
        let mut client = client.clone();
        client.goto(URL).await?;
        Ok(Self {
            email: client.wait_for_find(EMAIL_LOCATOR).await?,
            password: client.wait_for_find(PASSWORD_LOCATOR).await?,
            submit: client.wait_for_find(SUBMIT_LOCATOR).await?,
        })
    }

    pub async fn sign_in_as<T>(
        mut self,
        mail: T,
        password: &str,
    ) -> Result<Self, CmdError>
    where
        T: Into<MailLinkAddress>,
    {
        let mail: MailLinkAddress = mail.into();
        info!("Provide Credentials for {:?}...", mail.mail());
        self.email.send_keys(&mail.mail()).await?;
        self.password.send_keys(password).await?;

        let mut client = self.submit.clone().click().await?;

        client.wait_for_navigation(None).await?;

        Ok(self)
    }
}
