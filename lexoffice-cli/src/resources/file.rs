use lexoffice::model::File;
use lexoffice::request::Request;
use lexoffice::Client;
use lexoffice::Result;
use mime_guess::get_extensions;
use reqwest::header::CONTENT_TYPE;
use std::pin::Pin;
use std::str::FromStr;
use structopt::StructOpt;
use tokio::fs;
use tokio::prelude::*;
use tokio::stream::StreamExt;

/// file endpoint
#[derive(Debug, StructOpt)]
pub enum FileOpt {
    /// Uploads a file
    Upload(UploadOpt),
    /// Downloads a file
    Get(GetOpt),
}

impl FileOpt {
    pub async fn exec(&self, client: Client) -> Result<()> {
        let request = client.request::<File>();
        match self {
            Self::Upload(x) => x.exec(request).await,
            Self::Get(x) => x.exec(request).await,
        }
    }
}

#[derive(Debug, StructOpt)]
pub struct UploadOpt {
    file: String,
}

impl UploadOpt {
    pub async fn exec(&self, request: Request<File>) -> Result<()> {
        println!("{}", request.upload_path(self.file.clone()).await?);
        Ok(())
    }
}

#[derive(Debug, StructOpt)]
pub struct GetOpt {
    id: String,
    #[structopt(short, long)]
    output: Option<String>,
}

impl GetOpt {
    pub async fn exec(&self, request: Request<File>) -> Result<()> {
        let response = request.by_id_str(&self.id).await?;
        println!("{:?}", &self);
        let output = if let Some(output) = &self.output {
            output.clone()
        } else {
            let content_type = response
                .headers()
                .get(CONTENT_TYPE)
                .expect("has Content-Type Header")
                .to_str()
                .unwrap();
            let mime = mime::Mime::from_str(content_type).unwrap();
            format!(
                "{}.{}",
                self.id,
                get_extensions(mime.type_().as_str(), mime.subtype().as_str())
                    .unwrap()[0]
            )
        };
        let mut output: Pin<Box<dyn AsyncWrite>> = if output == "-" {
            Box::pin(tokio::io::stdout())
        } else {
            Box::pin(fs::File::create(output).await?)
        };
        let mut stream = response.bytes_stream();
        while let Some(x) = stream.next().await {
            output.write_all(&x?).await?;
        }
        Ok(())
    }
}
