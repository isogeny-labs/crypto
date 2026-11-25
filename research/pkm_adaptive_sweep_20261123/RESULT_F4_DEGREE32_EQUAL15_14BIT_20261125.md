# 14-bit equal-15 degree-32 panel — result — 2026-11-25

Panel run under `PROTOCOL_F4_DEGREE32_EQUAL15_14BIT_20261125.md` with binary
`c98e290c7c7d5aee30ed70e1c9aa19726e830854b09ae15921f02b09c648de2d` (commit
`a5503f4b`, `git_commit` field null). One 14-bit roster curve,
`icv1-fp14-tm43-158b6914`, 16 scalars, order interval, candidate, candidate,
interval, 15 abscissae in both arms. All four invocations exited zero, all 16
recovered scalars agreed across the four runs, `budget_exceeded` 0 everywhere.

## Result

Ratios are candidate / interval.

| Metric | Pair 1 | Pair 2 | Combined |
|:--|---:|---:|---:|
| Charged GAE | **0.2263** (6.434B / 1.456B) | **0.3107** (4.946B / 1.537B) | **0.2630** |
| External wall | 0.2600 (334.0 / 86.8 s) | 0.2755 (286.0 / 78.8 s) | — |
| Child CPU | 0.2317 (778.2 / 180.3 s) | 0.2305 (680.2 / 156.8 s) | — |
| Relations | 181 / 182 | 181 / 182 | 362 / 364 |
| Target trials | 7,018 / 6,633 | 7,018 / 6,633 | 14,036 / 13,266 |
| Median `s_over_rho` | 1,271,321 / 334,748 | — | — |

Relations per trial: 0.0258 interval, 0.0274 candidate (1.06×). Useful
relations per charged GAE: **3.80×** better for the candidate
(0.0274/0.0258 ÷ 0.2630).

The component gate passes on the single available 14-bit curve. This is
transfer of the equal-size relation-extraction component lead from 13 to 14
bits, on one curve and one selected auxiliary seed.

## The per-system factor is invariant in the field size

| Quantity | 13 bits (3 instances) | 14 bits | Ratio at 14 bits |
|:--|---:|---:|---:|
| Multiply-adds per F4 call, interval | 127.3–127.4M | 127.29M | — |
| Multiply-adds per F4 call, candidate | 3.97–3.98M | 3.96M | — |
| **Per-system factor** | **32.0–32.1×** | **32.15×** | invariant |
| Elimination steps per call, int/cand | 11.5–11.7 / 12.3–12.5 | 11.15 / 12.11 | — |
| Solving degree (max), int/cand | 6 / 5 | 6 / 5 | — |
| Max matrix (rows × cols), int | 1498 × 929 | 1497 × 925 | — |
| Max matrix, cand | 821 × 583 | 821 × 583 | — |
| Non-zeros per row, int / cand | 188.3 / 54.7 | 200.3 / 38.8 | — |
| UNSAT share of calls, int / cand | 0.854–0.910 / 0.871–0.930 | 0.971 / 0.972 | — |

The matrix dimensions are essentially unchanged when the field doubles and the
base shrinks from 18 to 15 abscissae: they are set by the arity/degree
structure of the system, not by `p` or by base size. That is why the 32× is
stable, and it is the strongest available evidence that the saving is
structural rather than a small-field accident.

## Charged GAE is price-dominated, and noisy

`total_gae` again equals the solver GAE exactly. The two baseline invocations
have **identical** multiply-add counts (6940 calls × 127.29M) but GAE of
6.434B and 4.946B, because the pinned `ns/op` calibration was 0.39 vs 0.34 in
the two runs. The candidate's `ns/op` was 3.44 and 3.02. So the 32× operation
saving reaches GAE as 1.4–10× per system depending on the measured price, and
part of the pair-to-pair spread in the ratio is calibration, not algorithm.

## Independent corroboration at 14 bits

The peer panel (`PROTOCOL_F4_DEGREE32_14BIT_SINGLE_CURVE_20261125.md`, control
`prime-interval:t=5,lo=1`, binary `1ae5170f…`, target seed 20261301) passed on
the same curve with the same candidate seed: combined charged GAE **0.3152**
(pairs 0.3070 / 0.3245), 360/376 relations over 14,044/14,290 trials. Store
`degree32_14bit_single_curve_20261301_cap900.jsonl`
(SHA-256 `b56bedf47740662ced9e5bb8d7618cabefc4fc93e9582271e4f7b30245dfd2d4`).

So at 14 bits, two different control constructions (`prime-abscissa:below=32`
and `prime-interval:t=5,lo=1`), two different binaries and two different
target seeds agree at 0.26 and 0.32. That is the strongest form of the 14-bit
statement available: the lead is not an artifact of how the equal-size control
is built. The two panels remain unpooled because their binaries and controls
differ.

## Claim boundary

Does not establish: instance replication at 14 bits (the rung has one roster
curve); a same-target rho crossover (`s_over_rho` is 3.3e5–1.3e6); a complete
collection → linear algebra → descent advantage; transfer to 15–18 bits; any
statement about s59, P-192 or the NIST curves. The parallel peer panel
(`prime-interval` control, different binary) has not been pooled.
