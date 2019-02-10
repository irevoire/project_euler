use std::io;
use std::cmp::max;

const SIZE: usize = 100;

fn parse() -> [[u32; SIZE]; SIZE] {
    let mut grid: [[u32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut y: usize = 0;

    loop {
        let idx = y + 1;
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        if bytes_read == 0 { break }

        for x in 0..idx {
            let num = &line[x * 3..(x + 1) * 3 - 1];
            grid[y][x] = num.parse().unwrap();
        }
        y += 1;
    }
    return grid;
}

fn main() {
    let triangle = parse();
    let mut sum = triangle[SIZE - 1];
    let mut y = SIZE - 1;

    while y != 0 {
        y -= 1;

        for x in 0..SIZE - 1 {
            sum[x] = triangle[y][x] + max(sum[x], sum[x + 1]);
        }
    }

    println!("{}", sum[0]);
}
