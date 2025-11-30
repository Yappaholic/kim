use clap::Parser;

#[derive(Parser)]
#[command(version, about = "KIM -> Kakoune IMproved", long_about = None)]
struct Cli {
  path: Option<String>,
  #[arg(short, long)]
  zen: bool,
}

// Some eye candy for the CLI
pub fn parse_args() -> Option<String> {
  let cli = Cli::parse();
  if cli.zen == true {
    println!("Make some good tools already!");
    std::process::exit(69);
  }
  cli.path.as_deref().map(|path| path.to_string())
}
