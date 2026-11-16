# Deviations from PROTOCOL.md / SPEC.md

No threshold, arm, seed rule, grid, decision rule or formula was changed.

## D1. Test T13 (Wiedemann planted system), construction detail
SPEC §9.1 T13 fixes `Rng::new(7)` for the system but not the draw order. With my first
draw order (3 uniformly random columns per row), column 0 was absent from every one of the
60 rows (about a 1% event), so `x[0]` was undetermined and the test failed (first
`cargo test` run: 11 passed, 1 failed, `t13_wiedemann`, left[0] = 0, right[0] = 937006; the
output of that first run was overwritten by the second `raw/test.log`). Minimal fix to the
test only: the first column of row `i` is `i mod 40` (so all 40 columns are covered), the
other two are random. The solver code is unchanged. Second run: 12 passed.

## D2. Exhausted-run counter split
SPEC M1 defines `s2`/`s3` at certification only. For a run that ends exhausted (no
certificate) the split is `cm_loop = s_end - s1`, `cm_verify = 0`. No run exhausted, so
this branch has no effect on the data (gate Q4 reports it).

## D3. Test helper
`mulmod` is the u128 form of SPEC §4 (as specified). `roots_of` for T9 is called with
`Rng::new(1)` (SPEC leaves the RNG unspecified).

## D4. Logging
`raw/COMMANDS.log` lists the commands run through a wrapper script
(`/tmp/claude-0/runall.sh`, the loops of PROTOCOL §9 verbatim, with logging added); the
`sha256sum raw/*.jsonl > raw/data.sha256` and analysis commands were run after it.
Commands run before the wrapper (build, test, selftests) are noted at the head of
COMMANDS.log in prose.

## D5. Repository hooks
No hook refused any command. `raw/REFUSALS.txt` is not created.
