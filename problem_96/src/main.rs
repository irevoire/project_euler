// use std::fs::File;
// use std::env;

mod utils;
use crate::utils::hash;
mod solver;

pub const SIZE: usize = 9;

fn main() {
    let mut sum: u32 = 0;

    for _ in 0..50 {
        let mut grid = utils::parse();
        utils::dump_table(grid);

        if !solver::solve(&mut grid, 0, 0) {
            println!("Your grid cannot be solved");
        } else {
            println!("Solved grid :");
        }

        utils::dump_table(grid);

        let res: u32 = (grid[hash(0, 0)].unwrap() as u32) * 100 +
            (grid[hash(1, 0)].unwrap() as u32) * 10 +
            (grid[hash(2, 0)].unwrap() as u32);
        println!("number is : {}", res);
        sum += res;
        println!("sum is : {}", sum);
        
    }
}
