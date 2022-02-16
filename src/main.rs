use std::io;
use std::io::Write;


fn main() {
    let mut name = String::new();
    //Printing one without the newline at the end
    print!("What is your name?");
    io::stdout().flush().unwrap();
    //Need to flush as the stdout is frequently line buffered by default https://stackoverflow.com/questions/37531903/how-do-i-print-output-without-a-trailing-newline-in-rust 
    io::stdin().read_line(&mut name).expect("Failed to read line");
    let name: String = name.trim().parse().expect("Please type a valid string.");
    println!("Hello, {}! \nWelcome to the world of pain and suffering.", name);
    
}
