use std::env;

fn main() {
    let result = follow_path_to_button("5", &env::args().nth(1).unwrap());
    println!("{}", result);
}

fn get_button(start: &str, direction: &str) -> String {
    let keypad = vec![["X", "X", "X", "X", "X", "X", "X"],
                      ["X", "X", "X", "1", "X", "X", "X"],
                      ["X", "X", "2", "3", "4", "X", "X"],
                      ["X", "5", "6", "7", "8", "9", "X"],
                      ["X", "X", "A", "B", "C", "X", "X"],
                      ["X", "X", "X", "D", "X", "X", "X"],
                      ["X", "X", "X", "X", "X", "X", "X"]];
    let (index_y, index_x) = get_button_position(start);
    let mut button = "X";
    match direction {
        "U" => button = keypad[index_y-1][index_x],
        "R" => button = keypad[index_y][index_x+1],
        "D" => button = keypad[index_y+1][index_x],
        "L" => button = keypad[index_y][index_x-1],
        _ => button = "0"
    }
    if button == "X" {
        start.to_string()
    } else {
        button.to_string()
    }
}

fn get_button_position(button: &str) -> (usize, usize) {
    let keypad = vec![["X", "X", "X", "X", "X", "X", "X"],
                      ["X", "X", "X", "1", "X", "X", "X"],
                      ["X", "X", "2", "3", "4", "X", "X"],
                      ["X", "5", "6", "7", "8", "9", "X"],
                      ["X", "X", "A", "B", "C", "X", "X"],
                      ["X", "X", "X", "D", "X", "X", "X"],
                      ["X", "X", "X", "X", "X", "X", "X"]];
    let y = &keypad.iter().position(|&x| x.contains(&button)).unwrap();
    let x = keypad[y.clone()].iter().position(|&x| x == button).unwrap();
    (y.clone(), x.clone())
}

fn follow_path_to_button(start: &str, instructions: &str) -> String {
    let mut current_location = start.to_string();
    for instruction in instructions.chars() {
        current_location = get_button(&current_location, &instruction.to_string());
    }
    current_location
}

#[test]
fn returns_the_button_to_press() {
    assert_eq!("5", get_button("5", "U"));
    assert_eq!("6", get_button("5", "R"));
    assert_eq!("A", get_button("A", "D"));
    assert_eq!("9", get_button("9", "D"));
}

#[test]
fn returns_position_of_button_in_vector() {
    assert_eq!((2, 5), get_button_position("9"));
    assert_eq!((1, 4), get_button_position("4"));
}

#[test]
fn returns_button_after_several_instructions() {
    assert_eq!("A", follow_path_to_button("5", "RD"));
    assert_eq!("A", follow_path_to_button("5", "RDL"));
    assert_eq!("C", follow_path_to_button("5", "RDRRR"));
}
