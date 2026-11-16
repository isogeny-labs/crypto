# R7.3: conductor-sensitive algorithms across an ℓ-volcano gap (second toy test of Conjecture 10′)

**Date.** 2026-10-31. **Code.** `examples/r7_3_conductor_sensitive.rs` (native Rust, toy
`p ∈ [2^20, 2^22)`). **Raw data.** `raw/` (161 JSON files incl. `raw/summary.json`).
**Background.** `RESEARCH_ISOGENY_FIBERS_ECDLP.md` §19.4 (R5.4: Conjecture 10, Theorem 4,
Conjecture 10′), §19.15 (R6.11: plain x-frame census, null). Not edited here.

## Hypothesis, falsifier, verdict

**Hypothesis H (from the task).** Some implemented algorithm whose cost depends on `End(E)`
(not only on the group structure of `E(F_p)`) shows a measurable crater-vs-floor cost gap on
ℓ-volcano craters/floors at toy size, i.e. Conjecture 10′ is non-vacuous.

**Falsifier (as stated).** For every algorithm tried the crater/floor cost-ratio CI contains 1
after the known automorphism-group baseline is removed ("null"), **or** a ratio CI excludes 1
(then report size, cause, and whether it exceeds the bounded factor Conjecture 10 allows).

**Verdict (measured; two parts, both stated plainly).**

1. **Unknown-scalar cost in rho iterations and relation yield: null.** After the automorphism
   baseline is removed, every crater/floor ratio CI contains 1 and is within about ±0.8%:
   automorphism-class rho at `j = 0` and `j = 1728` (conductor effect 1.0018 [0.9986, 1.0050] and
   0.9997 [0.9966, 1.0030], 32 instances each); the endomorphism-mixed rho at `h = 1` craters
   (iterations 1.0001 [0.9921, 1.0080], 40 instances); plain rho (0.9999 [0.9939, 1.0061]); the
   relation-yield class factor at `h(D) = 2, 3` craters (pooled 0.99985 [0.9993, 1.0004],
   `m = 3`, `|F| = 200`, 56 instances). The transport census identity holds exactly in 120/120
   checks. Conjecture 10′ is **untested-but-not-contradicted** at this scale for these
   algorithms.
2. **Counted field multiplications: a gap exists for one algorithm, and the falsifier's second
   branch fires there.** The endomorphism-mixed walk costs the floor **1.46×** the crater
   (1/0.6847, CI [1.443, 1.478]) in counted `F_p` multiplications. Cause (measured): the
   iteration counts are equal (1.0001); the whole gap is the cost of evaluating the cheapest
   explicit non-scalar endomorphism: floor/crater = **2.72 [2.59, 2.85]** per evaluation (112
   vs 304 mults on average; one affine addition costs 32–42). The floor endomorphism
   `ψ∘ω∘φ` is three Vélu maps, the crater one `ω` is one. That algorithm is dominated by plain
   rho at both levels (1.22× [1.20, 1.25] and 1.79× [1.77, 1.81] the plain multiplication
   count, on crater and floor), and its iteration count is not below plain (0.9993 [0.989, 1.009]
   on the crater): the best strategy here is to ignore `ω`, whose crater/floor cost ratio is
   0.9999 [0.9945, 1.0055]. So the gap is a constant-factor primitive cost of an inferior
   algorithm, polynomial in ℓ (2.5–2.9× over ℓ = 5…13), not a change in the cost of the best
   known attack. The factor 1.46 is below √3 ≈ 1.73, the largest rho constant R5.4 §2 allows,
   but nothing in the data bounds it independently of ℓ; Theorem 4 already prices this transport
   at poly(ℓ).

The *known* automorphism effect, which does move the cost across levels, is the baseline
control: the ζ-fold crater rho is cheaper than the `{±1}` crater rho by 0.5534 [0.5511, 0.5559]
(`j = 0`, ideal 1/√3 = 0.5774) and 0.6820 [0.6791, 0.6848] (`j = 1728`, ideal 1/√2 = 0.7071),
which shows that the harness resolves ~1% effects.

