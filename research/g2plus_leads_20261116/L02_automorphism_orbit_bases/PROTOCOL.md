# L02: automorphism-orbit factor bases on CM hyperelliptic families (frozen protocol, 2026-11-16)

Status: protocol only. No datum of this study existed when this file was written.
Every number this study will produce is `[pcc:unverified]` (one session, shared
4-core container, Python instrument, no independent replay). Lab numbers quoted
here are `[pcc:unverified]` assertions. This is not a benchmark: no leaderboard
row, no `benchmarks/` row, no speed claim. Cost is exact counts, never wall time.

Implementation contract: [`SPEC.md`](SPEC.md). Freeze: `frozen.sha256`
(sha256 of this file and SPEC.md), written before any code or data.

## 0. Prior art checked (grep; the research-kb tool is not installed)

| path | what it shows |
|---|---|
| `research/leads_20261111/EXISTING_LEADS.txt` | `LEAD-ICB-frobenius-orbit-factor-base`, `LEAD-ICB-orbit-linear-algebra`, `LEAD-RHO-automorphism-class-walks`, `LEAD-STR-automorphism-quotient-rho`: lead names only, all elliptic/Koblitz framed |
| `research/automorphism_ideas_20261021/README.md`, `backlog/XP-20261021-automorphism-ideas/{README.md,PROTOCOL.md,card.json}` | DGM-01 "preserve the factor base, compress its logarithm unknowns": the same mechanism, written as a plan; card status `blocked`, "no experiment was run or scheduled" |
| `research/notes/index-calculus/RESEARCH_GLV_INVARIANT_FACTOR_BASES.md` (table near line 69) | executed orbit folds exist only for elliptic curves: Koblitz Frobenius orbits, `j = 0` over `F_p` and `F_{p^3}`, cryptanalysis `prime_orbit_index_calculus.rs` (`Aut(E)` orbits, elliptic) |
| `research/notes/index-calculus/RESEARCH_HYPERELLIPTIC_IC_RHO.md` (round nine, ~line 1355) | hyperelliptic IC in the lab already folds `±P`; generic curves; "the involution is the only cheap automorphism" there |
| `research/prime_rho_automorphism_20261015/README.md` | automorphism-class rho measured on elliptic `j = 0, 1728` curves only |
| `research/genus_ladder_20261114/{PROTOCOL.md,RESULTS.md}`, `src/cryptanalysis/genus_ladder.rs` | lab genus-2/3 hyperelliptic IC (full base, 2LP) and negation rho on random curves; no CM family, no orbit base (grep for `orbit|automorph|zeta` in the five named source files finds no automorphism folding) |
| `../cryptanalysis` (grep `genus.?2.*(automorph|orbit)`) | no genus-2 automorphism IC; only GHS sweeper notes and an elliptic `j = 0` experiment |

