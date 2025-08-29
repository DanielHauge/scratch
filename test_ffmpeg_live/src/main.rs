use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() -> std::io::Result<()> {
    // Collect all frames (sorted)
    let mut frames: Vec<_> = fs::read_dir("frames")?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.is_file() && p.extension().map(|ext| ext == "jpg").unwrap_or(false))
        .collect();

    frames.sort();

    // Start ffmpeg process
    // We're streaming JPEGs via image2pipe
    let mut child = Command::new("ffmpeg")
        .args([
            "-y",
            "-f",
            "image2pipe",
            "-use_wallclock_as_timestamps",
            "1",
            "-i",
            "-",
            "-c:v",
            "libx264",
            "-pix_fmt",
            "yuv420p",
            "output_variable.mp4",
        ])
        // .args([
        //     "-y", // overwrite output file
        //     "-f",
        //     "image2pipe",
        //     "-framerate",
        //     "30",
        //     "-i",
        //     "-", // stdin
        //     "-c:v",
        //     "libx264",
        //     "-pix_fmt",
        //     "yuv420p",
        //     "output.mp4",
        // ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("failed to start ffmpeg");

    let stdin = child.stdin.as_mut().expect("failed to open stdin");

    // Stream frames to ffmpeg
    for frame in frames {
        let data = fs::read(&frame)?;
        stdin.write_all(&data)?;
        stdin.flush()?;
    }

    drop(stdin); // close stdin so ffmpeg knows we're done
    child.wait()?; // wait for ffmpeg to finish

    Ok(())
}
