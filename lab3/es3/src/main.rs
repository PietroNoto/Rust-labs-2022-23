use std::{env::args, fs::File, process::exit, io::{Read, BufReader}, str::FromStr};
use chrono::{NaiveTime, Duration};


pub struct Calendar
{
    schedule: Vec<(NaiveTime, NaiveTime)>,
    bounds: (NaiveTime, NaiveTime)
}

impl Calendar
{
    pub fn new(file: &str) -> Self
    {
        let o = File::open(file.to_string());   
        if o.is_err()
        {
            exit(1);
        }

        let mut br = BufReader::new(o.unwrap());
        let mut buf: String = String::new();
        if br.read_to_string(&mut buf).unwrap() <= 0
        {
            exit(1);
        }
        
        let lines: Vec<&str> = buf.split("\n").collect();
        let bounds: (NaiveTime, NaiveTime) = (NaiveTime::from_str(lines[0]).unwrap(), NaiveTime::from_str(lines[1]).unwrap());
        let mut schedule: Vec<(NaiveTime, NaiveTime)> = Vec::new();

        for i in (2..lines.len()).step_by(2)
        {
            schedule.push((NaiveTime::from_str(lines[i]).unwrap(), NaiveTime::from_str(lines[i+1]).unwrap()));
        }

        Calendar { schedule: schedule, bounds: bounds}  
    }


    pub fn find_holes(&self, duration: Duration) -> Option<Vec<(NaiveTime, NaiveTime)>>
    {
        let mut holes: Vec<(NaiveTime, NaiveTime)> = Vec::new();
        let mut start_time: NaiveTime = self.bounds.0;
        let mut end_time: NaiveTime = self.schedule[0].0;

        if Calendar::fits_meeting(start_time, end_time, duration)
        {
            holes.push((start_time, end_time));
        }

        for i in (0..self.schedule.len() - 1)
        {
            start_time = self.schedule[i].1;
            end_time = self.schedule[i+1].0;
            
            if Calendar::fits_meeting(start_time, end_time, duration)
            {
                holes.push((start_time, end_time));
            }
        }

        start_time = self.schedule.last().unwrap().1;
        end_time = self.bounds.1;
        if Calendar::fits_meeting(start_time, end_time, duration)
        {
            holes.push((start_time, end_time));
        }

        if holes.is_empty()
        {
            None
        }
        else 
        {
            Some(holes)    
        }
    }


    fn fits_meeting(start_time: NaiveTime, end_time: NaiveTime, duration: Duration) -> bool
    {
        let dif = end_time - start_time;
        dif.cmp(&duration).is_ge()
    }


    pub fn print_holes(holes: &Option<Vec<(NaiveTime, NaiveTime)>>)
    {
        match holes
        {
            Some(holes) =>
            {
                println!("Slots available:");
                for (s, e) in holes.iter()
                {
                    println!("({}, {})", s.format("%H:%M"), e.format("%H:%M"));
                }
            },
            None => {}
        }
    }


    pub fn common_holes(holes1: &Vec<(NaiveTime, NaiveTime)>, holes2: &Vec<(NaiveTime, NaiveTime)>)
        -> Option<Vec<(NaiveTime, NaiveTime)>>
    {
        let mut ch: Vec<(NaiveTime, NaiveTime)> = Vec::new();
        
        if holes1.is_empty()
        {
            return Some(holes2.clone());
        }
        else if holes2.is_empty()
        {
            return Some(holes1.clone());
        }
        for h1 in holes1.iter()
        {
            for h2 in holes2.iter()
            {
                let int1 = (h1.0).cmp(&h2.0).is_ge() && (h1.1).cmp(&h2.1).is_le();
                let int2 = (h2.0).cmp(&h1.0).is_ge() && (h2.1).cmp(&h1.1).is_le();
                
                if int1 && !int2
                {
                    ch.push(*h1);
                }
                else if !int1 && int2
                {
                    ch.push(*h2);
                }
            }
        }
        if ch.is_empty()
        {
            None
        }
        else 
        {
            Some(ch)    
        }
    }
}


fn main() 
{    
    let args: Vec<String> = args().skip(1).collect();
    let cal1 = Calendar::new(&args[0]);
    let cal2 = Calendar::new(&args[1]);
    let dur_r = <i64>::from_str(&args[2]);
    match dur_r
    {
        Ok(dur) => 
        {
            let holes1 = cal1.find_holes(Duration::minutes(dur));
            let holes2 = cal2.find_holes(Duration::minutes(dur));
            Calendar::print_holes(&holes1);
            Calendar::print_holes(&holes2);

            let result = Calendar::common_holes(&holes1.unwrap(), &holes2.unwrap());
            Calendar::print_holes(&result);
        },
        Err(_) => exit(1)
    } 
}
