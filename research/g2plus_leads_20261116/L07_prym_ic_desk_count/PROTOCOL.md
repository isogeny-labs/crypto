# L07 — Prym index calculus for genus 4-5: desk count

Pass: g2plus_leads_20261116. Lead directory:
`research/g2plus_leads_20261116/L07_prym_ic_desk_count`.
Status of every statement below: `[pcc:unverified]`. Lab numbers quoted from
other work are themselves `[pcc:unverified]` assertions. This is a desk study,
not a benchmark: no leaderboard row, no `benchmarks/` row, no speed claim.

Classification: long shot, expected null. The frozen decision rule is below;
the predicted outcome is **killed**, and that prediction is recorded here before
the sanity-check data exists so it cannot be moved afterward.

## 0. Prior art read (step 1)

Paths read, and why none duplicates this desk count:

- `research/ghs_n7_prym_trace_20261115/` (REPORT.md, THEOREM.md,
  HYPERELLIPTIC_CLASSIFICATION.md). Prym appears only as the 7-dimensional
  *complementary* Prym of a GHS W1 cover over `F_128` (Hess, Achter–
  Casalaina-Martin); it is a trace-class / branch-support screen for a
  genus-7 transfer target, not an index-calculus cost count for a
  general-dimension Prym. Different object, different question.
- `research/ghs_n7_prym_transfer_20261115/`, `ghs_n7_base_change_20261115/` —
  same GHS family; transfer-existence, not IC exponent.
- `research/cover_pushforward_20261112/PUSHFORWARD.md` — measured degree-2
  **bielliptic** covers `J(C) ~ E1 x E2`, one curve per prime, `p <= 12841`.
  Finding: the Prym side (`E2`) is annihilated by the pushforward and "nothing
  computed on it can see the target"; split-Prym, dimension 1, weight-2
  relations only. Explicitly disclaims higher-degree and **non-split** Pryms.
  Does not count the genus-4/5 étale-double-cover Prym exponent. Not a dup.
- `research/lead_sweep_20261110/LEAD-ICP-prym-witness-relation-diversity/`
  (RESULT.json, PROTOCOL.md). A toy relation-diversity sweep that used **no
  Prym geometry**; verdict `falsified`, and the atlas
  (`research/prime_ic_lead_heatmap_20261114/`) records it `open_blocked`
  because no mechanism to attach a prescribed target `Q` exists and Lemma T
  caps the toy route. Not this count.
- `research/chained_prime_transfer_20261111/LEADS_20261112.md` (Prym proof-note
  item): states `J_C ~ E x P`, `c_* c^* = [deg c]`, "a smooth-order Prym
  simplifies only the complement" — a split degree-2 statement, consistent
  with what we derive, not a dimension-g count.
- `research/genus_ladder_20261114/` (PROTOCOL.md, RESULTS.md, tables.md) and
  `src/cryptanalysis/genus_ladder.rs` — the measured double-large-prime (2LP)
  baseline. `fb_size_2lp(p,g,c) = ceil(c * p^(1-1/g))`; the asymptotic 2LP
  exponent is `2 - 2/g` (Gaudry–Thomé–Thériault–Diem). This is the baseline
  L07 must beat.
- `src/cryptanalysis/{hyperelliptic_index_calculus.rs, bielliptic_quartic.rs,
  diem_quartic.rs, plane_quartic_pic.rs, quartic_ladder.rs}` — cost
  conventions and the FPM unit.
- `cryptanalysis/experiments/{bielliptic-quartic, hyperelliptic-cover-
  infrastructure}/README.md` — no general-Prym IC count.

Conclusion of prior-art check: the general (non-split) Prym-of-étale-double-
cover index-calculus exponent count for Prym dimension 4 and 5 is **not**
present as executed work. `proceed = true`.

## 1. Geometry check of the brief (step 2)

The brief's geometry is checked and found **correct**; restated precisely.

Let `pi: D -> C'` be an unramified (étale) connected double cover, `sigma` its
free deck involution, `genus(C') = h`. Riemann–Hurwitz for an unramified degree-2
cover:

