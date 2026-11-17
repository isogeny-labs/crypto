# Full-D18 external XOR solver gate

The [frozen protocol](PROTOCOL.md) ran from clean source commit
`79c3c8f20a2bbea9f42e3b22955cecc8b4a70afb` and one release exporter
(`SHA-256 27afbe7cafa17ac04ab44768b414461d7f5e3c2de0d2a5ff28132394b049b2f8`).
CryptoMiniSat 5.14.7 was pinned to
`a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac`.
The primary `a=0` GF(2^83) full-D18 base has 32,643 signed-Frobenius
columns and 5,418,738 point rows. The compressed object is stored at
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/d18-full-direct-subgroup-v1/a0/objects/c295da8898cfad29a053621c768ca059eea4263f89f5ca78b4f98af43c383f3f.jsonl.gz`,
with SHA-256 `eaef94ef45b3d2d4cd2ec2f40c1173b4cdb2a437b7e618a858c73fb1caf3902b`.
A fresh local S3 GET of that object and its hash-bound wrapper passed full-object
provider SHA-256, length, encryption metadata, producer-manifest binding and
the exporter's complete signed-orbit import.

| Stage | Result | CNF clauses | XOR rows | XCNF bytes | Supervised wall | Maximum RSS |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| Constructed export | `PASS_XCNF_EXPORT_ONLY` | 2,807,752 | 3,901 | 50,518,392 | 98.983 s | 3,949,494,272 B |
| Public-natural export | `PASS_XCNF_EXPORT_ONLY` | 2,807,254 | 3,901 | 50,515,109 | 128.322 s | 2,879,537,152 B |
| Constructed CryptoMiniSat | `UNKNOWN_EXTERNAL_SOLVER_CAP` | same positive formula | 3,901 | input above | 1,210.016 s | unavailable |
| Public-natural CryptoMiniSat | gated off | same natural formula | 3,901 | input above | unrun | unrun |

Both formulas had 329,726 variables. The constructed XCNF SHA-256 is
`0d7bb7ef45ae19937220945129f61baccdd72b85c58bb7f16dcb56ca3ec24d7c`;
the public-natural XCNF SHA-256 is
`b0f91be5e1580c36f60e3a5bd5558193c7fc14d773f938656cae66fbcdcea4f3`.
The positive formula pins the public four-point constructed witness's x blocks;
the natural formula has no witness pins. Both use the same phase-first,
free-choice shared-root circuit. The exports performed no SAT search. The
uncompressed XCNF files are retained in the local scratch run; the source,
exact hashes and S3 base allow regeneration from the PR.

The constructed solver used one thread with `--maxtime 1200`,
`--maxconfl 100000000`, `--verb 1 --verbstat 1 --printsol 1`. It continued
searching until the independent 1,210-second process-group watchdog fired.
The supervisor recorded exit `-9`; stdout contains no terminal `s` or model
`v` line. The [independent audit](final-audit.json) classifies the arm as
censored. The watchdog also killed `/usr/bin/time` before it could emit a
maximum-RSS record, so that value is unavailable; the export arms have valid
maximum-RSS records. This is a protocol measurement limitation, and the
shared Mac had no hard CPU or memory isolation. The [negative controls](negative-controls.json)
reject an altered target, an altered XCNF digest, a missing terminal status
without a cap, and an attempt to start the natural arm without a passing
constructed audit.

The natural XCNF was exported and hashed, but its solver arm was intentionally
not run after the constructed gate failed. The retained [formula summary](formula-summary.json),
raw export receipts, compressed solver stdout and [validation receipt](validation.json)
bind the source, binaries, S3 input, exact commands, wall times and negative
controls. The required release library suite passed 4,165 tests with 117
ignored; the touched example passed 19 tests; the N83 and boundary Python
suites passed 63 and 52. Formatting and Python compilation passed.

The earlier native 1,200-second public-natural run was also censored. This
external arm gives no natural relation rate or rank estimate and is not a
throughput comparison on an isolated host. Selection still requires multiple
independent natural targets, useful rank, factor-base logs, unseen-target
descent, full cold one-target accounting and matched same-target rho.
`selected_best_total_runtime` remains `null`.
