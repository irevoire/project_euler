use project_euler::Divisors;

fn main() {
    let range = 1..1000;
    let (num, cycle) = range
        .clone()
        .enumerate()
        .filter_map(|(i, v)| 1_u32.cycle(v).map(|v| (i, v)))
        .fold((0, Vec::new()), |acc, val| {
            if acc.1.len() < val.1.len() {
                val
            } else {
                acc
            }
        });
    println!(
        "Longuest cycle in {:?} contains {} terms and is {}",
        range,
        cycle.len(),
        num + 1
    );
}
