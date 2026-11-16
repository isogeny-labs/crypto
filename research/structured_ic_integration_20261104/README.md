# C1–C3 consumer integration

2026-11-04. Producer report for PCC-20261104-458dda, standing unverified.
This report records native software integration and local development checks.
[pcc:unverified]

The consumer accepts a versioned strict JSON request, builds the structured
predicate/template once, returns counts and signed prefix witnesses for each
public query, and supports same-implementation semantic replay. The implementation
is [`structured_runner.rs`](../../crates/hashed-ecdl-tensor/src/structured_runner.rs);
the command is `tensor-structured`. [pcc:unverified]

| Requested integration | Entry point | Scope |
|---|---|---|
| C1 predicates | `membership: circuit` | Rational two-isogeny fibres and actual subgroup constraints in the existing complete point-sum network |
| C2 extraction | `execute_json`, `replay_json` | Exact counts, prefix witnesses, native arithmetic replay and all-or-nothing batch output |
| C3 reuse | `membership: fibre_message` | Root-based local membership diagram and immutable network reused across public query points |
| Cryptanalysis consumers | bindings API plus suite `structured_backend` and runner alias | Explicit separate backend types preserve the suite's older tensor API |
| Autoresearch retrieval | KN-OPEN-08ba7e | Pinned implementation pointers and the full-summation-message question |

See [integration.svg](integration.svg) and the
[user contract](../../docs/structured-prime-ic.md). All work/resource limits,
membership modes, signed-tuple policy and identity representation are explicit.
Setup and query diagnostic steps share one request budget. [pcc:unverified]

## Review and checks

The development panel checks public fixture counts/witnesses, same-source replay,
an answer changed and rehashed, source-pin mutation, nested duplicates, invalid
parameters, empty fibres in both modes and resource exhaustion after an earlier
query could finish. Library checks, Clippy, formatting and the declared minimum
Rust version accompany the consumer panel. Raw stdout/stderr are retained in
`checks/`; their observed summaries are in `checks.json`. These local checks are
producer assertions, not independently signed execution evidence. [pcc:unverified]

The first cryptanalysis compile exposed absent optional diagnostic fields in
its Meter. The common runner now uses only shared diagnostic fields. Initial
offline dependency resolution proposed unrelated cc/find-msvc downgrades; Cargo
precise updates restored their original versions. Earlier failed build logs
remain in the report. [pcc:unverified]

`selected_source_pins` is deliberately partial provenance. The semantic digest
binds request/policy/answers; replay recomputes those answers on matching selected
sources. It does not authenticate the producer, cover all build dependencies or
compare diagnostic counters. Receipts retain producer_assertion standing.

The present shared message is local membership. The remaining technical task is
a compact exact representation of the full constrained summation boundary with
construction and live-storage accounting. This dependency is recorded in
KN-OPEN-08ba7e. No finding or scientific-state promotion is made by this integration.

## Sources and reproducibility

The integration reuses the [landed C1–C3 implementation](../prime_ic_zero_filter_20261030/implementation/README.md)
and the existing counter, strict JSON record visitor, full point-addition charts
and fixed-budget error propagation. The primary context reviewed earlier in this
thread is [PKM2016](https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf),
[exact prefix counting](https://hexagonmath.org/2610.00600v1),
[weighted counting](https://hexagonmath.org/2610.00594v1),
[locality qualifications](https://hexagonmath.org/2610.00613v1), and
[Kosters–Yeo](https://arxiv.org/html/1503.08001v3). This integration changes the
consumer path and does not import additional complexity theorems.

```sh
cargo test --release --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --lib --test structured_cli
cargo clippy --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --all-targets -- -D warnings
cargo +1.78.0 check --locked --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --all-targets
cargo fmt --manifest-path crates/hashed-ecdl-tensor/Cargo.toml --check
```

Native algorithms, correctness controls and semantic replay remain in Rust.
Document rendering and metadata/index tools are separate. The existing native
workflow includes the new command through all-target checks; the cryptanalysis
workflow also covers the suite adapter. Conductor is absent in this workspace;
no reservation is asserted. Canonical performance/frontier graphs carry no new
qualified measurements from this software-integration panel. [pcc:unverified]
