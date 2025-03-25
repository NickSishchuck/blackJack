

    pub fn shuffle_cards(cards: &mut Vec<&str>){
        use rand::seq::SliceRandom;
        use rand::rng;
        let mut rng = rng();
        cards.shuffle(&mut rng);
    }

    pub fn show_cards(cards: &Vec<&str>){
        for card in cards {
            print!("{} ", card);
        }
        println!();
    }
