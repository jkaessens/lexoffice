use serde_json::json;
use tools::create_api_key;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mail, password) = tools::new_account().await?;

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
