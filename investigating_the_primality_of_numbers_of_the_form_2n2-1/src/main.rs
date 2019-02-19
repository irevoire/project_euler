use rug::Integer;
use rug::integer::IsPrime;
use rug::ops::Pow;

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
    let mut n = Integer::from(1);
    let mut cpt = Integer::from(0);

    while n < 50_000_000 {
        n += 1;

        let num = gen(&n);
        let prime = num.is_probably_prime(50);
        if prime == IsPrime::Yes { cpt += 1 }
        if prime == IsPrime::Probably { 
            if is_prime(&num) { cpt += 1 }
        }
        if n.is_divisible(&Integer::from(1_000_000)) {
            println!("new percentage {} stop at 50_000_000", n);
        }
    }

    println!("There is {} primes number", cpt);
}
