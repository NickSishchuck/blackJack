use std::io;
include!("test.rs");

    fn main(){
        test_message();
      let mut input = String::new();
      println!("Enter your name: ");
      io::stdin().read_line(&mut input).expect("Failed to read line"); // we need to expect errors rithtaway
      println!("Your name is: {}",input);
    }

