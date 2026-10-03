fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        let result = loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break remaining;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        };

        println!("final remaining: {result}");

        count += 1;
    }
    println!("End count = {count}");

    while_loop();
    for_loop();
    for_range_rev_loop();
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];
    
    for element in a {
        println!("the value is: {element}");
    }
}

fn for_range_rev_loop() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
