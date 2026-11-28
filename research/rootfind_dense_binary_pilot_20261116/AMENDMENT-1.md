# Amendment 1: binary-location source stamp, before second epoch

Filed after the first local epoch and before any second epoch. The fixed
`SPEC-K13-LOCAL.json`, its targets, arms, order, metrics and stopping rule do
not change. The first session `ECBS1h3cf526729f36` and its 200 records remain
unaltered.

The first binary was compiled from this worktree's Rust sources into another
worktree's shared `target/release/ecbench` path. `ecbench` obtained
`git_commit` at runtime from the binary's parent worktree, so the session
records `557f28efa880a07589dd806d42693697c0993411` rather than this
protocol's run commit `984cf46dc62046dcbc7131dea1cc63628368c812`.
`git diff` between those commits shows no changes under `src`, `crates`,
`isogeny_algos`, `Cargo.toml`, `Cargo.lock`, or `tools/factor-base`, and the
first binary has SHA-256
`c4b63be34e52621d5d43e9cfcb2107f596b1148c7d06715fb7560dcae32e5d80`.
The first epoch replayed all 160 measured deterministic runs exactly, but its
commit field does not satisfy the protocol's source-custody requirement.
Classify its benchmark record as **invalid for performance admission**;
retain its raw data and descriptive graph.

For a custody-only second epoch, copy the identical binary bytes into this
worktree's ignored `target/release/ecbench` and use that path, so the runtime
worktree capture identifies this branch. Run the unchanged spec once into a
new directory and a new `bench.record/v1` record. This is a repeat on seen
targets and cannot serve as an independent holdout or repair the L0 isolation
limit. Do not pool both epochs or treat the repeat as a new confirmation.
Preserve all incomplete and failed results if this correction fails.
