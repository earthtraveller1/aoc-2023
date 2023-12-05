fn main() {
    let input = std::fs::read_to_string("day4-input.txt").unwrap();

    let final_score = input.lines().fold(0, |final_score, line| {
        println!("---------------------------------------------");
        let line_parts = line.split(": ");
        let card_label = line_parts.clone().nth(0).unwrap().trim();
        let card_numbers = line_parts.clone().nth(1).unwrap().trim();
        println!("card label: {}", card_label);

        let card_numbers_parts = card_numbers.split(" | ");
        let winning_numbers = card_numbers_parts.clone().nth(0).unwrap().trim();
        let elf_numbers = card_numbers_parts.clone().nth(1).unwrap().trim();

        let winning_numbers = winning_numbers
            .split(" ")
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap());

        let elf_numbers = elf_numbers
            .split(" ")
            .filter(|s| !s.trim().is_empty())
            .map(|s| s.trim().parse::<i32>().unwrap());

        final_score
            + winning_numbers.fold(0, |score, num| {
                if let Some(_) = elf_numbers.clone().find(|n| *n == num) {
                    println!("Found match with {num}");
                    if score == 0 {
                        1
                    } else {
                        score * 2
                    }
                } else {
                    score
                }
            })
    });

    println!("Final score: {}", final_score);
}
