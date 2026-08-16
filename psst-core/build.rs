use gix_config::File;
use std::{env, fs, io::Write};
use time::OffsetDateTime;

fn main() {
    let outdir = env::var("OUT_DIR").unwrap();
    let outfile = format!("{outdir}/build-time.txt");

    let mut fh = fs::File::create(outfile).unwrap();
    let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
    write!(fh, r#""{now}""#).ok();

    let remote_url =
        get_remote_url().unwrap_or_else(|| "https://github.com/mateusdcc/psst".to_string());

    let outfile = format!("{outdir}/remote-url.txt");
    let mut file = fs::File::create(outfile).unwrap();
    write!(file, r#""{remote_url}""#).ok();
}

fn get_remote_url() -> Option<String> {
    let git_config = File::from_git_dir("../.git/".into()).ok()?;
    let raw = git_config.raw_value("remote.origin.url").ok()?;
    let mut url = raw.to_string();

    if url.contains('@') {
        let (domain, path) = url.strip_prefix("git@")?.split_once(':')?;
        url = format!("https://{domain}/{path}");
    }
    Some(url.trim_end_matches(".git").to_string())
}
