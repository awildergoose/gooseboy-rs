use std::fs;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=audio/");

    let audio_dir = "audio";
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let raw_dir = format!("{}/raw", out_dir);

    fs::create_dir_all(&raw_dir).unwrap();

    for entry in fs::read_dir(audio_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("wav") {
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let output = format!("{}/{}.raw", raw_dir, file_stem);

            let status = Command::new("ffmpeg")
                .args([
                    "-y",
                    "-i",
                    path.to_str().unwrap(),
                    "-f",
                    "s16le",
                    "-ac",
                    "1",
                    "-ar",
                    "44100",
                    &output,
                ])
                .status()
                .expect("Failed to run ffmpeg");

            if !status.success() {
                panic!("FFmpeg failed on {}", path.display());
            }
        }
    }
}
