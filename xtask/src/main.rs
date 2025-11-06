use std::{
    fs,
    path::Path,
    process::{Command, exit},
};

const TARGET: &str = "wasm32-unknown-unknown";

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
                eprintln!("warning: failed to read an entry: {}", e);
                continue;
            }
        };
        let path = entry.path();

        let maybe_name = if path.is_dir() {
            path.file_name().map(|s| s.to_string_lossy().into_owned())
        } else {
            None
        };

        let name = match maybe_name {
            Some(n) => n,
            None => continue,
        };

        if name.starts_with(".") {
            continue;
        }

        do_project(&name);
    }

    do_project("tests");
}

fn do_project(project: &str) {
    build(project);
    if !std::env::args().any(|a| a == "--no-copy") {
        copy(&format!("{}.wasm", project).to_owned());
    }
}

fn get_profile() -> String {
    let is_release = std::env::args().any(|a| a == "--release");
    (if is_release { "release" } else { "debug" }).to_string()
}

fn build(project: &str) {
    let profile = get_profile();
    let mut cmd = Command::new("cargo");

    cmd.args(["build", "-p", project, "--target", TARGET]);

    if profile == "release" {
        cmd.arg("--release");
    }

    let status = cmd.status().expect("failed to run cargo build");

    if !status.success() {
        eprintln!("build failed");
        std::process::exit(1);
    }
}

fn copy(filename: &str) {
    let profile = get_profile();
    let src_str = format!("target/{}/{}/{}", TARGET, profile, filename);
    let src = Path::new(&src_str);
    let dst = Path::new(&std::env::var("GOOSEBOY_SCRIPTS_FOLDER").expect(
        "the GOOSEBOY_SCRIPTS_FOLDER environment variable is missing! (ex: C:\\Users\\MyUser\\AppData\\Roaming\\.minecraft\\gooseboy\\scripts)"
    )).join(filename);

    if !src.exists() {
        eprintln!("error: {} not found", src.display());
        exit(1);
    }

    fs::create_dir_all(dst.parent().unwrap()).unwrap();
    fs::copy(src, dst).unwrap();
}
