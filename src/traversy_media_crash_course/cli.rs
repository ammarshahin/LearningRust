pub fn run() {
    //* get the arguments from the command line
    let args: Vec<String> = std::env::args().collect();
    println!("Args : {:?}", args);
    //* if input was : John Doe
    //* output will be >> Args : ["target\\debug\\learning_rust_repo.exe", "ammar", "shahin"]
    //* output                               args[0]                     , args[1],  args[2]

    let command = args[1].clone();
    if command == "ammar" {
        println!("Hi {}", command);
    } else {
        println!("Invalid !!!");
    }
}
