# bitop/v1 on a 16–40-bit prime ladder: index calculus with a per-size base

**Report:** [`REPORT.pdf`](REPORT.pdf) (source [`REPORT.typ`](REPORT.typ)).
**Protocol:** [`PROTOCOL.md`](PROTOCOL.md), frozen 2026-10-19 before any data,
with Deviations 1–4.
**Run reports:** [`reports/run-20261019T1942Z-train/`](reports/run-20261019T1942Z-train/)
(Phase A) and [`reports/run-20261019T2202Z-holdout/`](reports/run-20261019T2202Z-holdout/)
(Phase B).
**Unit:** `bitop/v1` ([`docs/units/BITOPS.md`](../../docs/units/BITOPS.md)), from
PR #1970. It is a reproducible accounting convention with frozen model
weights, not a physical gate count or synthesized-circuit cost. “Completely
priced” means `ecbench` records no declared unpriced work; it does not prove
every operation was instrumented. See the interpretation amendment in the
protocol.

Steps 1 and 2 of the bitop/v1 follow-ups:
1. measure the exponents more precisely, on more runs and bigger curves;
2. re-fit index calculus with its factor base chosen per curve size.

**Curves:** 25 `a = −3` prime-order curves, one per size from 16 to 40 bits.
**Phase A (train panel):** chooses the index-calculus base per size, in
bit operations.
**Phase B (hold-out):** 80 fresh targets a curve.
- Every one of the 12,080 hold-out runs verified.
- `ecbench` reports no declared unpriced work in each run; this status does not prove complete instrumentation.
- Every session passes `ecbench verify --replay 2`.

## Results (hold-out, `ecbench bound fit --unit bitop/v1`)

Exponent `α` in additions (cost ∝ `r^α`), with its 95% cluster-bootstrap interval:

| arm | 16–26 bits | 16–32 bits (toy tier) | 33–40 bits (medium tier) |
|:--|:--|:--|:--|
| rho, negation (reference) | 0.413 [0.388, 0.447] | 0.440 [0.418, 0.462] | 0.504 [0.449, 0.575] |
| BSGS, negation | 0.492 [0.479, 0.507] | 0.495 [0.489, 0.501] | 0.497 [0.475, 0.520] |
| kangaroo | 0.526 [0.462, 0.578] | 0.485 [0.463, 0.505] | 0.491 [0.401, 0.576] |
| IC pair table, **per-size base** | 0.558 [0.530, 0.588] | 0.595 [0.576, 0.617] | 0.646 [0.629, 0.659] |
| IC pair table, fixed base 32 | 0.792 [0.696, 0.888] | — (16–28: 0.838 [0.764, 0.921]) | — |
| IC root-finding m = 3, per-size base | — (16–28: 0.644 [0.629, 0.663]) | — | — |

**Precision (step 1).**
- **Interval widths:** 0.012–0.043 at 16–32 bits.
- **Comparison with PR #1970:** its ladder had widths of 0.07–0.31 from 88 runs per arm on 16–26 bits.
- **Target:** ±0.02 is roughly met. Half-widths are ±0.006 for BSGS, ±0.017 for root-finding, ±0.021 for the per-size pair table and ±0.022 for rho.
- **Exception:** kangaroo is wider (±0.021 to ±0.09).

**Positive control.** The protocol's rule says the instrument is suspect if
any generic arm's interval excludes ½ by more than 0.05.
- **It trips once:** rho at 16–26 bits, whose upper bound 0.447 is 0.053 below ½.
- **Every other range passes:** rho at 16–32 bits misses ½ by 0.038, within the tolerance, and rho at 33–40 bits contains ½.
- **BSGS:** contains ½ in every range, at ±0.006 over 16–32 bits.
- **Pattern:** rho's ratio to the floor falls with size: 1.47 at 16–26 bits, 1.33 at 16–32 and 1.05 at 33–40. That is consistent with fixed per-walk overhead that small curves do not amortise. This explanation was not tested.

**The fair test (step 2).**
- **Verdict:** the per-size base *bends toward ½*, by the protocol's preregistered rule. Its 16–26 upper bound, 0.588, is below the bitop/v1 study's fixed-base 0.766.
- **Same-study comparison:** its interval also lies entirely below the fixed base's from this same study (16–28: 0.573 [0.549, 0.600] against 0.838 [0.764, 0.921]).
- **Success criterion:** this meets the protocol's success criterion, a verified, measured improvement over the fixed-base index-calculus baseline under the declared bitop/v1 convention and recorded counters.
- **Cost ratios:** the fixed base costs 1.0× (16 bits), 3.0× (24) and 8.4× (28) what the per-size base costs.
- **Band:** the order holds across the AGENTS.md §5a band (weights 0, 0.1 and 1) at every size from 18 to 28 bits. At 17 bits the fixed base is cheaper, and at 16 bits the band does not separate them.

