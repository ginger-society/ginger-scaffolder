use clap::Parser;
use new::new_project;

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
    if let Some((username, repo_name)) = extract_username_and_repo(&args.git_url) {
        new_project(format!("{}/{}", username, repo_name));
    } else {
        eprintln!("Invalid Git URL: {}", args.git_url);
        std::process::exit(1);
    }
}

fn extract_username_and_repo(git_url: &str) -> Option<(String, String)> {
    let re = regex::Regex::new(r"^https://github.com/([^/]+)/([^/]+)$").unwrap();
    if let Some(captures) = re.captures(git_url) {
        let username = captures.get(1)?.as_str().to_string();
        let repo_name = captures.get(2)?.as_str().to_string();
        Some((username, repo_name))
    } else {
        None
    }
}
