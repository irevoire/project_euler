const SIZE: usize = 100 + 1;

fn main() {
    let mut numbers: [u32; SIZE] = [0; SIZE];
    for n in 0..SIZE {
        numbers[n as usize] = n as u32;
    }
; //, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20];

    let sum_square = sum_square(&numbers);
    println!("{}", sum_square);
    let square_sum = square_sum(&numbers);
    println!("{}", square_sum);
    println!("res = {}", square_sum - sum_square);
}

fn sum_square(arr : &[u32]) -> u32 {
    let mut res : u32 = 0;
    for n in arr {
        res += n.pow(2);
    }
    return res;
}

fn square_sum(arr : &[u32]) -> u32 {
    let mut res : u32 = 0;
    for n in arr {
        res += n;
    }
    res.pow(2)
}
