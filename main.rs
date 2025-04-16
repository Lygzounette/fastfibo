fn faster_fib_large_prod_4k2only(n: f64) -> f64 {
    let mut p: f64 = (n - 1.)*(n - 2.) / 60.;
    let mut pbis: f64 = 1.;
    let mut a: f64 = n - 2.;
    let mut b: f64 = 2.;
    for _ in 0..n as u32 >> 2 {
        let k: f64 = (b / a) * ((b+1.) / (a+1.));
        p = p*5.*k + 1.;
        pbis = pbis*5./k + 1.;
        a = a - 2.;
        b = b + 2.;
    }
    p * pbis
}

fn main() {
    let c: u32 = 600_000_000;
    use std::time::Instant;

    let now = Instant::now();
    // *********************************************************************************************
    faster_fib_large_prod_4k2only((4*(c >> 2)+2) as f64);
    // *********************************************************************************************
    
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
