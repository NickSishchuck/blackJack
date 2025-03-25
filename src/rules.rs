use std::thread;
use std::time::Duration;


pub fn show_rules() {


    println!("The game consists of 2 players, the player and the dealer");
    thread::sleep(Duration::from_secs(2));
    println!("The player will be dealt 2 cards and the dealer will be dealt 2 cards");
    thread::sleep(Duration::from_secs(2));
    println!("The 'buster' is the player or dealer who has a score of more than 21");
    thread::sleep(Duration::from_secs(2));
    println!("The player will be asked if they want to hit or stand");
    thread::sleep(Duration::from_secs(2));
    println!("The player can hit as many times as they want until they stand or bust");
    thread::sleep(Duration::from_secs(2));
    println!("The dealer will hit until they reach 17 or bust");
    thread::sleep(Duration::from_secs(2));
    println!("The player will win if they have a higher score than the dealer without busting");
    thread::sleep(Duration::from_secs(2));


}