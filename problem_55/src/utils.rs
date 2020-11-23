pub fn reverse(n: u128) -> u128 {
    let mut rev = 0;
    let mut n = n;

    loop {
        let digit = n % 10;
        rev = (rev * 10) + digit;
        n = n / 10;

        if n == 0 {
            break;
        }
    }
    return rev;
}

pub fn is_palindrome(n: u128) -> bool {
    n == reverse(n)
}
