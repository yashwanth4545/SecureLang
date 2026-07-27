use compiler::compile;

fn main() {
    let start = std::time::Instant::now();
    let source = "user login { username input password secure authenticate }";
    for _ in 0..10_000 {
        let _ = compile(source);
    }
    let duration = start.elapsed();
    println!("Benchmark: 10,000 compilations in {:?}", duration);
}
