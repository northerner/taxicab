use std::env;

fn main() {
    let result = follow_path_to_button(5, &env::args().nth(1).unwrap());
    println!("{}", result);
}

fn get_button(start: i32, direction: &str) -> i32 {
    let keypad = vec![[1, 2, 3],
                      [4, 5, 6],
                      [7, 8, 9]];
    
    let (index_y, index_x) = get_button_position(start);
    match direction {
        "U" => if (index_y == 0) { start } else { keypad[index_y-1][index_x] },
        "R" => if (index_x == 2) { start } else { keypad[index_y][index_x+1] },
        "D" => if (index_y == 2) { start } else { keypad[index_y+1][index_x] },
        "L" => if (index_x == 0) { start } else { keypad[index_y][index_x-1] },
        _ => 0
    }
}

fn get_button_position(button: i32) -> (usize, usize) {
    let keypad = vec![[1, 2, 3],
                      [4, 5, 6],
                      [7, 8, 9]];
    let y = &keypad.iter().position(|&x| x.contains(&button)).unwrap();
    let x = keypad[y.clone()].iter().position(|&x| x == button).unwrap();
    (y.clone(), x.clone())
}

fn follow_path_to_button(start: i32, instructions: &str) -> i32 {
    let mut current_location = start;
    for instruction in instructions.chars() {
        current_location = get_button(current_location, &instruction.to_string());
    }
    current_location
}

#[test]
fn returns_the_button_to_press() {
    assert_eq!(2, get_button(5, "U"));
    assert_eq!(6, get_button(5, "R"));
    assert_eq!(9, get_button(6, "D"));
    assert_eq!(9, get_button(9, "D"));
}

#[test]
fn returns_position_of_button_in_vector() {
    assert_eq!((2, 2), get_button_position(9));
    assert_eq!((1, 0), get_button_position(4));
}

#[test]
fn returns_button_after_several_instructions() {
    assert_eq!(9, follow_path_to_button(5, "RD"));
    assert_eq!(8, follow_path_to_button(5, "RDL"));
    assert_eq!(7, follow_path_to_button(5, "RDLLL"));
}
