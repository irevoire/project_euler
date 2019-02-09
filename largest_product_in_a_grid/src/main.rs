use std::io;

const SIZE: usize = 20;

fn parse() -> [[u32; SIZE]; SIZE] {
    let mut grid: [[u32; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    let mut y: usize = 0;

    loop {
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        if bytes_read == 0 { break }

        for x in 0..SIZE {
            let num = &line[x * 3..(x + 1) * 3 - 1];
            grid[y][x] = num.parse().unwrap();
        }
        y += 1;
    }
    return grid;
}

fn print_grid(grid: &[[u32; SIZE]]) {
    for line in grid {
        for el in line.iter() {
            print!("{:02} ", el);
        }
        print!("\n");
    }
}

fn main() {
    let grid = parse();
    let mut max = 0;

    print_grid(&grid);

    // checking to the right
    for y in 0..SIZE {
        for x in 0..SIZE - 3 {
            let mult = grid[y][x] * grid[y][x + 1] * grid[y][x + 2] * grid[y][x + 3];
            if mult > max { max = mult };
        }
    }

    for y in 0..SIZE - 3 {
        // checking to the bottom
        for x in 0..SIZE {
            let mult = grid[y][x] * grid[y + 1][x] * grid[y + 2][x] * grid[y + 3][x];
            if mult > max { max = mult };
        }

        // checking to the bottom right
        for x in 0..SIZE - 3 {
            let mult = grid[y + 1][x + 1] * grid[y + 1][x + 1] * grid[y + 2][x + 2] * grid[y + 3][x + 3];
            if mult > max { max = mult };
        }

        // checking to the left bottom
        for x in 4..SIZE {
            let mult = grid[y][x] * grid[y + 1][x - 1] * grid[y + 2][x - 2] * grid[y + 3][x - 3];
            if mult > max { max = mult };
        }
    }

    println!("max is {}", max);
}
