use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
};

mod assets;
mod compiler;
mod documents;
mod post;
mod posts;

use compiler::Compiler;

fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let output_dir = PathBuf::from(env::var("OUT_DIR")?).join("site");
    let cv_source = manifest_dir.join("cv/cv.typ");
    let blog_dir = manifest_dir.join("blog");
    let asset_dir = manifest_dir.join("assets");
    let typst_dir = manifest_dir.join("typst");

    assert!(manifest_dir.is_absolute());
    assert!(output_dir.is_absolute());

    println!("cargo:rerun-if-changed={}", cv_source.display());
    println!("cargo:rerun-if-changed={}", blog_dir.display());
    println!("cargo:rerun-if-changed={}", asset_dir.display());
    println!("cargo:rerun-if-changed={}", typst_dir.display());
    prepare_output(&output_dir)?;

    // Font and package discovery are expensive, so every Typst document shares one compiler.
    let compiler = Compiler::new(manifest_dir.clone());
    documents::compile_cv(&compiler, &cv_source, &output_dir.join("cv.pdf"))?;
    documents::write_math_font(&compiler, &output_dir.join("fonts/new-cm-math-regular.otf"))?;
    assets::generate(&asset_dir, &output_dir.join("assets.rs"))?;
    posts::compile(&compiler, &blog_dir, &output_dir)?;

    assert!(output_dir.join("assets.rs").is_file());
    assert!(output_dir.join("cv.pdf").is_file());
    assert!(output_dir.join("posts.rs").is_file());
    assert!(output_dir.join("posts").is_dir());
    Ok(())
}

fn prepare_output(output_dir: &Path) -> Result<(), Box<dyn Error>> {
    assert!(output_dir.is_absolute());
    assert_eq!(
        output_dir.file_name().and_then(|name| name.to_str()),
        Some("site")
    );

    if output_dir.exists() {
        fs::remove_dir_all(output_dir)?;
    }
    fs::create_dir_all(output_dir.join("fonts"))?;
    fs::create_dir(output_dir.join("posts"))?;

    assert!(output_dir.join("fonts").is_dir());
    assert!(output_dir.join("posts").is_dir());
    Ok(())
}

pub(crate) fn io_error(message: impl Into<String>) -> std::io::Error {
    let message = message.into();
    assert!(!message.is_empty());
    assert!(message.len() < u32::MAX as usize);

    let error = std::io::Error::other(message);
    assert_eq!(error.kind(), std::io::ErrorKind::Other);
    assert!(!error.to_string().is_empty());
    error
}
