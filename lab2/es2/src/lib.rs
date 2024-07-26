use std::{fs::{File, OpenOptions}, io::{Write, Seek, SeekFrom, Read}, path::Path};
use serde::{Serialize, Deserialize};
use fcntl::*;


pub struct CircularBuffer
{
    file: File,
    header_size: usize,
    payload_size: usize,
}


#[derive(Serialize, Deserialize)]
struct Header
{
    read_idx: usize,
    write_idx: usize,
    N: usize,
}


#[derive(Serialize, Deserialize)]
pub struct SensorData
{
    pub seq: u32,
    pub values: [f32; 10],
    pub timestamp: i64
}


impl CircularBuffer
{
    pub fn new(file_name: &str, N: usize) -> Option<CircularBuffer>
    {
        let rf = OpenOptions::new()
            .read(true)
            .write(true)
            .create(!Path::new(file_name).exists())
            .open(file_name);
        match rf
        {
            Ok(f) => 
            {
                let header_size = std::mem::size_of::<Header>();
                let payload_size = std::mem::size_of::<SensorData>();

                let sd: SensorData = SensorData { seq: 0, values: [0.0; 10], timestamp: 0 };
                let mut buf = 
                    CircularBuffer{file: f, header_size: header_size, payload_size: payload_size};
               
                buf.write_header(&Header { read_idx: 0, write_idx: 0, N: N /*, full: false */ });
                for i in 0..10
                {   
                    buf.write_payload(&sd, i);
                }

                Some(buf)
            }
            Err(_) => None
        }
    }


    pub fn get(&mut self) -> Option<SensorData>
    {
        if self.is_locked() && !self.lock(FcntlLockType::Read)
        {
            return None;
        }

        let mut h: Header = self.read_header();
        if h.read_idx == h.write_idx
        {
            eprint!("Buffer empty!");
            return None;
        }
        let rval = self.read_payload(h.read_idx);
        match rval
        {
            None => None,
            Some(sd) =>
            {
                h.read_idx = (h.read_idx + 1) % h.N;
                self.write_header(&h);

                if self.unlock()
                {
                    Some(sd)
                }
                else 
                {
                    None
                } 
            }
        }
    }


    pub fn put(&mut self, sd: &SensorData) -> bool
    {
        if self.is_locked() && !self.lock(FcntlLockType::Write)
        {
            return false;
        }

        let mut h: Header = self.read_header();
        if (h.write_idx + 1) % h.N == h.read_idx
        {
            eprint!("Buffer full!");
            false
        }
        else 
        {
            self.write_payload(sd, h.write_idx);
            h.write_idx = (h.write_idx + 1) % h.N;
            self.write_header(&h);

            self.unlock()
        }
    }


    pub fn is_locked(&self) -> bool
    {
        let lck = fcntl::is_file_locked(&self.file, None);
        match lck
        {
            Ok(res) => res,
            Err(_) => true
        }
    }


    pub fn lock(&mut self, locktype: FcntlLockType) -> bool
    {
        let lck = fcntl::lock_file(&self.file, None, Some(locktype));
        match lck
        {
            Ok(res) => res,
            Err(_) => false
        }
    }


    pub fn unlock(&mut self) -> bool
    {
        let lck = fcntl::unlock_file(&self.file, None);
        match lck
        {
            Ok(res) => res,
            Err(_) => false
        }
    }


    fn read_header(&mut self) -> Header
    {
        let mut ser_header = [0; std::mem::size_of::<Header>()];

        self.file.seek(SeekFrom::Start(0));
        self.file.read_exact(&mut ser_header);
        self.file.flush().unwrap();

        bincode::deserialize(&ser_header).unwrap()
    }


    fn read_payload(&mut self, index: usize) -> Option<SensorData>
    {
        if index >= 10
        {
            eprint!("Index out of range!");
            None
        }
        else
        {
            let mut ser_pl = [0; std::mem::size_of::<SensorData>()];

            self.file.seek(SeekFrom::Start((self.header_size + index * self.payload_size) as u64)).unwrap();
            self.file.read_exact(&mut ser_pl);
            self.file.flush().unwrap();

            Some(bincode::deserialize(&ser_pl).unwrap())
        }
    }


    fn write_header(&mut self, header: &Header)
    {
        let ser_header = bincode::serialize(header).unwrap();

        self.file.seek(SeekFrom::Start(0));
        self.file.write(&ser_header);
        self.file.flush().unwrap();
    }
    

    fn write_payload(&mut self, payload: &SensorData, index: usize)
    {
        let ser_payload = bincode::serialize(payload).unwrap();
        
        self.file.seek(SeekFrom::Start((self.header_size + index * self.payload_size) as u64));
        self.file.write(&ser_payload);
        self.file.flush().unwrap();
    }
}