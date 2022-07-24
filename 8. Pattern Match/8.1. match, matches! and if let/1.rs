//Question 1
// Fill the blanks

enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South|Direction::North  => { // Matching South or North here
            println!("South or North");
        },
        Direction::West => println!("West"),
    };
}

// Add direction crate to North,South & West in Ln15 & 18
