use std::mem;


pub struct CircularBuffer<T> 
{
    readIdx: usize,
    writeIdx: usize,
    actual_len: usize,
    buffer: Vec<Option<T>>,
}


#[derive(Debug, PartialEq, Eq)]
pub enum Error 
{
    EmptyBuffer,
    FullBuffer,
}


impl<T> CircularBuffer<T> 
{
    pub fn new(capacity: usize) -> Self 
    {
        let mut buffer: Vec<Option<T>> = Vec::<Option<T>>::new();
        buffer.resize_with(capacity + 1, Default::default);
        Self 
        { 
            readIdx: usize::default(), 
            writeIdx: usize::default(),
            actual_len: usize::default(), 
            buffer: buffer 
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> 
    {
        if (self.writeIdx + 1) % self.buffer.len() == self.readIdx
        {
            Err(Error::FullBuffer)
        }
        else 
        {
            self.buffer[self.writeIdx] = Some(_element);
            self.writeIdx = (self.writeIdx + 1) % self.buffer.len();
            self.actual_len += 1;
            Ok(())    
        }
    }


    pub fn read(&mut self) -> Result<T, Error> 
    {
        if (self.readIdx == self.writeIdx)
        {
            Err(Error::EmptyBuffer)
        }
        else 
        {
            let val = mem::take(&mut self.buffer[self.readIdx]).unwrap();
            self.readIdx = (self.readIdx + 1) % self.buffer.len();
            self.actual_len -= 1;
            Ok(val)
        }
    }


    pub fn clear(&mut self) 
    {
        self.readIdx = usize::default();
        self.writeIdx = usize::default();
        self.actual_len = usize::default();
       
        for el in self.buffer.iter_mut()
        {
            *el = None;
        }
    }


    pub fn overwrite(&mut self, _element: T) 
    {
        self.buffer[self.readIdx] = Some(_element);
        self.readIdx = (self.readIdx + 1) % self.buffer.len();
        self.writeIdx = (self.writeIdx + 1) % self.buffer.len();    
    }
}
