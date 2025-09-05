use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() -> anyhow::Result<()> {
    let mut ffmpeg = Command::new("ffmpeg")
        .args([
            "-f",
            "rawvideo",
            "-pix_fmt",
            "rgb24",
            "-s",
            "640x480",
            "-r",
            "30",
            "-i",
            "-",
            "-c:v",
            "libvpx",
            "-f",
            "webm",
            "-content_type",
            "video/webm",
            "-listen",
            "1",
            "http://0.0.0.0:8080/feed.webm",
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to start ffmpeg");

    let mut stdin = ffmpeg.stdin.take().unwrap();
    let frame_size = 640 * 480 * 3;

    thread::spawn(move || {
        let mut color: u8 = 0;
        loop {
            let frame: Vec<u8> = vec![color; frame_size];
            if stdin.write_all(&frame).is_err() {
                break;
            }
            color = color.wrapping_add(1);
            thread::sleep(Duration::from_millis(33));
        }
    });

    ffmpeg.wait()?;
    Ok(())
}
