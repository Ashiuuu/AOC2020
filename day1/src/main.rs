use std::fs::File;
use std::io::{BufReader, BufRead};

fn open_reader(filename: String) -> BufReader<File>
{
    let f = File::open(filename).unwrap();
    BufReader::new(f)
}

fn main()
{
    let mut v: Vec<i32> = Vec::new();

    let file = open_reader(String::from("./d1input.txt"));
    for line in file.lines()
    {
        v.push(line.unwrap().parse::<i32>().unwrap());
    }

    for x in &v
    {
        for y in &v
        {
            for z in &v
            {
                if x + y + z == 2020
                {
                    println!("Triplet found : {} {} {}", x, y, z);
                    println!("Result : {}", x * y * z);
                    return;
                }
            }
        }
    }
}
