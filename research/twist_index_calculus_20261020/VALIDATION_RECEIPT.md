# Quartic twist index-calculus validation receipt

22 October 2026. All inputs are public synthetic or known-answer points. The [report](QUARTIC_TWIST24_REPORT.md) states the measured scope and limits; the [frozen protocol](QUARTIC_TWIST_LINE_PROTOCOL.md) defines the runs.

| Check | Command or artifact | Result |
| --- | --- | --- |
| Full Rust library suite, serial | `TMPDIR=/home/ec2-user/.cache/gls-report-tmp cargo test --offline --release --lib -- --test-threads=1` | 4,475 passed, 0 failed, 119 ignored; 688.34 s. |
| Touched Rust examples | `cargo test --offline --release --example quartic_twist24 --example quartic_twist_mixed_lines --example quartic_twist_native_audit --example quartic_twist_mixed_summary --example quartic_twist24_summary --example quartic_twist_s5_pilot --example quartic_twist_2d_census --example quartic_twist_2d_audit --example quartic_twist_2d_p13 --example quartic_twist_2d_p13_audit`, then `cargo test --offline --release --example quartic_twist_pair_quotient_p13` | All compiled and ran their zero-test harnesses; the recorded experiment and audit executables ran separately. |
| Rust formatting | `cargo fmt --all -- --check` | Pass. |
| Lean proof audit | `ELAN_TOOLCHAIN=leanprover/lean4:v4.34.1 tools/lean/proof-gate.sh --out /home/ec2-user/.cache/quartic-lean-gate-20261022 research/twist_index_calculus_20261020/QuarticTwistSymmetry.lean` | [Receipt](proof_gate_quartic_20261022/audit.log): 8 theorems, 0 failures, 0 trusted axioms. The finite-field coordinate proof remains outside the formal module. |
| Repository validation gate | `python3 scripts/validation_gate.py check --base origin/main` | Pass; no IC leaderboard or `benchmarks/README.md` row was added. |
| Same-base native replay | [Audit](quartic_twist24_native_audit_v2.json) | 21 runs, 1,821 selected group relations, 1,008 matched rho solves, all recovered scalars checked; status `pass`. |
| Mixed-line native replay | [Audit](quartic_twist_mixed_native_audit_v1.json) | 42 runs, 69,045 query hit counts, 5,298 selected group relations and all row/rank/answer checks; status `pass`. |
| Two-dimensional native replay | [Audit](quartic_twist_2d_native_audit_v1.ndjson) | Independent norm test on 265,008 candidate abscissae and signed point-orbit check on every orbit; all eight records `pass`. |
| p=13 rank pilot replay | [Audit](quartic_twist_2d_p13_native_audit_v1.json) | All 7 query counts and 4,041 decompositions independently recomputed; 7 rows, rank 7, recovered 271, all group logs enumerated. |
| Pair-table quotient | [Record](quartic_twist_pair_quotient_p13_v1.json) | Full signed decomposition **sets** equal on all seven targets; all selected relations present. |
| A/A non-timing controls | [same-base](quartic_twist24_v2_aa.diff), [mixed-line](quartic_twist_mixed_v1_aa.diff), [p=13 pilot](quartic_twist_2d_p13_aa.diff), [pair quotient](quartic_twist_pair_quotient_p13_aa.diff) | All four diffs empty. |
| Text patch hygiene | `git diff --cached --check -- . ':(exclude)**/*.pdf' ':(exclude)*.pdf'` | Pass. PDF artifacts are inspected separately; Git treats some PDF byte streams as text. |

An earlier parallel `cargo test --offline --release --lib` attempt passed 4,474 tests and failed one unrelated filesystem cleanup test, `cryptanalysis::ecc2k130_merge::tests::torn_and_shrunk_inputs_fail_closed`, at `remove_dir_all` with `DirectoryNotEmpty`; 119 tests were ignored. The exact test passed on a serialized rerun, and the complete serial suite above passed without a code change to that module. This is retained as a failed-attempt observation, not reported as a cryptanalysis failure.

The registered `p=37,73,97` S5 pilot completed all six success/failure queries with exact group-oracle agreement; [p=37](quartic_twist_s5_p37.json), [p=73](quartic_twist_s5_p73.json), [p=97](quartic_twist_s5_p97.json). Its shared-host L0 times and field-operation counts are diagnostic. The repo validation gate does not admit a whole-method speed row because the main panel lacks IC-stream group-operation and hash/allocation accounting, a 0/0.1/1 GAE band and an isolated matched rho timing comparison. The S5 pilot also lacks a full relation stream and same-input msolve/Magma comparison.
