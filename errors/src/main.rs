// CMD :set RUST_BACKTRACE=1
//POWERSHELL :$Env:RUST_BACKTRACE=1

// use std::fs::{self};
// use std::io;

// fn read_usename_from_file() -> Result<String, io::Error> {
//     fs::read_to_string("hello.txt")
// }

use std::error::Error;
use std::fs::File;
fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;
    Ok(())
}
