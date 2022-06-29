use indicatif::ProgressIterator;
use packed_simd_2::{f64x4, f64x8};
use rayon::prelude::*;
use std::{time::Instant, vec::Vec};

fn main() {
    let d = 256;
    let n = 1_000_000;
    let x: Vec<f64> = vec![0.2; d];
    let y: Vec<f64> = vec![0.1; d];

    println!("Rust: (naive/brute force)");
    let start = Instant::now();
    for _i in (0..n).progress() {
        let _res: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
    }
    println!("printf: {:?} seconds", start.elapsed());

    println!("Rust: (SIMD f64x4)");
    let start = Instant::now();
    for _i in (0..n).progress() {
        let _res: f64 = x
            .chunks_exact(4)
            .map(f64x4::from_slice_unaligned)
            .zip(y.chunks_exact(4).map(f64x4::from_slice_unaligned))
            .map(|(a, b)| a * b)
            .sum::<f64x4>()
            .sum();
    }
    println!("printf: {:?} seconds", start.elapsed());

    println!("Rust: (SIMD f64x8)");
    let start = Instant::now();
    for _i in (0..n).progress() {
        let _res: f64 = x
            .chunks_exact(8)
            .map(f64x8::from_slice_unaligned)
            .zip(y.chunks_exact(8).map(f64x8::from_slice_unaligned))
            .map(|(a, b)| a * b)
            .sum::<f64x8>()
            .sum();
    }
    println!("printf: {:?} seconds", start.elapsed());

    // what is par_iter doing? splitting for multiple threads
    // https://github.com/rayon-rs/rayon/blob/master/src/iter/plumbing/README.md
    println!("Rust: (par_iter parallelism)");
    let start = Instant::now();
    for _i in (0..n).progress() {
        let _res: f64 = x.par_iter().zip(y.par_iter()).map(|(a, b)| a * b).sum();
    }
    println!("printf: {:?} seconds", start.elapsed());

    println!("Rust: (par_iter parallelism + SIMD f64x8)");
    let start = Instant::now();
    for _i in (0..n).progress() {
        let _res: f64 = x
            .par_chunks(8)
            .map(f64x8::from_slice_unaligned)
            .zip(y.par_chunks(8).map(f64x8::from_slice_unaligned))
            .map(|(a, b)| a * b)
            .sum::<f64x8>()
            .sum();
    }
    println!("printf: {:?} seconds", start.elapsed());
}