```
2 genus(D) - 2 = 2 (2h - 2)   =>   genus(D) = 2h - 1.
```

The Prym variety `P = Prym(D/C') = (ker(Nm: Jac(D) -> Jac(C')))^0` is a
principally polarized abelian variety of dimension

```
dim P = genus(D) - genus(C') = (2h - 1) - h = h - 1.
```

Set `g = dim P`. Then `h = g + 1` and `genus(D) = 2h - 1 = 2g + 1`. This
reproduces the brief exactly: Prym dimension `g`, base curve `C'` genus `g+1`,
cover `D` genus `2g+1`.

Dominance range (why "g <= 5"): the Prym map `P_h : R_h -> A_{h-1}` from étale
double covers of genus-`h` curves to ppav of dimension `h-1` is dominant for
`h <= 6` (Prym–Torelli; Beauville, Donagi–Smith for `h=6`). With `h = g+1`,
dominance holds for `g <= 5`. So a *generic* ppav of dimension `g in {4,5}` is a
Prym, which is why the lead targets exactly these dimensions. For `g = 6`
(`h = 7`) the Prym map is no longer dominant (generically finite onto a proper
subvariety), so the construction does not generically exist — the brief's
`g <= 5` cutoff is correct.

Correction / sharpening of one implicit point in the brief: the brief says a
ppav "isogenous to a hyperelliptic Jacobian is generally not a Jacobian." For
the cost comparison it does not matter whether the *target* is a hyperelliptic
Jacobian; what matters is that to run IC we must exhibit `D` and sieve on it.
The sieving curve is `D`, genus `2g+1`, regardless of the target's own genus.
This is the load-bearing fact for the exponent and is stated as H1 below.

## 2. Hypothesis and exact claim

**Factor base.** Anti-invariant degree-1 classes
`B = { [P - sigma(P)] : P in D(F_p) }`. Each such class lies in `P(F_p)`.
`|B| <= #D(F_p) ~ p + O(sqrt p)`, and `[P-sigma P] = -[sigma P - P]`, so the
distinct unordered base has size `~ p/2` (the fixed-point-free `sigma` pairs
points). This is the "half the group" structure already seen in the degree-2
bielliptic measurement.

**Relation family.** A relation is a principal anti-invariant divisor on `D`
supported on `B`, i.e. `(f) = sum_i e_i (P_i - sigma P_i)` for a rational
function `f` on `D` with `sigma^*(f) = c/f` (anti-invariant up to constants).
The candidate reduced divisors whose smoothness must be tested are reduced
divisors **on `D`**, of generic degree `genus(D) = 2g+1`.

**Exact claim (H1).** Because relation candidates are reduced divisors on the
genus-`(2g+1)` curve `D`, their complete-splitting probability over degree-1
places is governed by degree `2g+1`, giving:

```
full-base Prym IC exponent   :  2            (base ~ p, LA ~ p^2)
2LP Prym IC exponent         :  2 - 2/(2g+1)
```

not `2 - 2/g`. Concretely:

| g | h=g+1 | genus(D)=2g+1 | Prym 2LP  2-2/(2g+1) | hyperell. g 2LP  2-2/g | rho on Prym  g/2 |
|--:|------:|--------------:|---------------------:|-----------------------:|-----------------:|
| 4 |     5 |             9 |  2 - 2/9  = 1.7778   |   2 - 2/4 = 1.5000     |        2.0       |
| 5 |     6 |            11 |  2 - 2/11 = 1.8182   |   2 - 2/5 = 1.6000     |        2.5       |

Canonical-hyperplane (Diem plane-curve) IC on a non-hyperelliptic curve of the
cover's genus `2g+1` has the same leading exponent class `2 - 2/(2g+1)` and so
does not change the comparison.

**Decision-relevant inequality.** The Prym 2LP exponent `2 - 2/(2g+1)` exceeds
the hyperelliptic genus-`g` 2LP exponent `2 - 2/g` for every `g >= 1`, since
`2g+1 > g`. The gap is `2/g - 2/(2g+1) = (2g+2)/(g(2g+1)) > 0`.

