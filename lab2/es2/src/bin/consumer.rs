use core::time::Duration;
use std::thread::sleep;

use es2::CircularBuffer;


fn main()
{
    const NUM_SENS: usize = 10;
    
    let ocb: Option<CircularBuffer> = CircularBuffer::new("buffer.bin", NUM_SENS);
    match ocb
    {
        Some(mut cb) =>
        {
            let mut read_complete = false;
            loop 
            {
                for _ in 0..NUM_SENS
                {
                    read_complete = false;
                    while !read_complete
                    {
                        match cb.get()
                        {
                            Some(sd) =>
                            {
                                let mms = compute_float_min_max_sum(&sd.values);
                                println!("SENSOR {}: MIN: {:.2}, MAX: {:.2}, AVG: {:.2}", 
                                            sd.seq, 
                                            mms.0,
                                            mms.1,
                                            mms.2/(sd.values.len() as f32));
                                read_complete = true;
                            },

                            None => {}
                        }
                    }
                }
                sleep(Duration::from_secs(10));
            }
        }
        None => eprint!("Impossibile creare il buffer circolare")
    }
}


fn compute_float_min_max_sum(values: &[f32]) -> (f32, f32, f32)
{
    if values.len() == 0
    {
        return (f32::NAN, f32::NAN, f32::NAN);
    }

    let mut min = values[0];
    let mut max: f32 = min;
    let mut sum: f32 = max;

    for val in values
    {
        if !f32::is_nan(*val)
        {
            if *val < min
            {
                min = *val;
            }
            if *val > max
            {
                max = *val;
            }
            sum += *val;
        } 
    }
    (min, max, sum)
}
    