use std::{env::args, process::exit};
use std::fs::File;
use std::io::BufReader;

use es1::CData;

fn main() 
{
    const SIZE: usize = 100;
    let args: Vec<String> = args().skip(1).collect();
    if args.is_empty()
    {
        println!("Missing arguments");
        exit(1);
    }

    let input = File::open(&args[0]);
    match input
    {
        Ok(f) => 
        {
            let buffer = BufReader::new(f);
            let mut cdata: Vec<CData> = Vec::new(); 

            CData::from_file(buffer, SIZE, &mut cdata);
        },
        Err(_) => exit(1), 
    }
}
