mod bufread;
use clap::{App, Arg};
use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");
fn main(){
    //test function to print the output of the binary file
    let App = App::new("Rexdump")
        .version(VERSION)
        .about("minimalist hexdump implementation")
        .arg(
            Arg::with_name("input")
                .help("Input file")
                .value_name("FILE")
                .short("i"),
        )
        .get_matches();
    
    if App.is_present("input") {
        let mut data = bufread::HexDump::new(); // Might implement some struct members later
        data.read(App.value_of("input").unwrap());
    }
}
