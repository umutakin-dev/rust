
use std::mem;

struct Point
{
    x: f64,
    y: f64
}

fn origin() -> Point
{
    Point{x: 0.0, y: 0.0}
}

pub fn stack_and_heap()
{
    let p1 = origin();
    let p2 = Box::new(origin()); // allocate to the heap

    println!("p1 takes up {} bytes", mem::size_of_val(&p1));
    println!("p2 takes up {} bytes", mem::size_of_val(&p2));

    let p3 = *p2;
    println!("p3 takes up {} bytes", mem::size_of_val(&p3));
    println!("p3 x value is {}", p3.x);
}

fn main()
{
    stack_and_heap();
}
