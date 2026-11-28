# Two-large-prime hyperelliptic replay

This small crate compiles the exact `src/cryptanalysis/hyperelliptic_index_calculus.rs`, graph, Mumford/Cantor arithmetic, polynomial arithmetic, operation counters, and utility sources without compiling the repository's unrelated test fixtures. It is for public toy curves only.

From the repository root, run:

```sh
cargo test --locked --manifest-path research/hec_two_lp_replay_20261128/Cargo.toml two_large_prime
cargo test --locked --manifest-path research/hec_two_lp_replay_20261128/Cargo.toml two_place_oracle_reconstructs_genus_four_and_five_divisors
```

The focused tests check graph algebra, signed decomposition, combined Jacobian relations, and verified scalar recovery at genus 2, 3, 4, and 5. The genus-4/5 instance uses a smooth one-point model over `F_11`; it is not a performance benchmark.

The shim uses exhaustive square-root search for the small toy primes and a word-prime check. Its dense elliptic linear-algebra placeholder is unused by these sparse-path tests. Run the root crate's tests on a full checkout before treating this as a general integration gate. Do not run this shim's unfiltered test suite as a substitute for the root suite.
