use crate::{args::CliArgs, parse::parse_files, render::Renderer};
use clap::Parser;
use std::path::{Path, PathBuf};
use tokio::{
    fs::{create_dir_all, read_dir, File},
    io::AsyncWriteExt,
};

mod args;
mod parse;
mod render;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    println!("{:?}", args);

    let mut file_paths: Vec<PathBuf> = Vec::new();
    if let Some(dir) = args.input.dir {
        let files = get_bookmark_files_from_dir(&dir).await;
        match files {
            Some(mut values) => file_paths.append(&mut values),
            None => anyhow::bail!(
                "no html files found in {}",
                dir.as_os_str().to_string_lossy()
            ),
        }
    } else {
        let mut paths = args.input.files.unwrap();
        file_paths.append(&mut paths)
    }
    println!("starting parsing of {} files", file_paths.len());
    let links = parse_files(file_paths).await;
    let renderer = Renderer::new(links);
    let (output, ext) = if args.output_type.out_file_grouped_json {
        (renderer.into_grouped_json(), "json")
    } else if args.output_type.out_file_json_array {
        (renderer.into_json_array(), "json")
    } else if args.output_type.out_file_html {
        (renderer.into_html(), "html")
    } else {
        (renderer.into_txt(), "txt")
    };
    let filename = match args.file_name {
        Some(value) => value,
        None => "output.ext".to_owned(),
    };
    let mut out_path = match args.file_path {
        Some(v) => {
            let p = PathBuf::from(v);
            if !p.exists() {
                println!("creating dir since it does not exist");
                create_dir_all(&p).await.expect("could not create dir");
            }
            p
        }
        None => PathBuf::from("./"),
    }
    .join(filename);

    out_path.set_extension(ext);
    println!("saving file to: {:?}", out_path);
    let mut out_file = File::create(out_path).await.unwrap();
    out_file
        .write(output.as_bytes())
        .await
        .expect("could not write output");
    return Ok(());
}

async fn get_bookmark_files_from_dir(dir: &PathBuf) -> Option<Vec<PathBuf>> {
    let mut dir = read_dir(dir).await.expect("cannot read from dir");
    let mut bookmark_files = Vec::new();
    while let Some(entry) = dir.next_entry().await.unwrap() {
        let path = entry.path();
        if path.is_file() && path.extension().unwrap() == "html" {
            bookmark_files.push(path.clone());
        }
    }
    if bookmark_files.len() == 0 {
        return None;
    }
    return Some(bookmark_files);
}
