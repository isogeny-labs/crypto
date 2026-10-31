# Red team decision report: claimed k = 4 index-calculus crossover over F_{2^{4m}} (oracle gaudry-d4-bin)

## 1. Verdict

**The crossover is NOT established. Verdict: NOT SURVIVED (unsupported), not refuted.**

The claim needs IC below rho at m near 17. There is no clean IC measurement at m >= 16. The one m = 17 IC record comes from a killed run (`sessions/e-m17-ic.attempt1-killed`), which is not evidence. Every ratio at m = 17 below is illustrative only. At m = 15, the only clean IC/rho point, IC loses by about 5x even after the corrections that favour IC.

### Per-check verdicts

| Check | Verdict | Correction to IC/rho | Skeptic |
|---|---|---|---|
| Rho reference | Survives | 1.0 (sensitivity: x1.3 on IC/rho, estimate, if rho used batched inversion) | not refuted |
| Unit price | Partly survives | 0.64 at m = 17, 0.89 at m = 15 (random-operand microbench). The plausible range is 0.31 to 3.1 and the sign is undetermined. | not refuted. The geometric mean of the six m = 17 variants is about 0.94 (skeptic). |
| Unpriced work | Partly survives | 0.93 (estimate; bound 0.50 to 0.94) | not refuted |
| Replication | Fails | none (power problem) | no skeptic ran |

- **Rho reference.** The curves are generic ordinary curves with j = 1/b, and j is not in any proper subfield. Aut(E) = {+-1}, so negation is the only automorphism rho can use. `frobenius=1` is a solver-side degree reduction in the polynomial system, not a group automorphism. The rho floor S = 0.886 is about 3.5% generous to rho.
- **Unit price.** The 97 constant is not used by the binary oracle. The conversion is GAE = F_q mults / fp_muls_per_add (159 at m = 15, 161 at m = 17). It is a count ratio, not a time ratio. About 99.85% of IC cost is priced this way.
- **Unpriced work.** Rho's canonicalisations are uncharged, so every rho.negation record has `lower_bound = true`. IC is fully priced through the counted solver. Pricing rho fully would raise rho by about 1.06 to 1.07 (estimate, from Koblitz `ns_per_canon` pins; no f2m4 pin exists). The full-pricing methods (`ic.pipeline_full`, `rho.signed_frobenius_*_full`, a price-units command) do not exist in this tree or on origin/main.
- **Replication.**
  - m = 16: zero targets.
  - m = 17: 2 rho targets, 0 clean IC targets.
  - m = 15: 2 targets per arm.
  - m = 8, 10, 12: 2 targets each. m = 14: 1 target.
  - With n = 2 no 95% interval is possible (t interval about +-12.7 SE).
  - The replay of records on this host is not confirmed. Only an integrity-only verify of d-ladder was run: 18/18 verified, 0 problems. The full replay was skipped because d-ladder's recorded solve time is about 50 min, above the 30-minute limit.
  - SPEC-E-m16, SPEC-E-m18-* and SPEC-D-m14 have no session directories.

### Corrected IC / best-rho ratio (recorded ratio x surviving factors)

Recorded ratios come from the audits: 6.30 at m = 15, and 0.72 at m = 17 (0.62 on the same target). The m = 17 value is from the killed run.

| m | Clean data | Recorded | x unit-price (central) x unpriced 0.93 | Range over unit-price variants (x0.93) |
|---|---|---|---|---|
| 8 to 14 | yes, n = 1 to 2 | IC loses by 40x to 400x on S (IC S 396.9 and 599.6 at m = 8, 73 at m = 10, 42.7 to 45 at m = 12, 5.3 at m = 14, against rho S about 0.7 to 2.2) | not recomputed | not recomputed |
| 15 | yes, n = 2 per arm | 6.30 | **5.2** | 2.4 to 15.0 (all above 1) |
| 16 | none | none | none | none |
| 17 | rho only; IC from a killed run | 0.72 (illustrative) | 0.43 (illustrative) | 0.21 to 2.06 (straddles 1) |

- **Rho-reference sensitivity.** If rho were priced with batched inversion (an upper sensitivity, not applied above), every IC/rho entry would be multiplied by about 1.3 (estimate).
- **Direction of the unit-price correction.** Counted and random-operand pricing favour IC. Latency-bound and in-situ pricing hurt IC (corrections 1.5 to 3.1).
- **Direction of the unpriced-work correction.** Unpriced rho work makes the measured rho S too low, so E3 is harder to pass as measured, not easier.
- **m = 17 consistency.** The killed point is inconsistent with the scaling extrapolation below, which gives about 1.7 at m = 17 (recorded, estimate). That is another reason not to lean on one killed target.

### Where the corrected crossover m* lands (estimate, extrapolated, not measured)

The prime-field cross-check and the binary fits give these exponents of cost in q = 2^m:
- IC relations: 1.07 [1.01, 1.13] (binary rung-mean, n = 5 m values).
- IC linear algebra: 2.03.
- Rho: 2.00 [1.71, 2.29] (rung-mean, n = 6).

