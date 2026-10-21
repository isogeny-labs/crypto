# Review data dictionary

The frozen experiment and pricing choices are in [PROTOCOL.md](PROTOCOL.md).
The [manifest](results/manifest.json) points to the complete matched run.
All rows are machine-readable; values in the PDF and graph come from these
records through the native analyzer. No threshold-based result classification
is encoded in the files.

| File | Unit of observation | Main fields |
| --- | --- | --- |
| `results/matched/fb*-bits*.jsonl` | one cold candidate solve and one same-process source-rho solve for a public target | `workload_id`, `workload_sha256`, `algorithm_seed`, `planted`, `recovered`, `verified`, `counts`, `phases`, `native`, `source_add`, nested `rho` |
| `results/matched/targets.jsonl` | one joined arm/target comparison | `arm`, `bits`, `curve_p`, `target_index`, candidate/external/rho GAE, both ratios, processed divisors, rejected split relations, raw record path |
| `results/matched/price-grid.csv` | one arm, size slice, and pricing cell | `S_per_M`, `I_per_M`, `native_GAE_per_logical`, candidate/external/rho GAE and ratios, target and verification counts |
| `results/matched/summary.json` | run metadata and per-arm/per-size totals | number of raw and replayed rows, in-process rho checks, arm totals, ratio bands and pricing-cell counts |
| `results/kummer-ambiguity.jsonl` | one exact-order subgroup witness | unsigned input equality, distinct signed outputs, source-order and Jacobian checks |
| `results/pencil-pilot/f*-cohort.jsonl` | one curve setup row plus 60 cold-priced candidate/rho target pairs | source workload id/hash, recovered scalar and verification flags, phase counters, sparse elimination volume, native arithmetic and same-process rho |
| `results/pencil-pilot/price-grid.csv` | one completed pilot curve and pricing cell | full setup-charged candidate/rho GAE and ratio; 36 cells per curve |
| `results/pencil-pilot/replay/f*-cohort.jsonl` | independent archive-input rerun of a completed pilot curve | every non-wall field is compared with its primary row by the native validator |
| `results/complete-law-branches.jsonl` | one normalized model, 648 ordered 1+1 cases, 576 ordered 1+2 cases and 256 ordered 2+2 cases | `inverse`, `doubling`, `distinct` count matched 1+1 cases; four named 1+2 counters sum to 576; `generic_2plus2_checked` and `doubling_2plus2_checked` sum to 256; unexercised exceptional 2+2 cases require a separate sample |

The `counts` objects record field multiplications `m`, squarings `s`,
inversions `i`, field additions, source-curve additions, and square-root
calls. `phases` partitions those charges; `native` records logical work
whose price is swept from 0 to 1 GAE per declared operation. The `source_add`
object is the observed source-curve addition unit on that curve. Each pricing
cell first prices field operations using its `S_per_M` and `I_per_M`, then
divides by the source-addition price to give GAE. Ratios use the complete
one-target candidate price over either the wrapped external collector or
same-process negation rho.

The 35 primary matched JSONL files are compared byte-for-byte with a fresh
archive-input rerun by [replay_matched.sh](replay_matched.sh). The independent
native [validator](src/bin/validate_matched.rs) recomputes 15 headline values
from raw rows. [SHA256SUMS](SHA256SUMS) pins the study files. Wall-time logs
are retained as L0 diagnostics and do not enter the reported ratios.

For the pencil pilot, the [native analyzer](src/bin/analyze_pencil.rs) charges
the complete setup to each target and subtracts multiplications internal to
Fermat inversion when applying the separate `I` price. Scalar modular
arithmetic and one sparse-map update per `solver_row_updates` are priced as
native logical work. The [independent validator](src/bin/validate_pencil.rs)
recomputes prices, scalars and a replay comparison with wall fields removed.
The [law gate](src/bin/complete_law_gate.rs) implements only the named
projective formulas. Exceptional 2+2 branches are still a coverage gap, not an
arithmetic failure or a full-law cost observation. Both branch counts and
their per-model coverage appear in [LAW_GATE.pdf](LAW_GATE.pdf),
[complete-law-gate.svg](complete-law-gate.svg) and
[complete-law-2plus2.svg](complete-law-2plus2.svg).
