use std::{env::args, process::exit, time::Instant, sync::{Arc, Mutex}, thread::{self, JoinHandle}};
use itertools::{self, Itertools};


fn main() 
{
    const OPS: [char; 4] = ['+', '-', '*', '/'];
    let nthreads: usize = 12;
    
    let numbers: Vec<u8> = args().skip(1)
        .map(|n| 
        {
            match n.parse::<u8>()
            {
                Ok(n) => n,
                Err(_) => exit(1)
            }

        })
        .collect();
    
    let perms: Vec<Vec<&u8>> = [1,2,3,4,5].iter()
        .permutations(5)
        .unique()
        .collect();

    let total_len = perms.len();

    let op_perms: Vec<_> = OPS.iter()
        .combinations_with_replacement(4)
        .collect();

    let shared_sols = Arc::new(Mutex::new(Vec::<String>::new()));
    let mut threads = Vec::new();
    let shared_perms = Arc::new(perms);
    let shared_ops = Arc::new(op_perms);

    let now = Instant::now();

    for t in 0..nthreads
    {
        let inf = t * total_len / nthreads;
        let sup = inf + total_len / nthreads;

        let mut _sols = shared_sols.clone();
        let _perms = shared_perms.clone()[inf..sup].to_vec();
        let _op_perms = shared_ops.clone();
        
        threads.push(thread::spawn(move||
            {   
                for perm in _perms.iter()
                {
                    for op_set in _op_perms.iter()
                    {
                        if compute(op_set, perm) == Some(10)
                        {
                            let mut sols = _sols.lock().unwrap();
                            sols.push(op_to_string(op_set, perm));
                        }
                    }
                }
            }));     
    }
    for t in threads
    {
        t.join().unwrap();
    }

    let elapsed = now.elapsed();

    println!("I have found {} solutions: ", shared_sols.lock().unwrap().len());
    println!("Elapsed time: {:3}s", elapsed.as_secs_f32());
       
}


pub fn compute(op_set: &Vec<&char>, numbers: &Vec<&u8>) -> Option<i32>
{
    let mut partial: i32 = *numbers[0] as i32;
    for i in 1..numbers.len()
    {
        match op_set[i-1]
        {
            '+' => partial += *numbers[i] as i32,
            '-' => partial -= *numbers[i] as i32,
            '*' => partial *= *numbers[i] as i32,
            '/' => 
            {
                if *numbers[i] != 0
                {
                    partial /= *numbers[i] as i32;
                }
                else 
                {
                    return None;   
                }
            },
            _ => return None
        }
    }
    Some(partial)
}


pub fn op_to_string(op_set: &Vec<&char>, numbers: &Vec<&u8>) -> String
{
    let mut s: String = String::new();
    for i in 0..numbers.len()
    {
        s.push(char::from_digit(*numbers[i] as u32, 10).unwrap());
        if i < op_set.len()
        {
            s.push(*op_set[i]);
        }
    }
    s
}
