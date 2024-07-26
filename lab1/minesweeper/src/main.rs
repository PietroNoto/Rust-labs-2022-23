use std::fmt::Display;
use clap::Parser;
use minesweeper::annotate;

fn main()
{
    #[derive(Parser, Default, Debug)]
    struct Arguments
    {
        rows: usize,
        cols: usize,
        field: String,
    }

    let args = Arguments::parse();
    assert_eq!(args.field.len(), args.rows * args.cols);

    let mut inf: usize;
    let mut sup: usize;
    let mut field: Vec<&str> = Vec::new();

    for off in 0..args.rows
    {
        inf = off * args.cols;
        sup = inf + args.cols;
        field.push(&args.field[inf..sup]);
    }

    let ann_field = annotate(&field);
    print_vec(&ann_field);
}


pub fn print_vec<T : Display>(vec: &Vec<T>)
{
    for el in vec
    {
        println!("{}", el);
    }
}