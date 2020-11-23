use clap::{App, Arg};

pub fn parse_args() -> (String, String) {
    let matches = App::new("XOR Decryption")
        .arg(
            Arg::with_name("cipher")
                .short("c")
                .long("cipher")
                .help("Path of the cipher file")
                .takes_value(true)
                .default_value("cipher.txt"),
        )
        .arg(
            Arg::with_name("dict")
                .short("d")
                .long("dict")
                .help("Sets dictionary to use")
                .takes_value(true)
                .default_value("/usr/share/dict/words"),
        )
        .get_matches();

    (
        matches.value_of("cipher").unwrap().to_string(),
        matches.value_of("dict").unwrap().to_string(),
    )
}
