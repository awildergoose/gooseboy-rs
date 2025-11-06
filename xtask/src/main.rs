use std::{
    fs,
    path::Path,
    process::{Command, exit},
};

const TARGET: &str = "wasm32-unknown-unknown";

fn main() {
    let mut args = std::env::args();
    let project = {
        args.next();
        args.next()
            .expect("project name should be the first argument")
    };
    do_project(&project);
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
    let dst = Path::new(&std::env::var("GOOSEBOY_SCRIPTS_FOLDER").expect("the GOOSEBOY_SCRIPTS_FOLDER environment variable is missing! (ex: C:\\Users\\MyUser\\AppData\\Roaming\\.minecraft\\gooseboy\\scripts)")).join(filename);

    if !src.exists() {
        eprintln!("error: {} not found", src.display());
        exit(1);
    }

    fs::create_dir_all(dst.parent().unwrap()).unwrap();
    fs::copy(src, dst).unwrap();
}
