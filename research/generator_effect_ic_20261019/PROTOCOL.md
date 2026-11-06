# PROTOCOL E3a: do the base's cofactor classes explain the volcano study's IC yield effects?

**Frozen 2026-10-19, before any N0 was regressed on any yield.** Everything
after this commit that changes the data, the estimators, the gates or the
wording is a new, labelled protocol version, and both are reported. The
design and its reasoning are in [`PLAN.md`](PLAN.md) §1 and §4 (E3a). This
protocol freezes only E3a; E0, E1, E3b, E2 and E4 are not run here.

## 1. Question

In the volcano cost study (`research/isogeny-volcano-cost_20261017`, PROTOCOL
v1.1, branch `aburan28/youthful-ride-6u51pp-volcano-v2`) the IC relation
yield differed between isomorphic models of one curve (coordinate
τ = 4.7%), between curves of one level (horizontal τ = 2.2%), and, in the
exploratory ℓ = 2 contrast, between floor and crater curves (about −6.5%).

Working hypothesis (PLAN §1): all three are **N0 effects**. The
`prime-abscissa:size=32` base takes the smallest abscissae of the whole
group `E(F_p)`, not of ⟨G⟩. With `#E = h·r`, a target in ⟨G⟩ is a sum
`±P_i ± P_j` of base points only if the summands' cofactor classes `[r]P_i`
cancel. `exact_decomposition_ceiling` counts those multisets, `N0`, and
bounds the decomposition probability of a uniform subgroup target by
`N0/r`. If yield follows `N0/r`, then, once yield is divided by `N0/r`,
nothing is left for a model, a curve or a level to explain.

## 2. Data (existing; no new runs)

- **Sessions:** the committed v1.1 `ic-mitm` records, non-warm-up, status
  `verified`, from `sessions/<class>/spec/records.jsonl.gz`, with
  `l3-split/spec-r2` in place of `l3-split/spec` (the interrupted session
  stays out, as in `analysis-v1.1.json`). Integrity: the files are checked
  against `sessions/MANIFEST.sha256` (hashes of the uncompressed files)
  before use.
- **Not used:** `spec-aa` (rho only), `spec-bsgs`, `spec-icctl` (size 33),
  `j0`, `j1728`.
- **Workloads:** the 800 ordinary workload curves (six classes, 200 curves,
  4 models), 16 paired targets each, so 12,800 runs. A workload is
  identified by `(p, a, b, gx, gy)` and mapped to class, vertex, model and
  level through `classes/<class>/curves.jsonl`, exactly as `volcano-stats`
  does.
- **Per-run value:** the ratio `relations_found / targets_tried`, as in
  F3 of the volcano PROTOCOL.
- **Curve yield `Y_c`:** `Σ relations_found / Σ targets_tried` over the 16
  runs of workload `c`. Its weight is `R_c = Σ relations_found`.

## 3. N0 (computed natively, Rust, blind to yield)

- **Base:** `prime_factor_base(&inst, 32)` (`ic_boundary.rs:4396`; the
  committed tree has it at line 4431) with `inst` built from the workload's
  `curve_spec` (`p, a, b, group_order, r`, generator). 64 signed points.
- **Check that it is the measured base:** the sorted point keys are hashed
  as `ecbench` does (`factor_base_facts`, SHA-256 of the big-endian sorted
  keys) and compared with `factor_base.points_sha256` of the workload's
  records. **Any mismatch on any workload stops the analysis** (instrument
  fault: fix and re-run the census only). The census never reads a yield.
- **N0:** `exact_decomposition_ceiling(curve, &fb.points, r, 2)` →
  `(classes, N0/r)` with `m = 2`, the two-summand MITM oracle of the arm
  (`mitm:negation_folded=1`). `w_c = N0/r` (capped at 1 by the function;
  the cap is not reached at these sizes, and the census reports if it is).
  `m = 3` is computed and reported, not used.
- **`r`:** the `r` in the workload's `curve_spec`, the order of the
  generator's subgroup, with `h = group_order / r` reported.
- **Output:** `n0.json`, one row per workload with `w_c`, the class count,
  `h`, and the hash check.

## 4. Statistics and gates

All numbers come from the native `generator-n0` binary and
`volcano-stats --n0` (same code and seeds as the volcano analysis;
bootstrap 2,000, permutations 10,000 with 2,000 for the heterogeneity
nulls, seed 20261017). Python only draws figures.

**Reproduction check (not a gate).** `volcano-stats` without `--n0` on the
same records must reproduce the volcano study's IC-yield row of
`analysis-v1.1.json` (horizontal τ 0.0225, model τ 0.0467, floor/crater
0.9802). If it does not, the difference is reported and the gates below are
read against the reproduced values.

