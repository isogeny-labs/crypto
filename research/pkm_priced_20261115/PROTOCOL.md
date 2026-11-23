---
title: "Protocol (FROZEN 2026-11-15): the priced symmetric m = 4 PKM oracle, S5 build counted, parametric S5 precomputed"
date: 2026-11-15
status: FROZEN 2026-11-15 before any frozen-size data (hash in REGISTRATION.sha256, time in registered-at.txt). Only toy-size development checks (p = 12289) were run before freezing.
lead: IDG-20261115-SYM-PRICED (Lead 3 of the 2026-11-15 generator pass); advances LEAD-ICP-pkm-tower-algebraic-oracle
---

# Protocol: is the 2.75e5x F4-stage gain still there once the per-target S5 build is charged?

**Track: algebraic (prime-algebraic, PKM-Semaev on a Kummer base).** Class: component result, then the first priced row
on a track whose prime baseline is empty. No leaderboard row, no `S` claim and no improvement over the index-calculus
baseline is made by this protocol: "no pipeline is priced" stays true until Stage 1 closes. Every number below is
labelled **measured**, **derived** or **predicted**, and carries `[pcc:unverified]` until a pcc claim exists.

## 1. Question and why it is the next measurement

`research/pkm_symmetric_lead1_20261026` (README, "next measurements", item 3) counts only the `f4_fp` stage of the
symmetric m = 4 oracle: 4.010e5 field operations per system at N = 12 (1.102e11 for the plain chain, 2.75e5x, measured).
Per target the instrument also builds `S5(x1..x4, x_R)` by evaluating it on a 9^4 grid (6x6 Sylvester determinants mod p),
tensor-Lagrange interpolating, substituting `x_i = g w_i` and rewriting in `e1..e4` (`summation_poly_5`, `to_elementary`).
None of that is in the 4.010e5. Without it the cost per decomposition trial is unknown and the oracle cannot be compared
with the budget c* of `ideas/SYNTHESIS.md` Table 2 or with rho.

Lead (b): `S_{m+1}` has degree <= 2^{m-1} = 8 in `x_R`, so `S5(e; x_R)` is a degree <= 8 polynomial in `x_R` per
`e`-monomial. Computing it at 9 values of `x_R` once per curve and substituting per target should remove the build from the
per-target cost.

*Sources checked.* Instrument and cost line: `examples/pkm_symmetric_tower.rs`, `research/pkm_symmetric_lead1_20261026/README.md`,
`docs/ic/LEADERBOARD.md` section "Symmetric PKM tower, F4 stage at m = 4". Lead: `scratchpad/leads_generator.md` Lead 3.
Conventions: `research/ic_tableb_ksum_20261115/`. research-kb and the lead atlas were not queried by the preparing session
(not available there); the director repeats `kb_status` and a negative-form search ("symmetric summation polynomial
precompute degree in target", "PKM tower priced") before freezing and records the `source_id`s here.

## 2. Instrument and counted unit (frozen by this document)

One instrument, `examples/pkm_symmetric_tower.rs` (Rust), one binary built `--release --locked` from the commit that carries
this directory. New in this study, existing arms and rows unchanged (the counters never alter a value):

- **Counters** for the pre-F4 stage, per thread and per stage (`cnt`): `s4_eval`, `s3_eval`, `sylvester_det`,
  `vandermonde_inv`, `grid_interp`, `subst_gw`, `to_elementary` (these seven are "one grid build"), `param_xr_interp`
  (parametric precompute), `param_eval` (parametric per target), `planted_check` (excluded), `other` (tower assembly).
  Kinds: mulmod outside inversions, add/sub, inversion calls, mulmods inside Fermat inversions.
- **Unit.** `mulmod` = every executed field multiplication (inversions' included, as Fermat powers; the inversion calls are
  also reported). F4's unit is `F4Report::field_ops` = row multiply-subtracts, one mulmod and one subtraction each. The
  ratio build/F4 compares mulmods with F4 row operations: both are one multiply-add. Adds/subs are reported, not priced.
- **As-built counts are executed counts**, including the instrument's own inefficiencies (HashMap polynomial algebra for each
  `S3` evaluation, one Fermat inversion per pivot of each 6x6 determinant). A leaner grid build would be cheaper; the
  parametric mode is a different mode and is compared with the build as built. Report the inversion share beside the total.
- **Parametric mode** (`ParamS5`): at `x_R = 0..8` run the grid build, substitution and rewrite; interpolate every
  `e`-monomial in `x_R` (inverse Vandermonde on the 9 nodes); per target Horner-evaluate the monomial coefficients.
