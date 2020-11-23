fn main() {
    // since they don't start the index at 0
    let mut champernowne = "0".to_string();
    let mut cpt = 1;

    while cpt < 1_000_000 {
        champernowne.push_str(&cpt.to_string());
        cpt += 1;
    }

    println!("{}", &champernowne[0..20]);

    let d1: u32 = champernowne[1..2].parse().unwrap();
    let d10: u32 = champernowne[10..11].parse().unwrap();
    let d100: u32 = champernowne[100..101].parse().unwrap();
    let d1_000: u32 = champernowne[1_000..1_001].parse().unwrap();
    let d10_000: u32 = champernowne[10_000..10_001].parse().unwrap();
    let d100_000: u32 = champernowne[100_000..100_001].parse().unwrap();
    let d1_000_000: u32 = champernowne[1_000_000..1_000_001].parse().unwrap();

    let d12: u32 = champernowne[12..13].parse().unwrap();
    println!("d12 {}", d12 );

    println!("d1 {}", d1 );
    println!("d10 {}", d10 );
    println!("d100 {}", d100 );
    println!("d1_000 {}", d1_000 );
    println!("d10_000 {}", d10_000 );
    println!("d100_000 {}", d100_000 );
    println!("d1_000_000 {}", d1_000_000 );

    println!("res {}", 
             d1 *
             d10 *
             d100 *
             d1_000 *
             d10_000 *
             d100_000 *
             d1_000_000
            );
}
