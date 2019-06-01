use rayon::prelude::*;
use std::sync::Mutex;

const MAX: u64 = 1_000_000;

fn sum_of_divisors(n: u64) -> u64 {
    let mut sum = 0;

    for i in 1..n {
        if n % i == 0 {
            sum += i;
        }
    }

    sum
}

/// itern function for the amicable_chains function
/// will tail call herself until an element appear twice
/// Modify the given vector
/// return false if a number in the chain is too big
fn __amicable_chains(n: u64, chain: &mut Vec<u64>) -> bool {
    if n > MAX {
        return false;
    } else if chain.contains(&n) {
        chain.push(n);
        return true;
    }
    chain.push(n);
    __amicable_chains(sum_of_divisors(n), chain)
}

/// generate a chains of amicable number
/// `n`: the base to use for generating the chain
/// return a chain BUT `n` may not be part of the chain
fn amicable_chains(n: u64) -> Option<Vec<u64>> {
    let mut chain = Vec::new();
    if !__amicable_chains(n, &mut chain) {
        return None;
    }
    // the element you gave (which may not be part of the chain) is now last
    chain.reverse();
    // remove all the elements that are not parts of the chain
    let first = *chain.first().unwrap();
    loop {
        if let Some(last) = chain.pop() {
            // we got our chain
            if first == last {
                break;
            }
        } else {
            // no idea of what happened
            return None;
        }
    }
    Some(chain)
}

fn main() {
    let max = Mutex::new(0);
    let chains = Mutex::new(Vec::new());

    (1..MAX).into_par_iter().for_each(|n| {
        let chain = amicable_chains(n);
        if chain.is_none() {
            return;
        }
        let chain = chain.unwrap();
        let mut max = max.lock().unwrap();
        if *max < chain.len() {
            println!("Got a chain {:?}", chain);
            *max = chain.len();
            *chains.lock().unwrap() = chain;
        }
    });

    println!("Longuest chain is: {:?}", chains.lock().unwrap());
    println!(
        "Smallest element is: {}",
        chains.lock().unwrap().iter().min().unwrap()
    );
}
