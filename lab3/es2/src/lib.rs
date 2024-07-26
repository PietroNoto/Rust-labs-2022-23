pub struct MyCycle<I: Clone + Iterator>
{
   counter: usize,
   repeat: usize,
   iter: I,
   iter_copy: I
}


impl<I: Clone + Iterator> MyCycle<I>
{
    pub fn new(iter: I, repeat: usize) -> Self
    {
        let iter_copy = iter.clone();
        Self { counter: 0, repeat: repeat, iter: iter, iter_copy: iter_copy }
    }
}


impl<I: Clone + Iterator> Iterator for MyCycle<I>
{
    type Item = <I as Iterator>::Item;

    fn next(&mut self) -> Option<Self::Item> 
    {
         match self.iter.next()
         {
            Some(item) => Some(item),
            None =>
            {
                if self.counter + 1 == self.repeat
                {
                    None
                }
                else 
                {
                    self.counter += 1;
                    self.iter = self.iter_copy.clone();
                    self.iter.next()
                }
            }
         }
    }
}


impl<I: Clone + Iterator> Clone for MyCycle<I>
{
    fn clone(&self) -> Self 
    {
        Self 
        { 
            counter: self.counter.clone(), 
            repeat: self.repeat.clone(), 
            iter: self.iter.clone(), 
            iter_copy: self.iter_copy.clone() 
        }
    }
}
