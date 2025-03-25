
pub fn calculate_hand_value(hand: &Vec<&str>) -> i32 {
    let mut value = 0;
    let mut aces = 0;

    for card in hand {
        match *card {
            "A" => {
                aces += 1;
                value += 11;
            },
            "K" | "Q" | "J" => value += 10,
            num => value += num.parse::<i32>().unwrap_or(0),
        }
    }

    // Adjust for aces if necessary
    while value > 21 && aces > 0 {
        value -= 10;
        aces -= 1;
    }

    value
}


pub fn dealer_should_hit(hand: &Vec<&str>) -> bool {
    let value = calculate_hand_value(hand);
    value < 17
}
