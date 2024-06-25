use new::new_project;
use std::env;

mod new;
mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <git-url>", args[0]);
        std::process::exit(1);
    }

    let git_url = &args[1];
    if let Some((username, repo_name)) = extract_username_and_repo(git_url) {
        new_project(format!("{}/{}", username, repo_name));
    } else {
        eprintln!("Invalid Git URL: {}", git_url);
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
