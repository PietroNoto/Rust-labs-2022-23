use core::panic;
use std::{env::args, process::exit};

use luhn::is_valid;

fn main()
{
    let args: Vec<String> = args().skip(1).collect();
    if args.len() <= 0
    {
        println!("Missing arguments");
        exit(1);
    }
    else 
    {
        let valid = is_valid(&args[0]);
        println!("Code is{} valid", if valid {""} else {" not"});
    }
    
}