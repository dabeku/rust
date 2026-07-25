use std::f64::consts::PI;

fn factorial(n: u64) -> u64 {
    (1..=n).product()
}

fn fibonacci(n: usize) -> Vec<u64> {
    let mut fib = vec![0, 1];
    while fib.len() < n {
        let len = fib.len();
        let next = fib[len - 1] + fib[len - 2];
        fib.push(next);
    }
    fib
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    println!("==================================");
    println!("RUST MATHEMATICAL CALCULATIONS REPORT");
    println!("==================================\n");

    // Squares, cubes, square roots
    println!("1. BASIC NUMBER TABLE");
    println!("{:<5} {:<10} {:<10} {:<12}", "N", "Square", "Cube", "Sqrt");

    for i in 1..=25 {
        println!(
            "{:<5} {:<10} {:<10} {:.5}",
            i,
            i * i,
            i * i * i,
            (i as f64).sqrt()
        );
    }

    println!("\n================================");

    // Factorials
    println!("2. FACTORIAL TABLE");
    println!("{:<5} {:<20}", "N", "Factorial");

    for i in 1..=20 {
        println!("{:<5} {}", i, factorial(i));
    }

    println!("\n================================");

    // Fibonacci
    println!("3. FIRST 40 FIBONACCI NUMBERS");
    let fib = fibonacci(40);

    for (i, value) in fib.iter().enumerate() {
        println!("Fib({:>2}) = {}", i, value);
    }

    println!("\n==============================================");

    // Prime numbers
    println!("4. PRIME NUMBERS FROM 1 TO 200");

    let mut count = 0;

    for i in 1..=200 {
        if is_prime(i) {
            print!("{:>4}", i);
            count += 1;
            if count % 10 == 0 {
                println!();
            }
        }
    }

    println!("\n");
    println!("Total primes: {}", count);

    println!("\n==============================================");

    // Trigonometric table
    println!("5. TRIGONOMETRIC TABLE");
    println!("{:<8} {:<12} {:<12} {:<12}", "Angle", "Sin", "Cos", "Tan");

    for angle in (0..=360).step_by(15) {
        let rad = (angle as f64) * PI / 180.0;

        println!(
            "{:<8} {:<12.6} {:<12.6} {:<12.6}",
            angle,
            rad.sin(),
            rad.cos(),
            rad.tan()
        );
    }

    println!("\n==============================================");

    // Statistics
    println!("6. STATISTICS FOR NUMBERS 1..100");

    let numbers: Vec<f64> = (1..=100).map(|x| x as f64).collect();

    let sum: f64 = numbers.iter().sum();
    let mean = sum / numbers.len() as f64;

    let variance: f64 = numbers
        .iter()
        .map(|x| (x - mean).powi(2))
        .sum::<f64>()
        / numbers.len() as f64;

    let std_dev = variance.sqrt();

    println!("Count           : {}", numbers.len());
    println!("Sum             : {:.2}", sum);
    println!("Average         : {:.2}", mean);
    println!("Variance        : {:.2}", variance);
    println!("Standard Dev.   : {:.4}", std_dev);

    println!("\n==============================================");

    println!("7. EXPONENTIAL TABLE");
    println!("{:<5} {:<12} {:<12}", "N", "2^N", "e^N");

    for i in 0..=20 {
        println!(
            "{:<5} {:<12.0} {:<12.4}",
            i,
            2f64.powi(i),
            (i as f64).exp()
        );
    }

    println!("\n==============================================");
    println!("REPORT COMPLETE");
    println!("==============================================");
}
