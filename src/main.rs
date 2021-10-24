mod bufread;
use std::env;

fn main() {
    //test function to print the output of the binary file
    let mut data = bufread::HexDump::new();
    if env::args().len() == 2 {
        data.read(env::args().nth(1).unwrap().as_str());
    }
}
