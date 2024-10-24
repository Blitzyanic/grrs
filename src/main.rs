use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();

    println!("pattern: {:?}\n path: {:?}", args.pattern, args.path);
}
