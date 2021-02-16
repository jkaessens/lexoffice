use crate::model_builder::result::*;
use proc_macro2::TokenStream;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;
use tempfile::NamedTempFile;

const CACHE_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/index.html");

pub fn load_docs() -> Result<String> {
    println!("{:?}", CACHE_FILE);
    Ok(fs::read_to_string(CACHE_FILE)?)
}

fn quickfmt(token_stream: TokenStream) -> String {
    let mut temp = NamedTempFile::new().unwrap();
    temp.write_all(token_stream.to_string().as_bytes()).unwrap();

    let output = Command::new("rustfmt")
        .args(&["--edition", "2018", "--emit", "stdout"])
        .stdin(temp.reopen().unwrap())
        .output()
        .expect("failed to execute child");

    std::str::from_utf8(&output.stdout).unwrap().to_string()
}

pub fn write_token_stream(
    path: &Path,
    token_stream: TokenStream,
) -> Result<()> {
    let string = quickfmt(token_stream);
    fs::File::create(path)?.write_all(string.as_bytes())?;
    Ok(())
}
