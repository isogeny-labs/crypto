# Theta setup run, 2026-10-22 05:37 UTC

Protocol: `../../PROTOCOL.md`; input digest:
`280b06474489bd78cba9a1fedd548aac9ac529a843f60156060d692851432680`.
Run from repository root after `cargo build --release --locked --offline --bins`
in the study directory:

```
research/kummer_theta_20261022/target/release/kummer-theta-gate
research/kummer_theta_20261022/target/release/validate_theta
```

Validator stdout:

```
validated_models=21 validated_theta_tuples=1680 lookup_replay=pass selected_original_equations=pass
```

The producer replay was byte-identical to the first saved raw JSONL. The
aggregate jq check found 21 rows, 60 eligible orders and 10 square pairs
per model, and 1,680 qualifying tuples. `cargo fmt --check` and release
`cargo check --bins` passed. `THETA_GATE.pdf` has two parseable pages;
the PNG graph was inspected. No DLP walk or cost measurement ran.
