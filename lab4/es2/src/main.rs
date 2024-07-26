use std::{sync::{Arc, Mutex}, thread::{JoinHandle, self, sleep}, time::Duration};
use chrono::Local;
use rand::prelude::*;

use es2::{CircularBuffer, SensorData};


const NUM_SENS: usize = 10;

fn main() 
{
    const LEN: usize = 10;
    let nprod: usize = 1;
    let ncons: usize = 1;

    let mut shared_cb = 
        Arc::new(
            Mutex::new(
                    CircularBuffer::new(LEN)));

    let mut threads: Vec<JoinHandle<()>> = Vec::new();

    for _ in 0..nprod
    {
        let _cb = shared_cb.clone();
        threads.push(thread::spawn(|| producer(_cb)));
    }
    for _ in 0..ncons
    {
        let _cb = shared_cb.clone();
        threads.push(thread::spawn(|| consumer(_cb)));
    }
    
    for t in threads
    {
        t.join().unwrap();
    }
}


pub fn consumer(mutex_cb: Arc<Mutex<CircularBuffer>>)
{
    loop 
    {
        match &mut mutex_cb.lock()
        {
            Ok(cb) =>
            {
                match cb.read()
                {
                    Ok(sd) =>
                    {
                        drop(cb);
                        let mms = compute_float_min_max_sum(&sd.values);
                        println!("SENSOR {}: MIN: {:.2}, MAX: {:.2}, AVG: {:.2}", 
                                    sd.seq, 
                                    mms.0,
                                    mms.1,
                                    mms.2/(sd.values.len() as f32));
                    },
                    Err(_) => eprintln!("Empty buffer!")
                }
            },
            Err(_) => return
        } 
        sleep(Duration::from_secs(10));    
    }
}


pub fn producer(mutex_cb: Arc<Mutex<CircularBuffer>>)
{
    let mut values = [-1.0; NUM_SENS];
    loop 
    { 
        for sens in 0..NUM_SENS
        {
            sensor(&mut values);
            let timestamp = Local::now().timestamp();
            let sd = SensorData {seq: sens as u32, values: values, timestamp: timestamp};

            match &mut mutex_cb.lock()
            {
                Ok(cb) => 
                { 
                    match cb.write(sd)
                    {
                        Err(_) => eprintln!("Buffer full!"),
                        _ => {}
                    }
                }, 
                Err(_) => return
            }
        }
        sleep(Duration::from_secs(1));
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
