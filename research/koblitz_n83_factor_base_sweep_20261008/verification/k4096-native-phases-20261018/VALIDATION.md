# Prelaunch validation

The opt-in phase stream was exercised by
`chained_s3_frobenius_strategy_feeds_rank_and_target_solve` with
`N83_CHAIN_PHASE_TRACE=1`. The known-answer small-field run passed and emitted
three decomposition calls, three SAT entries and three accepted group lifts.
The test independently recomputes each emitted `[a]G+[b]Q` query. See
`development/positive-control.log` and `development/positive-audit.json`.

The audit accepted a separately generated valid N83 fixture-0 query and
rejected the same event with one changed x-coordinate. It also rejected a
positive-control stream in which the first `solver_enter` marker was replaced
while a later `solver_sat` marker remained. These are parser/group-replay
controls, not native factor-base search outcomes.

Before the cloud launch, the exact staged K4096 archive and all Git-tracked
panel and fixture sidecars matched the SHA-256 values in the frozen
`cloud-matched-k2048-k4096-20261018/input/fresh-inputs.json` receipt. Its S3
HEAD receipt is `development/staged-input-head.json`.

Required source validation passed: `cargo test --release --lib --locked`
(4,391 passed, 118 ignored), `cargo test --release --example
koblitz_n83_factor_base_export --locked` (19 passed), the study Python suite
(63 passed), and the boundary Python suite (52 passed). The commands' full
outputs are in `development/`. `rustfmt --check`, `git diff --check`,
`shellcheck`, and `bash -n` also passed.
