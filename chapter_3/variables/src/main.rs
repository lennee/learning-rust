
fn declare_vars() {
    let mut x = 5;
    println!("The value of x is {x}");
    println!("The value of (x + 2) is {}\n", x + 2);
    x = 6;
    println!("The value of x is {x}");
    println!("The value of (x + 2) is {}\n", x + 2);
}

fn declare_const() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("The number of seconds in three hours is {THREE_HOURS_IN_SECONDS}\n");
}

fn shadow_vars() {
    let x = 5;
    let x = x + 1;

    {
        let x = x + 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}\n");
}

fn main() {
    declare_vars();
    declare_const();
    shadow_vars();
}