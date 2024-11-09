use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args: Cli = Cli::parse();

    // TODO: add Bufreader instead of read_to_string
    let content: String = std::fs::read_to_string(&args.path)
        .expect("could not read file");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
