use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD_NO_PAD, URL_SAFE},
    Engine as _,
};

use crate::{get_reader, Base64Format};

pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD_NO_PAD.encode(&buf),
        Base64Format::UrlSafe => URL_SAFE.encode(&buf),
    };
    print!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_reader(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    let buf = buf.trim();

    let decoded = match format {
        Base64Format::Standard => STANDARD_NO_PAD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE.decode(buf)?,
    };
    let decoded = String::from_utf8(decoded)?;
    print!("{}", decoded);
    Ok(())
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        assert!(process_encode(input, Base64Format::Standard).is_ok());
    }
    #[test]
    fn test_process_decode() {
        let input = "fixture/b64.txt";
        process_decode(input, Base64Format::UrlSafe).unwrap();
    }
}
