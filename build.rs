use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new(".env");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        let mut parts = line.splitn(2, '=');
        let key = parts.next().unwrap();
        let value = parts.next().unwrap_or("");
        println!("cargo:rustc-env={}={}", key, value);
    }

    Ok(())
}