## Requirements status

| Requirement | Status |
|---|---|
| (i) GLV/endomorphism-accelerated rho, counted ops, matched seeds | **implemented, verified**, as an endomorphism-mixed walk with an explicit Vélu endomorphism, not a 2-dimensional GLV multi-exponentiation. GLV lattice reported for known-scalar splitting (§4). |
| (ii) ζ-fold on `j = 0, 1728` vs non-special neighbours, baseline separated | **implemented as automorphism-class rho** (full `Aut(E)` vs `{±1}`). The library `LineOracle` and the `F_{p²}` ζ-fold index calculus were **not run** (they target `F_{p^k}` line bases, not a prime field toy). |
| (iii) transport index calculus whose yield can depend on level | **partial**: transport census identity (exact, 120/120) and DLP transport with cost; no new level-dependent IC oracle was built; the plain x-frame yield is extended to `h > 1` (Exp H). |
| ≥ 20 instances per level, paired seeds, bootstrap 95% CIs | verified complete: 32 + 32 + 40 + 56 instances; seeds shared across levels of an instance (same step-table scalars, start scalars and hidden `k`; the curves differ); cluster bootstrap over instances (20 000 resamples). |
| `h(D) > 1` craters | **done** for the plain census (Exp H, `h = 2, 3`, 56 instances); not done for the endomorphism-based algorithms. |
| Native Rust, toy only, no sub-rho or deployed-curve claim | verified |
| Conductor task (`conductor check`) | **not run**: `conductor` is not installed in this session (CLAUDE.md says hooks warn and let edits through). Edits were limited to new files under `examples/` and `research/isogeny_fibers_20261010/round7/r7_3_conductor_sensitive/`. |

## 1. Instances (measured, certified)

For a prime ℓ ∈ {5, 7, 11, 13} and `D`, `p = (t² + |D|v²)/4`, `ℓ ∥ v`, `t ≢ 2 (mod ℓ)`,
`N = p + 1 − t`. The crater is the CM curve of `D` (`End = O_K`); the floors are the other
`F_p`-roots of `Φ_ℓ(j₀, Y)` (`End = Z + ℓO_K`); `#E = N` on every curve (certified by `[N]P = O`
on random points); the DLP lives in the prime-order subgroup `G_r` (`r = N/cof`), and `ℓ ∤ r`, so
each vertical isogeny is a group isomorphism `G_r → G_r`. Checked in every instance: the number of
floors equals `(ℓ − (D/ℓ))/u` (`u = 3, 2, 1` for `j = 0, 1728`, other), and every floor has
exactly one `F_p`-root of `Φ_ℓ`.

| Family | D | N / group | instances (each with all its floors) | floors per instance |
|---|---|---|---|---|
| Exp A, `|Aut| = 6` | −3 (`j = 0`) | `N` prime | 32 (8 per ℓ) | 2–4 |
| Exp A, `|Aut| = 4` | −4 (`j = 1728`) | `N = 2r` (a 2-torsion point always exists) | 32 | 2–6 |
| Exp B, `h = 1` | −19, −43, −67, −163 (10 each) | `N = n_ω · r`, `n_ω = (1+|D|)/4 ∈ {5, 11, 17, 41}` | 40 (10 per ℓ) | 4–14 |
| Exp H, `h = 2, 3` | −51, −91, −115, −123, −139, −187, −211, −235, −283, −307, −331, −403 | `N` prime | 56 (4–8 per `(D, ℓ)`) | `h(ℓ − (D/ℓ))` |

`D = −11` is excluded from Exp B (`n_ω = 3`: the two rational lines of `E[3]` have opposite
Frobenius eigenvalues and the x-coordinate eigenline condition cannot separate them). For
`D ∉ {−3, −4}`, `n_ω | N` is forced when `n_ω ∤ v` (for `n_ω = 3` both eigenvalues are nonzero and
distinct, hence one is 1), which is why the cofactor `n_ω` appears. `p ∈ [1.10·10⁶, 4.15·10⁶]`;
`r ∈ [3.9·10⁴, 4.1·10⁶]`.

