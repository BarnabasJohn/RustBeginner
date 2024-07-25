//The task of this app is build a program that searches for
//text in a given file from command line interface

use std::env;
use std::process;
use minigrep::Config;


// the env library is used to access command line arguments
// the fs library is used to access the file system ie r / w
// the process library allows the program to exit with panicking


fn main() {

    let args: Vec<String>= env::args().collect();

    let config = Config::new(&args).unwrap_or_else(
        |err|{
            eprintln!("Problem parsing arguments: {}", err);
            process::exit(1);
        }
    );

    //unwrap_or_else() is meant to handle result types, where if error
    //occurs the error enclosed is executed

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    if let Err(e)= minigrep::run(config){
        eprintln!("Application error: {}", e);
        process::exit(1)
    };

}


