# Test Dot Product in C++, MKL and Rust

Note from Alex:

I've been working on the Rust side in this fork, porting it to current
library versions.

The original implementation tested one extremely large vector dot product,
which is not really representative since the horizontal `sum` is only done
once. In practice, vectors tend to be maybe 32-1024 dim. A more
representative benchmark then is to run on _many_ vectors of (say)
dim=256.

For dim=256 and n=1_000_000, the output on my XPS13 was:

```
Rust: (naive/brute force)
printf: 254.178934319s seconds
Rust: (SIMD f64x4)
printf: 183.175553805s seconds
Rust: (SIMD f64x8)
printf: 161.44382163s seconds
Rust: (parallelism)
printf: 720.338301011s seconds
Rust: (parallelism + SIMD)
printf: 711.990750934s seconds
```

This worse performance actually makes more sense because we're not doing
the overall dot products in parallel, just the operations of each dot
product in parallel.

This is retained in the `bin` "original". Further changes happened in the
`bin` "fork". Note that [`indicatif` progress bar effects timings](https://github.com/console-rs/indicatif/issues/170), so
a `verbose` flag is at the top of "fork". The below had dim=256,
n=100_000.

```
# indicatif on
Rust: (naive/brute force)
printf: 18.10648789s
Rust: (SIMD f64x8)
printf: 17.184435915s
Rust: (par_iter parallelism + SIMD f64x8)
printf: 21.560859663s

# no indicatif on
Rust: (naive/brute force)
printf: 1.002513099s
Rust: (SIMD f64x8)
printf: 733.738948ms
Rust: (par_iter parallelism + SIMD f64x8)
printf: 309.76286ms
```

Fork, with blas ndarray:

```
Rust: (naive/brute force)
took: 1.077836212s
Rust: (SIMD f64x8)
took: 686.731468ms
Rust: (par_iter parallelism + SIMD f64x8)
took: 421.287732ms
Rust: (ndarray)
took: 108.83656ms
Rust: (par_iter parallelism + ndarray)
took: 113.398612ms
```

ndarray WITHOUT blas:

```
Rust: (ndarray)
took: 1.120636321s
Rust: (par_iter parallelism + ndarray)
took: 453.773839ms
```

## Requirements:
 - [Visual Studio 2017](https://docs.microsoft.com/zh-tw/visualstudio/releasenotes/vs2017-relnotes)
 - [Intel C++ compiler](https://software.intel.com/en-us/c-compilers)
 - [Intel MKL](https://software.intel.com/en-us/mkl)
 - [Armadillo](http://arma.sourceforge.net/)
 - [Eigen](http://eigen.tuxfamily.org/index.php?title=Main_Page)
 - [Rust](https://www.rust-lang.org/)

## Performance Results (in seconds):

| Programs                 | First Time  | Second Time | Third Time  | Avg. Time |
|--------------------------|-------------|-------------|-------------|-----------|
| C++ + MKL                | 0.0313977   | 0.0302247   | 0.0303358   |0.030652733|
| C++ + Armadillo + MKL    | 0.030196    | 0.0317884   | 0.0320485   |0.0313443  |
| C++ + Eigen + MKL        | 0.868       | 0.849       | 0.878       |0.865      |
| Rust                     | 0.088117961 | 0.095113075 | 0.094697091 |0.092642709|
| Rust (SIMD with f64x4)   | 0.075088962 | 0.075810549 | 0.069293656 |0.073397722|
| Rust ( SIMD with f64x8)  | 0.068858522 | 0.075586333 | 0.075168754 |0.073204536|
| Rust: (parallism)        | 0.035086217 | 0.035334371 | 0.035818443 |0.03541301 |
| Rust: (parallism + SIMD) | 0.025660633 | 0.025662228 | 0.025888573 |0.025737145|

