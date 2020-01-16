extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let mut debug: bool = false;

    let matches = App::new("onset-map-tools")
        .version("0.1")
        .author("Izio (Romain Billot) <romainbillot3009@gmail.com>")
        .about("A utility to convert map editors format output")
        .arg(
            Arg::with_name("debug")
                .help("turn on debugging information")
                .short("d")
                .long("debug"),
        )
        .subcommand(SubCommand::with_name("convert")
            .about("Convert Onset maps format")
            .version("0.1")
            .author("Izio (Romain Billot) <romainbillot3009@gmail.com>")
            .arg(Arg::with_name("input")
                .help("Coucou help")
                .required(true)))
        .get_matches();

    if matches.is_present("debug") {
        println!("Debug mode is enabled");
        debug = true;
    }

    match matches.subcommand(){
        ("convert", Some(convert_matches)) => {
            println!("Convert called with: {:?}", convert_matches);

        }
        ("", None) => println!("No subcommand was used"),
        _ => unreachable!()
    }

}