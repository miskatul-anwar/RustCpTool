use core::time;
use std::thread::sleep;

use cliclack::{confirm, progress_bar};

fn main() {
    let _should_continue = confirm("Do you want to continue?").interact();
    let progress = progress_bar(100);
    progress.start("Installation...");
    for _ in 0..11 {
        sleep(time::Duration::from_millis(1000));
        progress.inc(1);
    }
    progress.stop("Installation Complete");
}
