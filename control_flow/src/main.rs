
fn if_statement()
{
    let temp = 35;

    if temp > 30
    {
        println!("really hot outsise");
    }
    else if temp < 10
    {
        println!("really cold");
    }
    else
    {
        println!("temperature is ok");
    }

    let day = if temp > 20 {"sunny"} else {"cloudy"};
    println!("today is {}", day);

    println!("it is {}",
             if temp > 20 {"hot"} else if temp < 10 {"cold"} else {"ok"});

    println!("it is {}",
             if temp > 20 {
                 if temp > 30 {"very hot"} else {"hot"}
             }
             else if temp < 10 {"cold"} else {"ok"});
}

fn while_and_loop()
{
    let mut x = 1;

    while x < 1000
    {
        x *= 2;

        if x == 64 { continue; }

        println!("x = {}", x);
    }

    let mut y = 1;
    
    loop // while true
    {
        y *= 2;
        println!("y = {}", y);

        if y == 1<<10 { break; }
    }
}

fn for_loop()
{
    // 11 is not included in the for loop, it is the upper bound
    for x in 1..11
    {
        if x == 3 { continue; }

        if x == 8 { break; }
        
        println!("x = {}", x);
    }

    for (pos, y) in (30..41).enumerate()
    {
        println!("{}: {}", pos, y);
    }
}

fn match_statement()
{
    let country_code = 1001;

    let country = match country_code {
        44 => "UK",
        46 => "Sweden",
        7  => "Russia",
        1..=1000 => "unknown", // includes number 1000
        _ => "invalid"
    };

    println!("the country with the code {} is {}", country_code, country);

    let x = false;

    let s = match x{
        true => "yes",
        false => "no"
    };
    println!("s is {}", s);
}

fn main()
{
    // if_statement();
    //while_and_loop();
    //for_loop();
    match_statement();
}
