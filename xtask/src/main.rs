use std::{fs, process::Command};

fn main() {
    let mut args = std::env::args();
    args.next();
    let project = args
        .next()
        .expect("project name should be the first argument (or use `all`)");

    if project == "all" {
        build_all();
    } else {
        do_project(&project);
    }
}

fn build_all() {
    let entries = fs::read_dir("examples").unwrap_or_else(|_| {
        eprintln!("failed to read examples directory");
        std::process::exit(1);
    });

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                eprintln!("warning: failed to read an entry: {e}");
                continue;
            }
        };
        let path = entry.path();

        let maybe_name = if path.is_dir() {
            path.file_name().map(|s| s.to_string_lossy().into_owned())
        } else {
            None
        };

        let Some(name) = maybe_name else { continue };

        if name.starts_with('.') {
            continue;
        }

        do_project(&name);
    }

    do_project("tests");
}

fn do_project(project: &str) {
    let is_release = std::env::args().any(|a| a == "--release");
    let should_copy = !std::env::args().any(|a| a == "--no-copy");
    let mut cmd = Command::new("cargo-gooseboy");
    cmd.current_dir(std::env::current_dir().expect("failed to get current directory"));
    cmd.args(["pack", project]);

    if !should_copy {
        cmd.arg("--no-copy");
    }

    if is_release {
        cmd.arg("--release");
    }

    let status = cmd
        .status()
        .map_err(|e| {
            panic!(
                "failed to run command `{:?}: {}` at {:?}",
                cmd,
                e,
                cmd.get_current_dir()
            )
        })
        .expect("failed to get status");

    assert!(
        status.success(),
        "failed to run command `{:?}: {:?}` at {:?}",
        cmd,
        status.code(),
        cmd.get_current_dir()
    );
}
