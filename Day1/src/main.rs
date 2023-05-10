use std::fs;

fn part_1() -> i32 {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut count = 0;
    let mut vec: Vec<i32> = Vec::new();

    for line in contents.lines() {

        let x: Vec<bool> = line.chars().map(|x| x.is_ascii_digit()).collect();
        if x.is_empty() {
            vec.push(count);
            count = 0;
            continue;
        }
        count += line.parse::<i32>().unwrap();

    }
    vec.push(count);
    vec.sort();
    vec.reverse();
    vec[0]
}


fn part_2() -> i32 {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    let mut count = 0;
    let mut vec: Vec<i32> = Vec::new();

    for line in contents.lines() {

        let x: Vec<bool> = line.chars().map(|x| x.is_ascii_digit()).collect();
        if x.is_empty() {
            vec.push(count);
            count = 0;
            continue;
        }
        count += line.parse::<i32>().unwrap();

    }
    vec.push(count);
    vec.sort();
    vec.reverse();
    vec[0] + vec[1] + vec[2]
}




fn main() {
    
    println!("Part 1: {}", part_1());

    println!("Part 2: {}", part_2());
}
