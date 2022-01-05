# rust-video
Realtime video rendering in Rust. Inspired by https://nullprogram.com/blog/2017/11/03/

## Running
rust-video outputs to stdout, so you can pipe into a video player like mpv:
```bash
cargo run | mpv --no-correct-pts --fps=60 -
```
