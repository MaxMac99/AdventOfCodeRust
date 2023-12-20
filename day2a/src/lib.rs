use regex::Regex;

pub struct Set {
    pub red: i64,
    pub green: i64,
    pub blue: i64,
}

pub struct Game {
    pub id: i64,
    pub sets: Vec<Set>,
}

pub fn parse_line(line: &String) -> Game {
    let game_re = Regex::new(r"Game (?<id>\d+): (?<sets>(((\d+) (red|green|blue)(, )?)+(; )?)+)").unwrap();

    println!("{}", line);
    let caps = game_re.captures(line).expect("Could not match pattern");

    let set_re = Regex::new(r"((\d+) (red|green|blue)(, )?)+").unwrap();
    let dice_re = Regex::new(r"(?<num>\d+) (?<color>red|green|blue)").unwrap();

    let mut game = Game {
        id: *(&caps["id"].parse().unwrap()),
        sets: vec![],
    };


    for set_cap in set_re.captures_iter(&caps["sets"]) {
        let set_str = set_cap.get(0).unwrap().as_str();
        let mut set = Set { red: 0, green: 0, blue: 0 };
        for dice_cap in dice_re.captures_iter(set_str) {
            let [num_str, color] = dice_cap.extract::<2>().1;
            let num = num_str.parse().unwrap();
            match color {
                "red" => set.red = num,
                "green" => set.green = num,
                "blue" => set.blue = num,
                _ => {}
            }
        }

        game.sets.insert(0, set);
    }

    game
}
