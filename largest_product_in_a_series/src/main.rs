use std::io;

fn mul(arr: &[u128]) -> u128 {
    let mut mul = 1;
    for el in arr {
        mul *= el;
    }
    return mul;
}

fn main() {
    const SIZE: usize = 13;
    let mut arr: [u128; SIZE] = [1; SIZE];
    let mut max = 1;

    loop {
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        if bytes_read == 0 {
            println!("EOF reached");
            break;
        }

        for c in line.chars() {
            let i = match c.to_digit(10) {
                None => {
                    if c == 97 as char {
                        break;
                    } else {
                        continue;
                    }
                },
                Some(i) => i,
            };

            arr.rotate_left(1);
            arr[SIZE - 1] = i as u128;

            let mul = mul(&arr);
            if mul > max {
                max = mul;
                println!("new max is {} from {:?}", max, arr);
            }
        }
    }

    println!("{}", max);
}