Therefore the Prym construction is predicted to be **slower** than doing 2LP IC
directly on a hyperelliptic genus-`g` model, and only faster than rho on the
Prym (`g/2 = 2.0, 2.5` vs `1.78, 1.82`), which is not the bar.

## 3. Cost of constructing the cover (step (4) of the brief)

To run Prym IC on a target ppav `A` of dimension `g in {4,5}` one must first
exhibit an étale double cover `D -> C'` with `Prym(D/C') ~ A`, i.e. invert the
Prym map. This is Prym–Torelli inversion / a Schottky-type problem. It is:

- generically finitely many fibres for `g <= 5` (dominant), but with no known
  polynomial-time algorithm to produce `(D, sigma)` with rational equations
  over `F_p` from `A`;
- for `g = 5` (`h = 6`) at the very boundary of dominance (Donagi–Smith degree
  27 cover of the generic fibre), so even existence of a rational model is
  delicate;
- a strict one-time cost that, even if free, cannot lower the `2 - 2/(2g+1)`
  relation-phase exponent above.

This is recorded as a confounder-killer: even granting cover construction at
cost 0, H1 already loses to hyperelliptic 2LP.

## 4. Arms, controls, variables fixed

The numeric sanity check (optional in the brief; included to make H1's
splitting input checkable) measures only the **complete-splitting law**, which
is the one empirical quantity the exponent derivation rests on. It is not a
full IC run.

- **Baseline arm (B-GENUS):** empirical complete-splitting fraction of random
  effective degree-`d` divisors over a degree-1 factor base of a genus-`d`
  model, for `d = g` (= the hyperelliptic genus-g sieving degree). Expected
  `~ 1/g!`.
- **Prym arm (P-GENUS):** same instrument with `d = 2g+1` (= the Prym sieving
  degree on `D`). Expected `~ 1/(2g+1)!`. The ratio
  `splitfrac(d=g) / splitfrac(d=2g+1)` is the per-trial penalty that produces
  the exponent gap.
- **Positive control (POS):** `d = 1` and `d = 2` where the complete-splitting
  fraction is exactly computable by hand (`d=1` -> every divisor splits, frac
  = 1; `d=2` -> frac = fraction of degree-2 effective divisors that are a sum
  of two rational places), proving the counter can see and report the law.
- **Negative / A-A control (AA):** two independent seed streams of the **same**
  `d`; their measured fractions must agree within sampling error (no arm
  label effect). Gap near 0 expected.
- **Symbolic arm (TABLE):** evaluates the genus and exponent formulas of §1–§2
  for `g = 1..6`; no randomness. Confirms Riemann–Hurwitz, Prym dimension,
  dominance cutoff at `g=5`, and the three exponents. Hand-checkable.

Variables held fixed across arms: the field `F_p`, the RNG and its seed
schedule, the degree-1 place count `n` used as the factor base, the divisor
sampler, the "complete split" predicate, and the trial count `T`. Only the
target degree `d` (and, for AA, the seed stream) varies.

## 5. Instances

- Primes (small, per hard rules): `p in {101, 1009, 10007}`.
- Factor base size `n = p + 1` modelled as the number of degree-1 places of a
  curve over `F_p` (we use the abstract place model; see SPEC — no curve
  arithmetic is needed to measure the degree-`d` complete-splitting law, which
  depends only on `n` places among which effective degree-`d` divisors are
  drawn uniformly).
- Degrees `d in {1, 2, 3, 4, 5, 8, 9, 10, 11}` (covers POS, baseline g=4,5,
  and Prym genus(D)=9,11; 3,8,10 included for the 1/d! trend).
- Trials `T = 2_000_000` per (p, d, seed-stream). Two seed streams
  `seed in {20261116, 70261116}` for AA.
- Unit of cost: none charged here (counts only). Where the derivation quotes
  FPM it is defined as `FPM = (field multiplications) + 10*(inversions)`,
  squarings counted as multiplications, matching
  `src/cryptanalysis/genus_ladder.rs`.

