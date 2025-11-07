use png::Decoder;
use std::fs;
use std::fs::File;
use std::io::{BufReader, Write};
use std::process::Command;

pub fn convert_audio() {
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

pub fn convert_images() {
    println!("cargo:rerun-if-changed=images/");

    let img_dir = "images";
    let gen_dir = "src/generated";
    fs::create_dir_all(img_dir).unwrap();
    fs::create_dir_all(gen_dir).unwrap();

    let mut f = File::create(format!("{}/sprites.rs", gen_dir)).unwrap();
    writeln!(f, "// Auto-generated").unwrap();
    writeln!(f, "use std::sync::LazyLock;").unwrap();
    writeln!(f, "use gooseboy::sprite::Sprite;\n").unwrap();

    for entry in fs::read_dir(img_dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) == Some("png") {
            let file_stem = path.file_stem().unwrap().to_str().unwrap();
            let const_name = file_stem.to_uppercase();

            let file = File::open(&path).unwrap();
            let reader = BufReader::new(file);
            let decoder = Decoder::new(reader);
            let mut reader = decoder.read_info().unwrap();
            let mut buf = vec![0; reader.output_buffer_size().expect("failed buffer size")];
            let info = reader.next_frame(&mut buf).unwrap();
            let rgba = &buf[..info.buffer_size()];

            writeln!(f, "#[allow(dead_code)]").unwrap();
            writeln!(
                f,
                "pub static {}: LazyLock<Sprite> = LazyLock::new(|| Sprite::new_blended({}, {}, &[",
                const_name, info.width, info.height
            )
            .unwrap();

            for chunk in rgba.chunks(12) {
                write!(f, "    ").unwrap();
                for byte in chunk {
                    write!(f, "{},", byte).unwrap();
                }
                writeln!(f).unwrap();
            }

            writeln!(f, "]));\n").unwrap();
        }
    }
}
