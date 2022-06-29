extern crate blas_src;

use indicatif::{ParallelProgressIterator, ProgressBar, ProgressIterator};
use ndarray::Array1;
use ndarray_rand::{rand_distr::Uniform, RandomExt};
use packed_simd_2::f64x8;
use rayon::prelude::*;
use std::{time::Instant, vec::Vec};

fn main() {
    let verbose = false;
    let n = 1_000_00;
    let pb = if verbose {
        ProgressBar::new(n)
    } else {
        ProgressBar::hidden()
    };

    let d = 256;
    let x: Vec<f64> = vec![0.2; d];
    let y: Vec<f64> = vec![0.1; d];

    println!("Rust: (naive/brute force)");
    let start = Instant::now();
    for _i in (0..n).progress_with(pb.clone()) {
        let _res: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
    }
    println!("took: {:?}", start.elapsed());

    println!("Rust: (SIMD f64x8)");
    let start = Instant::now();
    for _i in (0..n).progress_with(pb.clone()) {
        let _res: f64 = x
            .chunks_exact(8)
            .map(f64x8::from_slice_unaligned)
            .zip(y.chunks_exact(8).map(f64x8::from_slice_unaligned))
            .map(|(a, b)| a * b)
            .sum::<f64x8>()
            .sum();
    }
    println!("took: {:?}", start.elapsed());

    // what is par_iter doing? splitting for multiple threads
    // https://github.com/rayon-rs/rayon/blob/master/src/iter/plumbing/README.md
    println!("Rust: (par_iter parallelism + SIMD f64x8)");
    let start = Instant::now();
    (0..n)
        .into_par_iter()
        .progress_with(pb.clone())
        .for_each(|_i| {
            let _res: f64 = x
                .chunks_exact(8)
                .map(f64x8::from_slice_unaligned)
                .zip(y.chunks_exact(8).map(f64x8::from_slice_unaligned))
                .map(|(a, b)| a * b)
                .sum::<f64x8>()
                .sum();
        });
    println!("took: {:?}", start.elapsed());

    let x = Array1::random(d, Uniform::<f32>::new(0., 1.));
    let y = Array1::random(d, Uniform::<f32>::new(0., 1.));

    println!("Rust: (ndarray)");
    let start = Instant::now();
    for _i in (0..n).progress_with(pb.clone()) {
        let _res: f32 = x.dot(&y);
    }
    println!("took: {:?}", start.elapsed());

    println!("Rust: (par_iter parallelism + ndarray)");
    let start = Instant::now();
    (0..n)
        .into_par_iter()
        .progress_with(pb.clone())
        .for_each(|_i| {
            let _res: f32 = x.dot(&y);
        });
    println!("took: {:?}", start.elapsed());
}
