# Resume: crypto @ claude/zealous-meitner-85ipb5

## Goal
Owner's ask (2026-11-10): run the index-calculus decomposition benchmarks on a ladder of binary
curves (Koblitz K and random R families, Weil descent of the Semaev polynomial) with the new
external M4GB backend (`m4gb-f2`, PR #3812) beside the in-house engines, and see whether solve time
per target is lower. Stage diagnostic (AGENTS.md section 2), never a speed row; all local numbers
`[pcc:unverified]`.

## Lead / hypothesis
Prediction: `inherited-f4` and `f4-f2` decide each target faster than `m4gb-f2` on every cell
(decision tree with degree-bounded bases against a full reduced basis); M4GB's ratio to the
reference grows with the number of variables. Falsifier: `m4gb-f2` median wall below `f4-f2` on any
cell with at least 24 variables.

## Spec
research/m4gb_descent_ladder_20261110/README.md (protocol), run_ladder.sh (the one launcher).
Command: target/release/ic descent --json --out <file> --families K,R --cells <n:n':2 ...>
--targets 8 --repeats 3 --budget-seconds 300 --solver fes-f2 --solver f4-f2 --solver inherited-f4
--solver m4gb-f2, with M4GB_BINARY pointing at a directory of solver_m4gb_n<N>_gf2 binaries
(patched upstream 01860a52, patches in third_party/m4gb-patches/, built by scripts/m4gb/build.sh).
Benchmark record: benchmarks/records/20261110T1513Z-m4gb-descent-ladder.json (open).

## State
Branch restarted on origin/main after PR #3812 merged. M4GB binaries in the session scratchpad
(not in Git; rebuild with scripts/m4gb/build.sh n16_gf2 ... n64_gf2). `ic` release build in
progress. Conductor, ecdb, research-kb and isolab tooling unreachable in this container.

## Next steps
1. Smoke test one cell (11:6:2, K, 2 targets); then run_ladder.sh under `benchrec.py run`.
2. Summarise per cell: median wall per engine, ratio to fes-f2 and to f4-f2, agreement with the
   reference; graph + PDF; close the record; benchmarks entry (no table row: stage diagnostic).
3. Session log entry; PR from this branch; auto-merge once complete.
