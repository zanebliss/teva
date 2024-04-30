use std::env::args;

mod git_client;
mod runners;

fn main() {
    let cached_files: Vec<String> = vec![];
    let args: Vec<String> = args().collect();
    let mut from_sha = "main";

    if args.len() > 1 {
        from_sha = &args[1];
    }

    git_client::do_work(from_sha.to_string(), cached_files)
}
