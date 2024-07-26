use std::{rc::Rc, cell::{Cell, RefCell}};


pub struct Prova<T>
{
    a: String,
    b: i32,
    f: RefCell<dyn Fn(&[T])>,
    f1: Box<dyn Fn(&[T])>
}

impl<T> Prova<T>
{
    pub fn new<F: Fn(&[T])>(a: String, b: i32, f: F, f1: F) -> Self
    {
        Self { a: a, b: b, f: Rc::new(f), f1: Box::new(f1) }
    }
}

fn main() 
{
    /*let mut prova = Prova {a: "ciao".to_string(), b: 30};
    let ref_prova = &prova;
    (*ref_prova).b = 1;
    (*ref_prova).a = "Prova".to_string();
    
    let ra: Box<i32> = Box::new(5);
    let a = *ra;
    
    let mut rc: Rc<i32> = Rc::new(10);
    *rc = 7;
    let val = *rc;
    println!("{}", val);

    let rc1 = rc;

    let weak = Rc::downgrade(&rc1);
    let b = &weak; 
    */



}


