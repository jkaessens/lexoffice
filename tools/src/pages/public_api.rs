use async_std::task::sleep;
use fantoccini::error::CmdError;
use fantoccini::Client;
use fantoccini::Element;
use fantoccini::Locator;
use log::info;
use std::time::Duration;

const URL: &str = "https://app.lexoffice.de/settings/#/public-api";

const CREATE_BUTTON_LOCATOR: Locator =
    Locator::Css("[translate=SETTINGS_PUBLIC_API_MGMT_CREATE_ACTION]");
const RECREATE_BUTTON_LOCATOR: Locator =
    Locator::Css("[translate=SETTINGS_PUBLIC_API_MGMT_REPLACE_ACTION]");
const CONFIRM_RECREATE_LOCATOR: Locator = Locator::Css(
    "[translate=SETTINGS_PUBLIC_API_MGMT_DIALOG_REPLACE_TOKEN_ACTION]",
);
const CONFIRM_CREATE_LOCATOR: Locator =
    Locator::Css("[translate=SETTINGS_PUBLIC_API_MGMT_DIALOG_LICENSE_ACTION]");
const API_KEY_CONTAINER_LOCATOR: Locator = Locator::Id("api-key");
const CONFIRM_CHECKBOX_LOCATOR: Locator =
    Locator::Css(".modal-body .checkbox label i");

pub struct PublicApiPage {
    submit: Element,
}

impl PublicApiPage {
    pub async fn navigate(client: Client) -> Result<Self, CmdError> {
        info!("Navigate to signup...");
        let mut client = client.clone();
        client.goto(URL).await?;
        info!("Checking if a key exists");
        let submit = if let Ok(submit) =
            Self::prepare_create_button(client.clone()).await
        {
            submit
        } else {
            Self::prepare_recreate_button(client.clone()).await?
        };
        Ok(Self { submit })
    }

    async fn prepare_recreate_button(
        mut client: Client,
    ) -> Result<Element, CmdError> {
        let recreate_button =
            client.wait_for_find(RECREATE_BUTTON_LOCATOR).await?;
        info!("Key exists, prepare dialog for recreation...");
        let mut client = recreate_button.click().await?;
        sleep(Duration::from_millis(500)).await;
        client.wait_for_find(CONFIRM_RECREATE_LOCATOR).await
    }

    async fn prepare_create_button(
        mut client: Client,
    ) -> Result<Element, CmdError> {
        let create_button = client.wait_for_find(CREATE_BUTTON_LOCATOR).await?;
        info!("Key does not exist, prepare dialog for creation...");
        let mut client = create_button.click().await?;
        sleep(Duration::from_millis(500)).await;
        let mut client = client
            .wait_for_find(CONFIRM_CHECKBOX_LOCATOR)
            .await?
            .click()
            .await?;
        client.wait_for_find(CONFIRM_CREATE_LOCATOR).await
    }

    pub async fn create_api_key(self) -> Result<String, CmdError> {
        let mut client = self.submit.click().await?;
        sleep(Duration::from_millis(500)).await;
        let api_key = client
            .wait_for_find(API_KEY_CONTAINER_LOCATOR)
            .await?
            .attr("value")
            .await?
            .expect("API field unset");

        Ok(api_key)
    }
}