Exp H: `v = ℓ` (so conductor 1 or ℓ only), ℓ split in `K` and non-principal, so the horizontal
ℓ-cycle visits all `h` craters; breadth-first search over `Φ_ℓ` roots (a root with one `F_p`-root
is a floor) found `h` craters and `h(ℓ − (D/ℓ))` floors in **56/56** instances. Pairs `(D, ℓ)` tried
first and discarded because `N` is never prime (3 splits in `K`, so `3 | N`; or `D ≡ 1 mod 8`,
so `N` is even): 35/13, 23/13, 31/5, 31/7, 59/5, 59/7, 83/7, 83/11, 107/11, 107/13.

## 2. Algorithms

* **Walk.** r-adding walk, 16-entry table of `S_i = a_iP + b_iQ`, index by the top 4 bits of
  `x·0x9E3779B97F4A7C15` of the *canonical* class representative; the classes are the orbits of the
  chosen automorphism group; each stored point keeps `(a, b)` with `R = aP + bQ`; a revisit with
  different exponents solves for `k` and is verified (`kP = Q`); a revisit with equal exponents is a
  fruitless cycle, escaped by doubling (≤ 8 times, then the walk is dropped and restarted). Memory
  is unbounded (all visited points kept), so the metric is iterations to the first useful collision.
  All 1 608 000 recorded runs found the correct `k` (`wrong_dlp_runs_total = 0`, no failed run).
  *Iterations* = additions + endomorphism evaluations + escape doublings; *mults* = `F_p`
  multiplications counted inside `Fp::mul` (a Fermat inversion is priced at its actual ~33
  multiplications; one affine addition is 32–42).
* **Exp A (classes).** `pm` = `{±1}`; `aut` = full `Aut(E)` (`ζ: (x, y) ↦ (βx, y)`, `i: (x, y) ↦
  (−x, iy)`), whose scalar on `⟨P⟩` is found by comparison with the group law. 3000 runs per curve
  and variant, seeds shared by every curve of the instance.
* **Exp B (endomorphism-mixed walk).** 2 of the 16 table slots (random positions) apply `ε`,
  `R ↦ ε(R)`, exponents `(a, b) ↦ (μa, μb)`. Crater: `ε = ω` (or `ω̄`), the Vélu isogeny whose kernel
  is the π-eigenline of `E[n_ω]` with eigenvalue `(t − v)/2` (`gcd(ψ_n, (x^p − x)ψ_μ² + ψ_{μ−1}ψ_{μ+1})`,
  certified by `kernel::verify_kernel`; codomain F_p-isomorphic to the crater). Floor: `ε = ψ∘ω∘φ`,
  `φ: E₁ → E₀`, `ψ: E₀ → E₁` the Elkies/Vélu vertical isogenies; degree ℓ²·n_ω; acts on `G_r` as
  `±ℓ·λ` (confirmed numerically in 40/40 instances for every floor). Vélu evaluation uses
  `X = x + Σ t_Q/(x − x_Q) + u_Q/(x − x_Q)²`, `Y = y·dX/dx` from power sums of the kernel
  polynomial. Gate: every map is checked on random points (on the codomain curve, additive:
  `f(P + Q) = f(P) + f(Q)`), and the eigenvalue of every composite is matched.
  600 runs per curve and variant.
* **GLV lattice.** For the measured eigenvalue μ of `ε`: Gauss-reduced basis of
  `{(a, b): a + bμ ≡ 0 (mod r)}`, mean `log₂ max(|k₁|, |k₂|)` over 2000 random `k` (Babai rounding).
* **Exp C.** Census (R6.11 definition, `m = 2, 3`, `|F| = 100`, smallx base) of a base on one level
  against its image under the vertical isogeny on the other; `φ(Q)` vs `k·φ(P)`.
* **Exp H.** R6.11's exact census (`m = 2, 3`, `|F| ∈ {100, 200}`, smallx), crater vs the mean of
  its own floors.

## 3. Results

