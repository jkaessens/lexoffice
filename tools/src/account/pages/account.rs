use async_std::task::sleep;
use fantoccini::error::CmdError;
use fantoccini::Client;
use fantoccini::Locator;
use log::info;
use std::time::Duration;

const URL: &str = "https://app.lexoffice.de/subscription/#/account";
const DELETE_BUTTON_LOCATOR: Locator =
    Locator::Css("button[translate='PACTUM_DELETE_ACCOUNT']");
const CONFIRM_DELETE_BUTTON_LOCATOR: Locator =
    Locator::Css("[ng-bind='confirmationDialogCtrl.options.confirmText']");

pub struct AccountPage {
    client: Client,
}

impl AccountPage {
    pub async fn navigate(client: Client) -> Result<Self, CmdError> {
        info!("Navigate to account...");
        let mut client = client.clone();
        client.goto(URL).await?;
        Ok(Self { client })
    }

    pub async fn delete_account(self) -> Result<(), CmdError> {
        info!("Pressing account delete button...");
        let delete_button = self
            .client
            .clone()
            .wait_for_find(DELETE_BUTTON_LOCATOR)
            .await?;
        delete_button.click().await?;

        sleep(Duration::from_secs(1)).await;
        info!("Confirming account delete...");
        let confirm_delete_button = self
            .client
            .clone()
            .wait_for_find(CONFIRM_DELETE_BUTTON_LOCATOR)
            .await?;
        confirm_delete_button.click().await?;
        Ok(())
    }
}
