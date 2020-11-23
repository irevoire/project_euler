use rug::Integer;
use rug::ops::Pow;

fn main() {
    let start = 2;
    let end = 101;
    let mut values = Vec::new();

    for a in start..end {
        for b in start..end {
            let val = Integer::from(a).pow(b);
            if values.contains(&val) { continue };
            values.push(val);
        }
    }

    println!("There is {} values", values.iter().count());
}
