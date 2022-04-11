#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
use ftp::FtpStream as FtpServer;
use std::fs::File;
use std::io;
use std::io::Read;
mod update;
fn main() {
    match send_update() {
        Ok(_) => println!("Success. Your printer should now say \"PROGRAMING\""),
        Err(e) => eprintln!("{e}"),
    };
}




