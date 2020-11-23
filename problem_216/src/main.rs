use rug::Integer;
use rug::integer::IsPrime;
use rug::ops::Pow;
use std::thread;
use std::sync::mpsc;

fn pgp(a: Integer, n: &Integer) -> Integer {
    a.pow_mod(&Integer::from(n - 1), n).unwrap()
}

fn is_prime(n: &Integer) -> bool {
    if pgp(Integer::from(2), n) == 1 &&
        pgp(Integer::from(3), n) == 1 &&
            pgp(Integer::from(5), n) == 1 &&
            pgp(Integer::from(7), n) == 1 {
                return true;
            }
    return false;

}

fn gen(n: &Integer) -> Integer {
    let n = n.pow(2);
    let n = Integer::from(n);
    &2 * n - &1
}

fn main() {
    let end = 50_000_000;
    let core = num_cpus::get();
    let split = end / core;
    let (tx, rx) = mpsc::channel();


    for i in 0..core {
        let tx1 = mpsc::Sender::clone(&tx);
        let slice = (split * i)..(split * (i + 1));

        thread::spawn(move || {
            let mut cpt = 0;

            for n in slice {
                let num = gen(&Integer::from(n));
                let prime = num.is_probably_prime(50);
                if prime == IsPrime::Yes { cpt += 1 }
                if prime == IsPrime::Probably {
                    if is_prime(&num) { cpt += 1 }
                }
            }

            tx1.send(cpt).unwrap();
        });
    }
    drop(tx);

    let cpt: u64 = rx.iter().sum();
    println!("There is {} primes number", cpt);
}