- **Equality check** (`compare_coeffs`): exact, mod p, over the union of supports, parametric against grid-built `S5(e)`,
  on every target, and on the planted target, which must also vanish at the planted decomposition in both modes.
- **Modes** of the binary: `--mode curve` (order, r, cofactor, generator, liftable part of V, derived yield, ecbench spec of the
  matched rho), `--mode calibrate` (`w_cal`), `--mode stage0`, `--mode summary`; no `--mode` is the legacy instrument.

## 3. Instances (frozen)

| N | m | t | p | curve, g, zeta | role |
|--:|--:|--:|--:|:--|:--|
| 12 | 4 | 3 | 786433 = 3*2^18 + 1 | the instrument's draws for (m = 4, t = 3): the committed rows' curve | primary |
| 16 | 4 | 4 | 786433 | the draws for (m = 4, t = 4) | secondary |

`--mode curve` prints `a, b, g, zeta, #E, r, cofactor, |V|, liftable`; the director records them in the freeze (they are
derived from the instrument's seed `0x504B4D53 ^ (m << 32) ^ t`, which also produced the committed rows).
Not run: N = 20 (aborted at 6 GB in the committed run), m = 3, N = 8 (V has 4 points; cannot be planted).

## 4. Arms

| arm | what | N = 12 | N = 16 |
|:--|:--|--:|--:|
| `grid` | symmetric oracle, S5 built per target as built | 24 targets build; 8 F4 | 24; 4 F4 |
| `param` | parametric precompute once per curve, per-target Horner | same 24 targets | same |
| `f4` | the quoted 4.010e5 stage (`f4_fp`, grevlex, bound n + d + 6) | 8 targets | 4 targets |
| `plain-chain` | reference oracle (1.102e11 ops, 66-70 s alone each) | 3 targets, paired | not run |
| `rho-neg` | matched negation-map rho, `ecbench run`, 200 targets, 3 rounds, 1 warm-up | yes | yes |
| `rho-neg-aa` | A/A control for rho-neg | yes | yes |

Controls: **planted** target (the planted summand set must satisfy every equation of both modes; both modes must agree on
it), **negative control** (a target with `x_R in V` is excluded and logged, never silently dropped), **A/A** on rho-neg
(same arm twice; the replicate spread of the target-level means is the noise floor for the rho side), the committed rows'
curve. Counted work is deterministic: build counts depend on the target only through zero coefficients, F4 counts agree
across targets to ~4e-7 (committed README), so the targets-per-N above are sufficient for the counts and are not a sample
of a noisy quantity.

## 5. Seeds (frozen)

- `x_R` of target `i`: `Rng(2026111501 ^ (t << 48) ^ i * 0xD1B54A32D192ED03)`, first draw mod p (independent of sharding).
- rho specs: `target_seed = 2026111531 + t`, measurement `seed = 2026111541 + t`.
- Local toy check (not frozen, not evidence): `--p 12289 --t 3 --seed 7` and similar.

## 6. Pricing (AGENTS.md 5a)

Algebraic arms' native work is field multiply-adds (F4 row operations, build mulmods) plus unemitted work. Priced at
`w` GAE per native operation, `w` in {0, 0.1, 1}, plus `w_cal` (best-of-7 timing of one mulmod over one affine group addition
on the executing host, `--mode calibrate`, labelled as such, not an isolated host) and `10 w_cal`, and the leaderboard's
legacy 1 GAE = 13 mulmods (`1/13`), shown only as a label for comparison with the lead's arithmetic. At `w = 0` the algebraic
arm has no counted group work, so its `S` is 0 and the ratio to rho is undefined: it is reported as undefined, never as a
win. Rho's `S(w)` is read from `ecbench band` at the three band weights (linear in `w`; the fit residual at 0.1 is
recorded) and evaluated at `w_cal`, `10 w_cal` and `1/13`. Two arms are ranked only where the order holds at every
weight (band) and, separately, "ranked at calibrated price" if it holds at 0, `w_cal`, `10 w_cal`.

