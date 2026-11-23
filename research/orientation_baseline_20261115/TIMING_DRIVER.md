# Timing driver configuration (unmeasured)

This supplements the pre-test PROTOCOL.md. No timing window has been run. The native `orientation-baseline-bench` driver compares pair-scan, scalar, batch and scalar-aa using the exact GF(256) fixtures already checked by the tests. Per base size: one complete warmup per arm, then five measured rounds, reversing arm order on odd rounds, with 16 repetitions of the eight-target stream per arm and round. No result-dependent filtering, retries, target substitution or first-hit stopping.

The interval includes cold point validation, index/scratch allocation, complete output generation and exact output-vector comparison. Fixture construction and reference-vector generation are outside every arm's interval. Records give source revision baked into the binary, arm, round, base size, target/repetition counts, output count and elapsed nanoseconds. Raw records alone are not L2 admission or a performance claim; the launch wrapper must bind the actual executable/image hashes, environment gates, clock/process metadata and A/A analysis. Missing admission must remain visible.

Development check, with no timer opened:

```
cargo run --release --manifest-path research/orientation_baseline_20261115/Cargo.toml --bin orientation-baseline-bench -- --check
```

After Git publication, budget/pool readback, and EKS L2 admission, build from the exact frozen revision with `BASELINE_SOURCE_REVISION` set to that 40-character revision. Run `orientation-baseline-bench --measure` only through the admitted wrapper. The executable refuses timing if the source macro was absent at build time. A supplied source string is not proof of binary provenance: independently hash and bind the tested binary and image. Proposed execution/budget limits remain those in PROTOCOL.md. No timed run is authorized by this file alone.
