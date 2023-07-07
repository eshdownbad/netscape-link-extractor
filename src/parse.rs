use once_cell::sync::Lazy;
use regex::Regex;
use std::path::PathBuf;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub async fn parse_files(file_paths: Vec<PathBuf>) -> Vec<String> {
    let mut handles = Vec::new();
    for path in file_paths {
        handles.push(tokio::spawn(parse_file(path)));
    }
    let mut links = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(mut values) => links.append(&mut values),
            Err(e) => eprintln!("error while parsing: {:?}", e),
        }
    }
    return links;
}

pub async fn parse_file(file_path: PathBuf) -> Vec<String> {
    static HREF_RE: Lazy<Regex> = Lazy::new(|| {
        Regex::new("(?i)href=\"(?P<link>\\S+)\"").expect("could not parse href regex")
    });
    let file = File::open(file_path).await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut links = Vec::new();
    while let Some(line) = lines.next_line().await.unwrap() {
        let matches_iter = HREF_RE
            .captures_iter(&line)
            .filter_map(|cap| cap.get(1))
            .map(|m| m.as_str().to_owned());
        links.extend(matches_iter);
    }
    return links;
}
