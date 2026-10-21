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
