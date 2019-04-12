# simd-euclidean

Blazing fast euclidean distance implementation, utilizing LLVM and Rust's auto-vectorization.

For vectors >= 32 elements, the SIMD-enabled algorithm is 2 to 8 times faster, with longer inputs provided greater speedups.

Two traits are exposed by this library, `Naive` and `Vectorized`, which both provide a `squared_distance` and `distance` function. 

```


```

The `Vectorized` trait attempts to heuristically determine which SIMD layout (F32x4, F32x8, etc) will be fastest with the given input size.

Shown below is a comparison between `Naive::distance` and `Vectorized::distance` on random vectors of single precision floating point numbers. 

![Benchmark, f32](lines.svg "Benchmark, f32")