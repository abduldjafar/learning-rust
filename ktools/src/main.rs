pub mod os_operation;
pub mod os_monitoring;

use os_operation::path::FileDescription;
use std::{fs, process::exit};
use clap::Parser;
use rayon::prelude::*;
use os_monitoring::SystemStats;

/// Simple program to get information about RAM, CPUS and file sizes in a path
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, help = "kind of process that want to execute (get_files_information,cpus,ram,mounting_path)")]
    stats: String,

    #[arg(short, long, help = "path that will be listed should be use when stats: get_files_information",default_value=".")]
    path: String,

    #[arg(short,long, help = "format size that will be printed (MB/GB/TB). should be use when stats: get_files_information",default_value="GB")]
    file_format: String,
}

fn main() {
    let args = Args::parse();
    let path = args.path;
    let format = args.file_format;
    let stats = args.stats;

    let path_operation = FileDescription::new();
    let systemstat = SystemStats::new();

    match stats.as_str() {
        "ram" => {
            systemstat.monitoring_memory_resources();
        }
        "cpus" => {
            println!("still developing");
        }
        "mounting_path" => {
            systemstat.monitoring_mounted_path();
        }
        "get_files_information" => {
            let divider: f32;

            let files: Vec<FileDescription> = path_operation.list_files(&path);

            divider = match format.as_str() {
                "MB" => 1024.0 * 1024.0,
                "GB" => 1024.0 * 1024.0 * 1024.0,
                "TB" => 1024.0 * 1024.0 * 1024.0 * 1024.0,
                _ => {
                    eprintln!("Invalid format: {}\nPlease run with parameter --help for details", format);
                    exit(0);
                }
            };

            let result: Vec<_> = files
                .par_iter()
                .map(|file| {
                    let file_name = format!("{}/{}", path, file.get_file_name());
                    let file_size = fs::metadata(&file_name).unwrap().len() as f32 / divider;
                    (file_name, file_size)
                })
                .collect();

            for data in result {
                println!("{}\t{}\t{}", data.0, data.1, format);
            }
        }
        _ => {
            println!("Please run with parameter --help for details");
        }
    }
}
