use std::env;
use std::os::linux::fs::MetadataExt;
use std::os::unix::fs::MetadataExt as foo;
use std::path::Path;
use std::process;

mod utils;
use utils::*;

fn main() {
    // collect file path from command line
    let file_path = env::args();
    let file_path: Vec<String> = file_path.collect();

    if file_path.len() != 2 {
        println!("didn't provide path to file, program quit");
        process::exit(1);
    }
    let file_path: &str = &file_path[1];

    let path = Path::new(file_path);

    let metadata = path.metadata().expect("metadata call failed");

    println!();
    println!("\tpath: {:#?}", path);
    println!("\ttype: {}", get_type(metadata.clone()));
    #[cfg(target_os = "linux")]

    println!(
        "\t\t    blocks : {}",
        format_memory_blocks(metadata.blocks())
    );
    println!(
        "\t\tblock size : {:?}",
        format_memory_blocks(metadata.st_blksize())
    );
    println!(
        "\t\t file size : {:?}",
        format_memory_blocks(metadata.size())
    );

    println!("\tdevice id : {:?}", metadata.dev());
    println!("\t  user id : {:?}", metadata.st_uid());
    println!("\t group id : {:?}", metadata.st_gid());

    println!("\t\tfile mode: {:?}", metadata.st_mode() & 0o777);

    println!("\tmodified : {:?}", get_formatted_date(metadata.mtime()));
    println!("\taccessed : {:?}", get_formatted_date(metadata.atime()));
    println!("\t created : {:?}", get_formatted_date(metadata.ctime()));

    #[cfg(target_os = "windows")]
    println!("file size: {} bytes", metadata.file_size());

    println!();
}
