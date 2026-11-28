# F3 — Pencil-sieve index calculus end to end at genus 2, every phase charged (frozen protocol, 2026-11-17)

Status of everything this follow-up produces: `[pcc:unverified]`. One producer,
shared 4-core host, no independent replay, no signed pcc claim, no §5a matched-rho
band run under isolation. **This is not a benchmark**: no leaderboard row, no
`benchmarks/` row, no speed claim, no wall-time metric. Synthetic random curves
only. Lab numbers quoted below are `[pcc:unverified]` assertions from the files
named. Follow-up of lead `L01_anchored_pencil_sieve` (2026-11-16); the single most
valuable next step named by L01's assessor (ASSESSMENT.md §"Single most valuable
next step").

Written-order note: this file, `SPEC.md` and `PILOT.md` were written and hashed
(`frozen.sha256`) before any decision datum exists. `PILOT.md` records checks run
only on gate quantities and instrument sanity (never the decision metric on the
decision instances); its numbers are fixtures that set gate reachability, not data.

## 0. Prior art and inheritance

| source | what it gives F3 | consequence |
|---|---|---|
| `g2plus_leads_20261116/L01_anchored_pencil_sieve/` (PROTOCOL.md, SPEC.md, RESULTS.md, ASSESSMENT.md, `l01/`) | The anchored pencil sieve at g=2: relations 3–5x cheaper per relation than an explicit-formula Cantor walk `[pcc:unverified]`, **relation phase only**. L01 did NOT charge target descent, table set-up on both arms, or the counted linear solve; and L01's H4 showed pencil rows cost 16–465x more to eliminate in a counted sparse solve. | F3 charges ALL phases end to end and lets the linear algebra decide. The pencil genus-2 Cantor arithmetic, #J, sieve, walk engine and dense mod-r solver in `l01/` are reused (copied, provenance in `pilot/COPIED_FROM_L01.sha256`); the Rust decision instrument re-implements them to reach larger p. |
| `crypto-c0b657b/research/genus_ladder_20261114/RESULTS.md`, `genus_ladder_scale_20261115/PROTOCOL.md` | Lab instrument defines the FPM unit `cantor_mul + 10·inv + rel_mul + g²·la_mul`; at g=2 the best IC arm is 2.8–5x **above** rho and "the cost is the relation walk"; rho exponent in p is 0.77–0.97, IC full-base 1.45, IC-2LP 0.50 `[pcc:unverified]`. Lab rho at g=2 p=1031: 9.74e4 FPM; best IC 2.74e5. | Defines arm W (the walk, lab-style) and arm R (negation rho), and the unit. F3 prices the mod-r (`la`) multiplications at the three band prices the brief fixes (below), of which the lab's `g²=4` is one point above the top band. |

## 1. Mathematics (checked; corrections to the brief)

Curve `C: y² = f(x)`, `f` monic squarefree of degree 5 over `F_p`, one point at
infinity. Genus `g = 2`. The pencil, point-to-λ map, fibre disjointness, the
`1/(g+1)! = 1/6` split fraction and the weight-`2g+1 = 5` homogeneous relation are
exactly as derived and checked in L01 PROTOCOL §1 (re-checked here; correct).

**Correction to the brief's #J method.** The brief says "compute #J from point
counts over F_p and F_p^2". Counting `#C(F_{p²})` by enumeration is `O(p²)` per
curve (L01's `instance.N2`): at `p = 2^16` that is ~4.3e9 field operations per
candidate curve, and admission needs many candidates — infeasible in budget. F3
**replaces** it with the standard cheap genus-2 order computation, which it also
verifies:
- `a1 = p + 1 − N1`, where `N1 = #C(F_p)` counted by an `O(p)` scan (one `chi`
  lookup per `x`).
- `#J = base + a2` with `base = p² + 1 − a1(p+1)` and the single unknown `a2`.
- `a2` lies in a band of width `≤ 4p` fixed by `a1`: writing `S = a1/(2√p)`,
  `a2 ∈ [2p(2|S|−1), 2p + pS²]` (derivation in SPEC §2.3). `#J = base + a2` is found
  by baby-step/giant-step on a random divisor over that band (`≤ 4√p` group
  operations), then **verified** by `[#J]·D = IDENT` on `confirm = 8` random
  divisors and `#J` inside the Weil interval `[(√p−1)⁴, (√p+1)⁴]`.
- Cross-checked against the `O(p²)` F_{p²} formula (`jac_order ∘ point_counts`) at
  `p ≤ 101` in a unit test (gate G-order). PILOT.md: 0 mismatches on 8 tiny primes;
  the cheap method returns a verified `#J` at `p = 8191` in 459 group operations.

The #J/order computation is common to all three arms (each needs the prime order
`r` and a subgroup generator `G`); it is charged **identically** to every arm
(§5), so it cancels in `P/W` and is carried in `P/R`, `W/R` (rho pays it too).