**But the exponent rises with size,** from 0.558 (16–26) to 0.595 (16–32) to
0.646 (33–40). Those intervals separate, so index calculus moves away from
rho as the curves grow.

**Distance remaining to rho** (mean bitop/v1 over rho's, same targets):

| bits | 16 | 20 | 24 | 28 | 32 | 36 | 40 |
|:--|--:|--:|--:|--:|--:|--:|--:|
| IC, per-size base | 6.2 | 9.4 | 14.1 | 23.0 | 32.1 | 58.7 | 78.3 |
| IC, fixed base 32 | 6.5 | 11.7 | 42.7 | 192.7 | — | — | — |
| IC root-finding, per-size base | 10.1 | 21.0 | 36.5 | 69.7 | — | — | — |

- **Rho is cheaper at every size and every band weight.** No curve has an amortization window against batch rho (`ecbench amortize`, 0 of 25).
- **A/A control:** it is degenerate. `rho-neg-aa` reuses `rho-neg`'s seeds, so its runs are identical copies (Deviation 4). It is reported as uninformative.

**Selected bases** ([`params.json`](params.json)):
- **Pair table:** from 26 abscissae at 16 bits to 4,390 at 40.
- **Root-finding:** from 32 to 512 at 16–28 bits.

Near the optimum the train cost curves are flat. The bases are within about a
factor of 1.5 of the 2026-10-15 campaign's GAE-selected ones, larger at some
sizes and smaller at others.

## Scope and limits

- **Curves and claims:** public synthetic curves and hash-to-subgroup targets; no security claim.
- **What is measured:** captured bitop/v1 model counts at isolation L0. Wall time is not used; counter completeness and physical implementation cost are not established.
- **Curve sample:** one curve per size, so curve-to-curve variation within a size is not sampled.
- **Fits are per tier:** ecbench refuses a fit across its toy and medium tiers (Deviation 3).
- **Selection crash:** one train run crashed. It was `pair-N617` at 39 bits, killed with signal 9 at 6.8 GB, most likely by the kernel's out-of-memory killer. That candidate was ineligible by the rule, and it is far from the selected 3,492.

## Files

| path | what |
|:--|:--|
| `curves.jsonl` | the 25 curves (`examples/ic_frontier_prime_curves.rs`) |
| `specs/` | frozen train, extension and hold-out specs (`bitop_ladder_analyse specs`; first written by `make_specs.py`) |
| `sessions/` | 46 train and 25 hold-out sessions, each with `verify.json` |
| `select/`, `params.json` | Phase A selection (`bitop_ladder_analyse select`; first written by `select.py`) and the frozen bases |
| `cards/` | task cards, receipts and director assessments (AGENTS.md "Model tiers") |
| `analysis/` | `analyse.sh` outputs: bounds, `compare`, `table`, `band`, `amortize`, `work` |
| `summary.json` | frozen summary the leaderboard reads |
| `analysis/gap_to_rho.json`, `summary.json` | `bitop_ladder_analyse gap` and `summary` |
| `figures/` | SVG/PNG figures and `plots.py`, which reads recorded values only |
| `logs/` | run and verify logs |

**Provenance.**
- **Binary:** every session ran the same binary, sha256 `372ed2eb2f19c5fd…`.
- **Recorded commits:** these move between sessions because documentation commits landed while cards ran. Code was unchanged throughout.
- **Dirty trees:** twelve extension sessions record a dirty tree. The cause was uncommitted card and assessment files.

## Provenance of the derived files

The study's own computations first ran in Python: `select.py`,
`make_specs.py`, and two inline scripts for `analysis/gap_to_rho.json` and
`summary.json`. AGENTS.md ("Implementation language: no Python") does not allow
that. [`examples/bitop_ladder_analyse.rs`](../../examples/bitop_ladder_analyse.rs)
replaces all four:

```
cargo build --release --bin ecbench --example bitop_ladder_analyse
target/release/examples/bitop_ladder_analyse select | gap | summary
target/release/examples/bitop_ladder_analyse --specs-out DIR specs train | holdout | ext …
```

Run again on 2026-10-20 over the committed sessions, it reproduces these files
**byte for byte**:
- `params.json`, `select/SELECTION.md` and `select/selection.json`;
- `analysis/gap_to_rho.json`;
- all 71 files in `specs/`.

`summary.json` also comes out identical except one value: its recorded
SHA-256 of `PROTOCOL.md`. The committed value is the hash of the protocol as
it stood when the summary was built (b1baf7887), before the owner's
interpretation amendment (608130dec). The committed file is kept as it is.
The Python files stay in the study as provenance, labelled legacy. The figure
script `figures/plots.py` only plots recorded values.

