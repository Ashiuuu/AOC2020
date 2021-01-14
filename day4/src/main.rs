use std::fs::File;
use std::io::{BufReader, BufRead};
extern crate regex;
use regex::Regex;

fn open_reader(filename: String) -> BufReader<File>
{
    let f = File::open(filename).unwrap();
    BufReader::new(f)
}

fn prepare_data(filename: String) -> Vec<String>
{
    let mut v: Vec<String> = Vec::new();

    let file = open_reader(filename);
    let mut curr: Vec<String> = Vec::new();
    for line in file.lines()
    {
        let l = line.unwrap();
        if l.is_empty()
        {
            v.push(curr.join(" "));
            curr = Vec::new();
        }
        else
        {
            curr.push(l);
        }
    }
    v.push(curr.join(" "));
    v
}

fn check_value(f: &str, k: &str) -> bool
{
    if f == "byr" {
        if k.len() == 4 {
            let ki = k.parse::<i32>().unwrap();
            if ki >= 1920 && ki <= 2002 {
                return true;
            }
        }
        return false;
    }
    if f == "iyr" {
        if k.len() == 4 {
            let ki = k.parse::<i32>().unwrap();
            if ki >= 2010 && ki <= 2020 {
                return true;
            }
        }
        return false;
    }
    if f == "eyr" {
        if k.len() == 4 {
            let ki = k.parse::<i32>().unwrap();
            if ki >= 2020 && ki <= 2030 {
                return true;
            }
        }
        return false;
    }
    if f == "hgt" {
        let hgt = Regex::new(r"([0-9]+)(cm|in)").unwrap();
        if hgt.is_match(k) {
            let m = hgt.captures_iter(k).next().unwrap();
            let p = m[1].parse::<i32>().unwrap();
            if &m[2] == "cm" {
                if p >= 150 && p <= 193 {
                    return true;
                }
            }
            if &m[2] == "in" {
                if p >= 59 && p <= 76 {
                    return true;
                }
            }
        }
        return false;
    }
    if f == "hcl" {
        let hcl = Regex::new(r"#[0-9a-f]{6}").unwrap();
        return hcl.is_match(k);
    }
    if f == "ecl" {
        let colors = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
        return colors.contains(&k);
    }
    if f == "pid" {
        let pid = Regex::new(r"^[0-9]{9}$").unwrap();
        return pid.is_match(k);
    }
    false
}

fn check_fields(v: &Vec<&str>) -> bool
{
    let check = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for x in check.iter()
    {
        let mut flag = false;
        for y in v
        {
            if x == y
            {
                flag = true;
                break;
            }
        }
        if flag == false
        {
            return false;
        }
    }
    true
}

fn check_fields2(f: Vec<&str>, k: Vec<&str>) -> bool
{
    let check = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    for x in check.iter()
    {
        let mut flag = false;
        for i in 0..f.len()
        {
            if x == &f[i]
            {
                flag = check_value(f[i], k[i]);
                break;
            }
        }
        if flag == false
        {
            return false;
        }
    }
    true
}

fn part_one(v: &Vec<String>)
{
    let mut count = 0;

    for x in v
    {
        let mut fields: Vec<&str> = Vec::new();
        for y in x.split(' ')
        {
            let temp = y.split(':').collect::<Vec<&str>>()[0];
            fields.push(temp);
        }
        if check_fields(&fields) == true
        {
            count += 1;
        }
    }

    println!("Found {} valid passwords !", count);
}

fn part_two(v: &Vec<String>)
{
    let mut count = 0;

    for x in v
    {
        let mut fields: Vec<&str> = Vec::new();
        let mut keys: Vec<&str> = Vec::new();
        for y in x.split(' ')
        {
            let temp = y.split(':').collect::<Vec<&str>>();
            fields.push(temp[0]);
            keys.push(temp[1]);
        }
        if check_fields2(fields, keys) == true
        {
            count += 1;
        }
    }

    println!("Found {} valid passwords !", count);
}

fn main()
{
    let v = prepare_data(String::from("d4input.txt"));

    part_one(&v);
    part_two(&v);
}