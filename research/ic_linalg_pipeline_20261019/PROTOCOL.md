# Protocol: the relation matrix inside `ic.pipeline`, sparse-cado against dense full-rank, at large factor bases

Frozen 2026-10-19, before any recorded session. Changes after data go under
"Deviations" in the README, never edited in here.

Answers the open item in `research/ic_leads_review_20261018/README.md` §3.3
("The relation matrix has never been measured"): it measures `linalg=sparse-cado`
(CADO-style filter, then block Wiedemann; PR #1874) against
`linalg=incremental-gauss-full-rank` inside the full ecbench pipeline, on the same
relation stream, as the factor base grows.

## Question

At factor bases of 1,024 to 16,384 columns on a 32-bit prime curve, what does the
choice of relation matrix change in the end-to-end `ic.pipeline` run: in `S`, in
the matrix's counted work (`row_ops`), and in the linear-algebra wall time?

## Boundary and reference

- Curve: `icv1-fp32-t7319-90da43e9` (registered; p = 2^32 − 5, a = −3, b = 31,
  prime order r = 4294959973, A = 2), built with `prime_explicit` as in
  `research/ic_frontier_campaign_20261015`.
- Reference arm: `rho.negation`. Floor `sqrt(pi/2A)` in `S` is recorded per run.
- Every IC arm uses `factor_base=prime-abscissa:size=N`, `oracle=mitm:negation_folded=1`,
  `targets=walk` (defaults written out by the harness). Only `linalg` differs
  between a size's baseline and candidate.

## Arms

| arm | method | N |
|:--|:--|:--|
| `rho-neg` | `rho.negation` | — |
| `dense-N` (baseline) | `ic.pipeline`, `linalg=incremental-gauss-full-rank` | 1,024 · 2,048 · 4,096 · 8,192 |
| `cado-N` (candidate) | `ic.pipeline`, `linalg=sparse-cado` | same |
| `cado-4096-aa` (control) | repeat of `cado-4096` | 4,096 |

Both IC linalgs run under the full-rank stop rule, so they read the same relations.

Session A (`specs/A-1k-8k.json`): the arms above, 8 targets, 5 rounds, 1 warm-up,
`order: alternate`, `isolation_required: L2`, timeout 3,000 s per run.
Session B (`specs/B-16k.json`): `rho-neg`, `dense-16384` and `cado-16384`,
4 targets (the first four of Session A's target law and seed), 2 rounds, 1 warm-up.
Its wall times are descriptive only (fewer than 5 rounds); its counts are
deterministic and count fully.

## Measures

1. `S` per arm and `ecbench compare` ratios: `cado-N` against `dense-N` and against `rho-neg`.
2. Linear-algebra `row_ops` and rows (deterministic counts from each record's
   `linear_algebra` phase).
3. Linear-algebra `wall_ns` per (workload, round) pair: totals ratio
   Σ cado / Σ dense per N, with the range of the per-pair ratios, and the A/A
   spread from `cado-4096-aa` against `cado-4096`.
4. Pricing band (AGENTS.md §5a): every arm's `*_uncharged` counters, rho's
   included, priced at 0, 0.1 and 1 GAE per operation on every arm alike. Two
   arms are ranked only where the order holds across the band.
5. Gap to the matched rho per N (`S_IC / S_rho`), reported as distance remaining.

## Predictions (pilot-informed)

Unrecorded pilots at N = 1,024 and 4,096 (an unregistered 32-bit curve) and
N = 8,192 (this curve, one target) informed these.

- P1. `S`, trials and relations are identical between `dense-N` and `cado-N`
  (linear algebra is unpriced in `ic.pipeline`; same relation stream).
- P2. `cado-N` counts more `row_ops` than `dense-N`, by about 1.5×. Two-term
  relations barely fill in, so the dense method's counted work stays near-linear.
- P3. `cado-N` spends less linear-algebra wall time than `dense-N` for N ≥ 4,096,
  about 4× less at 8,192, because the dense method's column scans are not
  counted in `row_ops`.
- P4. Oracle set-up (the |F|² pair table) dominates `S` at large N, so neither
  matrix moves the gap to rho.

## Decision rule

- The matrix choice "matters for `S`" only if, across the §5a band, the cado/dense
  `S` ratio's interval excludes 1 at some N. Otherwise it is an engineering
  (wall-time) result, not an advance (AGENTS.md §3).
- The wall-time result is reported as measured only where the runs are
  `admitted` and the cado/dense difference exceeds the A/A spread; otherwise
  it is descriptive.
- Failures, timeouts and unverified runs are reported, never dropped.

## Inadmissible

Changing the curve, targets, seeds, sizes, arms, unit or accounting after seeing
a recorded session; dropping a run; pooling sizes into a scaling claim.
