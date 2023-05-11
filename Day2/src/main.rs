use std::collections::HashMap;




fn part_1() {
    let pat = HashMap::from([
        ('X', 1),
        ('Y', 2),
        ('Z', 3),
    ]);

    let list = include_str!("../input.txt")
        .split("\n")
        .map(|x| x.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>().into_iter().collect::<String>())
        .collect::<Vec<_>>();
    
    let mut res = 0;
    for n in list {
        let val = n.chars().last().unwrap();
        match n.as_str() {
            "AZ" | "BX" | "CY" => res += pat.get(&val).unwrap(),
            "AX" | "BY" | "CZ" => res += pat.get(&val).unwrap() + 3,
            "AY" | "BZ" | "CX" => res += pat.get(&val).unwrap() + 6,
            _ => unreachable!(),
        };

    }
    println!("part 1: {}", res)
}


fn part_2() {

    let list = include_str!("../input.txt")
        .split("\n")
        .map(|x| x.chars().filter(|c| !c.is_whitespace()).collect::<Vec<_>>().into_iter().collect::<String>())
        .collect::<Vec<_>>();
    

    let mut res = 0;
    for n in list {
        let val = n.chars().next().unwrap();
        match n.chars().last().unwrap() {
            'X' => { res += match val {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                _ => unreachable!(),
            }},
            'Y' => { res += match val {
                'A' => 1,
                'B' => 2,
                'C' => 3,
                _ => unreachable!(),
            } + 3},
            'Z' => { res += match val {
                'A' => 2,
                'B' => 3,
                'C' => 1,
                _ => unreachable!(),
            } + 6},
            _ => unreachable!(),
        };

    }
    println!("part 2: {}", res)
}

fn main() {
    part_1();
    part_2();
    
}
