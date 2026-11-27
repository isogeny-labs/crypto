# m ≥ 3 does not change the exponent — 13-bit measurement and m-general model

**Route 3 of the subexponential ECDLP programme, 2026-11-25.**
Binary `1ae5170f…d8d08`, instance `icv1-fp13-tm13-9e2d11f6` (13 bits, prime order
`r = 4957`), public/toy only. Commands: `commands.md`. Store: `route3_store.jsonl`
plus the raw append-only bench store `m2_m3_13bit_20261185.jsonl`.

**Answer.** At equal base size the m = 3 chain costs **1.3 × 10⁴ times more per
F4 call than m = 2 and does not resolve at all at a 5 s budget** (0 of 8 calls
verified). The m = 2 advantage of the isogeny base (32×) *inverts* at m = 3
(the interval base becomes 53× cheaper). The chain's second quartic raises the
solving degree from 5 to > 8 while only one T step is saved. Counting exactly,
the m = 3 intermediate `x(P1+P2)` is a generic field element at every T
measured, so there is **no per-level pruning**: the pair-sum set is 0.89–1.00 ×
|V|², not O(|V|). Pruning appears only for subgroup-coset bases, and there it
costs a 40× yield collapse for a 32× decision saving. Under exact enumeration the
total is **p^1.0 for every m** (route 1's result, confirmed here). The only
construction that lowers the exponent — a reusable pair-sum table — needs
p^{2/3} **entries** (2^170 at 256 bits) and gives p^0.667 for m = 3, still above
rho's p^0.5. **m ≥ 3 is not a viable subexponential route in this family.**

## 1. m = 2 reference panel (frozen shape, complete)

16 repeats × 1000 trials, 5 s budget, 1 rho run, order interval, candidate,
candidate, interval. Both arms build **18 abscissae** on the same instance
(equal size, no fudge). All 32 rows per arm verified; recovered scalars agree
across the four calls.

| arm | base | vars/eqs | D_solve | ops/call (mean) | rel/trial | total GAE | median s/rho |
|:--|---:|---:|---:|---:|---:|---:|---:|
| isogeny `t=5,seed=5` | 18 | 10/11 | 5 | **3.978 M** | 0.1279 | 6.32e8 | 1.15e5 |
| interval `below=32` | 18 | 10/11 | 6 | 127.38 M | 0.1315 | 2.25e9 | 4.12e5 |
| ratio (int/cand) | — | — | — | **32.02×** | 1.03× | 3.55× | 3.57× |

These reproduce the frozen m = 2 numbers (3.97 M and 127.4 M per call) exactly.
Caveat that applies to every 13-bit row: the ideal m = 2 base is
p^{1/2} ≈ 90 abscissae; 18 ≈ p^{0.30} is the largest matched size available, so
the relation probability is 0.128 instead of ≈ 1 and 7.8× more targets are
needed. Both arms share the base, so the arm comparison is unaffected.

## 2. m = 3 F4 at the same size: censored, and the 32× inverts

2 repeats × 2 trials (16 × 1000 is infeasible: every call censors, so the full
protocol would need ≈ 4.7 h/call to report the same censoring). Same instance,
same 18 abscissae, same order.

| arm | vars/eqs | degrees | D_solve | Macaulay cols / rows / nnz-row | charged ops/call | verified |
|:--|---:|:--|---:|---:|---:|---:|
| isogeny `t=5,seed=5` | **16/17** | [4,4,2×15] | **unresolved** | 29,189 / 27,140 / 54.3 M | **5.24e10** (censored) | 0/8 |
| interval `below=32` | 16/17 | [4,4,2×15] | unresolved | 15,934 / 2,620 / 1.73 M | 9.95e8 (censored) | 0/8 |

Three things to read off it:

1. **The system, not the base, is what grew.** m = 2 is 10 variables with one
   quartic; the m = 3 chain is 16 variables with two quartics. That is 6 extra
   variables and a doubled quartic block for one fewer T step.
2. **Charged work without resolution is not a solve cost.** `relations_found`
   in these rows is the oracle's claim from *undecided* systems
   (`pkm_undecided_systems = 2`, `verified = false`, `recovered = null`). No
   m = 3 relation probability is measurable here, and the claimed hit rate 1.0
   must not be read as one.
3. **The 32× is a fixed-arity (m = 2) phenomenon.** At m = 3 the isogeny base
   produces the *denser* Macaulay matrix (29,189 columns, 54.3 M nnz/row against
   15,934 and 1.73 M), and the interval arm is 52.7× cheaper per call. The
   isogeny advantage is tied to the 10-variable single-quartic shape; when the
   free intermediate appears, the tower's sparsity advantage disappears.

## 3. m = 3 one T step down resolves — and shows a 2^13.7 offset

`prime-isogeny:t=4,seed=5` builds 11 abscissae. The same chain then resolves
deterministically (identical ops at 5 s and 30 s budgets):

| m | T (abs) | vars/eqs | degrees | D_solve | ops/call | censored |
|:--|---:|---:|:--|---:|---:|---:|
| 2 | 5 (18) | 10/11 | [4,2×10] | 5 | 3.978 M | 0 |
| 3 | 4 (11) | 13/14 | [4,4,2×12] | **6** | **1.75e10** | 0 |
| 3 | 5 (18) | 16/17 | [4,4,2×15] | > 8 | ≥ 5.24e10 | 8/8 |

`t=6` (26 abscissae) was not run: at `t=5` the m = 3 system is already
unresolved at 5 s and the charged work is 5.2e10, so one more T step can only be
worse and could not resolve inside any budget here. The T step *down* is the
informative direction, because it is the only one that resolves.

At the *same* T = 5 the m = 3 call is **13,182×** (2^13.69) the m = 2 call; at
one T lower it is still **4,406×** (2^12.1) and confirms that the m = 3 cost is
so far above m = 2 that no T reduction available at 13 bits can pay for it: the
whole m = 2 → m = 3 T saving is b/2 → b/3 = 2.17 T units, worth 15.72^2.17 =
2^8.6 at the measured m = 2 multiplier. Net per call: **2^5.1 ≈ 34× worse**, and
the atom count (triples, |V|³ vs pairs |V|²) then makes it worse again.

So the λ^T law — which *extrapolated* to m = 3 would give exponent 4.97/3 = 1.66
against 4.97/2 = 2.49, i.e. an apparent gain — does not hold: the chain changes
the system shape, and the measured per-call offset (+) dominates the T saving
(−). No m = 3 exponent gain is realised under F4.

## 4. Does the chain admit per-level pruning? No.

Pruning would need the intermediate `R1 = P1 + P2` to land in a small set so the
second link is cheap for every intermediate. Exact counts on the public 13-bit
curve (`sumset_sweep.json`; `s2 = |x(V±V)|`, `s3 = |x(V±V±V)|`):

| T | abscissae | s2 | s2/\|V\|² | s3 | s3/\|V\|³ | yield m=2 | yield m=3 |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 4 | 10 | 100 | **1.00** | 659 | 0.66 | 0.024 | 0.161 |
| 5 | 18 | 318 | **0.98** | 2,619 | 0.45 | 0.078 | 0.639 |
| 6 | 26 | 637 | **0.94** | 3,922 | 0.22 | 0.156 | 0.958 |
| 7 | 34ᵃ | 1,026 | **0.89** | 4,135ᵇ | 0.11ᵇ | 0.251 | 1.010 |

ᵃ T = 7 proxy: the true `t=7` interval base is 68 abscissae; enumerating 68³
triples is out of scope for this counting script, so 34 is used and the trend
is what is claimed. ᵇ saturated: 34³ = 39,304 ≫ p = 8,191, so `s3` is capped by
p, not by structure.

The pair set is quadratic at every T. The m = 3 intermediate is therefore a
generic element of F_p and the two links decouple: **the chain decision is a
plain 3-sum decision, with no tower-specific pruning**. Positive control
(`structure_probe.json`): a 32-element subgroup coset does collapse —
s2 = 48 = 1.5|V|, s3 = 64 = 2|V| — a 6.5× / 40× reduction. But the same
collapse cuts the m = 3 yield by 40× (s3 64 vs 2,589 at |V| = 18) while the
decision only gets 32× cheaper (|V|² → |V|), so the coset base is a net 1.25×
loss; its total exponent is p^{1+1/m} = p^{1.33} at m = 3, worse than p^1.0.

Validation of the model used for those yields: the binary's own exact pair-sum
report on a 13-bit prime curve gives `sums = 324`, `distinct_sums = 318`,
`exact_yield = 0.0768`, `model_yield = 0.0783` — i.e. q = c·|V|²/p with
c = 1.94, the constant used below.

### Per-decision cost, m = 3 (the lead's question 2)

| decision method | field ops per target | note |
|:--|---:|:--|
| exact, hash table of V | **342 adds** (324 pair sums + 18 probes) ≈ 16 µs at the binary's measured 47.8 ns/add | memory |V|² = 324 |
| algebraic tower tree, priced | **40,500 mults** (324 leaves × 125 mults/leaf, route 1) | no table |
| F4 (measured) | **≥ 5.24e10 charged ops, unresolved** | 8 orders above either |

Versus |V|² = 324 for "exact enumeration": the tree does not go below |V|² for
m = 3 — it multiplies it by the 125-mult leaf price. That is route 1's point,
now confirmed structurally: the tree removes the F4 wall (per-system p^0.5 for
m = 2) but the m = 3 chain needs |V|² = p^{2/3} of them per target.

## 5. m-general cost model

b = log2 p, |V| = 2^T = p^{1/m}, R = |V| relations needed, q = relation
probability per target. Total charged work is
`build + (R/q) × D_m`, where D_m is the per-target decision cost. With
`c₂ = 1.94`, `c₃ = 0.90` (measured), `λ_F4 = 15.72`, `C_F4(5) = 127.4 M`,
`125` mults/leaf (all measured):

| engine | closed form | exponent as m grows |
|:--|:--|:--|
| **A. F4** | (1 + log2 λ)/m = 4.97/m, *while the shape is fixed* | 2.49, 1.66, 1.24, 1.00, … |
| **B. tree, no table** (route 1) | \|V\|^{m−1} per target × \|V\|/q targets = \|V\|^m = p | **1.00 for every m** |
| **C. tree + reusable k-sum table** | min_k max(k, m−k+1)/m | 1.00, **0.667**, 0.75, 0.60, 0.667, 0.571, 0.625, 0.556, 0.60 → 0.5 |

Engine C's optimum needs |V|^k = p^{k/m} ≈ p^{1/2} **entries of memory**; at
256 bits the m = 3 table (k = 2) is 2^170 entries. Capped at a realizable
2^80-entry memory, m = 3 falls back to k = 1 and exponent **1.00** — i.e. engine
B. The exponent sequence in C never reaches 0.5 for finite m, and it only
approaches rho asymptotically with p^{1/2} memory, which is exactly rho's time
and far more memory than rho needs.

Note on accounting, stated because it is a genuine difference with route 1:
route 1's "one decision costs |V|^{m−1}, |V| relations needed" is engine B and
gives p^1.0 for all m. Allowing the target-independent k-sum table to be reused
across targets is what produces p^{2/m} (engine C) — m = 3 → p^{2/3}. Both are
above 0.5; the difference matters only for how large the m = 3 gain is claimed
to be, and in neither case is m ≥ 3 competitive with rho.

## 6. m = 2 versus m = 3 — the comparison table

| | m = 2 (measured) | m = 3 (measured) | m = 3 (model, best case) |
|:--|---:|---:|---:|
| base size | 18 | 18 (equal) | p^{1/3} |
| variables / equations | 10 / 11 | 16 / 17 | 3T+1 |
| solving degree | 5 | > 8 | — |
| ops per call (13 bits) | 3.98 M | ≥ 5.24e10 (censored) | 342 adds (exact) |
| per-call ratio | 1 | **13,182×** | 8.6e-5 × |
| relation prob per trial | 0.128 | not measurable | ≈ 1 |
| total GAE (13 bits) | 6.32e8 | ≥ 8.19e8 (2 trials, censored) | — |
| median s/rho | 1.15e5 | 1.24e6 (censored) | — |
| **implied exponent in p** | **1.00** (exact, engine B) / 2.49 (F4 law) | **1.00** (engine B) / **0.667** (engine C with p^{2/3} memory) | 0.667 |

The "implied exponent" for m = 3 is 1.00 at any realizable memory and 0.667 only
with p^{2/3} entries. Neither is below rho's 0.50. The measured 13-bit point
cannot resolve an exponent by itself (one size); it resolves the *mechanism*,
which is what rules the route out.

## 7. Verdict and next concrete step

**m ≥ 3 is not a viable subexponential route in this family.** It does not lower
the exponent below rho under any of the three engines; under F4 it is
catastrophically worse at equal size and unmeasurable (censored) at equal T; the
one construction that does lower it (a reusable k-sum table) needs memory
p^{1/2}, i.e. rho's own time in memory, and still lands at p^{2/3} for m = 3.
The 32× isogeny lead is a fixed-arity, m = 2 artifact and does not survive the
first increase in arity.

**Next step (concrete, cheap, decisive).** Test engine C directly at
20/22/24 bits, where the m = 3 pair table fits in memory (p^{2/3} = 2^13–2^16
entries): build the pair-sum table, then measure relations found per target and
total field ops over three sizes, and fit the exponent. Prediction from §5:
2/3, and the fit either confirms the mechanism at three rungs or kills engine C
outright. This needs no F4 and no new oracle. If it confirms 2/3, the only
surviving sub-rho direction remains route 1's join problem — making one tree
pass answer many targets, or reducing the number of relations below |V| —
which is a different object from arity.

## Claim boundary

Cost model fit plus two measured panels (m = 2 complete, m = 3 censored) plus
exact combinatorial counts at 13 bits, one instance, one auxiliary-map seed.
The m = 3 panel is 2 × 2, not 16 × 1000, because the protocol as specified is
infeasible at m = 3; the reduction is stated in §2 and no equal-size or
equal-trial claim is faked. The T = 7 sumset row is a 34-abscissa proxy. Exact
counts are combinatorial (no timing). Nothing here speaks to 18+ bits, s59,
P-192, the NIST curves, or any deployed key. No git repository file was
modified and nothing was committed.
