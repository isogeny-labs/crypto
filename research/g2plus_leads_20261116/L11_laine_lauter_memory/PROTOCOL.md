# L11 protocol: memory-bounded Diem double-large-prime relation collection on plane quartics (time-memory curve)

Frozen 2026-11-16, before any data exists. Lead L11 of the g2plus pass
(`research/g2plus_leads_20261116/`). Every number here, and every number the run
produces, is `[pcc:unverified]` until validated. This is not a benchmark: no
leaderboard row, no `benchmarks/` row, no speed claim, no comparison with rho.
Wall time is not a metric.

## 0. Prior art checked (grep; research-kb is not installed in this session)

| path | what it shows |
| --- | --- |
| `research/genus_ladder_20261114/PROTOCOL.md` §9 | "Single large prime, sieving, and the Laine-Lauter / Diem-Kochinke variants ... are not implemented." The quartic arm `Q_double_lp` uses `ceil(3 sqrt(p))` base points, unbounded graph. |
| `research/genus_ladder_scale_20261115`, `_bound_r_`, `_sge_`, `_harvest_` | quartic/genus-3 2LP measured for cost against rho; no memory metric, no memory cap. |
| `research/diem_quartic_relations_20261018/PROTOCOL.md` | GHS-quartic Diem relations, arms FULL / SLP (`kappa q^{2/3}`) / DLP (`kappa q^{1/2}`), unbounded graph; cost only, no memory arm. |
| `research/plane_quartic_program_20261018/{PROGRAM,CLOSING}.md` | program-level; memory appears only as a host cap (12 GB). |
| `src/cryptanalysis/quartic_ladder.rs` (`Ctx::line`, `run_diem`, `push_rel`) and `chained_transfer/ic_control.rs` (`cycles0`, `build_rows`) | line residual by polarisation; lines through base pairs; relation count = cyclomatic number of the component of vertex "1"; no eviction. Conventions reused here. |
| `research/leads_20261111/EXISTING_LEADS.txt` | `LEAD-ICB-large-prime-variants`, `LEAD-ICE-relation-graph-rank-large-prime` (generic); no memory-bounded lead. |
| `../cryptanalysis` (grep `laine`, `time-memory`, `memory-bounded`) | no hit on index calculus. |

Verdict: not a duplicate. No executed lab work measures stored-record memory or
a memory cap for the quartic large-prime graph.

The paper (Laine, Lauter, "Time-memory trade-offs for index calculus in genus 3",
J. Math. Cryptol. 9(2), 2015; ePrint 2014) could not be read: `eprint.iacr.org`
and the ECC 2015 slide host do not resolve from this session. The search
abstract confirms only: a variant of Diem's `O~(q)` quartic algorithm with a
double large prime strategy, analysed for time-memory trade-offs and
parallelisation. **Everything in §1 below is derived here, not taken from the
paper.**

## 1. Mathematics (derived; corrections to the brief)

Notation: `C/F_p` smooth plane quartic (genus 3), `N = #C(F_p) ~ p`, base
`B` of `b` rational points, `beta = b/p`. A line through two distinct base points
`P1, P2` meets `C` in `P1 + P2 + R1 + R2`. The residual binary quadratic splits
over `F_p` with probability about 1/2. Relation: `x_{P1}+x_{P2}+x_{R1}+x_{R2} - h = 0`
with `x_P = [P - P0]`, `h = [H - 4P0]`, `x_{P0} = 0`. Unknowns: `b - 1` base
columns plus `h`, i.e. `b` columns.

C1 (correction: edges are sums, not differences). A 2LP relation gives
`x_{R1} + x_{R2} = A_e` (A_e a combination of base columns and `h`); a 1LP
relation (`R2` in `B`) gives `x_{R1} + x_{"1"} = A_e` with the special vertex
"1" fixed at 0. In the connected component of "1" with `V` vertices (including
"1") and `E` edges, the unknowns are `V - 1` and the system has rank `V - 1`,
so it yields exactly `E - V + 1` base-only relations (the cyclomatic number,
as `ic_control::cycles0` counts). In a component without "1" a non-bipartite
component yields `E - V` relations, not `E - V + 1`; this pass counts only the
component of "1" (conservative, same as the lab code).