Verdict of the check: sound, not void, not a duplicate of executed work (L01 was
relation-phase only; no end-to-end pencil solve against a walk solve and matched
rho exists). `proceed = true`.

## 2. Hypothesis and exact claim

The question is **only** whether the pencil arm beats the walk arm end to end at
genus 2, and by how much, with every phase of both charged and the linear algebra
priced across a band. Genus-2 full-base index calculus is expected to lose to rho
asymptotically (`p²` linear algebra against `p` for rho); `P/R` and `W/R` are
reported as distance to rho, never as a pass/fail gate.

Let the total cost of an arm at linear-algebra price `w` be
`Cost_w = (Σ field multiplications + 10·Σ inversions over every charged phase) + w·(Σ multiplications mod r in the linear solve)`,
with `w ∈ {0, 0.1, 1}` FPM per mod-r multiplication (the brief's three prices).
Rho has no linear solve, so `Cost_w(R)` is the same for all `w`. All three arms
recover the planted logarithm `t` and verify `[t]G = T`; a run that does not verify
is a failure, not a data point.

- **H1 (primary).** On random genus-2 curves of near-prime Jacobian order over
  `F_p`, at the largest prime `p*` for which all three arms complete and verify on
  all 3 curves × 3 seeds, the pencil arm (P) costs at most `0.67×` the walk arm (W):
  `median_pairs P/W ≤ 0.67` at each of `w ∈ {0, 0.1, 1}`, with the paired 95%
  bootstrap interval excluding 1 at each `w`.
- **H2 (secondary, reported, no threshold).** The exponent of each arm's total
  `Cost_{0.1}` fitted in `log p` over the primes where all arms complete, with its
  95% interval; and the ratios `P/R`, `W/R` at `p*` and across p (distance to rho).
- **H3 (secondary, reported).** The linear-algebra share `w·la_mul / Cost_w` of
  each IC arm at `w = 1`, as p grows (does the pencil's dense-anchor linear algebra
  swamp its cheaper relation phase, as L01's H4 predicts).

## 3. Arms (each charged for its own set-up, lesson c)

Standalone Rust crate inside this directory (own empty `[workspace]`, `.gitignore`
= `target/`); 1 thread per run. **Unit:** `Cost_w` above. Every inversion anywhere
is one counted inversion. Squarings are counted as multiplications. The base
(field mul + 10 inv) and the mod-r `la_mul` are tallied separately per phase so
`Cost_w` is recomputable at any `w`.

Common to all arms (charged identically to each): instance set-up = curve search to
a near-prime admitted curve, the cheap `#J`/order computation (§1), picking `G` of
order `r`, planting `t`, forming `T = [t]G`.

- **P — pencil-sieve index calculus (candidate).** Per-arm set-up it alone pays:
  the y-per-x table (`sqrt`/`chi` tables and `f(x)` for all `x`, the base
  enumeration). Relation phase: anchored pencil sieve exactly as L01 arm `P_full`
  (disjoint anchor pairs from a seeded permutation of the base; forward-difference
  values of `a0, Π`; one batch inversion per tuple; one multiplication per point;
  buckets by λ; every λ with exactly 3 hits is a homogeneous weight-5 relation).
  **Descent of G and T** (the phase L01 omitted): the homogeneous pencil relations
  determine the base logs only up to one global scale, so G and T must be expressed
  over the base. Design: a Gaudry split-walk — draw random `(α, β)`, reduce
  `R = [α]G + [β]T`, apply the split test; on a full split over covered base
  columns, record the inhomogeneous relation `Σ ±x_j − (−hβ)t = hα` (SPEC §6.4).
  The descent walk is the expensive, honestly-charged part of arm P. Linear solve:
  counted sparse elimination mod r over the covered base columns + the `t` column,
  on the pencil's weight-5 rows with their 2-per-tuple dense anchor columns (§7),
  recovering all determined base logs and `t`.
- **W — Cantor-walk index calculus, lab-style (baseline).** Per-arm set-up it alone
  pays: the base enumeration + `disc_character`/`roots_of` tables. Gaudry r-adding
  walk with 16 jumps, one generic counted Cantor addition per trial, restart every
  `min(4096, max(16, ⌊√r/2⌋))` steps (lab amendment 1) by two scalar
  multiplications, `disc_character` parity prefilter, `roots_of` (Cantor–Zassenhaus),
  `v` at the roots; each walk relation `[α]G + [β]T = Σ ±P_j` tracks `(α, β)` and is
  the inhomogeneous row `Σ ±x_j − (−hβ)t = hα`. **Double large prime** is included
  as arm **W2** *only if* it can be made sound within budget (base
  `nb = ⌈2 p^{1−1/g}⌉`, ≤ 2 large primes, cycles on the large-prime graph as L01
  arm `B_2lp`; it must still recover and verify `t` through its own graph-to-relation
  reconstruction and solve). If W2 cannot be made to verify end to end within
  budget it is dropped and the primary comparison is P vs W (full base); this choice
  is recorded, not silently taken. Linear solve: the same counted sparse
  elimination mod r (§7) on the walk's weight-≤3 rows.
- **R — Pollard rho with the negation map (distance to rho).** Standalone r-adding
  negation-map rho in the prime-order subgroup `⟨G⟩`: 32 additive steps, canonical
  representative `min(key_of(X), key_of(−X))`, distinguished points (`dp_bits` low
  bits of the key zero), fruitless-cycle detection over a rolling window with a
  deterministic minimal-key doubling escape (SPEC §8; validated in PILOT). On a
  distinguished-point collision with `b1 ≠ b2`, `t = (a2−a1)/(b1−b2) mod r`, then
  verify `[t]G = T`. Cost = field mul + 10 inv over every Cantor operation (the
  "same Cantor FPM cost used in arm W"); no linear-algebra term. A run that fails to
  find a verifying collision within the hard step cap `60·√(πr/2)` is **retried**
  with a fresh walk seed (the retry is charged); after `K = 4` failed retries the
  instance is a failure, not a data point.

## 4. Instances, seeds, variables held fixed

- **Genus:** 2 only (the brief's "end to end at genus 2"; full-base index calculus).
- **Primes (explicit):** the smallest prime ≥ each power of two from 2^7:
  `p ∈ {127, 251, 509, 1021, 2039, 4093, 8191, 16381, 32749, 65521}`. The executor
  runs the ladder upward and records `exhausted` for any (arm, curve, seed) that
  exceeds the per-run op cap (SPEC §9.5) or the 2 GB guard; nothing is rerun with
  other parameters. `p*` = the largest prime at which **all three arms** complete
  and verify on **all 3 curves × 3 seeds**.
- **Curves:** `f = random_f(p, 2, s)` (lab SplitMix64 rule, SPEC §3.1), `s = 1, 2, …`,
  **admitted** if `f` squarefree and `#J = h·r` with `r` prime, `h ≤ 8`, `r > h`.
  Curve index 1, 2, 3 = the first three admitted seeds (at most 600 seeds tried;
  if fewer than 3 admit, the prime is dropped and reported).
- **Seeds:** 3 solver seeds `σ = 1, 2, 3` per curve. All randomness is derived
  SplitMix64 streams keyed by `(arm family, p, curve seed, σ)` (SPEC §3.2); P, W and
  R use the same curve, `G`, `T`, `t` and base, and independent streams. Pairing for
  the paired statistic is by `(curve index, σ)` → 9 pairs per prime.
- **Held fixed across arms at each (p, curve, σ):** field, curve, `#J`, `r`, `h`,
  `G`, `T`, planted `t`, the base columns, the dedup rule (canonical up to
  negation, SPEC §5.3), the price `I = 10`. The relation-count target is the lab
  margin `covered + 8 + ⌊covered/100⌋` (SPEC §6.5).
- **Charged to the arm that uses it only:** y-table (P), base + split tables (P, W),
  walk jump/restart table (W), rho step table (R); instance+order set-up to all
  three identically.

## 5. Metrics and unit

Per run (arm, p, curve, σ): the base tally split by phase — `setup_mul/inv`
(instance+order, common), `arm_setup_mul/inv` (this arm's own tables),
`rel_mul/inv` (relation search), `descent_mul/inv` (P and W target descent),
`la_mul/la_inv` (the mod-r solve) — plus relations, duplicates, covered columns,
homogeneous rank, `t_planted`, `t_recovered`, `verify` (bool), retries, status, and
a SHA-256 digest of the canonical relation rows. Rho also: steps, distinguished
points, collisions, fruitless-cycle escapes, group operations.

Derived (SPEC §10): `Cost_w(arm) = base_fpm + w·la_mul` for `w ∈ {0, 0.1, 1}`, with
`base_fpm = (Σ all *_mul) + 10·(Σ all *_inv)` and `la_mul` counted separately. Rho:
`base_fpm` only. Readings: `P/W`, `P/R`, `W/R` per (p, curve, σ) pair at each `w`;
cell statistic = **median over the 9 pairs** with a paired bootstrap 95% interval
(resample curves then σ, SPEC §10.3). Exponent fits: OLS of `log Cost_{0.1}` on
`log p` over the primes where the arm completes on ≥ 2 of 3 curves, 95% interval by
the same bootstrap.

**Unit statement (brief, verbatim intent).** The lab instrument defines FPM as
`cantor_mul + 10·inv + rel_mul + g²·la_mul`; F3 uses the same field-mul+10·inv base
over every phase and prices the mod-r `la` multiplications at the brief's three
prices `w ∈ {0, 0.1, 1}` instead of fixing `g² = 4`. The lab's `g² = 4` reading is
reported as a fourth point for continuity, labelled as the lab's, not a decision
price.

## 6. Correctness gates (all must pass before any metric is read; all shown reachable in PILOT.md)

- **G1 unit tests** (SPEC §11) pass, including hand values: `mix64`, `random_f`,
  the cheap `#J` vs the F_{p²} formula at `p ≤ 101` (G-order), the p=13 pencil
  example, exact op counts of the batch inversion / interpolation / `disc_character`,
  and the mod-r counter on a 3×3 hand solve.
- **G2 (verify, the central gate).** Every P, W and R run recovers the planted `t`
  **and** verifies `[t]G = T`; and the three arms recover the **same** `t` at each
  (p, curve, σ) (cross-arm agreement). Any non-verifying run is a failure for that
  arm (not a data point); any cross-arm disagreement voids the instance.
- **G3 (relation soundness).** Every pencil relation passes the exact polynomial
  identity `f − a_λ² = Π(x − x_k)` over its 5 points (uncounted); every walk and
  descent relation passes an independent Cantor-sum check (all at `p ≤ 4093`, 200
  sampled per run above). Any failure voids the run.
- **G4 (rank, lesson b).** The **stop rule is a rank condition**: tuples/relations
  are collected until the homogeneous pencil system over the covered columns has
  corank exactly 1 (pencil) / the inhomogeneous walk system has rank determining
  `t` (walk), checked by the elimination **before** `t` is read; `t` is accepted
  only if its column is uniquely determined. A rank-deficient solve adds one more
  tuple/relation batch (charged) up to the cap, else the instance fails.
- **G5 (dedup, lesson b).** Each relation is counted once; the dedup key is the
  canonical point multiset up to negation (SPEC §5.3). Duplicates are counted and
  reported, never admitted to the matrix or the denominator.
- **G6 pencil split rate** (rate gate, **calibrated at the study prime**, lesson a):
  pencil relations per tuple / `(p/6)`, pooled per cell, in `[0.85, 1.15]`. PILOT at
  `p = 8191`: 0.976.
- **G7 walk split rate** (rate gate, at the study prime): walk candidate split
  fraction / `(C(n,2)·4/#J)` in `[0.85, 1.15]`. PILOT at `p = 8191`: 1.011.
- **G8 rho sanity** (rate gate, at the study prime): all rho runs verify, and the
  mean over seeds of `steps/√(πr/2)` is in `[0.3, 3.0]` (calibrated: PILOT at
  `p = 8191`, `r ≈ 2^23`, mean 1.295, range 0.77–1.66, 4/4 verify).
- **G9 A/A determinism.** A rerun of `(p = 2039, curve 1, σ = 1)` for all three arms
  reproduces every integer counter and every rows digest exactly.

A failed gate voids the affected runs; the verdict for any cell it touches is
"inconclusive (void)" and the failure is reported verbatim. No threshold moves.

## 7. Linear solve (the phase that decides, charged identically to P and W)

One fixed counted solver mod r, used identically for both IC arms (SPEC §7): a
structured Gaussian elimination with Markowitz pivoting on the sparse rows, falling
to a dense Gauss–Jordan on the residual core when the active matrix is below
`CORE = 256` columns or its density exceeds `0.5`; every multiplication mod r is one
counted `la_mul`, every inversion mod r one `la_inv` (priced at `I = 10` in the base
like any inversion). It recovers the full determined log vector and `t` and the run
then verifies `[t]G = T` (G2). The pencil's 2-per-tuple dense anchor columns and
weight-5 rows versus the walk's weight-≤3 rows make this the phase where the pencil
can lose; that is the measurement.

## 8. Controls

- **Positive control** (known-sign effect): the naive full-λ scan over all `p`
  values of λ on the first pencil tuple produces exactly that tuple's relation set
  (G4-style identity) at ≥ 5× the sieve's cost per relation, in every cell (L01's PC,
  reused). Second positive control: the three arms recover the same `t` (G2).
- **A/A** (G9, determinism) plus the between-σ spread of `base_fpm` per arm and cell,
  reported as the noise floor (max/min over the 9 runs).
- **Negative / null:** the split-rate models (G6, G7) are observed-vs-model nulls
  that must sit at ratio 1; a deviation is an instrument alarm, not a result.

## 9. Decision rule (fixed now; numeric thresholds)

Let `p*` be the largest prime at which all three arms complete and verify on all
3 curves × 3 seeds (G2 holds for the whole cell), and let `M_w = median_{9 pairs}
(P/W)` at `p*` with its paired 95% bootstrap interval `[L_w, U_w]`.

- **Supported**: all gates G1–G9 pass, and `M_w ≤ 0.67` **and** `U_w < 1` at **each**
  of `w ∈ {0, 0.1, 1}`, **and** at least two primes have all arms complete (so an
  exponent fit exists).
- **Killed**: all gates pass and `M_{0.1} ≥ 1.0` at `p*` (the pencil is not cheaper
  end to end at the middle price).
- **Inconclusive**: anything else — including any gate or control failure, any arm
  failing to verify on the `p*` cell, `p*` existing at only one prime (no fit), or
  `0.67 < M_w < 1.0` without the interval excluding 1 at some `w`.

`P/R` and `W/R` are reported at `p*` and as exponent fits (distance to rho); they
are **never** a pass/fail gate. Expectation recorded in advance (not a rule): both
IC arms are at or above rho at g=2 end to end (`p²` LA vs `p` rho); the question is
only P vs W.

Predictions (not decision rules): P relation-phase `base_fpm` per relation ≈ 15
(L01); W per relation ≈ 130; pencil `la_mul` per column far above walk's (L01 H4);
`P/W` at `w = 0` well below 1 (cheap relations dominate) and rising toward or past 1
as `w → 1` (dense-anchor LA dominates) — H3.

## 10. Confounders and isolation

| confounder | isolation |
|---|---|
| relation usefulness / row weight differs (pencil weight 5 + dense anchors vs walk ≤ 3) | charged by the shared counted solver §7; `la_mul`, row/column-weight stats reported; this is the measured effect, not hidden |
| one arm needs a table the other does not | per-arm set-up charged to the arm that uses it (§3, lesson c); instance+order common, charged to all three |
| descent omitted (L01's gap) | arm P pays an explicit Gaudry split-walk descent of G and T (§3, §6.4); arm W's descent is folded in its walk rows |
| linear-algebra price unknown | priced across the whole band `w ∈ {0, 0.1, 1}`; decision requires all three |
| rho mispriced | rho group ops in the same field-mul+10·inv unit as W's Cantor steps; rho reported only as distance |
| per-curve split-rate variation (lab finding 4) | same curve/`G`/`T` for all arms; pairing by (curve, σ); 3 curves |
| seed luck | 3 σ; A/A rerun; paired bootstrap; spread reported |
| fruitless-cycle traps in rho (found in PILOT) | window cycle detection + deterministic min-key doubling escape (§8/SPEC §8); G8 checks all verify; retries charged |
| counter bugs | G1 hand counts incl. the mod-r counter; per-tuple count-formula check (SPEC §11) |
| rank deficiency / double counting | rank-based stop rule + explicit corank check before `t` is read (G4); dedup by canonical multiset (G5) |
| #J wrong | cheap method verified by annihilation + Weil band + F_{p²} cross-check at p ≤ 101 (G-order) |
| analyst choices | thresholds and the analysis program fixed here and in SPEC §10 |
| host load / time | counts only; CPU seconds recorded as a budget guard (diagnostic), never a metric |

## 11. Exact commands (executor, from this directory, one thread per run)

```sh
cd /home/user/crypto/research/g2plus_followups_20261117/F3_L01_pencil_end_to_end
# 0. freeze check
sha256sum -c frozen.sha256
# 1. build the standalone crate and run its unit tests (G1)
( cd rust && cargo test --release 2>&1 | tee ../raw/unittest.log )
( cd rust && cargo build --release 2>&1 | tee -a ../raw/build.log )
BIN=rust/target/release/f3
# 2. resume checkpoint BEFORE any run (AGENTS.md), then the grid
#    (the launcher is expected to refuse until .resume/RESUME.md is pushed)
for p in 127 251 509 1021 2039 4093 8191 16381 32749 65521; do
  for c in 1 2 3; do for s in 1 2 3; do
    $BIN run --p $p --curve-index $c --seed $s --arms P,W,R \
        --la-prices 0,0.1,1 --out raw/run_p${p}_c${c}_s${s}.jsonl
  done; done
done
# 3. A/A determinism rerun (G9)
$BIN run --p 2039 --curve-index 1 --seed 1 --arms P,W,R --la-prices 0,0.1,1 \
    --out raw/aa_p2039_c1_s1.jsonl
# 4. analysis: medians, paired bootstrap, exponent fits, decision rule
python3 -I analyze.py --raw raw --out-json raw/analysis.json --out-md raw/summary.md
sha256sum raw/*.jsonl raw/analysis.json raw/summary.md > raw/data.sha256
```

Each `run` has a CPU-time guard and a hard op cap (SPEC §9.5); a run that hits
either writes `status: "exhausted"` with its counts so far and is a missing cell
(reported, not rerun with other parameters). Per-run memory is expected under 2 GB
(largest objects: the base of ~`p/2` columns and the sparse matrix; the pencil's
fill is the risk at the top primes and is the measured effect). Output files are
kept under 50 KB each (per-run summary rows; no per-relation dumps).

## 12. Limits

Toy primes (≤ 2^16). Genus 2, full base, one host, counts not wall time, Python/Rust
counters, `I = 10` a convention. Even a supported H1 says nothing about beating rho:
genus-2 full-base IC is `p²`-LA-dominated and the lab's arms are at or above rho
`[pcc:unverified]`; F3 only asks whether the pencil beats the walk end to end, and
by how much, at the three LA prices. Everything `[pcc:unverified]`.
