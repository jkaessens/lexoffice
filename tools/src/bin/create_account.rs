use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let (mail, password) = tools::new_account().await?;

    println!(
        "{}",
        json!({
            "mail": mail.to_string(),
            "password": password
        })
    );

    Ok(())
}
