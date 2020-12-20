use serde_json::json;
use std::env::args;
use tools::create_api_key;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let mut args = args().skip(1);
    let mail = args.next().expect("No mail address provided!");
    let password = args.next().expect("No password provided!");

    let api_key = create_api_key(&mail.into(), &password).await?;

    println!(
        "{}",
        json!({
            "api_key": api_key,
        })
    );

    Ok(())
}
