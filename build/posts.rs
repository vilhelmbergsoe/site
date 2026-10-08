use std::{cmp::Reverse, collections::HashSet, error::Error, fmt::Write as _, fs, path::Path};

use crate::{
    compiler::Compiler,
    io_error,
    post::{self, CompiledPost},
};

pub fn compile(
    compiler: &Compiler,
    blog_dir: &Path,
    output_dir: &Path,
) -> Result<(), Box<dyn Error>> {
    assert!(blog_dir.is_dir());
    assert!(output_dir.join("posts").is_dir());

    let mut paths = Vec::new();
    for entry in fs::read_dir(blog_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|extension| extension.to_str()) == Some("typ") {
            paths.push(path);
        }
    }
    paths.sort();

    assert!(!paths.is_empty());
    assert!(paths.iter().all(|path| path.is_file()));

    let mut posts = Vec::with_capacity(paths.len());
    let mut slugs = HashSet::with_capacity(paths.len());
    for path in paths {
        println!("cargo:rerun-if-changed={}", path.display());
        let post = post::compile(compiler, &path)?;
        if !slugs.insert(post.slug.clone()) {
            return Err(io_error(format!("duplicate post slug: {}", post.slug)).into());
        }
        posts.push(post);
    }

    posts.sort_by_key(|post| Reverse(post.date));
    assert_eq!(posts.len(), slugs.len());
    assert!(posts.windows(2).all(|pair| pair[0].date >= pair[1].date));

    let output_path = output_dir.join("posts.rs");
    write_generated_posts(&output_path, &output_dir.join("posts"), &posts)?;
    assert!(output_path.is_file());
    assert!(fs::metadata(output_path)?.len() > 1_024);
    Ok(())
}

fn write_generated_posts(
    output_path: &Path,
    content_dir: &Path,
    posts: &[CompiledPost],
) -> Result<(), Box<dyn Error>> {
    assert!(!posts.is_empty());
    assert!(content_dir.is_dir());

    let mut generated = String::from("static GENERATED_POSTS: &[GeneratedPost] = &[\n");
    for post in posts {
        writeln!(generated, "    GeneratedPost {{")?;
        writeln!(generated, "        url: {:?},", post.slug)?;
        writeln!(generated, "        title: {:?},", post.title)?;
        writeln!(
            generated,
            "        date: ({}, {}, {}),",
            post.date.0, post.date.1, post.date.2
        )?;
        writeln!(generated, "        date_iso: {:?},", post.date_iso)?;
        writeln!(generated, "        date_display: {:?},", post.date_display)?;
        writeln!(generated, "        date_rss: {:?},", post.date_rss)?;
        writeln!(generated, "        archived: {},", post.archived)?;
        write!(generated, "        tags: &[")?;
        for tag in &post.tags {
            write!(generated, "{:?},", tag)?;
        }
        writeln!(generated, "],")?;
        let content_path = content_dir.join(format!("{}.html", post.slug));
        fs::write(&content_path, &post.content)?;
        let written = fs::read_to_string(&content_path)?;
        assert_eq!(written, post.content);
        assert!(!written.is_empty());
        assert_eq!(post.interactive, written.contains("data-live="));
        writeln!(
            generated,
            "        content: include_str!(concat!(env!(\"OUT_DIR\"), \"/site/posts/{}.html\")),",
            post.slug
        )?;
        writeln!(
            generated,
            "        estimated_read_time: {},",
            post.estimated_read_time
        )?;
        writeln!(generated, "        interactive: {},", post.interactive)?;
        writeln!(generated, "    }},")?;
    }
    generated.push_str("];\n");

    assert!(generated.starts_with("static GENERATED_POSTS"));
    assert!(generated.ends_with("];\n"));
    fs::write(output_path, generated)?;
    Ok(())
}