C2 (base size). With `b = c sqrt(p)`, the supply of base-pair lines is
`b^2/2 = c^2 p / 2`, giving about `c^2 p / 4` edges on about `p` vertices.
A giant component needs more than `p/2` edges, so `c > sqrt 2` is necessary
(lab value `c = 3`).

C3 (unbounded cost and memory). The giant component's excess after
`E = (1+eps) p/2` edges is about `(2/3) eps^3 p`; the stop needs `b` relations,
so `eps ~ (1.5 b/p)^{1/3} = (1.5 c)^{1/3} p^{-1/6}`. Lines `T_U ~ p (1 + eps)`.
Stored records (vertices + edges) `~ 1.4 p`. So time `O(p)`, memory `Theta(p)`.
Matches the brief's `O(p)` vertices.

C4 (static admissible-set cap; the brief's "cap M" with reject-on-arrival).
Keep a large prime only if it lies in a fixed admissible set `A` of density
`alpha` (hash of the point). Then `M ~ alpha p` vertices, edges arrive at rate
`~ alpha^2 / 2` per line, and the giant needs `~ alpha p / 2` edges, so
`T ~ p / alpha = p^2 / M`. **Elasticity `d log T / d log M = -1`**: halving the
memory doubles the line trials. Keeping the line supply sufficient needs
`c >= sqrt(2/alpha)`; with `b ~ p sqrt(2/M)` the curve ends where `b ~ M`,
i.e. `M ~ p^{2/3}`, `T ~ p^{4/3}` (the GTTD-like point). This contradicts the
brief's prediction for this family ("memory falls by a power of p at under 2x
time"): under C4 memory `p^{1-d}` costs time `p^{1+d}`.

C5 (root-growth rule; derived candidate for the brief's prediction). Store an
edge only if one endpoint is "1" or already in the component of "1"; discard
the rest. Then the root component `K` grows as `dK/dT = b/p + K/p`, so
`K(T) ~ b (e^{T/p} - 1)`, and its cycles accumulate as `~ K^2 / (4p)`. The stop
needs `K ~ 2 sqrt(p b) = 2 sqrt(c) p^{3/4}` and
`T_G / p ~ 0.25 ln p + ln(2/sqrt(c))` (`= 0.25 ln p + 0.14` at `c = 3`).
Memory `Theta(p^{3/4})` at time `Theta(p log p)`: memory falls by `p^{1/4}` at a
`log p` (not constant) time factor. Predicted ratios to U (heuristic, before
data): `r_T ~ 1.6` at `k = 12`, `~1.9` at `k = 14`; `r_M ~ 0.6` and `~0.45`.

C6 (single large prime). `b = 2 p^{2/3}`; 1LP records collide on the same large
prime; relations = full + (1LP records - distinct large primes). Heuristic
`T_S ~ p^{7/6}`, memory `~ 4 p^{5/6}`.

Linear algebra is not in the FPM metric (all arms); the final matrix's size `b`
and weight `nnz` are reported, and `b * nnz` as a Wiedemann proxy.

## 2. Hypotheses and exact claims

- H_brief (brief's prediction, as frozen by the main session): some memory-bounded
  arm reduces peak stored records by a power of `p` at under 2x line trials.
- K_brief (brief's kill): every memory cap below half the unbounded peak costs
  more than 4x the line trials.
- H_C4: for arm family `A_scaled`, the elasticity of trials in memory at `k = 14`
  lies in `[-1.3, -0.7]`.
- H_C5: arm `G` has memory exponent (in `p`) below U's by at least 0.10 and time
  exponent within U's + 0.15.

The only claims allowed after the run are the verdicts of §8 over the measured
range `p in {263, 1031, 4099, 16411}`, random smooth plane quartics, this
implementation, counted units. No asymptotic claim, no claim about the paper.

## 3. Arms (all relation collection by lines through two distinct base points, in one seeded random pair order)

| arm id | base size `b` | which relations are stored | memory rule |
| --- | --- | --- | --- |
| `U2` | `ceil(2 sqrt p)` | full, 1LP, 2LP | unbounded |
| `U` (baseline) | `ceil(3 sqrt p)` | full, 1LP, 2LP | unbounded |
| `U4` | `ceil(4 sqrt p)` | full, 1LP, 2LP | unbounded |
| `UAA` (A/A) | `ceil(3 sqrt p)` | as U | as U, independent base and pair-order seeds |
| `AF1` (identity control) | `ceil(3 sqrt p)` | as `AF` with `alpha = 1` | must reproduce U exactly |
| `AF2, AF4, AF8` | `ceil(3 sqrt p)` | only if every large prime is admissible, `alpha = 1/2, 1/4, 1/8` | static admissible set |
| `AS2, AS4, AS8` | `ceil(3 sqrt(p / alpha))` | as AF, `alpha = 1/2, 1/4, 1/8` | static admissible set, base scaled to keep line supply |
| `G` | `ceil(3 sqrt p)` | edge stored only if an endpoint is "1" or in the component of "1" | root growth (discard others on arrival) |
| `S` | `ceil(2 p^{2/3})` | full and 1LP only | single large prime |

Positive control: `AS8` (and `AF8`) must show the memory/time effect the
instrument is supposed to see: predicted `r_T` about 8 for `AS8`. Negative
(A/A) control: `UAA` vs `U`. Identity control: `AF1` must equal `U` bit for bit
(same lines, same counts).

## 4. Variables held fixed

Curve set per `p` (12 curves, shared by all arms); the line routine and its
counter (one implementation); the stop rule `count >= b + 8 + floor(b/100)`;
the eviction/admission rules above; seeds (§5); cost unit; Python version
recorded in each output. Arms with the same `b` share the base and the pair
order (common random numbers: `U`, `AF*`, `G` share them; `UAA` does not).

## 5. Instances and seeds

- Primes: `p_k` = smallest prime `>= 2^k` with `p = 3 (mod 4)`:
  `k = 8: 263`, `k = 10: 1031`, `k = 12: 4099`, `k = 14: 16411`.
- Curves: 12 per `p`, indices `i = 0..11`. Seed
  `s_curve(k, i) = 0x4C31310000000000 XOR (k << 32) XOR i`; SplitMix64 stream
  draws the 15 coefficients (`next() % p`, fixed monomial order of SPEC §2) of
  candidate 0, 1, ...; accept the first candidate that passes the smoothness
  test (SPEC §4) and has `N1 >= b_max + 10` where `b_max = max(ceil(3 sqrt(8p)),
  ceil(2 p^{2/3}))`. Attempts recorded.
- Rational points: enumerated once per curve (harness convenience, not part of
  any arm's cost; same for all arms; stated as a limit).
- Base and pair order: SPEC §6 seeds, functions of `(s_curve, b, salt)`;
  `salt = 0` for all arms except `UAA` (`salt = 0xAA`).
- Seeds per cell: 12 curves x 1 run = 12 (>= 10 required).
- Gate instances: Fermat quartic `x^4 + y^4 + z^4` at `p = 103` and `p = 127`.

## 6. Metrics and unit

Per run (one JSON line, SPEC §10):

- `T` = line trials (base pairs consumed, including tangent and non-split lines)
  until the stop rule, or the full supply if exhausted (`exhausted = true`).
- `FPM = mul + 10 * inv`, counted by the instrumented field counter; squarings
  count as multiplications; additions free. `FPM_total = FPM_setup (Taylor data
  of the b base points) + FPM_lines`. Point enumeration is not counted.
- Memory: `V` LP vertex records (including "1"), `E` stored LP edge records,
  `Fr` stored full relations, `D` dedup keys; `lp_records = V + E`;
  `mem_total = V + E + Fr + D + b` (base records). Records are never deleted
  in any arm, so the peak is the final value. Primary memory metric: `mem_total`.
- Census: counts of tangent / non-split / full / 1LP / 2LP lines, admitted and
  rejected relations, admissible fraction of large primes seen.
- Final matrix: rows (= relation count at stop), `nnz`, `b * nnz`.
- Unpriced counters: dict lookups, union-find `find` steps, RNG draws.

Derived per cell (arm, `k`), paired by curve against `U`:
`r_T,i = T_arm,i / T_U,i`, `r_F,i` (FPM_total), `r_M,i = mem_total_arm,i /
mem_total_U,i`; geometric mean over the 12 curves with a 95% percentile bootstrap
interval (2000 resamples of curves, seed `0x5EED`). Exponents: OLS slope of
`ln(metric)` on `ln p` over all per-curve values at `k = 8..14`, per arm, with a
bootstrap interval (resampling curve index within each `k`). Elasticity at
`k = 14` for AS family: OLS slope of `ln GM(T)` on `ln GM(mem_total)` over
`U, AS2, AS4, AS8` (4 points; CI by bootstrap over curves).

A cell with any exhausted run reports `r_T` as a lower bound (`>=`), using the
consumed supply as `T`.

## 7. Correctness gates (must pass before any metric is read)

- G0 unit tests (SPEC §11) all pass.
- G1 relation geometry (Diem): for every stored relation at `k in {8, 10}` and a
  seeded sample of 200 stored relations per run at `k in {12, 14}`: the four
  points are distinct, each satisfies `F = 0`, and they are collinear (3x3
  determinants with `P1, P2` zero). By Bezout and smoothness they are then the
  whole line section, so differences of relations are principal divisors
  `div(L1/L2)`. Zero failures allowed.
- G2 Pic-level check (principal differences, lattice index): Fermat quartic at
  `p = 103` and `127`; base = all rational points; all point pairs through the
  production line routine; integer relation lattice index `I` (SPEC §9) must
  equal `(p+1)^3`, after verifying `N2 = p^2 + 1 + 6p` (maximality over
  `F_{p^2}` forces `L(T) = (1 + p T^2)^3`, so `#Jac(F_p) = (p+1)^3`). Design
  feasibility was checked in scratch at `p = 103` (index `1124864 = 104^3`); the
  executor's run is the gate.
- G3 counting identity: for every run, the number of cycle rows built by the BFS
  elimination (SPEC §8) equals `count - Fr`; zero rows are counted and must be
  at most 1% of the rows of the run.
- G4 rank: for every run at `k in {8, 10}`, rank of the final rows mod
  `2^61 - 1` is recorded; at least 95% of runs per arm (pooled over `k = 8, 10`)
  have rank `>= b - 1`.
- G5 identity control: `AF1` and `U` give identical `T`, `FPM_total`,
  `mem_total` on every curve.
- G6 admissibility census: in AF2/AF4/AF8 the admissible fraction of distinct
  large primes seen is within `alpha +- 4 sqrt(alpha(1-alpha)/n_seen)` pooled per
  `k`.

If G0, G1, G2, G3 or G5 fails, no verdict is issued; the failure is reported.
If G4 fails for an arm, that arm's metrics are reported but excluded from the
verdict. If G6 fails, AF/AS arms are excluded from the verdict.

## 8. Decision rule (frozen; thresholds do not move)

Instrument validity (else verdict "instrument invalid"):
- A/A: `UAA` vs `U` geometric-mean `r_T` within `[0.85, 1.18]` at `k = 10, 12, 14`.
- Positive control: `AS8` `r_T` lower 95% bound `>= 3.0` at `k = 12` and `14`.

Let a cell be (bounded arm in {AF2, AF4, AF8, AS2, AS4, AS8, G}, `k`). `S` is
reported, not used in the verdict.

- **SUPPORTED (H_brief)**: one arm X has, at both `k = 12` and `k = 14`,
  upper bound of `r_M <= 0.5` and upper bound of `r_T < 2.0`; and X's
  `mem_total` exponent is at least 0.10 below U's with the bootstrap interval of
  the difference excluding 0 (memory falls by a power of `p`).
- **KILLED (K_brief)**: at both `k = 12` and `k = 14`, at least one cell has point
  `r_M <= 0.5`, and every cell with point `r_M <= 0.5` has lower bound of
  `r_T > 4.0` (an exhausted cell counts as `> 4.0` only if its supply bound
  exceeds `4 T_U`).
- **INCONCLUSIVE** otherwise; the measured time-memory curve and exponents are
  the result.

Secondary verdicts, reported separately: H_C4 (elasticity interval inside or
outside `[-1.3, -0.7]`; "inside" requires the whole 95% interval inside);
H_C5 (G's memory exponent `<= U's - 0.10` and G's `T` exponent `<= U's + 0.15`,
both by point estimate with intervals reported).

Prediction recorded before data: INCONCLUSIVE for H_brief/K_brief (AF2 near
`r_T ~ 2, r_M ~ 0.5`; G near `r_T ~ 1.6-1.9, r_M ~ 0.45-0.6`), H_C4 inside,
H_C5 supported.

## 9. Confounders and how each is isolated

- Base size changes cost (AS, S, U2/U4): the U2/U/U4 grid measures `T` versus `b`
  at fixed rules; AF arms change only the admission rule at fixed `b`.
- Random-order noise: common random numbers (shared base and pair order) for
  U/AF/G; A/A arm measures the seed noise; paired ratios per curve.
- Hash admissibility correlated with geometry: hash of normalised coordinates
  salted by the curve seed; census gate G6.
- Instrument: one counter class and one line routine for all arms; G5 checks it.
- Stop-rule artefacts (count vs rank): G3 and G4.
- Dedup and multi-edges: dedup only for relations with a residual in the base;
  multi-edges counted as cycles (valid 2-cycles); stated.
- Enumeration excluded from cost: same for all arms; stated as a limit.
- Linear algebra excluded from FPM: `b * nnz` reported per arm, so a memory saving
  bought with a bigger matrix is visible.
- Analyst: the analysis script computes the verdict mechanically from §8; arm
  labels are not masked (single analyst, stated).

## 10. Budget and commands

1 thread, each command under about 10 CPU-minutes and 2 GB. Working directory
`research/g2plus_leads_20261116/L11_laine_lauter_memory`. No hook-refused
command may be worked around; refusals are recorded verbatim in `raw/REFUSALS.txt`.

```
python3 test_l11.py                                  > raw/unit_tests.txt 2>&1
python3 l11.py gate-pic --p 103                      > raw/gate_pic_p103.json
python3 l11.py gate-pic --p 127                      > raw/gate_pic_p127.json
python3 l11.py curves --k 8                          # writes raw/curves_k8.jsonl, raw/points_k8_<i>.json
python3 l11.py curves --k 10
python3 l11.py curves --k 12
python3 l11.py curves --k 14 --curves 0-5
python3 l11.py curves --k 14 --curves 6-11
python3 l11.py run --k 8  --arms all                 --out raw/runs_k8.jsonl
python3 l11.py run --k 10 --arms all                 --out raw/runs_k10.jsonl
python3 l11.py run --k 12 --arms U,U2,U4,UAA,AF1,G   --out raw/runs_k12_a.jsonl
python3 l11.py run --k 12 --arms AF2,AF4,AF8,S       --out raw/runs_k12_b.jsonl
python3 l11.py run --k 12 --arms AS2,AS4,AS8         --out raw/runs_k12_c.jsonl
python3 l11.py run --k 14 --arms U,U2,U4,UAA,AF1,G   --curves 0-5  --out raw/runs_k14_a0.jsonl
python3 l11.py run --k 14 --arms U,U2,U4,UAA,AF1,G   --curves 6-11 --out raw/runs_k14_a1.jsonl
python3 l11.py run --k 14 --arms AF2,AF4,AF8         --curves 0-5  --out raw/runs_k14_b0.jsonl
python3 l11.py run --k 14 --arms AF2,AF4,AF8         --curves 6-11 --out raw/runs_k14_b1.jsonl
python3 l11.py run --k 14 --arms S                   --curves 0-11 --out raw/runs_k14_s.jsonl
python3 l11.py run --k 14 --arms AS2,AS4             --curves 0-11 --out raw/runs_k14_c.jsonl
python3 l11.py run --k 14 --arms AS8                 --curves 0-5  --out raw/runs_k14_d0.jsonl
python3 l11.py run --k 14 --arms AS8                 --curves 6-11 --out raw/runs_k14_d1.jsonl
python3 l11.py analyze --in 'raw/runs_k*.jsonl' --out raw/analysis.json --tsv raw/summary.tsv
sha256sum raw/* > raw/SHA256SUMS
```

If a `run` command would exceed 10 CPU-minutes, the executor splits it further by
`--curves` (same arms, same seeds; outputs concatenated by `analyze`), and records
the split. A cell is not dropped or shortened for budget without recording it as
skipped; no seed is redrawn.
