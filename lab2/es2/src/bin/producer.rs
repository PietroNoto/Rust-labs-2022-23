use core::time::Duration;
use std::thread::sleep;
use rand::prelude::*;
use chrono::prelude::Local;

use es2::{CircularBuffer, SensorData};


fn main()
{
    const NUM_SENS: usize = 10;
    let mut seqs: [u32; 10] = [0; NUM_SENS];

    let ocb = CircularBuffer::new("buffer.bin", NUM_SENS);
    match ocb
    {
        Some(mut cb) =>
        {
            let mut values = [-1.0; NUM_SENS];
            loop 
            {
                for sens in 0..NUM_SENS
                {
                    sensor(&mut values);
                    let timestamp = Local::now().timestamp();
                    let sd = SensorData {seq: seqs[sens], values: values, timestamp: timestamp};
                    seqs[sens] += 1;

                    while !cb.put(&sd) {}
                }
                sleep(Duration::from_secs(1));
            }
        }
        None => eprint!("Impossibile creare il buffer circolare")
    }
}


fn sensor(values: &mut [f32])
{
    const MAX_VAL: f32 = 10.0;
    for sens in 0..values.len()
    {
        let val: f32 = MAX_VAL * rand::thread_rng().gen::<f32>();
        values[sens] = val;
    }
}