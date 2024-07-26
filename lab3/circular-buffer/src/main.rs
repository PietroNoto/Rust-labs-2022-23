use circular_buffer::CircularBuffer;

fn main()
{
    let mut cb: CircularBuffer<i32> = CircularBuffer::new(5);

    for el in 1..20
    {
        if cb.write(el).is_err()
        {
            println!("Buffer full!");
            cb.overwrite(el);
            //let val = cb.read().unwrap();
            //println!("Read {}", val);
        }
        else 
        {
            println!("Element {} written successfully!", el);    
        }
    }
    cb.clear();
    
    let b = cb.read();

}