fn permutations(mut arr: Vec<u8>, start: usize, res: &mut Vec<String>) {
    let size = arr.iter().count();

    if size == start {
        let mut s = String::new();
//        arr.iter().map(|c| s.push(*c as char)).collect();
        for b in arr { s.push(b as char) };
        res.push(s);
        return;
    }

    for end in start..size {
        arr.swap(start, end);
        permutations(arr.clone(), start + 1, res);
        arr.swap(start, end);
    }
}

fn generate_all_permutations(value: String) -> Vec<String> {
    let mut res = Vec::new();
    let value = value.into_bytes();

    permutations(value, 0, &mut res);

    return res;
}

fn main() {
    let test = "1234567890".to_string();
    println!("Generating the permutations");
    let mut res = generate_all_permutations(test.clone());
    println!("Sorting the permutations");
    res.sort();
    println!("Searching for the good permutation");
    for (index, el) in res.iter().enumerate() {
        if index == 1_000_000 - 1 {
            println!("The {} permutation is {}", index, el);
            break;
        }
    }
}
