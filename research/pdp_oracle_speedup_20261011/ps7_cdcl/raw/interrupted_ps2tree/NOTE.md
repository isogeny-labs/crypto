# Interrupted first run (kept, not used for any number in the README)

Records of the first attempt, 2026-10-18, from a binary built on the tree of `exp/pdp-ps2-scalar` (the library of that
branch, `binary_hash.txt`), before the pull request was rebased onto `main`.  The container was restarted several times
(usage-limit stops); the stages `graph`, `fidelity`, `countcheck` and `counts` for `l = 8, 9` finished, `counts` for `l = 10`
stopped after four of six arms (`a1, a1u, a2, a2p`), `verify`, `reg`, `timing` never started.  Superseded by the complete
run in `../` made with the binary built on the tree of the pull request (`origin/main` plus these files).  Kept because
failed and interrupted runs are evidence, and because the deterministic counts of the two runs can be compared
(README section "Reproduction on main").  `count_check.jsonl` here is the only instrumented-counter record (see the README:
the instrumentation module is absent from `main`'s `semaev_decomp.rs`).
