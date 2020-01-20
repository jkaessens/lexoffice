use lexoffice::client::Client;
use std::env;
use std::fs::File;
use std::io::prelude::*;

fn get_api_key_from_env() -> Result<String, std::env::VarError> {
    env::var("LEXOFFICE_KEY")
}

fn get_api_key_from_file(
    home: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let mut file_name = home.to_string();
    file_name.push_str("/.lexoffice");

    let mut file = File::open(file_name)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let key = contents.trim().to_string();
    return Ok(key);
}

fn get_api_key() -> Result<String, Box<dyn std::error::Error>> {
    match get_api_key_from_env() {
        Ok(val) => return Ok(val),
        Err(_) => (),
    }
    match env::var("HOME") {
        Ok(val) => return get_api_key_from_file(&val),
        Err(_) => (),
    }
    return Err("failed to load API Key".into());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let key = get_api_key()?;
    println!("{:?}", get_api_key());

    let client = Client::create(&key)?;

    let resp = client.get::<lexoffice::data::profile::Profile>().await?;
    println!("{:?}", resp);
    Ok(())
}
