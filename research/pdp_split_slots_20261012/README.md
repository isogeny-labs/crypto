# Round 2: orbit-union τ-slots for the split and hybrid PDP, measured in ecbench

**Stage diagnostic plus end-to-end ecbench measurement; no speedup claim.**
Protocol `PROTOCOL.md` (registered before any measured run) with
amendments 1–3 (three container reclaims; nothing in the inputs changed).
Follows round 1 (`research/pdp_split_dreg_20261011`). Analysis
`ANALYSIS.json` / `ANALYSIS.md` (native `pdp_split_analyze`), report
`REPORT.pdf`, run reports under `reports/`.

## The question

Round 1 decomposed over the standard subspace `V = span(1, z, …, z^{ℓ−1})`
and projected the result. Here the same split and hybrid systems run as a
real `ic.pipeline` oracle (`pdp-split`) inside ecbench, on `V` and on its
Frobenius closure `∪ⱼ σʲ(V)`. On the closure, each query solves one
system per τ-slot tuple (summand `i` over `σ^{jᵢ}(V)`), and each signed
Frobenius orbit is one column.

## Part A — ecbench, matched rho, eight public targets per curve

`S` = charged group-addition equivalents / √r. **F4 arms' recorded S is a
lower bound** (F4's XORs are counted, not charged). Priced S adds the XORs at
the curve's pinned `ns_per_word_xor` (calibrated on another host): a
sensitivity, which still understates F4 because the count covers
elimination only. Medians over workloads; paired ratios are per workload.

| curve | arm | verified | recorded S | systems / run | priced S | priced ÷ strong rho | priced ÷ signed-Frob. rho | closure ÷ standard |
|---|---|--:|--:|--:|--:|--:|--:|--:|
| `icv1-f2m11-t67-05f5aa36` (ℓ = 4) | standard, unsplit g4 | 8/8 | 18.8 | 53.5 | 1,754 | 85× | 669× | — |
| | **closure, unsplit g4** | 8/8 | 16.3 | 23.0 | **696** | **36×** | 270× | **0.40** |
| | standard, S₃ chain g4 | 8/8 | 18.8 | 53.5 | 54,536 | 2,636× | 21,239× | — |
| | closure, S₃ chain g4 | 8/8 | 16.3 | 25.5 | 22,449 | 1,133× | 8,482× | 0.43 |
| `icv1-f2m13-t181-515ee569` (ℓ = 4) | standard, unsplit g4 | 8/8 | 16.9 | 70.5 | 2,031 | 142× | 1,130× | — |
| | **closure, unsplit g4** | 8/8 | 13.4 | 36.0 | **882** | **60×** | 467× | **0.40** |
| | standard, S₃ chain g4 | 8/8 | 16.9 | 70.5 | 76,539 | 5,341× | 42,564× | — |
| | closure, S₃ chain g4 | 8/8 | 13.4 | 36.0 | 43,211 | 3,026× | 24,034× | 0.51 |
| `icv1-f2m17-tm101-00378d4e` (ℓ = 5) | standard, unsplit g4 | 8/8 | 5.36 | 398 | 309,133 | 68,367× | 324,788× | — |
| | **closure, unsplit g4** | 8/8 | 3.63 | 156.5 | **115,110** | **25,257×** | 124,730× | **0.35** |
| `icv1-f2m19-tm797-9c54981b` (ℓ = 4)¹ | standard, unsplit g4 | 6/6 | 19.6 | 8,113 | 3,917 | 1,671× | 6,675× | — |
| | **closure, unsplit g4** | 5/5 | 1.84 | 2,532 | **2,143** | **889×** | 2,952× | **0.54** |
| | closure, S₃ chain g4 | 4/4 | 1.88 | 2,218 | 1,393,449 | 578,275× | 1,869,951× | — |

Measured rho (recorded S, median): strong 19.7 / 14.3 / 4.22 / 2.35 and
signed-Frobenius 2.70 / 1.80 / 1.00 / 0.52 at n = 11 / 13 / 17 / 19.

¹ n = 19 comes from two interrupted sessions (amendments 2–3): workloads
0–5 for rho and the standard arm, 0–4 for closure unsplit, 0–3 for the
closure chain; the rest is censored by container lifetime. The 20
(workload, arm) pairs the two sessions share are bit-identical in group
additions, XORs, systems and recovered logarithm.

### The §5a pricing band (AGENTS.md, owner rule 2026-10-15)

Every arm's unpriced native work is priced at w = 0, 0.1 and 1
group-addition equivalents per operation, rho included. For rho that is
canonicalisations, partition hashes and table inserts and queries. For the
F4 arms it is the solver's word XORs, for SAT its conflicts. A ranking
stands only where it holds at every weight.

- **IC arms against strong rho: unresolved.** Every F4 and SAT arm sits
  below strong rho at w = 0 (0.78–0.94× median) and far above it at
  w = 0.1 and 1 (1,215× to over 10⁸×). By §5a, none can be ranked against
  rho, in either direction.
- **Rankable arms.** Only the fully charged pair table (`ic-mitm-frob`,
  3.0–4.2× strong rho at every weight) and the standard-subspace arms at
  n = 13 and 19 (1.1× and 8.4× at w = 0, more above) are rankable, and rho
  is cheaper than all of them. Signed-Frobenius rho is cheaper than strong
  rho at every weight on n = 11, 13 and 17.
- **Closure against standard, same plan.** The median ratio at w = 0 / 0.1
  / 1 is:

  | curve | plan | ratio at w = 0 / 0.1 / 1 | workloads reversing at w ≥ 0.1 |
  |---|---|---|---|
  | n = 11 | unsplit | 0.84 / 0.40 / 0.40 | 0/8 |
  | n = 11 | chain | 0.84 / 0.43 / 0.43 | 0/8 |
  | n = 13 | unsplit | 0.79 / 0.40 / 0.40 | 0/8 |
  | n = 13 | chain | 0.79 / 0.51 / 0.51 | 1/8 |
  | n = 17 | unsplit | 0.67 / 0.35 / 0.35 | 0/8 |
  | n = 19 | unsplit | 0.10 / 0.54 / 0.54 | 1/5 |

  So the closure ranks below standard at every weight on every workload
  for unsplit at n = 11, 13 and 17 and for the chain at n = 11. The other
  two cases are unresolved per workload, though their medians favour the
  closure at every weight.
- The "priced" column above uses the pinned XOR ratio: one further
  calibrated point beside the band, never a replacement for it.

## Part B — stage diagnostic on the closure (576 runs, all complete and exact)

Six round-1 cells, four targets × four slot tuples, `g ∈ {0, 2, 4}`, plans
unsplit `S₄` and the `S₃` chain. Yield per system λ is exhaustively counted
over 512 random (target, slot-tuple) pairs. Round 1's standard-base figures
(`research/pdp_split_dreg_20261011`) are the comparison; columns are
orbit-folded on the closure and negation-folded on V.

| cell `m:a:n:ℓ` | λ per system, closure | λ per target, standard (round 1) | ratio | columns closure / standard | solving degree, unsplit g0 / g4 (both bases) |
|---|--:|--:|--:|--:|---|
| 3:1:11:4 | 0.521 | 0.258 | 2.0 | 5 / 6 | 7 / 5 |
| 3:0:13:4 | 0.617 | 0.156 | 4.0 | 9 / 10 | 7 / 5 |
| 3:1:17:4 | 0.0020 | 0 (in 256) | — | 5 / 6 | 7 / 4–5 |
| 3:1:19:4 | 0.0059 | 0.0039 | 1.5 | 7 / 8 | 7 / 4–5 |
| 3:0:13:5 | 3.44 | 0.770 | 4.5 | 13 / 17 | 7 / 6 |
| 3:1:17:5 | 0.158 | 0.035 | 4.5 | 13 / 15 | 7 / 6 |

Solving degree per system matched round 1's standard-base value within ±1
on all 36 (cell, plan, g) medians. The relation-phase projections are in
`ANALYSIS.md`; they **understate** the measured cost (at n = 11 the
projection puts closure unsplit g4 at 9× strong rho, Part A measures 36×),
because `columns + 1` relations is fewer than the linear algebra actually
consumed. Part A's measured figures supersede them.

## Part C — `m = 2` on the dimension-9 invariant base, n = 17

`koblitz-orbit:divisor=0;1`, eight public targets, every run verified.

| arm | recorded S | priced S | ÷ strong rho | charge |
|---|--:|--:|--:|---|
| `mitm-frobenius:m=2` | 17.10 | 17.10 | **4.18×** (3.65–4.53) | complete |
| `descent-algebraic` + SAT (native XOR) | 3.43 | — | 0.86× (0.71–0.92) | lower bound: conflicts unpriced |
| `pdp-split` m = 2, g = 0 | 3.43 | 12,593 | 2,919× | lower bound + XOR sensitivity |
| `pdp-split` m = 2, g = 4 | 3.42 | 189 | 46× | lower bound + XOR sensitivity |
| strong rho / signed-Frobenius rho | 4.10 / 0.94 | | | |

The three algebraic arms find the same relations (recorded S agrees to two
digits); their recorded S is the group-operation floor of any oracle that
returns those relations. The fully charged pair table on the same base is
the cheapest complete IC on this curve, as before.

## Predictions, scored

| | prediction | outcome |
|---|---|---|
| R1 | closure unsplit solves ≥ 1.5× fewer systems than standard on every curve | **held**: 2.3×, 2.0×, 2.5×, 3.2× (n = 11, 13, 17, 19) |
| R2 | closure unsplit's priced S below standard's on every curve | **held** at the pinned ratio (0.40, 0.40, 0.35, 0.54). Under the §5a band it holds on every workload at n = 11, 13 and 17. At n = 19, 1 of 5 workloads reverses at w ≥ 0.1, though the median is 0.10 / 0.54 / 0.54 |
| R3 | λ per system on the closure ≥ 1.5× standard λ where both nonzero | **held**: 2.0–4.5× (n = 19: 1.5×, on 3 hits in 512 systems) |
| R4 | solving degree equal on both bases (±1) | **held**: 36/36 |
| R5 | no IC arm's median priced S below strong rho's median S | **held** at the pinned ratio (closest: closure unsplit at n = 11, 36×). Under §5a the F4 and SAT arms are **unresolved** against rho: below it at w = 0, far above at w ≥ 0.1 |
| R6 | every ecbench run verifies; every Part B run complete and exact | **held** for every run recorded; n = 19 has censored workloads (amendments 2–3) |
| R7 | no `pdp-m2` arm's priced S below `ic-mitm-frob`'s S | **held**: 189 vs 17.1 |

## Reading (interpretation, labelled)

- The Frobenius closure is the better base for these systems on every curve:
  the same per-system cost and degree, 1.5–4.5× more decompositions per
  system, so 2–3× fewer systems per relation and 0.35–0.54× the priced cost.
  That is the full effect predicted before the run (≤ m!), not more.
- It does not change the comparison with rho. Distance remaining (owner
  success criterion, 2026-10-15): the best F4 arm sits 36× (n = 11) to
  25,000× (n = 17, ℓ = 5) above strong rho at the pinned ratio, and
  1,215×–1.2·10⁶× at w = 0.1. The fully charged pair table on the
  invariant base (3.0–4.2× at every weight) beats every F4 arm on the one
  curve where both ran.
- **Recorded S below rho is an accounting artefact.** Every F4 and SAT arm's
  recorded S sits at or below strong rho because their solver work is not
  charged. Those figures are lower bounds; they must not be read as wins.

## Limits and gaps

- n = 19 is censored (container reclaims): workloads 4–7 for the chain,
  5–7 for closure unsplit, 6–7 for the rest.
- The XOR pricing omits F4's per-call work; it is a sensitivity, not a cost.
- Chain arms are absent where one run takes hours (n = 17; standard at n = 19).
- Small curves (`r ≤ 2^18`), one round, L0; no wall-time claims.

## Exploration and round 3

`EXPLORATION.md` records the unregistered sweeps run while this round was
in flight (closure ℓ-sweep, hybrid g-sweep, the cost-per-unknown ladder at
n = 23). They motivate round 3 (`research/pdp_split_round3_20261018`).
