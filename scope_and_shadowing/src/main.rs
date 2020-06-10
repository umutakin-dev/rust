fn main() {

    scope_and_shadowing();
    
}

fn scope_and_shadowing()
{
    let a = 123;

    // create a new scope
    {
        let b = 456;
        println!("inside, b = {}", b);

        let a = 777;
        println!("inside, a = {}", a);
    }

    println!("outside, a = {}", a);
    
}