**Derived pipeline cost.** `S_alg(w) = (R / y) * (c_build + c_F4) * w / sqrt(r)` plus the parametric setup once per solve,
with `R` = relations needed = liftable x of V + 1, `y` = yield per target = `16 C(L,4) / #E` (derived: 8 C(L,4) distinct
x-values of sums, a random point hits one with probability 8 C(L,4) / ((#E-1)/2)), `L` = number of `x in V` that lift to
the curve (`--mode curve`). The lead used `8 C(|V|,4) / r` with every point of V liftable; `--mode curve` prints both. If
`L < |V|` the lead's yield is too high by up to a factor of C(|V|,4)/C(L,4) (70x for L = 4 of 8), which moves every
`S/rho` prediction below up by that factor. The priced S is a **derived** quantity from a measured per-target cost and a
derived yield: Stage 1 (below) measures the yield.

**Unpriced work, listed per 5a (lower bound, implementation gap):** root recovery of relation summands, linear algebra on the
relation matrix, verification `[d]G = Q`, `x_R` lift/decoding, per-curve tower assembly (reported separately), allocation.
Rho (ecbench) leaves canonicalisation, partition hash and table work to the band.

## 7. Predictions (frozen before data; prior in the lead)

All **predicted** from the lead's hand estimates, `[pcc:unverified]`:

| quantity | prediction | band | prior |
|:--|:--|:--|--:|
| (a) grid build / F4, N = 12 (mulmod per target) | 1.7-2.3 (7e5-9e5 against 4.0e5) | [1.0, 4.0] | 0.70 holds (>= 1x) |
| (b) parametric per-target / F4 | <= 1e-2 (<= 4e3 ops) | exact coefficient equality on every target | 0.90 |
| (b) parametric precompute | ~6e6 once per curve (9 grid builds) | breakeven within ~100 targets | |
| (c) priced S / rho-neg at N = 12 | 2e5-1e6 | | 0.55 |
| (c) priced S / rho-neg at N = 16 | 1e7-1e9 | | |
| priced S within 100x of rho | | | < 0.01 |

The prediction (c) uses the lead's yield; read it together with the derived yield of section 6 (the curve decides). A
priced S/rho outside the interval is a result, not a failure; it is reported with the yield that produced it.

## 8. Stages and the decisions

**Stage 0** (this protocol's gate; no ecbench): counters + coefficient check on >= 20 targets per N (frozen: 24) and the
planted target, at N = 12 and N = 16; F4 on 8 (N = 12) and 4 (N = 16) of them; plain-chain on 3 targets at N = 12.
Local development check at the toy size comes first (section 10).

**Kill (stop; report; Stage 1 does not run):**
1. the as-built grid build is `< 0.2x` the F4 stage per target at N = 12 (mean over the F4 targets): the lead's hand estimate was
   wrong by more than 5x and the 2.75e5x stands as a priced component ratio;
2. **any** coefficient mismatch between parametric and grid-built `S5(e)` on any target or on the planted target;
3. the parametric build cannot be amortised: precompute / (grid - param per target) `> 100` targets;
4. audit first, do not believe: priced S/rho `< 1e4` at N = 12 (below the c* argument; look for unpriced work).

A kill on 1 or 3 is reported as the result; a kill on 2 is a bug until proven otherwise (the interpolation is exact if
the degree-8 claim holds; a mismatch with all targets failing the same way means the degree bound is wrong, which is a
finding about `S5`, not about the code).

**Stage 1** (separate protocol, not authorised by this draft): cold pipeline at p = 786433, 200 targets per N, relation
harvest to rank |V ∩ E| + 1, the counted root recovery, linear algebra and verification `[d]G = Q`, measured yield, matched
rho-neg in the same session, replay and independent recompute for a `VALIDATION.json`. Only then can a row exist. Until
then every `S` here is derived.

## 9. Not claimed

Wall time (evidence only when uncontended; the run is sequential for that reason); an end-to-end gain (Stage 0 prices a
component and derives the rest); anything for m >= 5 (`S_{m+1}` has degree 16 in each variable at m = 5, a 17^5 grid, and
the symmetric presentation is not obviously a win there); the m >= 8 question; a security statement; any leaderboard row.
Lean obligation if (b) enters a claim: `summation_poly_degree_in_target` (degree of `S_{m+1}` in the target <= 2^{m-1}).

## 10. Landing

Checkpoint (`scripts/checkpoint.sh init`, `.resume/RESUME.md`) before the run; benchrec record
(`tools/benchrec.py init|run|add-raw|metric|close`), raw data by SHA-256 in S3, progress report in the same PR;
`research-visuals` report, graph and PDF; the validation gate for any row. Route (owner, 2026-11-15): RunPod CPU pod, because the monthly Iso Lab budget is exhausted. Binaries are built from the
frozen commit by the main session (the `pod/build.sh` steps, run locally), checked by SHA-256 on the pod, and `pod/run.sh`
runs unchanged. Budget: the owner's $20 ceiling shared with R1c/R1d.
