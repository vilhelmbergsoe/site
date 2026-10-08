use std::{
    collections::HashSet,
    error::Error,
    fmt::Write as _,
    fs,
    path::{Component, Path},
};

use crate::io_error;

pub fn generate(source_dir: &Path, output_path: &Path) -> Result<(), Box<dyn Error>> {
    assert!(source_dir.is_dir());
    assert_eq!(
        output_path.extension().and_then(|value| value.to_str()),
        Some("rs")
    );

    let mut assets = Vec::new();
    collect_source_assets(source_dir, source_dir, &mut assets)?;
    assets.push(Asset::generated("cv.pdf", "cv.pdf")?);
    assets.push(Asset::generated(
        "fonts/new-cm-math-regular.otf",
        "fonts/new-cm-math-regular.otf",
    )?);
    assets.sort_by(|left, right| left.public_path.cmp(&right.public_path));

    let mut paths = HashSet::with_capacity(assets.len());
    for asset in &assets {
        if !paths.insert(asset.public_path.clone()) {
            return Err(io_error(format!(
                "duplicate public asset path: {}",
                asset.public_path
            ))
            .into());
        }
    }

    assert!(!assets.is_empty());
    assert_eq!(assets.len(), paths.len());
    write_manifest(output_path, &assets)?;
    assert!(output_path.is_file());
    assert!(fs::metadata(output_path)?.len() > 1_024);
    Ok(())
}

fn collect_source_assets(
    root: &Path,
    directory: &Path,
    assets: &mut Vec<Asset>,
) -> Result<(), Box<dyn Error>> {
    assert!(root.is_dir());
    assert!(directory.is_dir());

    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_source_assets(root, &entry.path(), assets)?;
        } else {
            if file_type.is_file() {
                let metadata = entry.metadata()?;
                if metadata.len() == 0 {
                    return Err(io_error(format!(
                        "asset must not be empty: {}",
                        entry.path().display()
                    ))
                    .into());
                }
                assets.push(Asset::source(root, &entry.path())?);
            } else {
                return Err(io_error(format!(
                    "asset must be a regular file: {}",
                    entry.path().display()
                ))
                .into());
            }
        }
    }

    assert!(directory.starts_with(root));
    assert!(assets.len() <= u32::MAX as usize);
    Ok(())
}

fn write_manifest(output_path: &Path, assets: &[Asset]) -> Result<(), Box<dyn Error>> {
    assert!(!assets.is_empty());
    assert!(assets
        .windows(2)
        .all(|pair| pair[0].public_path < pair[1].public_path));

    let mut generated = String::from("static ASSETS: &[Asset] = &[\n");
    for asset in assets {
        writeln!(generated, "    Asset {{")?;
        writeln!(generated, "        path: {:?},", asset.public_path)?;
        writeln!(generated, "        content_type: {:?},", asset.content_type)?;
        match asset.location {
            Location::Source => writeln!(
                generated,
                "        data: include_bytes!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/assets/\", {:?})),",
                asset.include_path
            )?,
            Location::Output => writeln!(
                generated,
                "        data: include_bytes!(concat!(env!(\"OUT_DIR\"), \"/site/\", {:?})),",
                asset.include_path
            )?,
        }
        writeln!(generated, "    }},")?;
    }
    generated.push_str("];\n");

    assert!(generated.starts_with("static ASSETS"));
    assert!(generated.ends_with("];\n"));
    fs::write(output_path, generated)?;
    Ok(())
}

fn asset_path(root: &Path, path: &Path) -> Result<String, Box<dyn Error>> {
    assert!(root.is_absolute());
    assert!(path.is_file());

    let relative = path.strip_prefix(root)?;
    let mut parts = Vec::new();
    for component in relative.components() {
        match component {
            Component::Normal(part) => {
                parts.push(part.to_str().ok_or_else(|| {
                    io_error(format!("asset path is not UTF-8: {}", path.display()))
                })?)
            }
            _ => {
                return Err(io_error(format!("invalid asset path: {}", path.display())).into());
            }
        }
    }
    let public_path = parts.join("/");

    assert!(!public_path.is_empty());
    assert!(!public_path.starts_with('/'));
    Ok(public_path)
}

fn content_type(path: &str) -> Result<&'static str, Box<dyn Error>> {
    assert!(!path.is_empty());
    assert!(!path.ends_with('/'));

    let extension = Path::new(path)
        .extension()
        .and_then(|value| value.to_str())
        .ok_or_else(|| io_error(format!("asset has no extension: {path}")))?;
    let content_type = match extension {
        "css" => "text/css; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "otf" => "font/otf",
        "pdf" => "application/pdf",
        "png" => "image/png",
        "svg" => "image/svg+xml",
        "txt" => "text/plain; charset=utf-8",
        "webp" => "image/webp",
        "woff2" => "font/woff2",
        extension => {
            return Err(
                io_error(format!("unsupported asset extension .{extension}: {path}")).into(),
            );
        }
    };

    assert!(content_type.contains('/'));
    assert!(!content_type.starts_with('/'));
    Ok(content_type)
}

struct Asset {
    public_path: String,
    content_type: &'static str,
    include_path: String,
    location: Location,
}

impl Asset {
    fn source(root: &Path, path: &Path) -> Result<Self, Box<dyn Error>> {
        assert!(root.is_dir());
        assert!(path.is_file());

        let public_path = asset_path(root, path)?;
        let content_type = content_type(&public_path)?;

        assert!(!public_path.is_empty());
        assert!(content_type.contains('/'));
        Ok(Self {
            include_path: public_path.clone(),
            public_path,
            content_type,
            location: Location::Source,
        })
    }

    fn generated(public_path: &str, include_path: &str) -> Result<Self, Box<dyn Error>> {
        assert!(!public_path.is_empty());
        assert!(!include_path.is_empty());

        let content_type = content_type(public_path)?;

        assert!(!public_path.starts_with('/'));
        assert!(!include_path.starts_with('/'));
        Ok(Self {
            public_path: public_path.to_owned(),
            content_type,
            include_path: include_path.to_owned(),
            location: Location::Output,
        })
    }
}

#[derive(Clone, Copy)]
enum Location {
    Source,
    Output,
}
