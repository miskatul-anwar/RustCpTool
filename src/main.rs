use core::time;
use std::thread::sleep;

use cliclack::{confirm, progress_bar};

fn main() {
    let _should_continue = confirm("Do you want to continue?").interact();
    let progress = progress_bar(100);
    progress.start("Installation...");
    for _ in 1..=5 {
        sleep(time::Duration::from_millis(1000));
        progress.inc(20);
    }
    progress.stop("Installation Complete");
}
