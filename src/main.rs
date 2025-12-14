use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};

/// Solves x^2 - N*y^2 = 1 using the Chakravala method.
/// Returns (x, y).
fn chakravala(n: u32) -> Option<(BigInt, BigInt)> {
    let n_big = BigInt::from(n);

    // 1. Check if N is a perfect square (no solution if so)
    let sqrt_n = n_big.sqrt();
    if &sqrt_n * &sqrt_n == n_big {
        println!("N={} is a perfect square. No solution exists.", n);
        return None;
    }

    // 2. Initialisation
    // We want a^2 - N*b^2 = k.
    // Standard start: b = 1, a = closest integer to sqrt(N).
    let mut a: BigInt = n_big.sqrt();
    let mut b: BigInt = BigInt::one();
    
    // Adjust 'a' to be the closest integer to sqrt(N)
    // currently a = floor(sqrt(N)). Check if ceil(sqrt(N)) is closer.
    let root = n_big.sqrt();
    let diff1 = (&n_big - &root * &root).abs();
    let root_plus = &root + &BigInt::one();
    let diff2 = (&root_plus * &root_plus - &n_big).abs();

    if diff2 < diff1 {
        a = root_plus;
    } else {
        a = root;
    }

    let mut k: BigInt = &a * &a - &n_big * &b * &b;

    println!("Starting triple: a={}, b={}, k={}", a, b, k);

    // 3. Main Loop
    // Cycle until k = 1.
    // If k = -1 or -2, or 2, the method guarantees convergence to 1 quickly.
    while k != BigInt::one() {
        // Find m such that:
        // 1. (a + b*m) is divisible by k
        // 2. |m^2 - N| is minimized
        let m = find_optimal_m(&n_big, &a, &b, &k);

        // Update a, b, k using Bhaskara's identity (Samasa)
        // new_k = (m^2 - N) / k
        // new_a = (a*m + N*b) / |k|
        // new_b = (a + b*m) / |k|
        
        let abs_k = k.abs();
        
        let new_k = (&m * &m - &n_big) / &k;
        let new_a = (&a * &m + &n_big * &b) / &abs_k;
        let new_b = (&a + &b * &m) / &abs_k;

        a = new_a;
        b = new_b;
        k = new_k;

        // println!("Step: a={}, b={}, k={}", a, b, k); // Uncomment for debug
    }

    Some((a, b))
}

/// Finds 'm' such that (a + b*m) % k == 0 and |m^2 - N| is minimized.
fn find_optimal_m(n: &BigInt, a: &BigInt, b: &BigInt, k: &BigInt) -> BigInt {
    let abs_k = k.abs();
    let target = n.sqrt();

    let mut best_m: Option<BigInt> = None;
    let mut min_diff: Option<BigInt> = None;

    // Search range: |k| + 2 (or a reasonable cap if |k| is huge)
    let limit = abs_k.to_u64().unwrap_or(1000).saturating_add(2);

    for offset in 0..limit {
        let o = BigInt::from(offset);
        let candidates = if offset == 0 {
            vec![target.clone()]
        } else {
            vec![&target + &o, &target - &o]
        };

        for candidate in candidates {
            if candidate <= BigInt::zero() { continue; }

            // Check divisibility: (a + b*m) % |k| == 0
            let sum = a + b * &candidate;
            if &sum % &abs_k == BigInt::zero() {
                let diff = (&candidate * &candidate - n).abs();

                if best_m.is_none() || min_diff.as_ref().map_or(true, |d| diff < *d) {
                    min_diff = Some(diff);
                    best_m = Some(candidate);
                } else {
                    // If we've already found a valid m and differences are increasing,
                    // it's reasonable to break early.
                    if offset > 5 { break; }
                }
            }
        }

        if best_m.is_some() && offset > abs_k.to_u64().unwrap_or(0).min(10) {
            // found a candidate and searched reasonably far: stop
            break;
        }
    }

    best_m.expect("Failed to find valid m (should not happen in Chakravala)")
}

fn main() {
    // Example: Solve x^2 - 61y^2 = 1
    // 61 is a famous test case (solutions are large).
    let n = 61;
    println!("Solving Pell's equation x^2 - {}y^2 = 1...", n);
    
    match chakravala(n) {
        Some((x, y)) => {
            println!("--- Solution Found ---");
            println!("x = {}", x);
            println!("y = {}", y);
            
            // Verify
            let lhs = &x * &x - BigInt::from(n) * &y * &y;
            println!("Check: x^2 - {}y^2 = {}", n, lhs);
        }
        None => println!("Could not solve."),
    }
}