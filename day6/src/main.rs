use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::HashSet;


fn open_reader(filename: String) -> BufReader<File>
{
    let f = File::open(filename).unwrap();
    BufReader::new(f)
}

fn prepare_data(filename: String) -> Vec<Vec<String>>
{
    let mut v: Vec<Vec<String>> = Vec::new();

    let file = open_reader(filename);
    let mut temp: Vec<String> = Vec::new();
    for line in file.lines()
    {
        let l = line.unwrap();
        if l.is_empty() == true {
            v.push(temp);
            temp = Vec::new();
        }
        else {
            temp.push(l);
        }
    }
    v.push(temp);
    v
}

fn part_one(v: &Vec<Vec<String>>) {
    let mut count = 0;

    for x in v {
        let mut set: HashSet<char> = HashSet::new();
        for y in x {
            for z in y.chars() {
                set.insert(z);
            }
        }
        count += set.len();
    }

    println!("Count is {}", count);
}

fn part_two(v: &Vec<Vec<String>>) {
    let mut count = 0;
    for x in v {
        let mut set: HashSet<char> = HashSet::new();
        for i in 0..x[0].len() {
            set.insert(x[0].chars().nth(i).unwrap());
        }
        for y in 1..x.len() {
            let mut temp: HashSet<char> = HashSet::new();
            for i in 0..x[y].len() {
                temp.insert(x[y].chars().nth(i).unwrap());
            }
            set = set.intersection(&temp).cloned().collect();
        }
        count += set.len();
    }

    println!("Total count is {}", count);
}

fn main()
{
    let v = prepare_data(String::from("d6input.txt"));

    part_one(&v);
    part_two(&v);
}
