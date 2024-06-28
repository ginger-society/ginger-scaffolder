use clap::Parser;
use utils::fetch_all_available_templates;

mod new;
mod utils;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The GitHub repository URL
    #[clap(value_parser)]
    git_url: String,
}

fn main() {
    let args = Args::parse();
    fetch_all_available_templates(args.git_url)
}
