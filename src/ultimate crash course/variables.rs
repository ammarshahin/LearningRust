const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

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
}
