use clap::{Args, Parser};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct CliArgs {
    #[command(flatten)]
    pub input: InputArgs,
    ///name of file that should be used for the output.
    ///Do not specify extension as it is added automatically
    #[arg(short = 'n', long = "file_name")]
    pub file_name: Option<String>,
    ///the path for output file. defaults to the current directory
    #[arg(short = 'o', long = "out")]
    pub file_path: Option<String>,

    #[command(flatten)]
    pub output_type: OutputType,
}
#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct OutputType {
    ///formats the extracted data as html
    #[arg(long = "html", group = "output-type")]
    pub out_file_html: bool,
    ///formats the extracted data into a  jon array of objects based on domain
    #[arg(long = "json", group = "output-type")]
    pub out_file_grouped_json: bool,
    ///formats the extracted data into a json array
    #[arg(long = "array", group = "output-type")]
    pub out_file_json_array: bool,
    ///formats data as a txt file with newline seperation of values
    #[arg(long = "txt", group = "output-type")]
    pub out_file_txt: bool,
}

#[derive(Args, Debug)]
#[group(required = true, multiple = false)]
pub struct InputArgs {
    ///files from which links need to be extracted from.
    #[arg(short, long)]
    pub files: Option<Vec<PathBuf>>,
    ///if this flag is used all .html files with the doctype NETSCAPE-Bookmark-file-1 in the directory will be parsed
    #[arg(short, long)]
    pub dir: Option<PathBuf>,
}
