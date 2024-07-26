pub mod tests;

use std::env::args;


fn capitalize(s: &str) -> String
{
    let mut char_array: Vec<_> = s.chars().collect();
    for i in 0..char_array.len()
    {
        if char_array[i].is_ascii_alphabetic()
        {
            if (i > 0 && char_array[i-1] == ' ') || i == 0
            {
                char_array[i] = char_array[i].to_ascii_uppercase();
            }
        }
    }
    char_array.iter().collect()
}


fn main() 
{
    let args: Vec<String> = args().skip(1).collect();
    assert!(args.len() > 0);

    let cap = capitalize(&args[0]);
    println!("Capitalized text: {}", cap);
}


