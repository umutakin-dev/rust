
const MEAING_OF_LIFE:u8 = 42; // no fixed address

static mut Z:i32 = 123;

fn main() {
    
    println!("{}", MEAING_OF_LIFE);

    unsafe
    {
        println!("{}", Z);
    }
    
}
