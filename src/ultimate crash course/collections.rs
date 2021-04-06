use std::collections::HashMap;

pub fn run() {
    /******************************* vectors ************************/
    let mut v1: Vec<u8> = Vec::new();
    for i in 0..=10 {
        v1.push(i);
    }
    println!("v1= {:?}", v1);

    let v2 = vec![22, 33, 44, 55];
    println!("v1= {:?}", v2);

    /******************************* HashMap ************************/
    let mut h: HashMap<u8, bool> = HashMap::new();
    for i in 0..=10 {
        h.insert(i, false);
    }
    println!("h= {:?}", h);

    // remove a value from the hash map
    let element = h.remove(&5).unwrap();
    println!("element= {}", element);
    println!("h= {:?}", h);
}
