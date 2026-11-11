# Fixed successor and independent field replay

Development correctness crate overF101;read ../SUCCESSOR_CHECKPOINT.md and
frozen ../SUCCESSOR_GLOBAL_PROOF.md for exact standing and remaining work.

Run with Rust1.99.1 from this directory:

```sh
sha256sum -c reference.sha256
cargo fmt -- --check
cargo clippy --all-targets --offline --locked -- -D warnings
cargo test --release --offline --locked -- --nocapture
```

The original consumer seven.rs constructor is copied byte-identically as
src/seven-reference.rs,SHA bb640465bfe9406e5b5e275b5127c73417b48cec5a90006d2f2bb87f72f40ae9.
Existing isogeny_algos field/poly/curve/Kohel/HLP engines are dependencies;
no new arithmetic engine is implemented. Five final tests cover the old
exact elliptic7 controls,the actual next nondual kernels/C2,independent flat
Psi TSV validity/Frobenius,C2 quotient charts and finite-ring49 field-boundary
certificates. e11fb7/190ae8/b07c76/4a98a7 remain unsigned. [pcc:unverified]

The independent replay checks actual curve-divisor identities/Frobenius,
not independent Jacobian addition or kernel membership. Whole-Jacobian
successor evaluation is the next separate task. Source/raw snapshots and
Clippy06 failure are retained in ../evidence. No timings here are benchmarks.
[pcc:unverified]
