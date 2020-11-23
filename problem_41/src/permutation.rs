fn permutations(mut arr: Vec<u8>, start: usize, res: &mut Vec<String>) {
    let size = arr.iter().count();

    if size == start {
        let mut s = String::new();
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

pub fn generate_all_permutations(value: String) -> Vec<String> {
    let mut res = Vec::new();
    let value = value.into_bytes();

    permutations(value, 0, &mut res);

    return res;
}
