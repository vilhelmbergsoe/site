use std::{error::Error, fs, path::Path};

use typst_layout::PagedDocument;

use crate::{
    compiler::{emit_diagnostics, emit_warnings, Compiler},
    io_error,
};

pub fn compile_cv(
    compiler: &Compiler,
    source_path: &Path,
    output_path: &Path,
) -> Result<(), Box<dyn Error>> {
    assert!(source_path.is_file());
    assert_eq!(
        source_path
            .extension()
            .and_then(|extension| extension.to_str()),
        Some("typ")
    );

    let source = fs::read_to_string(source_path)?;
    assert!(source.contains("Vilhelm Bergsøe"));
    assert!(!source.is_empty());

    let world = compiler.world("cv/cv.typ", source);
    let compiled = typst::compile::<PagedDocument>(&world);
    emit_warnings(&world, &compiled.warnings)?;

    let document = match compiled.output {
        Ok(document) => document,
        Err(diagnostics) => {
            emit_diagnostics(&world, &diagnostics)?;
            return Err(io_error("Typst CV compilation failed").into());
        }
    };
    let pdf = match typst_pdf::pdf(&document, &typst_pdf::PdfOptions::default()) {
        Ok(pdf) => pdf,
        Err(diagnostics) => {
            emit_diagnostics(&world, &diagnostics)?;
            return Err(io_error("Typst PDF export failed").into());
        }
    };

    assert!(pdf.starts_with(b"%PDF-"));
    assert!(pdf.len() > 1_024);
    fs::write(output_path, pdf)?;

    let written = fs::read(output_path)?;
    assert!(written.starts_with(b"%PDF-"));
    assert!(written.len() > 1_024);
    Ok(())
}

pub fn write_math_font(compiler: &Compiler, output_path: &Path) -> Result<(), Box<dyn Error>> {
    assert_eq!(
        output_path
            .extension()
            .and_then(|extension| extension.to_str()),
        Some("otf")
    );
    assert!(output_path.is_absolute());

    let font = compiler
        .math_font()
        .ok_or_else(|| io_error("typst-assets does not contain NewCMMath-Regular"))?;

    assert_eq!(font.info().family, "New Computer Modern Math");
    assert!(font.data().len() > 100_000);
    fs::write(output_path, font.data())?;

    let written = fs::read(output_path)?;
    assert_eq!(written.len(), font.data().len());
    assert!(written.len() > 100_000);
    Ok(())
}
