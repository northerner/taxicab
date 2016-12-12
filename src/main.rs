use std::str::FromStr;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut file = File::open("triangle").unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer);
    let result = count_triangles(&buffer);
    println!("{}", result);
}

fn is_triangle(edges_input: Vec<i32>) -> bool {
    let mut edges = edges_input.clone();
    edges.sort();
    (edges[0] + edges[1]) > edges[2]
}

fn count_triangles(triangle_list: &str) -> i32 {
    let mut count = 0;
    for line in triangle_list.lines() {
        let triangle: Vec<i32> = line
            .split_whitespace()
            .map(|x| i32::from_str(x).unwrap())
            .collect();
        if is_triangle(triangle) {
            count = count + 1;
        }
    }
    count
}

#[test]
fn returns_true_if_valid_triangle() {
    assert_eq!(true, is_triangle(vec![5, 5, 5]));
    assert_eq!(false, is_triangle(vec![5, 5, 20]));
}

#[test]
fn returns_count_of_valid_triangles() {
    assert_eq!(2, count_triangles("5 5 5\n5 5 20\n5 5 5"));
}
