fn main() {
    let input = std::fs::read_to_string("day2-input.txt").unwrap();

    let mut sum = 0u32;

    input.lines().for_each(|line| {
        let mut parts = line.split(": ");
        let game_part = parts.next().unwrap();
        let stuff_part = parts.next().unwrap();

        let game_id: u32 = game_part.split(" ").nth(1).unwrap().parse().unwrap();
        println!("game id - {}", game_id);

        let mut game_possible = true;

        let _stuffs = stuff_part.split(|del| del == ',' || del == ';').map(|s| s.trim()).for_each(|stuff| {
            let mut stuff_parts = stuff.split(' ');
            println!("\tstuff - {}", stuff);
            let count: u32 = stuff_parts.next().unwrap().parse().unwrap();
            let color = stuff_parts.next().unwrap();

            match color {
                "red" => if count > 12 {
                    game_possible = false;
                },
                "green" => if count > 13 {
                    game_possible = false;
                },
                "blue" => if count > 14 {
                    game_possible = false;
                },
                _ => todo!()
            }
        });

        if game_possible {
            sum += game_id;
        }
    });

    println!("Answer: {}", sum);
}
