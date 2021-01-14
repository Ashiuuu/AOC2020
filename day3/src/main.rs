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
    let mut x = 0;
    let mut count = 0;

    for i in 0..v.len()
    {
        let c = v[i].chars().nth(x).unwrap();
        if c == '#'
        {
            count += 1;
        }
        x = (x + 3) % v[i].len();
    }
    println!("Found {} trees !", count);
}

fn part_two(v: &Vec<String>)
{
    let xs = [1, 3, 5, 7, 1];
    let ys = [1, 1, 1, 1, 2];
    let mut count: Vec<i64> = Vec::new();

    for k in 0..5
    {
        let mut curr_count = 0;
        let mut x = 0;
        let mut y = 0;
        for _i in 0..v.len()
        {
            let c = v[y].chars().nth(x).unwrap();
            if c == '#'
            {
                curr_count += 1;
            }
            x = (x + xs[k]) % v[y].len();
            y += ys[k];
            if y >= v.len()
            {
                break;
            }
        }
        count.push(curr_count);
    }

    let mut result: i64 = 1;
    for n in count
    {
        result *= n;
    }
    println!("Final result : {}", result);
}


fn main()
{
    let v = prepare_data(String::from("d3input.txt"));

    part_one(&v);
    part_two(&v);
}
