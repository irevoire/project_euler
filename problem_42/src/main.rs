use std::io;

fn parse() -> Vec<String> {
    let mut names: Vec<String> = Vec::new();

    loop {
        let mut line = String::new();
        let bytes_read = io::stdin().read_line(&mut line).unwrap();
        if bytes_read == 0 { break }
        let chars = line.chars();

        let mut name = String::new();
        for c in chars {
            if c == ',' {
                names.push(name);
                name = String::new();
            } else if c != '"' { 
                name.push_str(&c.to_string());
            }
        }
        names.push(name);
    }
    return names;
}

fn hash(s: String) -> u64 {
    s.bytes().fold(0, |acc, b| acc + (b + 1 - 'A' as u8) as u64)
}

fn is_triangular(n: u64) -> bool{
    let tmp = 8 * n + 1;
    let tmp = (tmp as f64).sqrt() - 1.0;
    let tmp = tmp / 2.0;
    return tmp.trunc() == tmp;
}

fn main() {
    let names = parse();
    let mut triangle_word = 0;

    for name in names.iter() {
        if is_triangular(hash(name.to_string())) {
            triangle_word += 1;
        }
    }

    println!("There is {} triangle words", triangle_word);
}
