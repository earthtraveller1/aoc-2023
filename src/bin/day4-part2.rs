struct Card {
    number: i32,
    multiplier: i32,
    winning_numbers: Vec<i32>,
    our_numbers: Vec<i32>,
}

fn main() {
    let input = std::fs::read_to_string("day4-input.txt").unwrap();

    let mut cards = input
        .lines()
        .map(|line| {
            println!("---------------------------------------------");
            let line_parts = line.split(": ");
            let card_label = line_parts.clone().nth(0).unwrap().trim();
            let card_numbers = line_parts.clone().nth(1).unwrap().trim();
            println!("card label: {}", card_label);

            let card_number = card_label
                .split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .nth(1)
                .unwrap()
                .trim()
                .parse::<i32>()
                .unwrap();

            println!("card number: {card_number}");

            let card_numbers_parts = card_numbers.split(" | ");
            let winning_numbers = card_numbers_parts.clone().nth(0).unwrap().trim();
            let elf_numbers = card_numbers_parts.clone().nth(1).unwrap().trim();

            let winning_numbers = winning_numbers
                .split(" ")
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            let elf_numbers = elf_numbers
                .split(" ")
                .filter(|s| !s.trim().is_empty())
                .map(|s| s.trim().parse::<i32>().unwrap())
                .collect::<Vec<i32>>();

            Card {
                number: card_number,
                multiplier: 1,
                our_numbers: elf_numbers,
                winning_numbers,
            }
        })
        .collect::<Vec<Card>>();

    for i in 0..cards.len() {
        let total = cards[i].our_numbers.iter().fold(0, |total, number| {
            if let Some(_) = cards[i].winning_numbers.iter().find(|n| *n == number) {
                total + 1
            } else {
                total
            }
        });

        let current_multiplier = cards[i].multiplier;
        println!(
            "Number: {}, Multiplier: {}, Total: {}",
            cards[i].number, current_multiplier, total
        );
        println!("i = {i}");

        for j in (i + 1)..(i + total + 1) {
            println!("j = {j}");
            if let Some(card) = cards.iter_mut().find(|c| c.number == (j as i32) + 1) {
                println!("Won a copy of card {}", card.number);
                card.multiplier += current_multiplier;
            }
        }
    }

    let answer = cards
        .iter()
        .fold(0, |answer, card| answer + card.multiplier);

    println!("Answer: {}", answer);
}
