use std::env;

fn main() {
    let distance = get_distance(&env::args().nth(1).unwrap());
    println!("{}", distance);
}

#[derive(PartialEq)]
#[derive(Debug)]
struct Move {
    direction: Direction,
    distance: i32
}

#[derive(PartialEq)]
#[derive(Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

fn parse_move(facing: &Direction, movement: &str) -> Move {
    let (direction, distance) = movement.split_at(1);
    let new_direction = match (facing, direction) {
        (&Direction::North, "L") => Direction::West,
        (&Direction::North, "R") => Direction::East,
        (&Direction::East, "L") => Direction::North,
        (&Direction::East, "R") => Direction::South,
        (&Direction::South, "L") => Direction::East,
        (&Direction::South, "R") => Direction::West,
        (&Direction::West, "L") => Direction::South,
        (&Direction::West, "R") => Direction::North,
        (_, _) => Direction::North,
    };
    Move{direction: new_direction, distance: distance.parse::<i32>().unwrap()}
}

fn get_distance(route: &str) -> i32 {
    let mut x_distance = 0;
    let mut y_distance = 0;
    let mut facing = Direction::North;
    let mut instructions = route.split(", ");
    loop {
        match instructions.next() {
            Some(x) => {
                let move_value = parse_move(&facing, x);
                facing = move_value.direction;
                y_distance = match facing {
                    Direction::North => y_distance + move_value.distance,
                    Direction::South => y_distance - move_value.distance,
                    _ => y_distance,
                };
                x_distance = match facing {
                    Direction::East => x_distance + move_value.distance,
                    Direction::West => x_distance - move_value.distance,
                    _ => x_distance,
                };
                println!("x: {}", x_distance);
                println!("y: {}", y_distance);
            },
            None => { break }
        }
    }
    x_distance.abs() + y_distance.abs()
}

#[test]
fn returns_a_parsed_move() {
    assert_eq!(Move{direction: Direction::East, distance: 3},
               parse_move(&Direction::North, "R3"));
    assert_eq!(Move{direction: Direction::West, distance: 8},
               parse_move(&Direction::North, "L8"));
    assert_eq!(Move{direction: Direction::South, distance: 6},
               parse_move(&Direction::West, "L6"));
}

#[test]
fn returns_a_total_distance() {
    assert_eq!(5, get_distance("L3, R2"));
    assert_eq!(12, get_distance("L7, R2, L3"));
    assert_eq!(2, get_distance("L3, R2, R3"));
}
