use crate::mail_link::MailLinkAddress;
use debug_rs::debug;
use fantoccini::error::CmdError;
use fantoccini::Client;
use fantoccini::Element;
use fantoccini::Locator;

const SIGNUP_URL: &str = "https://app.lexoffice.de/signup";

const EMAIL_LOCATOR: Locator = Locator::Id("email");
const PASSWORD_LOCATOR: Locator = Locator::Id("password");
const ACCEPT_GDPR_LOCATOR: Locator = Locator::Id("acceptGDPR");
const SUBSCRIBE_NEWSLETTER_LOCATOR: Locator =
    Locator::Id("subscribeNewsletter");
const SUBMIT_LOCATOR: Locator = Locator::Id("signup-btn");

pub struct SignUpPage {
    email: Element,
    password: Element,
    accept_gdpr: Element,
    subscribe_newsletter: Element,
    submit: Element,
}

impl<'a> SignUpPage {
    pub async fn navigate(client: Client) -> Result<Self, CmdError> {
        debug!("Navigate to signup...");
        let mut client = client.clone();
        client.goto(SIGNUP_URL).await?;
        Ok(Self {
            email: client.wait_for_find(EMAIL_LOCATOR).await?,
            password: client.wait_for_find(PASSWORD_LOCATOR).await?,
            accept_gdpr: client.wait_for_find(ACCEPT_GDPR_LOCATOR).await?,
            subscribe_newsletter: client
                .wait_for_find(SUBSCRIBE_NEWSLETTER_LOCATOR)
                .await?,
            submit: client.wait_for_find(SUBMIT_LOCATOR).await?,
        })
    }
    pub async fn sign_up_as<T>(
        mut self,
        mail: T,
        password: &str,
    ) -> Result<Self, CmdError>
    where
        T: Into<MailLinkAddress>,
    {
        let mail: MailLinkAddress = mail.into();
        debug!(format!("Provide Credentials for {:?}...", mail.mail()));
        self.email.send_keys(&mail.mail()).await?;
        self.password.send_keys(password).await?;

        self.accept_gdpr.clone().click().await?;

        self.subscribe_newsletter.clone().click().await?;

        self.submit.clone().click().await?;

        Ok(self)
    }
}
