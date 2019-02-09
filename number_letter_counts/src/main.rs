mod number;

fn nb_char(s: String) -> u32 {
    let mut nb = 0;
    for c in s.chars() {
        if c == ' ' || c == '-' { continue };
        nb += 1;
    }
    return nb;
}

fn main() {
    let mut literal = number::Number::new();
    let mut sum = 0;

    for n in 1..1001 {
        sum += nb_char(literal.parse(n));
        println!("{} => {}", n, literal.parse(n));
    }

    println!("Counted {} chars", sum);
    println!("I missed the one of 'hundred' and 'thousand' so you should add 6 : {}", sum + 6);
}
