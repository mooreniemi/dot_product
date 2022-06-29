use packed_simd_2::{f64x4, f64x8};
use rayon::prelude::*;
use std::{time::Instant, vec::Vec};

fn main() {
    let n = 100000000;
    let x: Vec<f64> = vec![0.2; n];
    let y: Vec<f64> = vec![0.1; n];

    println!("Rust:");
    let start = Instant::now();
    let res: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
    println!("res: {}", res);
    println!("printf: {:?} seconds", start.elapsed());

    println!("Rust: (SIMD f64x4)");
    let start = Instant::now();
    let res: f64 = x
        .chunks_exact(4)
        .map(f64x4::from_slice_unaligned)
        .zip(y.chunks_exact(4).map(f64x4::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x4>()
        .sum();
    println!("res: {}", res);
    println!("printf: {:?} seconds", start.elapsed());

    println!("Rust: (SIMD f64x8)");
    let start = Instant::now();
    let res: f64 = x
        .chunks_exact(8)
        .map(f64x8::from_slice_unaligned)
        .zip(y.chunks_exact(8).map(f64x8::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x8>()
        .sum();
    println!("res: {}", res);
    println!("printf: {:?} seconds", start.elapsed());

    println!("Rust: (parallelism)");
    let start = Instant::now();
    let res: f64 = x.par_iter().zip(y.par_iter()).map(|(a, b)| a * b).sum();
    println!("res: {}", res);
    println!("printf: {:?} seconds", start.elapsed());

    println!("Rust: (parallelism + SIMD)");
    let start = Instant::now();
    let res: f64 = x
        .par_chunks(8)
        .map(f64x8::from_slice_unaligned)
        .zip(y.par_chunks(8).map(f64x8::from_slice_unaligned))
        .map(|(a, b)| a * b)
        .sum::<f64x8>()
        .sum();
    println!("res: {}", res);
    println!("printf: {:?} seconds", start.elapsed());
}
