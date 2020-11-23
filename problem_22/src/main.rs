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

fn main() {
    let mut names = parse();
    let mut sum = 0;
    names.sort();

    for (index, name) in names.iter().enumerate() {
        sum += hash(name.to_string()) * ((index + 1) as u64);
    }

    println!("sum is {}", sum);
}
