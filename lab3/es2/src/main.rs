use es2::MyCycle;

fn main() 
{
    let a = vec![1, 2, 3]; 
    
    let mut mc = MyCycle::new(a.iter(), 0);
    
    loop 
    {
        let el = mc.next();
        match el
        {
            Some(val) => println!("{}", val),
            None => break
        }    
    }
}
