# Relation-collection architecture qualification

Native bounded prototypes for four requested leads. This package reuses the
existing ordinary affine reference in `aps-partition-lab` and the signed
large-variable reducer in `pencil-symbolic-graph`. It does not call APS/F4,
ecbench, rho, an external solver, a network service or a clock.

| Lead | Implemented boundary | Still open |
| --- | --- | --- |
| Large-prime partials | Exact point keys, one/two-large-point rows, source checks, existing graph elimination, retained dependent/full rows | APS polynomial partial producer, signed point-key normalization and complete-cost model |
| Recursive decomposition | Four distinct index slots, exact two-level pair supports, all witnesses, cold and shared queries, replay binding | Higher arity and production builder/F4 routing |
| Multiple targets | One graph with separate target coefficient columns and independent dense small-system recovery | Common-factor-base production sampler and full single/batch cost accounting |
| Conditional-yield partitions | Four rules scored only on pilot targets, immutable selection, disjoint target check and fixed-rule baseline | Solver-cost predictor, power analysis and meaningful growing-size validation |

The partial producer is an explicitly generic affine matching baseline. It
enumerates deterministic public label draws and computes residual points; it
does not provide a new smoothness algorithm. The known small base is P, 2P, 3P;
the large pool is the six points 9P through 14P. Each draw is deliberately
repeated once to exercise dependent-row retention. All repeated and failed
attempts count against the draw cap. Targets enter only as points; disclosed
scalars are used by the CLI as checks after recovery.

The recursive pair construction costs binomial(M,2) group additions and stores
all witnesses, including equal sums and opposite pairs. Queries retain distinct
index quadruples in increasing order. This is ordinary meet-in-the-middle,
not a new exponent result. The batch row has shared setup once; cold per-target
rows rebuild it. Neither row is a headline speed claim.

Training scores cell tuple volume per useful relation. It enumerates pilot
roots completely, rather than claiming a cheap predictor exists. All four
scores, including zero yields, remain in the model. Selected and fixed-rule
held-out rows retain all complete roots beside admitted roots and zero-B rows.
Training, inference and reference work are separate; no post-test retraining.

Fixtures are E/F17: y²=x³+2x+2, P=(5,1), order 19, and
E/F101: y²=x³+59x+6, P=(78,4), order 107. The library only constructs these
fixtures. These are public tiny known-answer controls, not unknown-log beacon
targets. No cryptographic size or general-prime-field conclusion follows.

## Run bounded checks

```sh
cargo test --release --locked --manifest-path tools/aps-collection-architectures/Cargo.toml
cargo clippy --release --locked --manifest-path tools/aps-collection-architectures/Cargo.toml --all-targets -- -D warnings
cargo run --release --locked --manifest-path tools/aps-collection-architectures/Cargo.toml -- transport.json --transport-control
cargo run --release --locked --manifest-path tools/aps-collection-architectures/Cargo.toml -- qualification.json
```

Use distinct stdout files; the binary owns its output via `create_new` before
doing work and refuses overwrite. Transport control intentionally fails all
four arms without calling a collector, preserves every arm and exits zero.
Its zero is transport success, never a scientific result. Scientific mode
retains failed arms and exits nonzero if any arm is incomplete.

`Work` counts selected API events only. Graph semantic counters are also
retained. Field arithmetic, scalar elimination, comparisons, allocations,
target/pool construction, oracle validation and serialization are not fully
priced. Total cost and memory remain unknown. No wall-clock number, operation
ratio, speed, scaling or exponent claim is made.

See the [frozen bounded qualification plan](../../research/aps_collection_architectures_20261123/PROTOCOL.md)
for the exact workload and remaining admission requirements.
