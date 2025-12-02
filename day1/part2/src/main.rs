use std::fs;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

struct Dial {
    position: i64,
    password: i64,
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read input");
    let initial_dial = Dial {
        position: 50,
        password: 0,
    };

    println!("Starting at 50");

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

        apply_clicks(previous_dial, clicks, direction)
    });

    println!(
        "Dial positon: {}, dial password: {}",
        dial.position, dial.password
    );
}

fn apply_clicks(dial: Dial, clicks: i64, direction: Direction) -> Dial {
    if clicks == 0 {
        return dial;
    }

    let new_position = match direction {
        Direction::Left => dial.position - 1,
        Direction::Right => dial.position + 1,
    };

    let new_position = if new_position > 99 {
        0
    } else if new_position < 0 {
        99
    } else {
        new_position
    };

    let new_password = if new_position == 0 {
        dial.password + 1
    } else {
        dial.password
    };

    let new_dial = Dial {
        position: new_position,
        password: new_password,
    };

    apply_clicks(new_dial, clicks - 1, direction)
}
