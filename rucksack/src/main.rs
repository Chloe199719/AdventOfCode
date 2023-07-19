use std::collections::HashMap;

use rucksack::input;
fn main() {
    let mut priority:HashMap<char, u32> = HashMap::new();
        priority.insert('a', 1);
        priority.insert('b', 2);
        priority.insert('c', 3);
        priority.insert('d', 4);
        priority.insert('e', 5);
        priority.insert('f', 6);
        priority.insert('g', 7);
        priority.insert('h', 8);
        priority.insert('i', 9);
        priority.insert('j', 10);
        priority.insert('k', 11);
        priority.insert('l', 12);
        priority.insert('m', 13);
        priority.insert('n', 14);
        priority.insert('o', 15);
        priority.insert('p', 16);
        priority.insert('q', 17);
        priority.insert('r', 18);
        priority.insert('s', 19);
        priority.insert('t', 20);
        priority.insert('u', 21);
        priority.insert('v', 22);
        priority.insert('w', 23);
        priority.insert('x', 24);
        priority.insert('y', 25);
        priority.insert('z', 26);
        priority.insert('A', 27);
        priority.insert('B', 28);
        priority.insert('C', 29);
        priority.insert('D', 30);
        priority.insert('E', 31);
        priority.insert('F', 32);
        priority.insert('G', 33);
        priority.insert('H', 34);
        priority.insert('I', 35);
        priority.insert('J', 36);
        priority.insert('K', 37);
        priority.insert('L', 38);
        priority.insert('M', 39);
        priority.insert('N', 40);
        priority.insert('O', 41);
        priority.insert('P', 42);
        priority.insert('Q', 43);
        priority.insert('R', 44);
        priority.insert('S', 45);
        priority.insert('T', 46);
        priority.insert('U', 47);
        priority.insert('V', 48);
        priority.insert('W', 49);
        priority.insert('X', 50);
        priority.insert('Y', 51);
        priority.insert('Z', 52);
        let input = input();
    let line_splited = input.split("\n")
    .collect::<Vec<&str>>();
    let mut  tuplevect = Vec::new();
    for line in &line_splited {
        let length = line.len();
        let first = &line[0..length/2];
        let second = &line[length/2..length];
        tuplevect.push((first,second));
    }
    let mut sum = 0;
    for (first,second) in &tuplevect {
       let mut hash_map:HashMap<char, u32> = HashMap::new();
       for letter in first.chars() {
            hash_map.entry(letter).or_insert(0);
       }
         for letter in second.chars() {
            let x = hash_map.get(&letter);
            match x {
                Some(_) => {
                   let value =  priority.get(&letter).unwrap();
                     sum = sum + value;
                     hash_map.remove(&letter);

                }
                None => ()
            };

         }
    }
    // println!("{}",sum);
    let mut sum2 = 0;
    let split_group_of_3 = line_splited.chunks(3).collect::<Vec<_>>();
    for group in &split_group_of_3 {
        let mut hash_map:HashMap<char, u32> = HashMap::new();
        for char in group[0].chars() {
            hash_map.entry(char).or_insert(0);
        }
        for char in group[1].chars() {
            let x = hash_map.get(&char);
            match x {
                Some(_) => {
                    hash_map.insert(char, 1);
                    
                }
                None => ()
            };
        }
        for char in group[2].chars() {
            let x = hash_map.get(&char);
            match x {
                Some(1) => {
                    let value =  priority.get(&char).unwrap();
                    sum2 = sum2 + value;
                    hash_map.remove(&char);
                }
                Some(_) => (),
                None => ()
            };
        }
        println!("{:?}",hash_map);
    }
    println!("{}",sum2);
    // println!("{:?}",split_group_of_3);


    // println!("{:?}",tuplevect)
}
