# rust-video
Realtime video rendering in Rust. Inspired by https://nullprogram.com/blog/2017/11/03/

## Running
rust-video outputs to stdout, so you can pipe into a video player like mpv:
```bash
cargo run --release | mpv --no-correct-pts --fps=60 -
```
> **NOTE**: Running in debug mode will almost definitely cause buffering