**G1: slope.** WLS of `ln Y_c` on `ln w_c` over the 800 workloads, weights
`R_c`. Pass if the point estimate lies in **[0.9, 1.1]**. Also reported:
the 95% interval from a cluster bootstrap (2,000 replicates; clusters are
the 200 curves, i.e. `(class, vertex)`, with their 4 models), the OLS slope,
and the slope with class fixed effects and with curve fixed effects (the
last uses only the within-curve, between-model variation, which is the
coordinate effect). A workload with `R_c = 0` is excluded from the fit and
counted; the count is reported.

**Adjustment.** For the second and third gates each IC-yield run's
numerator is divided by its workload's `w_c`
(`volcano-stats --n0 n0.json`, exponent 1, fixed, not fitted). Dividing the
numerator rather than the denominator keeps `(num, den)` pairs exchangeable
under the null of no residual heterogeneity, which is what the permutation
null shuffles.

**G2: horizontal τ.** On the adjusted yield, the DerSimonian–Laird τ
between curves of one level (strata = class × level) is **at or below the
95th percentile of its permutation null** (the volcano study's reading for
"falls to its null"). Unadjusted value: 2.2% against 1.4%.

**G3: coordinate τ.** On the adjusted yield, the τ between isomorphic
models of one curve (strata = curves) is at or below the 95th percentile of
its permutation null. Unadjusted value: 4.7% against 2.5%.

**G4: ℓ = 2 floor/crater contrast.**
- `C` = natural log of the v1.1 stratified floor/crater yield ratio
  restricted to the three ℓ = 2 classes (`l2-ramified`, `l2-split`,
  `l2-inert`): the ratio of sums within each class, equal-weight mean of
  the logs. Floor is the deepest level, crater level 0.
- `C_raw` is computed on the measured yield, `C_adj` on the adjusted yield.
  The explained fraction is `E = 1 − C_adj / C_raw`.
- Pass if **E ≥ 0.80**. If the 95% bootstrap interval of `C_raw` (curves
  within class × level) includes 0, G4 is **not testable** and is reported
  as such, not as a pass or a failure.
- Secondary, reported only: the same for `l2-ramified` and `l2-split`
  alone (the two classes where the exploratory contrast was nominally
  significant), and for the three ℓ = 3 classes (a control: both `C` should
  be near 0). The 90% interval and permutation p of `C_adj` are reported.

**Verdict.**
- All of G1–G4 pass (G4 may be "not testable"): "yield follows the base's
  cofactor classes: slope …, residual τ … (null …)".
- Otherwise the failed gates are named and PLAN §4 E3a applies: the
  coordinate effect has another cause and E3b is re-planned. The report
  does not soften a failed gate.

## 5. Power and limits, stated in advance

- `N0/r` is an upper bound on the decomposition probability of a uniform
  subgroup target. Proportionality of yield to it across classes assumes a
  constant shortfall (duplicate sums, the negation fold). A slope in range
  therefore supports, but does not prove, the mechanism; classes differ in
  `p`, `r` and `h`, so the pooled slope also carries between-class
  variation. The fixed-effect slopes are reported to separate the two.
- Per-workload yield has about 20 relations per run and 16 runs, so a
  single workload's `ln Y` has a standard error near 0.06. The 800
  workloads give a slope interval much narrower than the ±0.1 gate unless
  `ln w` has little spread; its width is reported with G1.
- The Cochran/DL τ cannot distinguish a residual of 1% from none at this N;
  "at or below the null p95" is the gate, and the residual τ is reported
  beside it.
- Scope: 24-bit `p`, `prime-abscissa:size=32`, MITM `negation_folded`, the
  six ordinary volcano classes. No security statement. It says nothing
  about generators (E1) or about a changed base (E3b).

## 6. Disclosures

- The yields, the τ values above and the exploratory ℓ = 2 contrast were
  already published in the volcano study and were known when this protocol
  was written. **No N0 had been computed, and no N0 had been compared with
  any yield, when this file was frozen.**
- One records file (`l2-inert/spec`) was opened to read the record schema.
  No yield was tabulated from it.
- Tool changes made after freezing (new `generator-n0` binary, `--n0` in
  `volcano-stats`) are implementations of §3 and §4; any change to an
  estimator's definition is a new protocol version.

## 7. Does not count

- Changing the gates, the thresholds, the adjustment exponent, or the data
  after seeing the regression.
- Dropping a class, curve or model because of its residual.
- Re-running the census to change `w_c` (it is deterministic; a re-run is
  only to fix a hash-check fault).
- Reading E3a's outcome as evidence about generators (E1) or about a changed
  base (E3b).