Ratios are geometric means over instances, with 95% percentile CIs from 20 000 cluster-bootstrap
resamples over instances; `n` = instances. "Crater/floor" divides the crater mean by the mean over
that instance's floors.

### Exp A: automorphism-class rho (iterations; mults in `raw/summary.json`)

| D | aut / pm on the crater (Aut effect) | ideal 1/√(|Aut|/2) | crater `pm` / floor `pm` (conductor effect) | crater `aut` / floor `pm` (total) |
|--:|--|--:|--|--|
| −3 (n = 32) | 0.5534 [0.5511, 0.5559] | 0.5774 | **1.0018 [0.9986, 1.0050]** | 0.5544 [0.5524, 0.5564] |
| −4 (n = 32) | 0.6820 [0.6791, 0.6848] | 0.7071 | **0.9997 [0.9966, 1.0030]** | 0.6817 [0.6793, 0.6841] |

Per ℓ (n = 8 each), conductor effect (iterations): D = −3: 1.0000 [0.9922, 1.0079], 1.0034 [0.9989,
1.0082], 1.0039 [0.9989, 1.0086], 0.9999 [0.9926, 1.0068]; D = −4: 0.9971 [0.9921, 1.0027], 1.0011
[0.9934, 1.0104], 0.9996 [0.9941, 1.0051], 1.0010 [0.9962, 1.0058] (ℓ = 5, 7, 11, 13). In multiplications:
the Aut effect is 0.6255 [0.6225, 0.6286] / 0.7659 [0.7619, 0.7699] (the canonicalisation costs extra
multiplications) and the conductor effect 1.0017 [0.9986, 1.0048] / 0.9997 [0.9968, 1.0029].

Baseline. The observed Aut effect falls ~4% short of the ideal (0.553 vs 0.577; 0.682 vs 0.707). The
cause was not investigated (escape doublings are 10.1 per run for `aut` and 39.4 for `pm` on average,
so it is not simply more escapes). The baseline is taken from the crater itself (`aut/pm`), not from
the ideal; multiplying the total by the *ideal* baseline gives 0.9602 [0.9569, 0.9638] and 0.9641
[0.9607, 0.9675], which is this implementation shortfall, not a conductor effect (the conductor effect
is the `pm/pm` column).

### Exp B: endomorphism-mixed rho, `h = 1` craters (40 instances, 380 floor curves)

| quantity (crater / floor unless stated) | ratio, 95% CI |
|---|--|
| plain walk, iterations | 0.9999 [0.9939, 1.0061] |
| plain walk, `F_p` mults | 0.9999 [0.9945, 1.0055] |
| `ε`-walk, iterations | **1.0001 [0.9921, 1.0080]** |
| `ε`-walk, `F_p` mults | **0.6847 [0.6764, 0.6932]** (floor costs 1.460×) |
| `ε` / plain iterations, crater | 0.9993 [0.9890, 1.0092] |
| `ε` / plain iterations, floor | 0.9991 [0.9957, 1.0028] |
| `ε` / plain mults, crater | 1.2249 [1.2022, 1.2489] |
| `ε` / plain mults, floor | 1.7888 [1.7658, 1.8131] |
| cost of one `ε` evaluation, floor / crater | 2.7208 [2.5944, 2.8475] |
| same in units of one addition, floor / crater | 2.7208 [2.5933, 2.8471] |

`ε` evaluation, floor/crater by ℓ = 5, 7, 11, 13: 2.48 [2.29, 2.66], 2.71 [2.49, 2.92],
2.78 [2.52, 3.06], 2.94 [2.68, 3.17] (n = 10 each). Mean cost: crater 112.4 mults (range 83–160),
floor 303.8 (257–362); one affine addition 32–42. The ε slots apply on average 52 of 430 iterations
per crater run.

GLV lattice (known-scalar splitting, not a DLP quantity): mean `log₂ max(|k₁|, |k₂|)` floor minus
crater = +0.15 bits (sd 0.69, n = 40, standard error 0.11); ratio 1.018 [0.991, 1.047].

