//* const global variables
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

//* non const global variables
//* NOTE: the use of global mutable variable is highly unsafe and not recommended so the use of unsafe block must be used when ever it's used

static mut GLOBAL_VAR: bool = false;

pub fn run() {
    let x = 5;
    {
        let x = x + 95; //* here x has shadowed the value 5 with (5 + 95)
        println!("x = {}", x); //* here we print the value 100 (The shadow value)
    } //* here the inner scope ends and the shadow value ends and the original value is returned  = 5
    println!("x = {}", x); //* here we print the value 5 (The original value)

    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    println!("Firing {} of my {} missiles...", ready, missiles);
    println!("{} missiles left", missiles - ready);

    // global variable
    another_fun();
    unsafe {
        if GLOBAL_VAR == true {
            println!("it Worked!!!");
        }
    }
}

fn another_fun() {
    unsafe {
        GLOBAL_VAR = true;
    }
}
