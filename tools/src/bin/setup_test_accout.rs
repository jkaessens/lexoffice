use serde_json::json;
use tools::account::create_api_key;
use tools::account::new_account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let (mail, password) = new_account().await?;

    let api_key = create_api_key(&mail, &password).await?;

    println!(
        "{}",
        json!({
            "mail": mail.to_string(),
            "password": password,
            "api_key": api_key,
        })
    );

    Ok(())
}
