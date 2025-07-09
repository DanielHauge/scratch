use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let bar = ProgressBar::new(5252);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] - ({eta_precise}) {bar:40.cyan/blue} {percent}% {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );
    for _ in 0..5252 {
        bar.inc(1);
        std::thread::sleep(std::time::Duration::from_millis(5));
    }
    bar.finish_with_message("Done!");
}
