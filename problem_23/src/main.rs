use project_euler::Num;

fn main() {
    let nums = (1..28123)
        .filter(|el| el.is_abundant())
        .collect::<Vec<u32>>();
    let mut abundant_sum = 0;
    for i in 1..28123 {
        for a in &nums {
            let mut finished = false;
            if a > &i {
                abundant_sum += i;
                break;
            }
            for b in &nums {
                if a + b > i {
                    break;
                }

                if a + b == i {
                    finished = true;
                    break;
                }
            }
            if finished {
                break;
            }
        }
    }
    println!("there is: {}", abundant_sum);
}
