use euler::Prime;

const NB: u64 = 4;

fn main() {
    let mut i = 1;
    loop {
        let mut numbers: Vec<u64> = (i..(i + NB)).collect();
        numbers.reverse();
        if let Some(fail_pos) = numbers.iter().position(|x| {
            let mut div = x.all_prime_divisors().collect::<Vec<u64>>();
            div.dedup();
            div.len() as u64 != NB
        }) {
            i += NB - fail_pos as u64;
        } else {
            println!("{:?}", i);
            return;
        }
    }
}
