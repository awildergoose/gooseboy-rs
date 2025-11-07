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
            let out_dir = std::env::var("OUT_DIR").unwrap();
            let out_bin = format!("{}/{}.bin", out_dir, file_stem);

            let file = File::open(&path).unwrap();
            let decoder = png::Decoder::new(BufReader::new(file));
            let mut reader = decoder.read_info().unwrap();

            let info = reader.info();
            let width = info.width;
            let height = info.height;
            let color = info.color_type;

            let mut buf = vec![0; reader.output_buffer_size().unwrap()];
            let frame_info = reader.next_frame(&mut buf).unwrap();

            let pixels = match color {
                png::ColorType::Rgb => {
                    let mut rgba = Vec::with_capacity((width * height * 4) as usize);
                    for chunk in buf[..frame_info.buffer_size()].chunks_exact(3) {
                        rgba.extend_from_slice(&[chunk[0], chunk[1], chunk[2], 255]);
                    }
                    rgba
                }
                png::ColorType::Rgba => buf[..frame_info.buffer_size()].to_vec(),
                _ => panic!("unsupported color type: {:?}", color),
            };

            std::fs::write(out_bin, pixels).unwrap();

            writeln!(f, "#[allow(dead_code)]").unwrap();
            writeln!(
                f,
                "pub static {}: LazyLock<Sprite> = LazyLock::new(|| {{
    let data = include_bytes!(concat!(env!(\"OUT_DIR\"), \"/{}\")); 
    Sprite::new_blended({}, {}, data)
}});\n",
                const_name,
                format_args!("{}.bin", file_stem),
                width,
                height
            )
            .unwrap();
        }
    }
}