The IC/rho ratio then falls by a factor 2^-0.93, about 0.52, per unit of m. Anchoring on the m = 15 ratio gives:

- Recorded ratio 6.3: m* is about 17.9.
- Corrected central ratio 5.2: m* is about 17.6.
- Corrected range 2.4 to 15: m* is about 16.3 to 19.2.
- The rho exponent interval (1.71 to 2.29) alone moves m* by several units.

Conclusion: **m* lies at or above 17, probably outside the measured range (m <= 15 for IC).** The claimed crossover "near m = 17" is at best at the edge of what the data can say. The pricing choice moves it by 1.5 m to 2 m in either direction.

Protocol status against the E-conditions (from the audits):
- E3 (IC below both theory and measured rho at m = 17) is untested.
- E4 holds: linear algebra is about 0.4% of GAE at m = 15, under the 5% limit.

Other caveats:
- At m = 17 rho S is 1.18 on average over 2 targets (1.33x theory). Per-target rho scatter is 2x to 5x, against 1% to 6% for IC.
- The f-lp and e-m17-rho sessions ran at isolation L0 (PSI 29.9 and 84.1), so their wall times are not comparable with the L1 sessions.

## 2. Prime-field cross-check (chase4)

The prime-field cross-check survives: the two fields' slopes agree. Exponents of cost in q:

| Quantity | Prime field (chain-1 rung-mean) | Prime field (all clean, rung-mean) | Binary (plain Wiedemann arm) |
|---|---|---|---|
| IC relations | 0.96 [0.65, 1.27], n = 12 | 1.00 [0.81, 1.18] (per-run 1.08 [0.94, 1.21], n = 69) | 1.07 [1.01, 1.13], n = 5 |
| IC linear algebra | 2.04 [1.85, 2.22] | 1.94 [1.88, 2.01] | 2.03 [1.98, 2.08] |
| Rho GAE | 1.93 [1.76, 2.09] | 1.86 [1.78, 1.95] (pooled per-run 1.82, n = 581) | 2.00 [1.71, 2.29], n = 6 |

- **What it supports.** IC relation work scales at about q^1, rho at about q^2. IC total equals the relation phase, because relations are over 99.9% of GAE. This supports the scaling extrapolation used for m* above.
- **Why S is unusable as an exponent.** ecbench S is not a usable exponent in either field. S/GAE is constant within each p (or m) and falls about as q^-2, so the S slopes are the GAE slope minus 2.
- **Limits of the comparison.**
  - It compares exponents only. Factor bases and cofactors differ across fields (fghr-y-line with h up to 16 versus bin-t-line with max_cofactor 2).
  - Binary IC has only five m values (8, 10, 12, 14, 15), no m = 16 or 17 point, and no large-prime variant in the plain arm.
  - Per-run intervals ignore between-curve scatter in trials per relation, so the rung-mean intervals are the more honest ones.
- **What it does not show.** It does not show that the constant at m near 17 favours IC. It shows only that the slope difference exists, and the constant offset at m = 15 (6.3) decides where it crosses.

## 3. Frozen replication plan (to run only after review)

**Goal.** Decide whether IC beats the strongest legitimate rho in a priced unit that survives both counted and hardware pricing, at m = 16 and 17, with at least 8 fresh targets per arm.

### Arms, per m

1. `ic-wied`: gaudry-d4-bin, plain Wiedemann, as in the claim (`macaulay=11, frobenius=1, sparsest=1, replay=1`), unchanged binary.
2. `rho-neg`: the existing rho.negation, as a regression control against old records.
3. `rho-strong` (new, built with the ecbench-extend skill). This is negation-map rho with distinguished points, batched (Montgomery) inversion, and canonicalisation charged. Its cost is counted in F_q multiplications: about 118 to 125 per addition, an estimate to be measured. This addresses the two rho understatements found: the unamortised inversion (about x1.3) and the uncharged canonicalisation (about x1.07).
4. Optional: `ic-lp` (large-prime variants) at m = 16 and 17 only, as an exploratory arm. It cannot decide the verdict.

### m values and targets

- **m values.** 14 (calibration), 15 (re-anchor), 16 and 17 (decision).
- **Targets.** 8 fresh targets per arm per m, a fresh curve per target drawn by the existing constructor.
- **Seeds.** `seed_i = first 64 bits of sha256("gaudry-k4-redteam-v2" || m || i)`, i = 0..7, committed to a spec file before any run. Algorithm seeds are separate and are also derived from the hash. The old targets (W588..., W761..., d5f7..., W652...) are excluded.
- **Pairing.** All arms run on the same 8 targets, in the same session, interleaved.

### Pricing

