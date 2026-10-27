# N83 external XOR-SAT pilot: a satisfiable exported instance remains capped

The [frozen protocol](PROTOCOL.md) exported the exact Gray K600 4+2 `all_x` planted-witness encoding as an 11,641,543-byte XOR-DIMACS file with 118,684 variables, 548,445 CNF clauses and 6,059 XOR rows. Its SHA-256 is `c62fe682879435e8f1ee30a80ac7830f2aef81dd4f600d10a004ac45a4f914b8`; all three clean CryptoMiniSat calls used those identical bytes. The pinned local solver is CryptoMiniSat 5.14.7, binary SHA-256 `a3f85c3709b5e2a040bf82a4a604d1c7b9f10219bbf180a9e0f72319a2e892ac`, one thread. These are local L0 measurements without the hosted 4-GiB container cap.

| External conflict limit | Solver disposition | Solver wall, L0 | Peak RSS, L0 |
| ---: | --- | ---: | ---: |
| 5,000 | `INDETERMINATE` | 0.233 s | 53.4 MB |
| 100,000 | `INDETERMINATE` | 5.949 s | 100.8 MB |
| 1,000,000 | `INDETERMINATE` | 57.661 s | 143.5 MB |

The independent [audit](audit_external_calls.py) passed and saved [its sanitized receipt](audit.json). It checks each committed-source result, exact XOR-DIMACS and export hashes, raw solver status and output digests, limits, per-process memory and wall receipts, and a tree hash of every raw local file. The first two calls encountered runner bugs in interpreting CryptoMiniSat's `s INDETERMINATE` and exit code 15; their raw artifacts and producer-failure receipts remain in the audit inventory. The corrected calls use source commit `73ee5686a62c9e75cd8d29acd5e73fd3410bb494` and report `UNKNOWN_solver_limit`, not UNSAT or zero yield.

The stronger satisfiability control passed. The native worker found the known x tuple at 2,049 conflicts and 156.231 local wall seconds, then exported its **complete 118,684-variable assignment**. The separate [native assignment checker](audit_native_export.py) verified that assignment against all 548,445 clauses and 6,059 XOR rows of the *original pre-solve external-solver input*. Its eleven x blocks match the independently rebuilt six-full-point witness, and all five S3 nodes vanish over the original F₂⁸³. The native model bit file has SHA-256 `ad95e05e171740aa08efb39bf2b28fd9c7c501f3f5ee4e7c5cced3c9a1c725d9`. Re-serializing the native solver after search produced a different byte hash, but the full assignment check proves that the pre-solve export is satisfiable; the native SAT solver mutates watched-literal order during search.

As a corruption check, flipping the first model bit made the independent checker reject the exported formula at DIMACS line 4. The unmodified model and all three clean external-call artifacts replayed successfully.

The conservative charge for the five external attempts, including two producer failures, plus the native model call is six full 600-second envelopes, **3,600 seconds**. The full release-library validation took another 265.59 seconds and passed 4,084 tests with 117 ignored; the touched example release test and 63 N83 Python tests passed. The raw formula, solver logs and model bits remain in local research scratch, bound by tree and file hashes in `audit.json`; the three factor-base objects are already in the requested S3 prefix. This pilot does not show external-solver superiority, natural-target relation yield, useful rank, linear algebra, individual-log extraction, or fully charged index-calculus runtime. `selected_best_total_runtime` remains null.

The subsequent [complete-model CryptoMiniSat control](MODEL_PIN_RESULT.md) added
every native assignment bit as a unit clause to the byte-identical original
export and returned SAT; flipping only the first added unit returned UNSAT.
Its independent audit passed. The control incurred another conservative 600
seconds across one infrastructure-failed sandbox pair and one valid local pair.

A [four-arm configuration screen](CONFIG_SCREEN_RESULT.md) crossed default or
VSIDS branching with default or widened Gaussian limits on the same known-answer
export. Both default-limit arms remained indeterminate; both widened-limit arms
hit the outer timeout. Its independent audit passed, and another conservative
600 seconds is charged. No arm advances to a hosted natural-search comparison.
