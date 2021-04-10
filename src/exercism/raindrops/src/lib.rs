pub fn raindrops(n: u32) -> String {
    //unimplemented!("what sound does Raindrop #{} make?", n)
    let mut msg = String::new();
    if n % 3 == 0 {
        msg.push_str("Pling");
    }

    if n % 5 == 0 {
        msg.push_str("Plang");
    }

    if n % 7 == 0 {
        msg.push_str("Plong");
    }

    if msg.is_empty() {
        msg = format!("{}", n); // here wo used the format! macro to convert integer to string
    }

    msg
}
