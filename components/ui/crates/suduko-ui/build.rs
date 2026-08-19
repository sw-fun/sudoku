// Bake build provenance into the binary: host, short commit sha, UTC
// timestamp. Rerun when HEAD moves so the sha stays fresh.
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=../../../../.git/HEAD");

    let out = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR set"));
    let src = format!(
        "pub const GIT_SHA: &str = \"{}\";\npub const BUILD_HOST: &str = \"{}\";\npub const BUILD_TIMESTAMP: &str = \"{}\";\n",
        shell("git", &["rev-parse", "--short", "HEAD"]),
        shell("hostname", &["-s"]),
        shell("date", &["-u", "+%Y-%m-%d %H:%M UTC"]),
    );
    fs::write(out.join("build_info.rs"), src).expect("write build_info.rs");
}

fn shell(program: &str, args: &[&str]) -> String {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "unknown".into())
}