### Exp C: transport

120 checks (3 floors in each of 40 instances; `|F| = 100`): the census of the base on one level
equals that of its image on the other **exactly** in both directions (`D₂` and `D₃`), 120/120;
`φ(Q₁) = k·φ(P₁)` 120/120; transporting a floor DLP to the crater costs 187 multiplications on
average (two `φ` evaluations), 1.2% of a crater rho solve (max 2.3%).

### Exp H: relation-yield class factor, `h(D) = 2, 3` (56 instances, 12 discriminants)

Crater / mean-of-its-floors, geometric mean over instances (`c = D_m·m!/|F|^m`):

| `m` | `|F|` | pooled ratio |
|--:|--:|--|
| 2 | 100 | 1.0002 [0.9999, 1.0004] |
| 3 | 100 | 1.0007 [1.0000, 1.0015] |
| 2 | 200 | 0.9998 [0.9996, 1.0001] |
| 3 | 200 | 0.9998 [0.9993, 1.0004] |

Per discriminant (n = 4–8 instances), 5 of 48 intervals exclude 1, by at most 0.34%, with both
signs (D = −283, −211: 0.9988–0.9994; D = −187: 1.0017–1.0034). 48 uncorrected intervals at 95%
would give about 2.4 by chance; these are not claimed as effects.

## 4. Interpretation (labelled)

* **Why the iteration counts match (derived).** Over `F_p` every endomorphism of an ordinary curve
  acts on `G_r` as a scalar; the walk's dynamics on `Z/r` depend on `μ` only through its
  multiplicative behaviour, and `μ = λ` (crater) and `μ = ±ℓλ` (floor) are both "generic" mod `r`.
  An endomorphism of infinite order gives no equivalence classes, so unlike a unit it cannot shrink
  the search space. The measurement agrees (`ε/plain` iterations 0.999 at both levels).
* **What does depend on `End(E)` (measured).** (a) `|Aut|`: the known effect, 1.8× and 1.47×.
  (b) The cost of evaluating the cheapest explicit endomorphism (2.7×). (c) Nothing else tested.
* **Prior repository work.** `src/bin/volcano_rho.rs` (plain rho across strata with shared seeds)
  and `research/conductor_gap_cm_orbits_20261015` (53-bit, C++/ecbench: orbit-folded IC and rho;
  floor/crater native CPU 0.98 [0.87, 1.11], and the observation that the CM-orbit workload is
  reproducible at either level by the generic recurrence `P[k+2] = P[k+1] − [5]P[k]`) are consistent
  with this result: a generic scalar action removes the need for an explicit endomorphism. This
  study differs in size (toy), implementation (new Rust code), and the explicit Vélu endomorphisms.

## 5. Limits

* Toy sizes (`p < 2^22`, `r` between 4·10⁴ and 4·10⁶; rho takes ~200–1600 iterations). No
  extrapolation to cryptographic size; no sub-rho or deployed-curve claim.
* One vertical step (floor depth 1; `v_ℓ(f) = 1`); crater `h(D) ∈ {1, 2, 3}`; ℓ ≤ 13.
* The `ε` slot fraction is fixed at 2/16 (one point); the conclusion "ε does not reduce iterations"
  is for this walk design and `q = 1/8`. A different use of the endomorphism is not excluded.
* Exp B uses only `D ∈ {−19, −43, −67, −163}`, `n_ω = 5…41`; cofactor `n_ω` on the group order.
  `h > 1` endomorphism algorithms were not implemented (no small-norm generator is available from
  the same eigenline construction).
* Walk quality: the aut-class implementation reaches 96% of the ideal factor (cause not
  investigated); the baseline is measured on the crater itself so the cross-level comparison is
  unaffected.
* "Paired seeds" means the same RNG streams (step-table exponents, starts, hidden `k`) on every
  curve of an instance; the point streams differ because the curves differ.
* Timing is not used; all costs are counts.
* Multiple comparisons: Exp H per-discriminant intervals are unadjusted.

## 6. Commands

