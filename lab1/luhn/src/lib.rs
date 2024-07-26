/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool 
{
    if code.len() <= 1
    {
        println!("Code is too short!");
        return false;
    }
    else 
    {
        let mut code_ch: Vec<char> = 
            code.chars()
                .filter(|c| *c != ' ')
                .collect();

        if code_ch.len() <= 1
        {
            println!("Code is too short!");
            return false;
        }
        else
        {
            match code_ch.iter().find(|c| !(*c).is_numeric())
            {
                None => {},
                Some(_) => 
                {
                    println!("Invalid format");
                    return false;
                },
            }
            let mut code_int: Vec<u32> = code_ch.iter()
                .map(|c| c.to_digit(10).unwrap())
                .rev()
                .collect();

            let mut sum: u32 = 0;
            let mut rep: u32;
            for i in (0..code_int.len() - 1).step_by(2)
            {
                rep = 2 * code_int[i+1];
                if rep > 9
                {
                    rep -= 9;
                }
                sum = sum + code_int[i] + rep;
            }

            if sum % 10 == 0 {true} else {false}
        }  
    }
}
