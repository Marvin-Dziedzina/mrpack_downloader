use std::{
    env,
    ffi::OsStr,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use clap::{Arg, Command};
use mrpack::MrPack;

mod mrpack;

const VERSION: &str = env!("CARGO_PKG_VERSION");

const OUT_DIRECTORY: &str = "out_mrpack";

fn main() -> Result<(), &'static str> {
    let matches = Command::new("MRPACK Downloader")
        .version(VERSION)
        .about("MRPACK DOWNLOADER downloads all mods from a .mrpack Modrinth modpack")
        .arg(
            Arg::new("mrpack_path")
                .long("mrpack_path")
                .short('p')
                .help("The path to the .mrpack unpacked modpack file")
                .required(true),
        )
        .arg(
            Arg::new("out_path")
                .long("out_path")
                .short('o')
                .help("The path where the out_mrpack directory gets created")
                .default_value("./"),
        )
        .get_matches();

    println!("Preparing paths");

    let mrpack_path_string = matches.get_one::<String>("mrpack_path").unwrap();
    let mrpack_path = PathBuf::from(mrpack_path_string);

    let out_path_string = matches.get_one::<String>("out_path").unwrap();
    let out_path_base = PathBuf::from(out_path_string);
    let mut out_path: PathBuf = out_path_base.clone();
    out_path.push(OUT_DIRECTORY);
    fs::create_dir_all(&out_path).expect("Failed to create all folders that are missing");

    println!("Created directories");

    println!("Check if mrpack exists");
    if !fs::exists(&mrpack_path).expect("Failed to verify that mrpack path exists") {
        return Err("Failed to find MRPACK file");
    };

    println!("Mrpack exists");

    let mrpack = get_mrpack(&mrpack_path);
    println!("Deserialization of mrpack done");

    let mut successful_downloaded: Vec<String> = Vec::new();
    let mut failed_downloads: Vec<String> = Vec::new();

    println!("Starting to download mods...");
    for file in mrpack.files.iter() {
        let mod_name: &str = match file
            .path
            .file_name()
            .unwrap_or(OsStr::new("Unknown Mod"))
            .to_str()
        {
            Some(mod_name) => mod_name,
            None => "Unknown Mod",
        };

        let mut mod_path = out_path.clone();
        mod_path.push(mod_name);

        println!("Downloading {}...", mod_name);

        let mut download_failed = false;
        for download_link in file.downloads.iter() {
            println!("Downloading from {}", download_link);

            let mod_data = match reqwest::blocking::get(download_link) {
                Ok(mod_data) => mod_data,
                Err(e) => {
                    download_failed = true;
                    println!(
                        "Failed to download {} from link \"{}\" with error: {}",
                        mod_name, download_link, e
                    );
                    continue;
                }
            };

            let mut file = match File::create(&mod_path) {
                Ok(file) => file,
                Err(e) => {
                    download_failed = true;
                    println!("{} failed to install with error: {}", mod_name, e);
                    continue;
                }
            };
            let mod_bytes = match mod_data.bytes() {
                Ok(bytes) => bytes,
                Err(e) => {
                    download_failed = true;
                    println!("{} failed to get bytes with error: {}", mod_name, e);
                    continue;
                }
            };

            match file.write_all(&mod_bytes) {
                Ok(_) => (),
                Err(e) => {
                    download_failed = true;
                    println!("{} failed to write to drive with error: {}", mod_name, e);
                    continue;
                }
            };

            println!("{} sucessfully downloaded!", mod_name);
            successful_downloaded.push(mod_name.to_owned());
            break;
        }

        if download_failed {
            failed_downloads.push(mod_name.to_owned());
        };
    }

    println!(
        "Successfully installed {} from {} mods",
        successful_downloaded.len(),
        successful_downloaded.len() + failed_downloads.len()
    );

    let mut successful_mods: String = String::new();
    for success_mod in successful_downloaded.iter() {
        successful_mods.push_str(success_mod);
        successful_mods.push('\n');
    }
    println!(
        "Successful downloaded mods {}:\n{}",
        successful_downloaded.len(),
        successful_mods
    );

    let mut failed_mods: String = String::new();
    for failed_mod in failed_downloads.iter() {
        failed_mods.push_str(failed_mod);
        failed_mods.push('\n');
    }
    println!(
        "Failed downloaded mods {}:\n{}",
        failed_downloads.len(),
        failed_mods
    );

    Ok(())
}

fn get_mrpack(mrpack_path: &PathBuf) -> MrPack {
    let mrpack = File::open(&mrpack_path).expect("Failed to open mrpack file");
    serde_json::from_reader(mrpack).expect("Failed to read mrpack")
}