Verdict: the mechanism is literature prior art (Duursma, Gaudry and Morain,
ASIACRYPT 1999, cited in the lab's own automorphism plan) and is executed in the
lab for elliptic curves only. A measured genus >= 2 instance (`y^2 = x^5 + a`,
`y^2 = x^7 + a`) has not been run here. Not a duplicate. No novelty is claimed;
this is a measurement of a known mechanism in the lab's counted unit.

## 1. Mathematics, checked and corrected

Let `q in {5, 7}`, `g = (q - 1)/2`, `p` prime with `p = 1 (mod 2q)`, `a in F_p^*`,
`C_a : y^2 = x^q + a` (genus `g`, one point at infinity, imaginary model).
Let `zeta in F_p` have order `q`.

1. `rho(x, y) = (zeta x, y)` and `iota(x, y) = (x, -y)` are `F_p`-rational
   automorphisms; `<rho, iota> = mu_{2q}` (cyclic, order 10 or 14). On Mumford
   pairs, `rho(u, v) = (u', v')` with `u'_i = zeta^(d - i) u_i` (`d = deg u`) and
   `v'_i = zeta^(-i) v_i`. (Check: `u'(x) = zeta^d u(zeta^-1 x)` has root
   `zeta x_0`, and `v'(zeta x_0) = v(x_0)`.)
2. `1 + rho + ... + rho^(q-1) = 0` on `J` (the quotient `C / <rho>` is `P^1`). So if
   `#J = h r` with `r` prime and `r` not dividing `h`, `rho` acts on the order-`r`
   subgroup `<G>` as multiplication by some `lambda` with `Phi_q(lambda) = 0 mod r`,
   `lambda != 1`. Hence `r = 1 (mod q)`, and `mu_{2q}` acts freely on `<G> \ {0}`.
3. Logarithms of factor-base elements are the projected logs
   `l(F) = log_G(c F)`, `c = h * (h^-1 mod r)`. `rho` commutes with `c`, so
   `l(rho^k(+-F)) = +- lambda^k l(F) (mod r)`. Points fixed by `rho` (`x = 0`) have
   `(lambda - 1) l = 0`, so `l = 0`; Weierstrass points are 2-torsion, `l = 0`.
4. **Correction of the brief's group size.** The lab's plain full base already
   identifies `P` with `-P` (one column per abscissa) and the lab's rho reference
   is the negation rho `sqrt(pi r / 4)`. The new symmetry relative to that
   baseline is `<rho>` of order `q`, not `|G| = 2q`. For `x != 0`, `f(zeta x) = f(x)`,
   so the base is a union of `rho`-orbits of size exactly `q` plus possibly
   `x = 0`: `n_B = (n_A - delta0) / q` exactly, `delta0 = [a is a nonzero square]`.
5. **Corrected prediction.** Relations needed fall by `~q` (5 or 7), not 10 or
   14 (same admissible set, same split probability `~1/g!`, `n_B + 10` versus
   `n_A + 10` relations). Linear algebra falls by `q^2` only under a dense or
   Wiedemann-type model `dim * nnz`; under the counted sparse elimination of SPEC
   (rows of weight `<= g`) fill is small and it falls by roughly `q`. Rho with the
   full class falls by `sqrt(q)` relative to the negation rho (expected
   `sqrt(pi r / (4 q))` steps), less a canonicalisation overhead of about
   `2q` field multiplications per step. Net improvement of the IC/rho gap
   `N = (IC_A / R_neg) / (IC_B / R_aut)`: about `q / sqrt(q) = sqrt(q)`, i.e.
   **2.24 at genus 2 and 2.65 at genus 3** when the relation phase dominates,
   up to `q^1.5` (11.2, 18.5) only when linear algebra dominates under a
   quadratic model. The brief's 3.2 (`10 / sqrt 10`) compares against a baseline
   without the negation fold, which the lab does not use. The brief's kill
   threshold 2.0 is kept unchanged; it sits just under the corrected prediction,
   so this is a sharp test.
6. Applying `rho` to a relation produces `lambda`-multiples of the same row: no
   extra rank. The gain is in unknowns, not in free relations.
7. Picard curve `y^3 = x^4 + a` (optional in the brief): excluded. It needs
   plane-curve Picard arithmetic that has no standard-library Python instrument
   here, and does not fit the per-run budget. Recorded as not attempted.
8. Scope limitation: the lab's best genus-2 arm is double large prime
   (`[pcc:unverified]` IC/rho 2.82 at p = 1031, `research/genus_ladder_20261114/RESULTS.md`).
   This study folds the **full base** only. A supported verdict here does not
   show an improvement over the lab's best genus-2 arm; orbit folding of the 2LP
   arm is the follow-up.

## 2. Hypothesis and exact claim

H1 (primary, genus 2): on `y^2 = x^5 + a`, the orbit base (arm B) narrows the
full-base IC gap to the matched automorphism rho by a factor
`N = (IC_A / R_neg) / (IC_B / R_aut) >= 2.0` in FPM.

H2 (secondary, genus 3): the same on `y^2 = x^7 + a`, threshold 2.0.

Exact claim if supported: "On the stated synthetic `y^2 = x^5 + a` Jacobians with
`p` from 2^12 to 2^14, in counted FPM (field multiplications + 10 inversions,
linear algebra modulo `r` weighted `g^2`), the orbit full base reduces full-base
IC cost by `D1` and narrows the gap to matched rho by `N` (95% CI), with gap
`IC_B / R_aut` remaining." Nothing about wall time, deployed curves, or the 2LP arm.

## 3. Instances

All selection is by rule; no hand-picking. Seeds derive from labels via
`seed_of(label) = int.from_bytes(sha256(label).digest()[:8], 'big')` (SPEC 2).

- **F5** (CM, genus 2): bit sizes `k in {10, 11, 12, 13, 14}`. For each `k`, scan
  primes `p = 1 (mod 10)` upward from `2^k` (first: 1031, 2081, 4111, 8221,
  16411). For each `p`, scan `a = gamma^i mod p`, `i = 0..9` (`gamma` the smallest
  primitive root; these represent all 10 twist classes). Admit iff
  `#J = h r`, `r` prime, `r > h`, `h <= 64`. Take the first 3 admitted `(p, a)` in
  scan order; at most 30 primes per `k`.
- **F7** (CM, genus 3): `k in {8, 9, 10}`, primes `p = 1 (mod 14)` from `2^k`
  (first: 281, 547, 1051), `a = gamma^i`, `i = 0..13`, same admission, 3 curves,
  at most 15 primes per `k`.
- **C5 / C7** (non-CM controls, genus 2 / 3): for each admitted F5 / F7 curve, the
  same `p`; `f = x^q + c_{q-1} x^{q-1} + ... + c_0` with `c_i` from
  `Random(seed_of("ctrl:{q}:{p}:{j}"))`, `j = 0, 1, ...`; require `f` squarefree
  and some `c_i != 0` for `1 <= i <= q-1`; same admission rule; first admitted `j`.
- `#J`: genus 2 from `N_1` plus baby-step giant-step over the `a_2` window; genus 3
  from `N_1, N_2` plus baby-step giant-step over the `a_3` window; exactly one
  candidate must annihilate 32 random divisors (SPEC 5).
- Per (curve, solver seed `s`): `G = [h] D` (random `D`, `G != 0`, `[r] G = 0`),
  `T = [t] G`, `t` uniform in `[1, r)`, both from `Random(seed_of("inst:{curve_id}:{s}"))`.
  Instance set-up is charged to no arm.
- Solver seeds: IC `s in {1, 2, 3}`; rho `s in {1..24}` (genus 2) and `{1..12}` (genus 3).
  IC and rho with the same `s` share `G`, `T`.

## 4. Arms

All arms use one instrument: the same `F_p` counters, the same Cantor group law,
the same split test, the same elimination, the same rho walk code.

- **A0** (positive control): full base, no negation fold: one column per point
  `(x, y)`, entry `+1` (multiplicity). `ncols = 2 n_A`.
- **A** (baseline, lab convention): one column per abscissa `x` (`P ~ -P`), entry
  `+-1`. `ncols = n_A`.
- **B** (treatment): one column per `rho`-orbit of abscissae excluding `x = 0`,
  entry `+- lambda^k`; `x = 0` points enter with coefficient 0 (dropped).
  `ncols = (n_A - delta0) / q`.
- A0, A, B on the same (curve, seed) consume the **identical** trial stream
  (same r-adding walk); each stops at its own target `ncols + 10` relations.
- **R_neg**: rho on classes `{+-W}`, class size 2.
- **R_aut**: rho on classes `mu_{2q} W`, class size `2q`.
- On control curves C5/C7: A, B, R_neg, R_aut, where B and R_aut run with `q = 1`
  (`lambda = 1`, no `rho`) and must reproduce A and R_neg exactly (A/A control).

## 5. Metrics and unit

Per IC run: trials, relations, columns, `rel_mul`, `rel_inv` (all `F_p` work of the
relation phase including walk set-up, group additions, split test, root finding,
orbit reduction), `la_mul`, `la_inv` (mod `r`), matrix `dim`, `nnz`, max fill.
Per rho run: steps, look-ahead extra additions, walks, abandoned walks, `mul`,
`inv` (all set-up, steps, canonicalisation).

FPM band `(I, kappa) in {(1, 1), (10, g^2), (30, g^2)}`; primary `(10, g^2)`:

- `IC_FPM = rel_mul + I rel_inv + kappa (la_mul + I la_inv)`
- `R_FPM = mul + I inv`

Derived per cell (family, `k`): means over curves and seeds;
`D1 = IC_A / IC_B`; `rho_gain = R_neg / R_aut`; `N = D1 / rho_gain`;
`rel_ratio = relations_A / relations_B`; `pos_ratio = relations_A0 / relations_A`;
remaining distance `gap_B = IC_B / R_aut` and `gap_A = IC_A / R_aut`.
Modelled (reported only, never decisive): `LA_model = 2 dim nnz`.

Uncertainty: hierarchical bootstrap (resample curves within the cell, then seeds
within curve, separately per arm), 2000 replicates, RNG seed 20261116, 95%
percentile interval. Pooled `N` for a family = geometric mean of the per-cell
`N` over the cells named in section 6; bootstrap the pooled statistic by
resampling within every cell jointly.

## 6. Decision rule (fixed before data)

Primary: family F5, pooled over `k in {12, 13, 14}`, primary FPM reading.

- **SUPPORTED** if the 95% CI lower bound of pooled `N >= 2.0`.
- **KILLED** if the 95% CI upper bound of pooled `N < 2.0`.
- **INCONCLUSIVE** otherwise.

Band robustness: the same rule is evaluated at `(1, 1)` and `(30, g^2)`; if the
verdicts differ, the result is reported as "band-dependent" with all three.
Secondary: F7 pooled over `k in {9, 10}`, same thresholds, reported as its own
verdict. Per-cell values and `k = 10, 11` (F5), `k = 8` (F7) are reported, not
decisive. Prediction check (reported, not decisive): `rel_ratio` within 5% of
`(n_A + 10) / (n_B + 10)`; `rho_gain` in `[0.75 sqrt q, 1.15 sqrt q]`.

## 7. Correctness gates (all must pass before any metric is read)

- G0: `python3 code/selftest.py` passes every test in SPEC 12.
- G1 (per curve): unique `#J` survivor; `r` prime; `[h r] D = 0` on 32 random
  divisors; for F5/F7 exactly one `lambda` in the roots of `Phi_q mod r` with
  `rho(G) = [lambda] G`, and `r = 1 mod q`.
- G2 (per CM curve): transport check on 20 random base points `P`:
  `[h] rho(P - inf) == [lambda] ([h] (P - inf))`.
- G3: every run certified (`[t] G == T` by an independent double-and-add, `t`
  equal to the planted value). Uncertified runs are excluded and counted; a cell
  with more than 5% uncertified runs in any arm is void.
- G4 (paired stream): for every (curve, seed), the cumulative trial digests of
  A0, A, B agree at every common checkpoint (every 1000 trials).
- G5 (orbit identity): `ncols_B * q + delta0 == ncols_A` and `ncols_A0 == 2 ncols_A`
  on every CM run.
- G6 (A/A): on every control run, B equals A and R_aut equals R_neg in every
  counter.
- G7 (positive control, the instrument sees a known fold): every F5/F7 cell has
  mean `pos_ratio` in `[1.8, 2.2]`. If G7 fails the verdict is INCONCLUSIVE
  (instrument), whatever `N` is.

## 8. Confounders and how each is isolated

- Relation-search noise: arms share one trial stream (G4); B's run is a prefix of A's.
- Linear-algebra ordering: one deterministic Markowitz elimination (SPEC 8) for all arms.
- Rho variance (Rayleigh, CV ~0.5 per run): 24 seeds x 3 curves per genus-2 cell
  (12 at genus 3); R_neg and R_aut share seeds, `G`, `T`.
- Fruitless cycles in R_aut: Bos-Kleinjung-Lenstra look-ahead plus walk abandonment
  at `20 * 2^k_dp` steps; every abandoned step is charged and reported.
- Canonicalisation cost: counted in R_aut's FPM (not free).
- Curve-specific effects (cofactor, twist): 3 curves per cell, admission by rule;
  controls at the same `p`.
- Instrument (Python generic Cantor, not the lab's Rust explicit formulas):
  absolute FPM is instrument-specific; only within-instrument ratios are read.
- Analyst: thresholds and analysis (`code/analyze.py`, SPEC 11) fixed here; the
  analysis program does not read arm labels in any tunable way.
- Host/scheduling: counts only, single thread.

## 9. Commands (executor runs exactly these, from the lead directory)

Each line runs under `ulimit -v 2000000` and `timeout 900`; one thread.
If a repository hook refuses a command, record the refusal verbatim in
`raw/REFUSALS.txt` and continue with the rest.

```
cd /home/user/crypto/research/g2plus_leads_20261116/L02_automorphism_orbit_bases
python3 code/selftest.py > raw/selftest.txt 2>&1
python3 code/select_instances.py --family F5 --bits 10,11,12,13,14 --out raw/instances_F5.jsonl
python3 code/select_instances.py --family F7 --bits 8,9,10 --out raw/instances_F7.jsonl
python3 code/select_instances.py --family C5 --cm raw/instances_F5.jsonl --out raw/instances_C5.jsonl
python3 code/select_instances.py --family C7 --cm raw/instances_F7.jsonl --out raw/instances_C7.jsonl
python3 code/gates.py --instances raw/instances_F5.jsonl,raw/instances_F7.jsonl,raw/instances_C5.jsonl,raw/instances_C7.jsonl --out raw/gates_instances.jsonl
for k in 10 11 12 13 14; do for arm in A0 A B; do python3 code/run_ic.py --instances raw/instances_F5.jsonl --bits $k --arm $arm --seeds 1,2,3 --out raw/ic_F5_b${k}_${arm}.jsonl; done; done
for k in 8 9 10; do for arm in A0 A B; do python3 code/run_ic.py --instances raw/instances_F7.jsonl --bits $k --arm $arm --seeds 1,2,3 --out raw/ic_F7_b${k}_${arm}.jsonl; done; done
for k in 10 11 12 13 14; do for arm in A B; do python3 code/run_ic.py --instances raw/instances_C5.jsonl --bits $k --arm $arm --seeds 1,2,3 --out raw/ic_C5_b${k}_${arm}.jsonl; done; done
for k in 8 9 10; do for arm in A B; do python3 code/run_ic.py --instances raw/instances_C7.jsonl --bits $k --arm $arm --seeds 1,2,3 --out raw/ic_C7_b${k}_${arm}.jsonl; done; done
for k in 10 11 12 13 14; do for arm in Rneg Raut; do python3 code/run_rho.py --instances raw/instances_F5.jsonl --bits $k --arm $arm --seeds 1-24 --out raw/rho_F5_b${k}_${arm}.jsonl; done; done
for k in 8 9 10; do for arm in Rneg Raut; do python3 code/run_rho.py --instances raw/instances_F7.jsonl --bits $k --arm $arm --seeds 1-12 --out raw/rho_F7_b${k}_${arm}.jsonl; done; done
for k in 10 11 12 13 14; do for arm in Rneg Raut; do python3 code/run_rho.py --instances raw/instances_C5.jsonl --bits $k --arm $arm --seeds 1-6 --out raw/rho_C5_b${k}_${arm}.jsonl; done; done
for k in 8 9 10; do for arm in Rneg Raut; do python3 code/run_rho.py --instances raw/instances_C7.jsonl --bits $k --arm $arm --seeds 1-6 --out raw/rho_C7_b${k}_${arm}.jsonl; done; done
python3 code/analyze.py --raw raw --out raw/analysis.json --tsv raw/summary.tsv
sha256sum raw/*.jsonl raw/*.json raw/*.tsv raw/*.txt > raw/data.sha256
```

If one command exceeds its `timeout`, the executor records it in
`raw/TIMEOUTS.txt` and does not change parameters; the affected cell is void.

## 10. What is not done

No wall time, no Rust, no 2LP arm, no Picard curve, no deployed curve, no
leaderboard or benchmarks row, no git state change by the executor.
