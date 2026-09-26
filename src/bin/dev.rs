use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
    process::{Child, Command},
    sync::mpsc,
    thread,
    time::Duration,
};

use notify::{Event, EventKind, RecursiveMode, Watcher};

const WATCH_PATHS: &[&str] = &["src", "blog", "assets", "cv", "Cargo.toml", "Cargo.lock"];

fn main() -> Result<(), Box<dyn Error>> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    assert!(root.is_absolute());
    assert!(root.join("Cargo.toml").is_file());

    let (sender, receiver) = mpsc::channel();
    let mut watcher = notify::recommended_watcher(move |event: notify::Result<Event>| {
        if sender.send(event).is_err() {
            eprintln!("file watcher stopped");
        }
    })?;

    for relative_path in WATCH_PATHS {
        let path = root.join(relative_path);
        assert!(path.exists());
        assert!(path.starts_with(root));
        let mode = if path.is_dir() {
            RecursiveMode::Recursive
        } else {
            RecursiveMode::NonRecursive
        };
        watcher.watch(&path, mode)?;
    }

    let mut server = if build_site(root)? {
        Some(spawn_site(root)?)
    } else {
        None
    };

    println!("watching for changes");

    // This is the development supervisor's event loop; it terminates when the
    // process receives a signal or the watcher disconnects.
    loop {
        let event = receiver.recv()?;
        let mut rebuild = match event {
            Ok(event) => matches!(
                event.kind,
                EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
            ),
            Err(error) => {
                eprintln!("watch error: {error}");
                false
            }
        };

        // Editors commonly produce a burst of rename and write events for one
        // save, so collapse them into a single build.
        thread::sleep(Duration::from_millis(150));
        while let Ok(event) = receiver.try_recv() {
            match event {
                Ok(event) => {
                    if matches!(
                        event.kind,
                        EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                    ) {
                        rebuild = true;
                    }
                }
                Err(error) => eprintln!("watch error: {error}"),
            }
        }

        if rebuild {
            println!("change detected; rebuilding");
            if build_site(root)? {
                if let Some(mut process) = server.take() {
                    if process.try_wait()?.is_none() {
                        process.kill()?;
                    }
                    process.wait()?;
                }
                server = Some(spawn_site(root)?);
            }
        }
    }
}

fn build_site(root: &Path) -> Result<bool, Box<dyn Error>> {
    assert!(root.is_absolute());
    assert!(root.join("Cargo.toml").is_file());

    let status = Command::new("cargo")
        .args(["build", "--features", "dev", "--bin", "site"])
        .current_dir(root)
        .status()?;

    assert!(status.code().is_some());
    assert_eq!(status.success(), status.code() == Some(0));
    Ok(status.success())
}

fn spawn_site(root: &Path) -> Result<Child, Box<dyn Error>> {
    let executable = site_executable(root);
    assert!(executable.is_file());
    assert!(executable.is_absolute());
    if env::var_os("CARGO_TARGET_DIR").is_none() {
        assert!(executable.starts_with(root));
    }

    println!("starting {}", executable.display());
    Ok(Command::new(executable).current_dir(root).spawn()?)
}

fn site_executable(root: &Path) -> PathBuf {
    assert!(root.is_absolute());
    assert!(root.join("Cargo.toml").is_file());

    let mut target = env::var_os("CARGO_TARGET_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target"));
    if target.is_relative() {
        target = root.join(target);
    }
    target.push("debug");
    target.push(format!("site{}", env::consts::EXE_SUFFIX));

    assert!(target.is_absolute());
    assert!(target.ends_with(format!("site{}", env::consts::EXE_SUFFIX)));
    target
}
