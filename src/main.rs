use clap::{Arg, Command, arg, command};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let matches = Command::new("MRPACK Downloader")
        .version(VERSION)
        .about("MRPACK DOWNLOADER downloads all mods from a .mrpack Modrinth modpack")
        .arg(
            Arg::new("mrpack_path")
                .long("mrpack_path")
                .short('p')
                .help("The path to the .mrpack modpack file"),
        )
        .arg(
            Arg::new("out_path")
                .long("out_path")
                .short('o')
                .help("The path where the out_mrpack directory gets created")
                .default_value("./"),
        )
        .get_matches();
}