```
export CARGO_TARGET_DIR=<target> CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0
cargo +1.98 build --release --example r7_3_conductor_sensitive
research/isogeny_fibers_20261010/round7/r7_3_conductor_sensitive/run_all.sh \
    <target>/release/examples/r7_3_conductor_sensitive raw
python plot.py raw .          # plots recorded values only (matplotlib)
```
`run_all.sh` lists every invocation (Exp A: 8 instances per ℓ per `D ∈ {3, 4}`, 3000 runs; Exp B: 10
per ℓ, 600 runs, 3 census floors; Exp H: `--max 4 --sizes 100,200` per `(D, ℓ)`). The raw
records in `raw/` were produced by an identical earlier build of this source (debug prints removed
and clippy fixes applied afterwards); the final build reproduces `r7_3_expA_d3_ell5_i0.json` and
`r7_3_expB_ell5_i0.json` byte for byte. Gates: `rustfmt --edition 2021 --check`, `cargo +1.98
clippy --release --example r7_3_conductor_sensitive -- -D warnings` clean. The Exp H pairs from
the first list that yielded nothing are not in the script (see §1).

## 7. Figures and report

`fig1_expA_aut_baseline.svg`, `fig2_expB_endo_walk.svg`, `fig3_eps_cost_glv.svg`,
`fig4_expH_class_number.svg` (PNG copies alongside); `report.typ` / `report.pdf`.

## Note text (ready to paste into the note's §20)

**R7.3: conductor-sensitive algorithms across a volcano gap (toy, native).** Conjecture 10′ says a
prime-field algorithm whose cost depends on `End(E)` beyond `O^×` and `E(F_p)[h]` could separate a
crater from a floor across a prime conductor gap. R6.11 tested only a plain x-frame base. R7.3
implements automorphism-class rho (full `Aut(E)` at `j = 0, 1728` against `{±1}`) and a rho whose
walk applies an explicit Vélu endomorphism (`ω` on the crater, `ψωφ`, degree ℓ², on the floor), with
shared seeds, 32/32/40 instances and cluster-bootstrap 95% CIs, and extends the relation-yield census
to `h(D) = 2, 3` craters (56 instances). Results (measured): the automorphism group changes the rho
iteration count by 0.5534 [0.5511, 0.5559] (`j = 0`) and 0.6820 [0.6791, 0.6848] (`j = 1728`), the
known baseline; with it removed, the conductor effect is 1.0018 [0.9986, 1.0050] and 0.9997 [0.9966,
1.0030]; the endomorphism-mixed walk needs the same number of iterations at both levels (1.0001
[0.9921, 1.0080]) and no fewer than plain rho (0.9993 [0.989, 1.009]); the `h > 1` class factor is
0.9998 [0.9993, 1.0004]; a transported base has exactly the census of its source (120/120).

The one measured End-dependent gap is in counted multiplications of the endomorphism-mixed walk:
the floor costs 1.46× [1.44, 1.48] the crater, entirely from evaluating the cheapest explicit
non-scalar endomorphism (2.72× [2.59, 2.85] per evaluation, three Vélu maps instead of one, 2.5–2.9×
for ℓ = 5…13). That walk is 1.22× (crater) and 1.79× (floor) dearer than plain rho, so the best
algorithm here (ignore `ω`) has crater/floor cost 0.9999 [0.9945, 1.0055]. This is a polynomial-in-ℓ
constant on an inferior algorithm, consistent with Theorem 4 and not a change in the cost of the best
known attack.

Limits. Toy `p < 2^22`, one vertical step, ℓ ≤ 13, `h ≤ 3`; the endomorphism algorithms only for
`h = 1`; endomorphism slot fraction fixed at 1/8; the library `LineOracle`/`F_{p²}` ζ-fold and a
level-dependent index-calculus oracle were not run. Conjecture 10′ is untested-but-not-contradicted
at this scale for the algorithms tried; no claim about cryptographic sizes, sub-rho cost, or any
deployed curve. Files: `research/isogeny_fibers_20261010/round7/r7_3_conductor_sensitive/`.