## 6. Metrics and unit

Exact integer counts, seeds recorded, never wall time:

- `n_trials`, `n_complete_split` per (p, d, seed) -> `frac = n_complete_split
  / n_trials` as an exact rational reported to 10 decimals.
- `expected_1_over_d_factorial = 1/d!` for reference.
- For TABLE: integer genus/dimension values and the exponents as exact
  fractions and decimals.

## 7. Correctness gates (must pass before any metric is read)

1. TABLE arm reproduces, for `g=1..6`: `h=g+1`, `genus(D)=2g+1`,
   `dim P = g`, dominance flag `true` iff `g<=5`. Any mismatch -> stop.
2. POS: measured `frac` for `d=1` equals exactly `1.0`; for `d=2` equals the
   closed form in SPEC §tests within 3 sigma binomial. Else the sampler/
   predicate is wrong -> stop, fix, re-run; do not read H1.
3. AA: `|frac_streamA - frac_streamB|` for equal `d` is within
   `4 * sqrt(frac(1-frac)/T)` at every (p,d). Else there is an
   arm/seed confound -> stop.
4. Monotonicity sanity: `frac` strictly decreasing in `d` at fixed `p`.
5. Determinism: re-running a (p,d,seed) cell reproduces the exact integer
   counts.

## 8. Decision rule (frozen, numeric)

Primary decision is **analytic** (H1, §2): the lead is **supported** only if
some anti-invariant Prym relation family yields a 2LP (or better) exponent
**strictly below** the hyperelliptic genus-`g` 2LP exponent `2 - 2/g` for
`g = 4` or `g = 5`, with the cover-construction cost not dominating.

- **Killed** if, as derived, every identified Prym relation family has exponent
  `>= 2 - 2/g` (the derivation gives `2 - 2/(2g+1) > 2 - 2/g`). This is the
  kill criterion named in the brief: "no family below the hyperelliptic 2LP
  exponent."
- **Supported** if a concrete family with a written-out relation construction
  achieves exponent `< 2 - 2/g` at `g in {4,5}`. (Not expected.)
- **Inconclusive** if a candidate family cannot be assigned a defensible
  exponent from the degree of its relation divisors.

The numeric sanity check does **not** gate the decision; it only confirms the
splitting-law input `frac(d) ~ 1/d!` and the penalty ratio
`frac(g)/frac(2g+1)` that the derivation uses. If POS/AA/TABLE gates fail, the
numbers are discarded but the analytic kill still stands on §2.

Predicted verdict, recorded pre-data: **killed** at both `g=4` and `g=5`.
A kill is a valid, useful outcome and will be reported plainly in DERIVATION.md.

## 9. Confounders and isolation

- *Instrument can't see the effect* -> POS arm (`d=1,2` closed forms).
- *Seed/label artifact* -> AA arm, interleaved seed streams.
- *Wrong genus of the sieving curve* (the whole crux) -> TABLE arm pins
  `genus(D)=2g+1` by Riemann–Hurwitz; the derivation uses that, not `g`.
- *Free cover construction assumed* -> §3 shows kill holds even at
  construction cost 0.
- *Split vs non-split Prym* -> the degree-2 bielliptic prior
  (`cover_pushforward_20261112`) is the split, dimension-1 case; this lead is
  the general dimension-`g` case, and the exponent `2-2/(2g+1)` covers both.

## 10. Commands the executor runs (outputs under raw/)

Python 3 standard library only, one thread, each run << 10 CPU-min, << 2 GB.

```
cd research/g2plus_leads_20261116/L07_prym_ic_desk_count
python3 -I src/genus_table.py        > raw/genus_table.tsv
python3 -I src/split_law.py          > raw/split_law.jsonl
python3 -I -m unittest -v src/test_l07.py 2> raw/tests.txt
```

Then assemble `DERIVATION.md` from §1–§3 plus the measured
`frac(g)/frac(2g+1)` penalty rows, with the frozen verdict from §8.

All produced statements are `[pcc:unverified]`.
