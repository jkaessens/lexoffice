use std::env::args;
use tools::account::delete_account;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    let mut args = args().skip(1);
    let mail = args.next().expect("No mail address provided!");
    let password = args.next().expect("No password provided!");

    delete_account(&mail.into(), &password).await?;

    Ok(())
}
