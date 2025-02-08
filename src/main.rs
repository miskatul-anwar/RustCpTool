use fancy::printcoln;
use std::{env, process::exit, time::Instant};

fn time_took(start: Instant) {
    printcoln!("took: [green]{:.2?}\n", start.elapsed());
    exit(1);
}

fn success_message(file_name: &String) {
    printcoln!(
        "[green]PROCESS:[:] Successfully created [yellow|italic]`{}.rs[:]`!",
        file_name
    );
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let start = Instant::now();

    if args.len() < 2 {
        printcoln!("[red]  No file name provided!\n [green] create_rs <file_name>");
        time_took(start);
    }

    let file_name = &args[1];

    if file_name.ends_with(".rs") {
        printcoln!("  Enter the file name without '.rs' extension!\n [italic]e.g.,[:] [:blue]'my_file'[:] instead of [blue]'my_file.rs'");
        time_took(start);
    }

    println!("Processing file: {}", file_name);
    success_message(file_name);
}
