# Six-class GLV rho research path

The native `six_class_rho` worker implements the six-class orbit walk from
[Curve Smash commit 1f084c1](https://github.com/Layr-Labs/curvesma.sh/commit/1f084c1914c81a0cdc830b176453adba5dc19ab9).
It is a research `BigUint` worker. A correct distinguished point or planted
toy logarithm demonstrates arithmetic interoperability, not a practical
secp256k1 solve or the submission's conditional 126.96-bit cost.

## Commands

`six_class_challenge generate TEMPLATE PUBLIC PRIVATE` checks the pinned
secp256k1 curve, generator, GLV constants, and upstream profile hash. It draws
the private scalar from OS randomness using rejection sampling below the
subgroup order, computes `Q=[x]G`, and writes a mode-0600 private audit record
and a public job in distinct directories. A zero draw stops explicitly:
the current worker has no infinity target encoding. `six_class_challenge
verify PUBLIC ANSWER` reads `{"x":"<64 lowercase hex digits>"}`, checks
`x<n` and `[x]G=Q`, and emits a job-bound `valid` result. It does not read
the audit record. The `crypto-autoresearcher` harness runs the custody,
solver, and answer checker in separate pinned networkless containers, with
the private mount absent from the latter two.

`six_class_compare JOB MAX_WALKERS TRIALS` runs negation rho / six-class rho /
negation rho on the same frozen target for each trial. The repeated baseline
arms are an A/A timing and deterministic work control. Fresh setup, walks,
in-memory distinguished-point lookup, collision processing, and answer
checks are inside each arm's measured time. The JSON keeps every trial's
timings and counters, including null logarithms. A/A repeats reuse identical
seeds and cannot estimate collision-success probability. The output lists
field arithmetic, allocation, memory high-water, networking, coordination,
and success amplification as unpriced. Its wall times are not Curve Smash
instruction-equivalent scores.

The harness owns the five-clause falsifiable H1 matrix, isolated receipts,
and one bounded reviewer-selected follow-up. Cairn's native `sixclass_dp`
checker independently checks the class predicate and `aG+bQ` equation for
both p25 and secp256k1 jobs. A point equation alone does not establish walker
provenance; sampled re-walks remain a separate requirement.

## Reproduce local checks

```sh
cargo test --bin six_class_challenge
cargo test --lib cryptanalysis::six_class_rho
cargo build --bin six_class_rho --bin six_class_compare --bin six_class_challenge
```

Use the harness p25 fixture for a short comparison. The independent and
full-curve receipts live in `crypto-autoresearcher/harness/evidence/`.
Production H1 distributions, fixed-limb throughput, and full-work cost are
open obligations; no score promotion follows from these commands.
