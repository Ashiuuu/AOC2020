use std::fs::File;
use std::io::{BufReader, BufRead};

fn open_reader(filename: String) -> BufReader<File>
{
    let f = File::open(filename).unwrap();
    BufReader::new(f)
}

fn prepare_data(filename: String) -> Vec<String>
{
    let mut v: Vec<String> = Vec::new();

    let file = open_reader(filename);
    for line in file.lines()
    {
        v.push(line.unwrap());
    }
    v
}

fn part_one(v: &Vec<String>)
{
   let mut max = 0;

    for l in v
    {
        let id = l.chars().map(|c| match c {
            'B' | 'R' => 1,
            _ => 0,
        }).fold(0, |acc, x| acc * 2 + x);
        if id > max {
            max = id;
        }
    }
    println!("Max ID found : {}", max);
}

fn input_to_vec(v: &Vec<String>) -> Vec<i32>
{
    let mut ret: Vec<i32> = Vec::new();
    for l in v {
        let id = l.chars().map(|c| match c {
            'B' | 'R' => 1,
            _ => 0,
        }).fold(0, |acc, x| acc * 2 + x);
        ret.push(id);
    }
    ret
}

fn part_two(v: &Vec<String>)
{
    let mut ids = input_to_vec(v);
    ids.sort();
    let gap = ids.windows(2).find(|w| w[0] + 1 != w[1]).unwrap();

    println!("Found seat at {}", gap[0] + 1);
}

fn main()
{
    let v = prepare_data(String::from("d5input.txt"));
    part_one(&v);
    part_two(&v);
}