- **Primary unit.** Counted F_q multiplications, with one affine group addition equal to the measured `fq_muls_per_add` on that curve (159 and 161). Rho is charged in the same unit, with canonicalisations and the batched inversion included.
- **Secondary unit.** Hardware wall time, measured on isolab at L2/L3 isolation (ecbench-independent-runner skill), with no other jobs. Report the ratio of IC to rho CPU time per target.
- **Per-op calibration.** Re-measure ns per counted solver multiplication in situ (the pricing choice that flips the sign in the unit-price audit), and ns per rho step with the strong implementation.
- **Reporting.** Report both units and never pick the one that favours either arm after the fact.

### Estimated wall time (estimates, 4 vCPUs, one job per vCPU; the 4 vCPUs may not be 4 physical cores)

| m | IC (8 targets) | rho (8 targets) | Combined |
|---|---|---|---|
| 14 | about 1.5 h | negligible | about 1.5 h |
| 15 | about 2.8 h (1.4 h per target) | about 0.2 h | about 3 h |
| 16 | about 5.6 h (2.8 h per target) | about 0.8 h | at least 6.4 h |
| 17 | about 11.2 h (5.6 h per target) | about 7.5 h (3.7 h per target) | at least 19 h |

The total is about 30 h wall. The m = 16 and 17 figures come from the replication audit and are extrapolated from m = 15 cost per trial (3.1 to 3.3 ms, about 190 trials per relation). The pilot suggested up to 3.7 h and 7.6 h per IC target at m = 16 and 17.

### Preflight gates

- Before any new run, run `ecbench verify --replay` on one existing clean session (d-ladder takes about 50 min) to confirm that this host's binary reproduces the records.
- Require isolation level L1 or better for every session (PSI preflight below 5). Do not mix L0 and L1 timings.
- Refuse to record a killed or partial run; keep `*.attemptN-killed` out of analysis.

### Decision rules (frozen before data)

Let r_i = log(IC_i / bestrho_i) per paired target, and compute the mean and a 95% bootstrap and t interval over the 8 targets.

1. **Crossover survives at m = 17** iff the upper 95% bound of the mean ratio IC/rho-strong at m = 17 is below 1 in the counted unit, AND the point estimate is below 1 in the hardware unit.
2. **Crossover refuted** if the lower 95% bound at m = 17 is above 1 in either unit.
3. **Inconclusive otherwise.** Add 8 more fresh targets once. If still inconclusive, report inconclusive.
4. **Slope check.** Fit log ratio against m on the 4 m values. Require the fitted slope to agree with -0.93 within its 95% interval. Report m* with its interval.
5. **Early stop.** If at m = 16 the lower 95% bound of the ratio is above 2 in both units, skip m = 17 and report "crossover not within reach".
6. **Order of arms.** Do not drop a target because of a long wall time. Drop one only for a verification failure, and report every drop.

### Extension to m = 18 to 20 (ONLY if the crossover survives)

- **IC.** Use the large-prime variant (cofactor/large-prime relations, the lp arms) to cut the relation count, with the factor-base size and max_cofactor frozen in advance from the m = 16 and 17 pilot.
- **Targets.** At least 4 fresh targets per arm per m (8 where feasible). By the exponents above (estimates), m = 18 costs about 2x per IC target (about 12 h) and 4x per rho target (about 15 h), so 8 targets is about 24 to 30 h wall at m = 18 and more beyond. At m = 19 and 20 use 4 targets for IC and rho-strong. If rho cost is too high, fall back on its measured mean S, validated at m <= 17 against the 0.886 floor, with that substitution labelled.
- **Slope fit.** Fit log(GAE) against m for each arm over m = 14 to 20, with a rung-mean 95% interval. The claim holds in the extension only if the IC exponent is below the rho exponent with non-overlapping intervals, and the m* from the fit lies at or below the smallest m where the direct comparison favoured IC.
- **Same rules.** The same pricing, preflight and decision rules apply, with the early-stop rule applied per m.

## 4. Open items

- No full-pricing methods exist in this tree or on origin/main; they must be built before the new pricing is used. The 1.07 and 1.3 figures are estimates until then.
- The F_q4 multiply (22 F_q mults) and inversion (107) might be cheaper with Karatsuba or better inversion. Such savings would hit IC's solver pricing more than rho's, and are untested.
- `frobenius_reducible` is empirical ("true on every curve tried, not proved"). The oracle refuses via a guard if it fails, so a silent error is unlikely, but nobody tested how often it fails on m = 17 curves.
- The e-m17-ic killed attempt left a record while `session.json` says `running` with `records_written` 0. This integrity oddity is unexplained.
- The `rho-neg-aa` control arm (identical S to rho-neg) is not named in PROTOCOL.md.

Supporting files (audit outputs) are under `/tmp/claude-0/-home-user/e260bf71-4677-5a1e-b0d3-1c3899bb5656/scratchpad/redteam/out/` in the subdirectories `rho-ref/`, `unit-price/`, `unpriced-work/`, `replication/` and `prime-crosscheck/`.
