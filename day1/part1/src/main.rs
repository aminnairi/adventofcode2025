use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

struct Dial {
    position: i64,
    password: u64,
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read input");
    let initial_dial = Dial {
        position: 50,
        password: 0,
    };

    let dial = content.lines().fold(initial_dial, |previous_dial, line| {
        let direction = match line.chars().next() {
            Some(character) => match character {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Line should start with either L or R"),
            },

            None => panic!("Line should start with either L or R"),
        };

        let clicks: i64 = line
            .chars()
            .skip(1)
            .collect::<String>()
            .parse()
            .expect("Cannot parse line");

        let new_dial_position = match direction {
            Direction::Right => previous_dial.position + clicks,
            Direction::Left => previous_dial.position - clicks,
        };

        let new_dial_position = new_dial_position % 100;

        if new_dial_position == 0 {
            return Dial {
                position: new_dial_position,
                password: previous_dial.password + 1,
            };
        }

        Dial {
            position: new_dial_position,
            password: previous_dial.password,
        }
    });

    println!(
        "Dial positon: {}, dial password: {}",
        dial.position, dial.password
    );
}
