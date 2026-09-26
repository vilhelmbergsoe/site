use std::{env, fs, io, path::PathBuf};

const CV: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/site/cv.pdf"));

fn main() -> io::Result<()> {
    assert!(CV.starts_with(b"%PDF-"));
    assert!(CV.len() > 1_024);

    let mut arguments = env::args_os().skip(1);
    let output = arguments
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("cv.pdf"));
    if arguments.next().is_some() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "usage: cargo run --bin cv -- [output.pdf]",
        ));
    }

    assert!(!output.as_os_str().is_empty());
    assert_ne!(output.file_name(), None);
    fs::write(&output, CV)?;

    let written = fs::read(&output)?;
    assert_eq!(written.len(), CV.len());
    assert_eq!(written, CV);
    println!("wrote {}", output.display());
    Ok(())
}
