
use std::env;
use std::io::prelude::*;
use std::process::Command;
use std::path::PathBuf;
use std::fs::File;

fn main() {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("git_version.rs");
    let mut branch = Command::new("git")
        .args(&["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()
        .map(|out| out.stdout)
        .and_then(|out| String::from_utf8(out).ok());

    let mut commit = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .ok()
        .map(|out| out.stdout)
        .and_then(|out| String::from_utf8(out).ok());

    if let Some(br) = branch {
        branch = Some(br.replace('\n', ""))
    } else {
        eprintln!("obtaining git branch failed");
    }

    if let Some(cm) = commit {
        commit = Some(cm.replace('\n', ""))
    } else {
        eprintln!("obtaining git commit failed");
    }

    let mut file = File::create(out_path).expect("couldn't open git_version.rs");

    write!(
        file,
        "const BRANCH: Option<&str> = {:?};\nconst COMMIT: Option<&str> = {:?};",
        branch,
        commit
    ).expect("couldn't write git version");
}
