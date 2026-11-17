# T4/T5: selecting the PKM isogeny tower seed (2026-11-03)

**Class:**
- An exact census, an engine check, and an end-to-end ecbench session with
  full discrete-log solves.
- A point-estimate improvement screen of the best algebraic arm: the 6-target
  interval [0.379, 1.084] still includes 1 and V2 failed, so this session
  alone does not establish the gain. T9 later replicated it on 16 targets.
  It does not change the order against the generic baseline or rho.

Protocol: [PROTOCOL.md](PROTOCOL.md), frozen at `27482d63a0`; the T5 addendum
was frozen at `a0e3605125`.

## Lead

- `prime-isogeny:t=6,seed=S` builds V = x(R' + ⟨Q⟩) on an auxiliary curve.
  The base is the points of the target curve with x ∈ V.
- The seed picks the auxiliary curve and the coset. The F4 system has the
  same shape for every seed.
- The incumbent algebraic arm `pkm-iso-t6-g4` used seed 0. Here the seed is
  selected by J, the count of signed base pairs whose [r]-images cancel. J is
  log-free.

## Census (exact; five family-rule miniatures; t = 4–7; 2,000 seeds per cell; 0 build failures)

| | prediction | outcome |
|:--|:--|:--|
| R1 | selected ÷ mean-seed coverage ≥ 1.5 in all 15 cells with t ≥ 5 | **held**: 1.62–3.24 |
| R2 | selected ≥ 0.9 × oracle-best in all 20 cells | **held**: equal in every cell |
| R3 | k = 20, t = 6: selected ÷ seed-0 ≥ 1.5 | **held**: seed 1093 covers 528 against 172 (3.07×). Seed 0 sits at the 6th percentile. |

Source: `analysis/summary.json` and `data/k*.jsonl`. The data is in the
artifact store; hashes are in `data/SHA256SUMS`.

## Engine (R4 held): `curve25519_family f4cost 20 isogeny[@1093] 6 4 100 2026111506`

| arm | cols | covered | degree | terms | mul-adds/target | F4 mul-adds / relation (derived) |
|:--|--:|--:|--:|--:|--:|--:|
| seed 0 | 25 | 172 | 5 | 16 | 710,090 | 5.42·10⁸ |
| seed 1093 | 46 | 528 | 5 | 16 | 703,850 (0.991×) | **1.75·10⁸ (3.10× lower)** |

## End-to-end (T5): ecbench session `ECBS1h405f20219036`

- k = 20 miniature, 6 public targets (`target_seed` 2026110340).
- Full solves (relations, linear algebra, and the DLP checked by ecbench):
  48/48 verified.
- `ecbench band`. S = (counted GAE + w × unpriced native operations) / √r.

| arm | S (w=0) | S (w=0.1) | S (w=1) |
|:--|--:|--:|--:|
| rho-neg | 1.447 | 1.547 | 2.448 |
| proj-a (generic baseline) | 12.053 | 12.931 | 20.831 |
| pkm-iso-t6-g4, seed 0 (incumbent) | 29.892 | 1.674·10⁶ | 1.674·10⁷ |
| **pkm-iso-t6-g4, seed 1093** | **26.448** | **1.073·10⁶** | **1.073·10⁷** |

- **Band verdict:** `pkm-iso-t6-g4-s1093 < pkm-iso-t6-g4` at every weight.
- **Paired** (`ecbench compare`, linearised t, df 5): seed 1093 ÷ seed 0 is
  - 0.885 [0.570, 1.373] at w = 0;
  - **0.641 [0.379, 1.084]** at w = 0.1 and w = 1.
- Seed 1093 is cheaper on 5 of 6 targets. On the sixth, seed 0 had a lucky
  solve (S = 8.3 at w = 0).
- With 6 targets the interval still includes 1. The point estimate and the
  band order favour seed 1093; significance is not established.
- F4 elimination work summed over the six solves: 3.64·10¹⁰ → 2.33·10¹⁰
  mul-adds.

| | prediction (Addendum T5) | outcome |
|:--|:--|:--|
| V1 | s1093 verifies 6/6 | **held** |
| V2 | mean S ratio ≤ 0.6 at w = 0.1 and 1 | **failed**: 0.641 |
| V3 | s1093 < seed 0 at w = 0 (means) | **held**: 0.885. The interval includes 1. |
| V4 | both algebraic arms stay behind proj-a and rho-neg at w = 0.1 and 1 | **held** |

**Why 0.64× and not 1/3.**
- Seed 1093 has 46 columns against 25, so a solve needs about 1.84× more
  relations.
- Each relation costs 3.1× less, so the net gain is about 1.7×.
- End-to-end F4 cost ∝ columns × r / coverage, so a model on that quantity
  predicts 0.599. Measured: 0.641.
- The coverage-only selector optimised the wrong objective; see the next
  lead.

### The seed search, charged

ecbench does not see the seed search. The K = 2000 search at k = 20, t = 6
cost 1.35·10⁷ group operations: tower builds plus the [r]P selector. That
adds 3.7·10⁴ to S.

| | S (w=0) | S (w=0.1) | S (w=1) |
|:--|--:|--:|--:|
| seed 1093, search charged to one target | 3.73·10⁴ | 1.110·10⁶ (0.663× seed 0) | 1.077·10⁷ (0.643×) |
| seed 1093, search amortised (per-curve precomputation) | 26.448 | 1.073·10⁶ | 1.073·10⁷ |

- Charged to a single target, the search loses at w = 0 and still wins at
  w = 0.1 and 1, so the band order is unresolved.
- Amortised over targets on the same curve, the search is ordered better at
  every weight.

## Distance remaining

| | to proj-a (generic baseline) | to rho-neg |
|:--|:--|:--|
| w = 0 | 2.19× | 18.3× |
| w = 0.1 | 8.3·10⁴× | 6.9·10⁵× |
| w = 1 | 5.2·10⁵× | 4.4·10⁶× |

Rho remains ahead on every track. The algebraic track moved from 9.6·10⁵×
(w = 0.1, previous session) to 6.9·10⁵× here. Those come from different
target sets.

## Next lead (T9, not preregistered here)

- Select the seed by **J ÷ columns** (log-free), not by J.
- From the census at k = 20, t = 6 this picks seed 1308: coverage 498 on
  36 columns. The coverage-per-column model predicts an end-to-end F4 ratio
  of **0.497×** against seed 0 (0.599× predicted for seed 1093, 0.641×
  measured).
- This is a prediction to freeze, not a result.

## Deviations

- The spec uses `order: alternate` with 1 measured round; ecbench warns that
  this cannot be balanced. S is a counted unit, so arm order does not change it.
- No A/A arm in this session. The earlier session's A/A copy of
  `pkm-iso-t6-g4` was identical to the arm.
- Measurement quality is 0/100 for wall time (L0); every claim here is in
  counted units.
- Conductor and the AWS CLI are absent: no S3 checkpoint and no EKS route.
- Replay audit (`ecbench/verify.json`): 48 records, 48 verified, 8 nonce-selected replays reproduced exactly, 0 problems.
