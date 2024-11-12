use std::time::Instant; 

fn main() {
    let start = Instant::now();

    for i in 1..=1_000_000 {
        println!("Looping {}", i); 
    }

    let duration = start.elapsed();
    println!("Time processed: {:?}", duration);
}
