use std::mem;


pub struct CircularBuffer<T: Default> 
{
    readIdx: usize,
    writeIdx: usize,
    actual_len: usize,
    buffer: Vec<T>,
}


#[derive(Debug, PartialEq, Eq)]
pub enum Error 
{
    EmptyBuffer,
    FullBuffer,
}


impl<T: Default> CircularBuffer<T> 
{
    pub fn new(capacity: usize) -> Self 
    {
        let mut buffer: Vec<T> = Vec::<T>::new();
        buffer.resize_with(capacity + 1, T::default);
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
            self.buffer[self.writeIdx] = _element;
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
            let val = mem::take(&mut self.buffer[self.readIdx]); 
            self.readIdx = (self.readIdx + 1) % self.buffer.len();
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
            *el = T::default();
        }
    }


    pub fn overwrite(&mut self, _element: T) 
    {
        self.buffer[self.readIdx] = _element;
        self.readIdx = (self.readIdx + 1) % self.buffer.len();
        self.writeIdx = (self.writeIdx + 1) % self.buffer.len();
    }
}
