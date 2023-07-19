pub mod os_operation;

use os_operation::path::FileDescription;
use std::{fs, process::exit, collections::HashMap};
use clap::Parser;
use rayon::prelude::*;


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// page number
    #[arg(short, long, help = "path that will be listed")]
    path: String,

    #[arg(short, long, help = "format size that will be printed (MB/GB/TB)")]
    format: String,
}

fn main() {
    let args = Args::parse();
    let path = args.path;
    let format = args.format;

    let path_operation = FileDescription::new();
    let divider: f32;

    let  files: Vec<FileDescription> = path_operation.list_files(&path);

    divider = match format.as_str() {
        "MB" => 1024.0 * 1024.0,
        "GB" => 1024.0 * 1024.0 * 1024.0,
        "TB" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
        _ => {
            eprintln!("invalid format: {}\n please run with parameter --help for details", format);
            exit(0)
        },
    };

    let result: Vec<_> = files.par_iter()
    .map(|file| {
        let file_name = format!("{}/{}", path, file.get_file_name());
        let file_size = fs::metadata(&file_name).unwrap().len() as f32 / divider;
        (file_name, file_size)
    })
    .collect();

    for data in result{
        println!("{}\t{}\t {}",data.0,data.1,format)
    }
}
