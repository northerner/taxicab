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
    let mut route_stops = vec![(0, 0)];
    loop {
        match instructions.next() {
            Some(move_string) => {
                let move_value = parse_move(&facing, move_string);
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
                let &(last_x, last_y) = route_stops.last().unwrap();
                if last_x != x_distance {
                    let x_step_range = get_step_range(last_x, x_distance);
                    for x in x_step_range {
                        if route_stops.last().unwrap() != &(x, y_distance) {
                            route_stops.push((x, y_distance));
                        }
                    }
                } else {
                    let y_step_range = get_step_range(last_y, y_distance);
                    for y in y_step_range {
                        if route_stops.last().unwrap() != &(x_distance, y) {
                            route_stops.push((x_distance, y));
                        }
                    }
                }
            },
            None => { break }
        }
    }
    println!("{:?}", route_stops);
    x_distance.abs() + y_distance.abs()
}

fn get_step_range(last: i32, new: i32) -> Vec<i32> {
    match new > last {
        true => (last..new+1).collect(),
        false => (new..last+1).rev().collect()
    }
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
