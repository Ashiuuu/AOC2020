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

fn count_char(s: &String, c: char) -> u32
{
    let mut count = 0;
    for chr in s.chars()
    {
        if chr == c
        {
            count = count + 1;
        }
    }
    count
}

fn parse_to(s: &String) -> (u32, u32, char, &str)
{
    let v = s.split(' ').collect::<Vec<&str>>();
    let ret3 = v[1].chars().nth(0).unwrap();
    let ret4 = v[2];

    let v2 = v[0].split('-').collect::<Vec<&str>>();
    let ret1 = v2[0].parse::<u32>().unwrap();
    let ret2 = v2[1].parse::<u32>().unwrap();

    (ret1, ret2, ret3, &ret4)
}

fn test_entry(s: &String) -> bool
{
    let (low, high, c, st) = parse_to(s);
    let count = count_char(&String::from(st), c);

    if count >= low && count <= high
    {
        return true;
    }
    false
}

fn part_one(v: &Vec<String>)
{
    let mut count = 0;

    for line in v
    {
        if test_entry(line) == true
        {
            count = count + 1;
        }
    }
    println!("Found {} correct passwords for part 1 !", count);
}

fn part_two(v: &Vec<String>)
{
    let mut count = 0;

    for line in v
    {
        let (i1, i2, c, s) = parse_to(line);
        let c1 = match s.chars().nth((i1 - 1) as usize)
        {
            Some(x) => x,
            None => continue,
        };
        let c2 = match s.chars().nth((i2 - 1) as usize)
        {
            Some(x) => x,
            None => continue,
        };
        if (c1 == c && c2 != c) || (c1 != c && c2 == c)
        {
            count = count + 1;
        }
    }

    println!("Found {} correct passwords for part 2 !", count);
}

fn main()
{
    let v = prepare_data(String::from("d2input.txt"));

    part_one(&v);
    part_two(&v);
}
