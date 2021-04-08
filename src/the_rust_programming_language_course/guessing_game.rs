use rand::Rng;

pub fn run() {
    //* thread_rng() : will give us the articular random number generator
    //* gen_range() :  it takes two numbers as arguments and generates a random number between them
    // Create and generate the secret number
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please Enter your guess: ");
        let mut guess = String::new();
        match std::io::stdin().read_line(&mut guess) {
            Ok(_) => {
                let guess: u8 = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Invalid Input !!! ");
                        continue;
                    }
                };

                if guess > secret_number {
                    println!("Too big!!!");
                } else if guess < secret_number {
                    println!("Too small!!!");
                } else {
                    println!("You win!!!");
                    break;
                }
            }
            Err(_) => {
                println!("Invalid Input !!! ");
                continue;
            }
        }
    }
}
