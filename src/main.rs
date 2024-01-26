use clap::Parser;

mod command;
mod interface;

#[derive(Parser)]
struct Cli {
    pattern: String,
    // path: std::path::PathBuf,
}

#[allow(unused_variables)]
fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    dotenv::dotenv().ok();

    match args.pattern.as_str() {
        "test" => command::verification()?,
        "diff" => command::diff()?,
        "sync" => command::sync()?,
        _ => println!("no such command: {:?}", args.pattern),
    }

    Ok(())
}
