use async_std::task::sleep;
use fantoccini::Client;
use feed_rs::model::Entry;
use feed_rs::parser::parse;
use log::info;
use std::error::Error;
use std::time::Duration;

#[derive(Clone, Debug)]
pub struct MailLinkAddress {
    name: String,
}

impl From<String> for MailLinkAddress {
    fn from(name: String) -> Self {
        Self { name }
    }
}

impl From<&String> for MailLinkAddress {
    fn from(name: &String) -> Self {
        name.clone().into()
    }
}

impl From<&str> for MailLinkAddress {
    fn from(name: &str) -> Self {
        name.to_string().into()
    }
}

impl From<&MailLinkAddress> for MailLinkAddress {
    fn from(name: &MailLinkAddress) -> Self {
        name.clone()
    }
}

impl ToString for MailLinkAddress {
    fn to_string(&self) -> String {
        self.name()
    }
}

impl MailLinkAddress {
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn mail(&self) -> String {
        format!("{}@trashmail.de", self.name)
    }
}

fn index_link(name: &str) -> String {
    format!("https://www.trashmail.de/inbox-api.php?name={}", name)
}

fn entry_link(name: &str, id: &str) -> String {
    format!(
        "https://www.trashmail.de/mail-api.php?name={}&id={}",
        name, id
    )
}

pub struct MailLink {
    mail: MailLinkAddress,
    client: Client,
}

impl MailLink {
    pub fn new<T>(mail: T, client: Client) -> Self
    where
        T: Into<MailLinkAddress>,
    {
        Self {
            mail: mail.into(),
            client,
        }
    }

    async fn get_entries(&self) -> Result<Vec<Entry>, Box<dyn Error>> {
        info!("Getting list index...");
        let body = reqwest::get(&index_link(&self.mail.name()))
            .await?
            .bytes()
            .await?;

        let feed = parse(body.as_ref())?;

        Ok(feed.entries)
    }

    pub async fn wait_for_welcome(&self) -> Result<String, Box<dyn Error>> {
        info!("Waiting for welcome link...");
        self.wait_for_link("Bitte bestätigen Sie ihren Account!")
            .await
    }

    pub async fn wait_for_delete_confirum(
        &self,
    ) -> Result<String, Box<dyn Error>> {
        info!("Waiting for delete confirmation...");
        self.wait_for_link("lexoffice - Zugang löschen").await
    }

    pub async fn wait_for_link(
        &self,
        title: &str,
    ) -> Result<String, Box<dyn Error>> {
        info!("Getting link from a mail with title {:?}...", title);

        let entry = self.wait_for_title(title).await?;
        if let Some(content) = entry.content {
            if let Some(body) = content.body {
                self.get_button_link(body).await
            } else {
                Err("Content has no body".into())
            }
        } else {
            Err("Entry has no content".into())
        }
    }

    async fn wait_for_title(
        &self,
        title: &str,
    ) -> Result<Entry, Box<dyn Error>> {
        info!("Waiting for Entry with title {:?}...", title);

        loop {
            if let Some(entry) = self.get_by_title(title).await? {
                return Ok(entry);
            }
            sleep(Duration::from_secs(30)).await;
        }
    }

    async fn get_button_link(
        &self,
        html: String,
    ) -> Result<String, Box<dyn Error>> {
        info!("Extracting link from mail...");

        let mut client = self.client.clone();
        let href = client
            .execute(
                r#"
                const [content] = arguments;
                document.body.innerHTML = content;
                return document.querySelector("a.mcnButton").href             
            "#,
                vec![html.into()],
            )
            .await?;
        href.as_str()
            .map(|x| x.to_owned())
            .ok_or_else(|| "Cannot find button link".into())
    }

    async fn get_entry(
        &self,
        id: &str,
    ) -> Result<Option<Entry>, Box<dyn Error>> {
        info!("Getting entry with id {:?}...", id);

        let body = reqwest::get(&entry_link(&self.mail.name(), id))
            .await?
            .bytes()
            .await?;

        let feed = parse(body.as_ref())?;

        Ok(feed.entries.get(0).map(|x| x.to_owned()))
    }

    async fn get_by_title(
        &self,
        title: &str,
    ) -> Result<Option<Entry>, Box<dyn Error>> {
        info!("Searching for entry with title {:?}...", title);

        let entries = self.get_entries().await?;
        let entry = entries.iter().to_owned().find(|e| {
            e.title.as_ref().map(|t| t.content.as_str()) == Some(&title)
        });
        if let Some(entry) = entry {
            self.get_entry(&entry.id).await
        } else {
            Ok(None)
        }
    }
}
