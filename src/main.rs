mod naive;
mod single_thread_optimized;

#[derive(Parser)]
struct Cli {
    /// The module to run (defaults to "latest")
    module: Option<String>,

    /// The input file path (defaults to "./setup/measurements.txt")
    file_path: Option<String>,
}

use clap::Parser;

const LATEST: &str = "single_thread_optimized";

fn main() {
    let cli = Cli::parse();

    let module = cli.module.as_deref().unwrap_or(LATEST);
    let file_path = cli
        .file_path
        .as_deref()
        .unwrap_or("./setup/measurements.txt");

    match module {
        "naive" => naive::run(file_path),
        "single_thread_optimized" => single_thread_optimized::run(file_path),
        _ => {
            panic!("Unknown module: {}", module);
        }
    }
}
