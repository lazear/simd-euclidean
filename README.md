# simd-euclidean

Blazing fast euclidean distance primitive, utilizing LLVM and Rust's auto-vectorization.

For vectors >= 32 elements, the SIMD-enabled algorithm is 2 to 8 times faster (dependent on length of vector: 2x speed up at 32 elements, 4x at 128 elements, 6x at 512 elements, etc)

Two traits are exposed by this library, `Naive` and `Vectorized`, which both provide a `squared_distance` and `distance` function. 

The `Vectorized` trait attempts to heuristically determine which SIMD layout (F32x4, F32x8, etc) will be fastest with the given input size.