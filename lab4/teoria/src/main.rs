use std::sync::{Arc, Mutex};

fn main() 
{
    let shared_data = Arc::new(Mutex::new(Vec::<i32>::new()));
}
