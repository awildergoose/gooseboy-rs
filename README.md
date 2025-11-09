<div align="center">
  <h1>gooseboy-rs</h1>
  <a href="https://www.rust-lang.org/"><img alt="Written in Rust" src="https://img.shields.io/badge/Written%20in-Rust-orange?logo=rust&logoColor=white" /></a>
  <a href="https://webassembly.org/"><img alt="Compiled to WebAssembly" src="https://img.shields.io/badge/Compiled%20to-WebAssembly-654FF0?logo=webassembly&logoColor=white" /></a>
  <h3>Official standard library for the <a href="https://github.com/awildergoose/gooseboy">Gooseboy</a> Minecraft mod</h3>
</div>

# Setup

-   Install [Rust 1.82+](rust-lang.org) if you haven't already
-   Install the WebAssembly target:

```bash
rustup target add wasm32-unknown-unknown
```

-   Set the `GOOSEBOY_CRATES_FOLDER` environment variable so your crates automatically get copied to the crates folder, (open the WASM menu, click the open crates folder button)

# Building

#### to compile all crates including examples, run:

```bash
cargo ball
```

to compile a specific project, check `.cargo/config.toml`'s aliases

##### note: for all these commands, you can pass `--release` to make builds that are smaller, and run **_significantly_** faster, but take longer to compile
