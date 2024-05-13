// use std::env;
use std::fs::File;
use std::io::{self, prelude::*, BufReader};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new(".env");
    let file = File::open(&path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.splitn(2, '=');
        let key = parts.next().unwrap().to_uppercase();
        let value = parts.next().unwrap_or_default();
        println!("cargo:rustc-env={}={}", key, value);
    }

    Ok(())
}
