# Prime PKM Semaev / $T_2$-Torus Acceleration, `m >= 8` Specialization Ladder & Aligned P-256 Auxiliary Curve Chase

- **Date:** `2026-11-19`
- **Branch:** `cursor/prime-pkm-aux-tower-3858` ([PR #4820](https://github.com/isogeny-labs/crypto/pull/4820), tracked under [workspace PR #32](https://github.com/isogeny-labs/workspace/pull/32))
- **Machine-readable receipt:** `research/prime_pkm_aux_chase_20261119/RESULTS.json` (`schema = prime_pkm_aux_chase_20261119/v1`)
- **KB status:** `KB unavailable` (`scripts/research-kb` exited `127` in this worker environment)
- **Standing:** Every empirical measurement, timing, operation count, scaling fit, and unproved degree-slope projection in this report is tagged `[pcc:unverified]`.

---

## 1. Executive Summary & Discriminating Verdicts

| Lead / Hypothesis | Empirical & Mathematical Result | Scientific Verdict |
| --- | --- | --- |
| **P0-1c (a+b): Parent-to-child $\mathbb{F}_p$ basis inheritance + Schwartz–Zippel minimal-`1` refutation trace replay** (`f4_fp_tower.rs`, `pkm_semaev.rs`, `pkm_tower.rs`) | `100%` exact `[d]G == Q` recovery across all prime-algebraic rungs (`k = 12, 14, 16, 20` / `alg-k20`) and `M13`/`M31` Dickson-$T_2$ solves `[pcc:unverified]`. Cuts `alg-k20` (`icv1-fp20-t727-cd198a38`) wall time `21.30x` (`167.79s -> 7.88s`) and `k=14..16` wall time `22.50x–36.16x`, while cutting counted solver ops and priced `S(0.1)` by `1.16x–2.17x` on algebraic rungs and `1.28x–3.23x` on `M31` `[pcc:unverified]`. | **Engineering wall-time win; priced `S(w)` misses `>= 20x` target and remains far above Pollard $\rho$** (`after_s_w01 = 1.01e6` vs `rho_s_w01 = 0.84` on `alg-k20` `[pcc:unverified]`). |
| **P0-1c (c1): Curve-once $X_R$-power-split `ParamSymmetricSemaev` (`ParamS3`–`ParamS5`)** (`symmetrized_semaev.rs`) | Replaces per-target grid interpolation (`2,951,786` $\mathbb{F}_p$ mults at `m=4`) with curve-once offline interpolation plus `O(1)` scalar combinations in $X_R^j$ (`7`, `97`, and `3,547` $\mathbb{F}_p$ mults/target for `m = 2, 3, 4` = `264.3x`, `668.0x`, `832.2x` specialization speedup; `0` mismatches across random tuples and curve sums on `alg-k20` and `M31`) `[pcc:unverified]`. | **Hold (`264x–832x` specialization speedup)** `[pcc:unverified]`; eliminates per-target symmetric specialization bottleneck, leaving F4 linear algebra dominant. |
| **P0-1c (c2): Algebraic tower root descent vs $O(\|V\|^m)$ `grid()` fallback** (`pkm_semaev.rs`) | Cuts `grid()` $\mathbb{F}_p$ mults `6.01x` on `alg-k20` (`136,320 -> 22,677`) and `2.0x` on `k=16` `[pcc:unverified]`, and avoids $O(2^{mt})$ blowup as $t$ grows. However, on tiny `t = 3` (`\|V\| = 8`) Dickson `m = 3`, per-branch Tonelli–Shanks square roots cost `28,775` vs `9,792` `grid()` mults (`M31`) `[pcc:unverified]`. | **Mixed at tiny $t \le 3$; asymptotic win as $2^{mt}$ grows** `[pcc:unverified]`. Precomputing the static $2^t$ domain binary tree in `prepare()` would remove runtime `sqrt_fp` calls on Kummer/Dickson towers. |
| **P1-3a: `KM3-SPEC-LADDER` + Lean `GrowthRationalPrimeCutoff` check for `m = 8` Kummer & $T_2$-torus (`w = 1, 2`)** | Any linear degree floor $d_F(8, t) \ge c \cdot t$ with $c > 0$ gives $\alpha = c/8 > 0$, forcing $r^{\Omega(1)}$ (non-subexponential) complexity. However, Lean's unconditional `rational_prime_growth` slope $c = 1/14$ ($\alpha = 0.00893$) and Cayley half-degree transfer $c = 1/28$ ($\alpha = 0.00446$) lie strictly below the $\rho$-parity cutoff $\alpha^*$ (`kill_all = false, kill_one = false` at both $w=1, 2$), whereas sharp $c = 1.0$ ($\alpha = 0.125$) kills both Kummer and $T_2$-torus under both single-target floors (`E_all(w=1) = 0.6212 > 0.5`, `E_all(w=2) = 1.2424 > 0.5`) `[pcc:unverified]`. Empirical `f4_tower` runs confirm exact equality $d_F^{\text{Dickson}}(m,t) = d_F^{\text{Kummer}}(m,t)$ across all tested $(m, t)$ `[pcc:unverified]`. | **Subexponential $L_p(1/2)$ ruled out unconditionally; sub-$\rho$ ($r^{< 1/2}$) kill for $m \ge 8$ remains conditional on proving a sharper degree slope ($\alpha > 0.0903$ at $w=1$, $\alpha > 0.0350$ at $w=2$ under the $\gamma < 3.5$ floor)** `[pcc:unverified]`. |
| **P2-5: Aligned P-256 auxiliary curve (`w^2 = x^3 - 3x`, $j = 1728$) & scaled $p \equiv 3 \pmod 4$ towers** | Native `[u64; 4]` NIST P-256 Vélu 2-isogeny tower reproduces `FB1h16882188194e` (`288` signed points at `depth = 8`) byte-for-byte `[pcc:unverified]`. Across `dense`, `tower`, `paired`, and `oriented`, `tower` has the smallest column count (`10–15` cols at `d = 2..4`), while `paired`/`oriented` lower generator degree to `3` at the expense of `20–34` variables at `d = 8` `[pcc:unverified]`. On scaled `M19` / `M31` shared-$x$ 2-isogeny towers, trace replay gives `1.91x–3.38x` ops/call and `1.94x–8.41x` wall speedups without lowering `D = 5..6` `[pcc:unverified]`. | **No presentation Collapse on aligned $j = 1728$ auxiliary curve** `[pcc:unverified]`; trace replay accelerates constants (`1.9x–3.4x` ops) without changing degree growth. |
| **`CONTINUATION_PROTOCOL.md` Panel A: Bounded positive-call scaling $\hat{\gamma}$ ($\le 4\text{ GiB}$ RAM)** | Whole-call positive cost $c_{\text{pos}}(B) = B^{\hat{\gamma}}$ on Kummer (`p = 786433`) and $T_2$-Dickson (`p = 131071`) fits $\hat{\gamma}_{\text{pos}} = 3.884$ (`[3.487, 4.281]`) and $3.858$ (`[3.490, 4.226]`) at `m = 2, t = 2..5`, jumping to $\hat{\gamma}_{\text{pos}} = 7.669$ (`[6.102, 9.237]`) and $7.660$ (`[6.402, 8.918]`) at `m = 3, t = 1..3` `[pcc:unverified]`. | **`KILL_AT_MEASURED_SCALE` for $m \ge 3$ ($\gamma_{\text{LCB}} > 6.10 \gg 3.5$)** `[pcc:unverified]`; Kummer and $T_2$-Dickson scale identically within `< 0.03` in $\gamma$ `[pcc:unverified]`. |

---

## 2. Lead P0-1c: Native Architectural Changes, Bugfix & End-to-End Benchmarks

### 2.1 Implemented Code Changes (Within Owned Scope)

1. **`src/cryptanalysis/f4_fp_tower.rs` — Schwartz–Zippel Minimal-`1`-Witness `CompiledRefutation` Replay & `f4_tower_incremental`:**
   - Added `f4_tower_incremental` so child split nodes start directly from a parent's reduced $\mathbb{F}_p$ tower Gröbner basis instead of rebuilding target-independent S-pairs from scratch.
   - Added `f4_tower_learn_refutation` and `f4_tower_replay_refutation` (`CompiledRefutation`): during `prepare()` calibration on a random UNSAT abscissa $x_R \in \mathbb{F}_p$, we record each F4 degree step's exact column indexing, pivot row order, row-reduction sequence, and backward-slice only the ancestor basis elements required to derive $1 \in G$. Per online target, `f4_tower_replay_refutation` replays only those rows over $\mathbb{F}_p$; if any pivot vanishes ($\Pr \le D/p$ by Schwartz–Zippel) or `1` is not produced, it falls back cleanly to full `f4_tower` (`0` false negatives across all unit and end-to-end tests `[pcc:unverified]`).
2. **`src/cryptanalysis/symmetrized_semaev.rs` — Curve-Once $X_R$-Power-Split `ParamSymmetricSemaev` (`ParamS3`–`ParamS5`):**
   - Exploits Semaev degree $\le 2^{m-1}$ in $X_R$ (`3, 5, 9` $X_R$-powers for `m = 2, 3, 4`) to interpolate $\tilde{S}_{m+1}(e_1, \dots, e_m; X_R) = \sum_{j=0}^{2^{m-1}} C_j(e_1, \dots, e_m) X_R^j$ once per curve in `prepare()`, reducing per-target specialization to `O(1)` Horner / linear combinations in $X_R^j$.
3. **`src/cryptanalysis/ic_framework/pkm_semaev.rs` & `src/bin/ecbench/pkm_tower.rs` — Parent-to-Child Basis Inheritance, Algebraic Root Descent & Upstream `Isogeny` `guess > 0` Bugfix:**
   - Wired `InheritedF4` / `LeafCompiled` into `PkmSemaevOracle` (`trace=0|1,inherit=0|1,descent=0|1`) and `PkmTowerOracle`, plus `Membership::Dickson` ($T_2$-torus quadratic tower $y_{i,j}^2 - y_{i,j+1} y_{i,j} + 1 = 0$) and algebraic univariate/quadratic basis descent eliminating the exponential $O(|V|^m)$ `grid()` scan when `descent=1`.
   - **Upstream bug fixed (`src/cryptanalysis/ic_framework/pkm_semaev.rs:1799`):** Under `Membership::Isogeny` with `guess > 0`, dummy variables at levels `j >= t - guess` previously failed `j + 1 == t - guess` (`j + 1 > t - guess`), leaving `next: Some(...)` pointing across summand boundaries into summand `i + 1` instead of `next: None` (`y_{i,j} = 0`). Fixed to `j + 1 >= t - self.guess as usize`, restoring `100%` soundness for `Isogeny` with `guess >= 2`.

### 2.2 `ParamS3`–`ParamS5` Curve-Once Specialization Audit (`RESULTS.json`)

| Curve / Field | $m$ | Polynomial | Active $e$-Monomials | Offline Setup Mults (Once) | Online Spec Mults / Target | Baseline Grid Interp Mults | Spec Speedup | Mismatches (48 Random + 24 Curve Sums) |
| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `alg-k20` ($p = 563{,}153$) | 2 | `ParamS3` | 6 `[pcc:unverified]` | 659 `[pcc:unverified]` | 7 `[pcc:unverified]` | 1,850 `[pcc:unverified]` | **264.3x** `[pcc:unverified]` | 0 `[pcc:unverified]` |
| `alg-k20` ($p = 563{,}153$) | 3 | `ParamS4` | 35 `[pcc:unverified]` | 21,785 `[pcc:unverified]` | 97 `[pcc:unverified]` | 64,800 `[pcc:unverified]` | **668.0x** `[pcc:unverified]` | 0 `[pcc:unverified]` |
| `alg-k20` ($p = 563{,}153$) | 4 | `ParamS5` | 495 `[pcc:unverified]` | 27,207,299 `[pcc:unverified]` | 3,547 `[pcc:unverified]` | 2,951,786 `[pcc:unverified]` | **832.2x** `[pcc:unverified]` | 0 `[pcc:unverified]` |
| `M31` ($p = 2^{31}-1$) | 2 | `ParamS3` | 6 `[pcc:unverified]` | 920 `[pcc:unverified]` | 7 `[pcc:unverified]` | 1,850 `[pcc:unverified]` | **264.3x** `[pcc:unverified]` | 0 `[pcc:unverified]` |
| `M31` ($p = 2^{31}-1$) | 3 | `ParamS4` | 35 `[pcc:unverified]` | 22,945 `[pcc:unverified]` | 97 `[pcc:unverified]` | 64,800 `[pcc:unverified]` | **668.0x** `[pcc:unverified]` | 0 `[pcc:unverified]` |
| `M31` ($p = 2^{31}-1$) | 4 | `ParamS5` | 495 `[pcc:unverified]` | 27,351,110 `[pcc:unverified]` | 3,547 `[pcc:unverified]` | 2,951,786 `[pcc:unverified]` | **832.2x** `[pcc:unverified]` | 0 `[pcc:unverified]` |

### 2.3 Prime-Algebraic ECDLP Ladder (`k = 12, 14, 16, 20` / `alg-k20`) vs Matched `rho.negation`

All runs execute full `ecbench` blind `ic` solves (`Before: trace=0,inherit=0,descent=0` vs `After: trace=1,inherit=1,descent=1`) and verify `[d]G == Q` on the curve (`verified_dlog = true` on all 8 solves `[pcc:unverified]`):

| Rung (`slug`) | Target | Oracle Config | `rho` $S(0 / 0.1 / 1)$ | Before $S(0 / 0.1 / 1)$ | After $S(0 / 0.1 / 1)$ | Solver Ops (`Before -> After`) | `S(0.1)` Speedup | Wall (`Before -> After`) | Wall Speedup |
| --- | ---: | --- | --- | --- | --- | --- | ---: | --- | ---: |
| `k=12` (`icv1-fp12-tm53-6c9ff4bb`) | 0 | `m=2,guess=2` | `4.39 / 4.72 / 7.63` `[pcc:unverified]` | `15.62 / 12,360.29 / 123,462.27` `[pcc:unverified]` | `15.62 / 10,657.37 / 106,433.08` `[pcc:unverified]` | `7.38M -> 6.36M` `[pcc:unverified]` | **1.16x** `[pcc:unverified]` | `991.0ms -> 306.7ms` `[pcc:unverified]` | **3.23x** `[pcc:unverified]` |
| `k=12` (`icv1-fp12-tm53-6c9ff4bb`) | 1 | `m=2,guess=2` | `2.54 / 2.69 / 4.09` `[pcc:unverified]` | `15.05 / 9,031.27 / 90,177.25` `[pcc:unverified]` | `15.05 / 7,720.95 / 77,074.03` `[pcc:unverified]` | `5.39M -> 4.60M` `[pcc:unverified]` | **1.17x** `[pcc:unverified]` | `610.5ms -> 208.3ms` `[pcc:unverified]` | **2.93x** `[pcc:unverified]` |
| `k=14` (`icv1-fp14-tm143-dec4cf6b`) | 0 | `m=2,guess=2` | `2.06 / 2.21 / 3.54` `[pcc:unverified]` | `34.48 / 56,008.88 / 559,778.50` `[pcc:unverified]` | `34.48 / 34,262.66 / 342,316.23` `[pcc:unverified]` | `67.38M -> 41.16M` `[pcc:unverified]` | **1.63x** `[pcc:unverified]` | `8,432.2ms -> 233.2ms` `[pcc:unverified]` | **36.16x** `[pcc:unverified]` |
| `k=14` (`icv1-fp14-tm143-dec4cf6b`) | 1 | `m=2,guess=2` | `1.61 / 1.71 / 2.57` `[pcc:unverified]` | `29.18 / 29,590.22 / 295,639.58` `[pcc:unverified]` | `29.18 / 18,116.13 / 180,898.64` `[pcc:unverified]` | `35.58M -> 21.75M` `[pcc:unverified]` | **1.63x** `[pcc:unverified]` | `4,348.8ms -> 126.8ms` `[pcc:unverified]` | **34.29x** `[pcc:unverified]` |
| `k=16` (`icv1-fp16-t295-8d3c3165`) | 0 | `m=2,guess=2` | `2.38 / 2.55 / 4.06` `[pcc:unverified]` | `29.88 / 472,959.51 / 4,729,326.23` `[pcc:unverified]` | `29.88 / 231,129.28 / 2,311,023.94` `[pcc:unverified]` | `1.090B -> 532.5M` `[pcc:unverified]` | **2.05x** `[pcc:unverified]` | `22,843.2ms -> 749.1ms` `[pcc:unverified]` | **30.49x** `[pcc:unverified]` |
| `k=16` (`icv1-fp16-t295-8d3c3165`) | 1 | `m=2,guess=2` | `1.27 / 1.33 / 1.88` `[pcc:unverified]` | `33.42 / 612,314.10 / 6,122,840.23` `[pcc:unverified]` | `33.42 / 299,114.20 / 2,990,841.19` `[pcc:unverified]` | `1.411B -> 689.2M` `[pcc:unverified]` | **2.05x** `[pcc:unverified]` | `21,384.4ms -> 950.4ms` `[pcc:unverified]` | **22.50x** `[pcc:unverified]` |
| **`k=20` (`icv1-fp20-t727-cd198a38`, `alg-k20`)** | **0** | **`m=2,guess=3`** | **`0.78 / 0.84 / 1.34`** `[pcc:unverified]` | **`14.26 / 2,200,750.96 / 22,007,381.24`** `[pcc:unverified]` | **`14.26 / 1,012,019.19 / 10,120,063.56`** `[pcc:unverified]` | **`16.50B -> 7.59B`** `[pcc:unverified]` | **2.17x** `[pcc:unverified]` | **`167,787.3ms -> 7,877.0ms`** `[pcc:unverified]` | **21.30x** `[pcc:unverified]` |
| `k=12` (`icv1-fp12-tm53-6c9ff4bb`) | 0 | `m=3,guess=1,mode=sym` | `4.39 / 4.72 / 7.63` `[pcc:unverified]` | `20.18 / 32.92M / 329.20M` `[pcc:unverified]` | `20.18 / 35.51M / 355.08M` `[pcc:unverified]` | `19.71B -> 21.25B` `[pcc:unverified]` | **0.93x** `[pcc:unverified]` | `11,578.2ms -> 12,471.9ms` `[pcc:unverified]` | **0.93x** `[pcc:unverified]` |

- **Why wall speedup (`21.3x–36.2x`) vastly exceeds counted-op speedup (`1.63x–2.17x`) on `k=14..20` `[pcc:unverified]`:** Scratch `f4_tower` spends most of its wall clock in symbolic preprocessing (hash-table monomial lookup, `BTreeMap` pair-queue maintenance, reducer closure, and interreduction) which `f4_fp_tower` does not charge as $\mathbb{F}_p$ muladds; `CompiledRefutation` eliminates `100%` of online symbolic preprocessing on UNSAT leaves while cutting actual $\mathbb{F}_p$ row muladds by `2.05x–2.17x` `[pcc:unverified]`.
- **Why `k=12, m=3, guess=1, mode=sym` regressed slightly (`0.93x`) `[pcc:unverified]`:** Over small `p = 3529` with `8` leaf branches (`guess = 1`), `prepare()` pays calibration F4 work on all `8` leaves, and pivot-zero fallbacks ($\Pr \approx D/3529$) trigger occasional full F4 re-runs on top of the replay attempt.

### 2.4 `M13` & `M31` ($p = 2^{31}-1$) Dickson / $T_2$-Torus End-to-End ECDLP Solves (`m = 3, 4, 5`)

Every solve recovers the exact secret discrete logarithm via mod-$r$ Gaussian elimination (`verified_dlog = true` across all 6 rows `[pcc:unverified]`):

| Curve / Protocol | $m$ | $t$ | Mode | `rho` $S(0.1)$ | Before Ops/Call | After Ops/Call | Ops/Call Speedup | Before $S(0.1)$ | After $S(0.1)$ | `S(0.1)` Speedup | Wall (`Before -> After`) | Wall Speedup |
| --- | ---: | ---: | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- | ---: |
| `M13` ($p = 8191$, blind walk) | 3 | 3 | `chain` | `2.05` `[pcc:unverified]` | `164.32M` `[pcc:unverified]` | `166.57M` `[pcc:unverified]` | `0.99x` `[pcc:unverified]` | `4.96e7` `[pcc:unverified]` | `5.03e7` `[pcc:unverified]` | `0.99x` `[pcc:unverified]` | `9,716.6ms -> 10,031.6ms` `[pcc:unverified]` | `0.97x` `[pcc:unverified]` |
| `M13` ($p = 8191$, blind walk) | 4 | 2 | `sym` | `2.05` `[pcc:unverified]` | `136.67M` `[pcc:unverified]` | `55.52M` `[pcc:unverified]` | **2.46x** `[pcc:unverified]` | `3.00e7` `[pcc:unverified]` | `1.22e7` `[pcc:unverified]` | **2.46x** `[pcc:unverified]` | `9,722.6ms -> 2,896.8ms` `[pcc:unverified]` | **3.36x** `[pcc:unverified]` |
| `M31` ($p = 2^{31}-1$, $T_2$ + LA) | 3 | 3 | `chain` | `1.12` `[pcc:unverified]` | `164.98M` `[pcc:unverified]` | `84.28M` `[pcc:unverified]` | **1.96x** `[pcc:unverified]` | `6.67e10` `[pcc:unverified]` | `2.07e10` `[pcc:unverified]` | **3.23x** `[pcc:unverified]` | `1,545.8ms -> 1,060.4ms` `[pcc:unverified]` | **1.46x** `[pcc:unverified]` |
| `M31` ($p = 2^{31}-1$, $T_2$ + LA) | 4 | 2 | `chain` | `1.12` `[pcc:unverified]` | `50.30M` `[pcc:unverified]` | `41.60M` `[pcc:unverified]` | **1.21x** `[pcc:unverified]` | `1.13e12` `[pcc:unverified]` | `7.87e11` `[pcc:unverified]` | **1.43x** `[pcc:unverified]` | `936.6ms -> 587.9ms` `[pcc:unverified]` | **1.59x** `[pcc:unverified]` |
| `M31` ($p = 2^{31}-1$, $T_2$ + LA) | 4 | 2 | `sym` | `1.12` `[pcc:unverified]` | `136.78M` `[pcc:unverified]` | `73.08M` `[pcc:unverified]` | **1.87x** `[pcc:unverified]` | `3.17e12` `[pcc:unverified]` | `1.19e12` `[pcc:unverified]` | **2.66x** `[pcc:unverified]` | `1,846.5ms -> 1,015.5ms` `[pcc:unverified]` | **1.82x** `[pcc:unverified]` |
| `M31` ($p = 2^{31}-1$, $T_2$ + LA) | 5 | 2 | `chain` | `1.12` `[pcc:unverified]` | `992.34M` `[pcc:unverified]` | `974.15M` `[pcc:unverified]` | **1.02x** `[pcc:unverified]` | `2.66e13` `[pcc:unverified]` | `2.08e13` `[pcc:unverified]` | **1.28x** `[pcc:unverified]` | `3,947.9ms -> 4,625.0ms` `[pcc:unverified]` | `0.85x` `[pcc:unverified]` |

- **Root-cause diagnosis of the `m = 3, t = 3` Dickson `grid()` vs `descent` crossover (`9,792` vs `28,775` mults on `M31`) `[pcc:unverified]`:** At `t = 3` (`|V| = 8`), `grid()` evaluates $S_4$ over $8^3 = 512$ triples (`~1,088` $\mathbb{F}_p$ mults per SAT call). By contrast, `solve_univariate_roots_in_domain` invokes modular square roots (`sqrt_fp`, costing $\approx \log_2 p = 31$ squarings per quadratic discriminant on `M31`) across partially specialized Gröbner basis polynomials at each recursion node (`~3,197` mults per SAT call `[pcc:unverified]`). On `alg-k20` (`t = 6`, $|V| = 64$), `descent` wins `6.01x` (`136,320 -> 22,677` mults `[pcc:unverified]`), and precomputing the static $2^t$-node quadratic root table per tower level in `prepare()` would eliminate runtime `sqrt_fp` calls altogether.

---

## 3. Lead P1-3a: `KM3-SPEC-LADDER` + `GrowthRationalPrimeCutoff` Specialization Check (`m = 8`)

### 3.1 Review of `GrowthRationalPrimeCutoff.lean` & P-256 ($a = -3$) Scope

In `/Volumes/SSD990-2/lfd-summation-systems/formal/ICFallDegrees/GrowthRationalPrimeCutoff.lean`, `rational_prime_growth` proves that for `m = 2` (`S_3`) over $\mathbb{Q}$ with $a \in \mathbb{Q} \setminus \{0, -1, -3\}$, $b = -1 - a$, and binary-indicator / cyclic-shift coordinates of dimension $k \ge 24$:
\[
\frac{k - 10}{14} < \text{LFD} \le k + 2.
\]
- **Why $a = -3$ is excluded in `PrimeM2General`:** Under the normalization $b = -1 - a$ (putting $x = 1$ on $y^2 = x^3 + a x + b$), setting $a = -3$ forces $b = 2$, which has discriminant $\Delta = -16(4(-3)^3 + 27 \cdot 2^2) = 0$ (the singular nodal cubic $y^2 = (x-1)^2(x+2)$). NIST P-256 has $a = -3$ with non-singular $b_{\text{P256}} \not\equiv \pm 2 \pmod p$, so rescaling P-256 to $b' = -1 - a'$ shifts $a' \neq -3$; verified in `RESULTS.json` (`lean_cutoff_checks`, `12` rational parameter checks `[pcc:unverified]`).

### 3.2 `KM3-SPEC-LADDER` Monotonicity & Exponent Table for `m = 8` (`w = 1, 2`)

Specializing $y_{m, j} = c_j$ fixes summand $m$ to a single curve point $P_m \in V$ and reduces the $m$-summand $S_{m+1}$ tower system to the $(m-1)$-summand $S_m$ system with shifted target $R' = R - P_m$, giving $d_F(m, t) \ge d_F(m - 1, t) \ge \dots \ge d_F(2, t)$. Over $N = m t$ tower variables with $d_F(m, t) \sim c \cdot t = \alpha N$ ($\alpha = c / m$), the Macaulay column count scales as $\binom{N}{\alpha N} \asymp 2^{h(\alpha) N} = B^{m h(\alpha)}$, so per-call F4 work scales as $B^\gamma$ with $\gamma = w m h(\alpha)$ and optimal subgroup-order exponent (`B = r^\beta`):
- **Pre-floor:** $E_{\text{pre}} = \min_\beta \max(\beta, 1 - m\beta + \beta \gamma) = \frac{1}{1 + m - \gamma}$ for $\gamma < m$ (`\alpha_1^* = 0.15614`, `\alpha_2^* = 0.05628` vs $1/2$ `[pcc:unverified]`, matching `research/prime_leads_chase_20261113/tables.md`).
- **All-output single-target floor (`CONTINUATION_PROTOCOL.md`, requires $\gamma < 3.5$ at $m=8$):** $E_{\text{all}} = \min_\beta \max(\beta, 1 - m\beta + \beta\gamma, \frac{1}{2} - \frac{m-1}{2}\beta + \beta\gamma)$, yielding critical slope $\alpha^*_{\text{all}}(w=1) = 0.09031$ and $\alpha^*_{\text{all}}(w=2) = 0.03497$ `[pcc:unverified]`.
- **One-output single-target floor (requires $\gamma < 3.0$ at $m=8$):** $E_{\text{one}} = \min_\beta \max(\beta, 1 - m\beta + \beta\gamma, \frac{1}{2} - \frac{m-2}{2}\beta + \beta\gamma)$, yielding $\alpha^*_{\text{one}}(w=1) = 0.07245$ and $\alpha^*_{\text{one}}(w=2) = 0.02864$ `[pcc:unverified]`.

| Tower Family | Degree-Slope Source ($c$ in $d_F \ge c \cdot t$) | $w$ | $\alpha = c/8$ | $\gamma = 8 w h(\alpha)$ | $E_{\text{pre}}$ | $E_{\text{all}}$ | $E_{\text{one}}$ | Rules Out $r^{o(1)}$? | Killed ($E_{\text{all}} \ge 0.5$)? | Killed ($E_{\text{one}} \ge 0.5$)? |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | :---: | :---: | :---: |
| Kummer ($2^t \mid p-1$) | Lean `rational_prime_growth` ($c = 1/14$) | 1 | `0.00893` `[pcc:unverified]` | `0.589` `[pcc:unverified]` | `0.2378` `[pcc:unverified]` | `0.2378` `[pcc:unverified]` | `0.2500` `[pcc:unverified]` | **Yes** | `false` `[pcc:unverified]` | `false` `[pcc:unverified]` |
| Kummer ($2^t \mid p-1$) | Lean `rational_prime_growth` ($c = 1/14$) | 2 | `0.00893` `[pcc:unverified]` | `1.178` `[pcc:unverified]` | `0.2557` `[pcc:unverified]` | `0.2557` `[pcc:unverified]` | `0.2722` `[pcc:unverified]` | **Yes** | `false` `[pcc:unverified]` | `false` `[pcc:unverified]` |
| Kummer ($2^t \mid p-1$) | Sharp $m=2$ Kummer LFD $d_F \ge t$ ($c = 1.0$) | 1 | `0.12500` `[pcc:unverified]` | `4.349` `[pcc:unverified]` | `0.4300` `[pcc:unverified]` | **`0.6212`** `[pcc:unverified]` | **`0.6686`** `[pcc:unverified]` | **Yes** | **`true`** `[pcc:unverified]` | **`true`** `[pcc:unverified]` |
| Kummer ($2^t \mid p-1$) | Sharp $m=2$ Kummer LFD $d_F \ge t$ ($c = 1.0$) | 2 | `0.12500` `[pcc:unverified]` | `8.697` `[pcc:unverified]` | **`1.2424`** `[pcc:unverified]` | **`1.2424`** `[pcc:unverified]` | **`1.2121`** `[pcc:unverified]` | **Yes** | **`true`** `[pcc:unverified]` | **`true`** `[pcc:unverified]` |
| $T_2$ Torus / Dickson ($2^t \mid p+1$) | Cayley $\mathbb{F}_{p^2}$ half-degree of Lean ($c = 1/28$) | 1 | `0.00446` `[pcc:unverified]` | `0.330` `[pcc:unverified]` | `0.2307` `[pcc:unverified]` | `0.2307` `[pcc:unverified]` | `0.2500` `[pcc:unverified]` | **Yes** | `false` `[pcc:unverified]` | `false` `[pcc:unverified]` |
| $T_2$ Torus / Dickson ($2^t \mid p+1$) | Cayley $\mathbb{F}_{p^2}$ half-degree of Lean ($c = 1/28$) | 2 | `0.00446` `[pcc:unverified]` | `0.660` `[pcc:unverified]` | `0.2398` `[pcc:unverified]` | `0.2398` `[pcc:unverified]` | `0.2500` `[pcc:unverified]` | **Yes** | `false` `[pcc:unverified]` | `false` `[pcc:unverified]` |
| $T_2$ Torus / Dickson ($2^t \mid p+1$) | Cayley half-degree of sharp $d_F \ge t$ ($c = 0.5$) | 1 | `0.06250` `[pcc:unverified]` | `2.698` `[pcc:unverified]` | `0.3174` `[pcc:unverified]` | `0.3855` `[pcc:unverified]` | `0.4623` `[pcc:unverified]` | **Yes** | `false` `[pcc:unverified]` | `false` `[pcc:unverified]` |
| $T_2$ Torus / Dickson ($2^t \mid p+1$) | Cayley half-degree of sharp $d_F \ge t$ ($c = 0.5$) | 2 | `0.06250` `[pcc:unverified]` | `5.397` `[pcc:unverified]` | **`0.5550`** `[pcc:unverified]` | **`0.7709`** `[pcc:unverified]` | **`0.7996`** `[pcc:unverified]` | **Yes** | **`true`** `[pcc:unverified]` | **`true`** `[pcc:unverified]` |
| $T_2$ Torus / Dickson ($2^t \mid p+1$) | Direct leading-ideal $y_j^2 \to y_{j+1}+2$ ($c = 1.0$) | 1 | `0.12500` `[pcc:unverified]` | `4.349` `[pcc:unverified]` | `0.4300` `[pcc:unverified]` | **`0.6212`** `[pcc:unverified]` | **`0.6686`** `[pcc:unverified]` | **Yes** | **`true`** `[pcc:unverified]` | **`true`** `[pcc:unverified]` |
| $T_2$ Torus / Dickson ($2^t \mid p+1$) | Direct leading-ideal $y_j^2 \to y_{j+1}+2$ ($c = 1.0$) | 2 | `0.12500` `[pcc:unverified]` | `8.697` `[pcc:unverified]` | **`1.2424`** `[pcc:unverified]` | **`1.2424`** `[pcc:unverified]` | **`1.2121`** `[pcc:unverified]` | **Yes** | **`true`** `[pcc:unverified]` | **`true`** `[pcc:unverified]` |

### 3.3 Empirical `KM3-SPEC-LADDER` Equivalence Between Kummer and $T_2$-Dickson (`RESULTS.json`)

Running `f4_tower` on Kummer ($p = 65{,}537$) and $T_2$-Dickson ($p = 131{,}071$) confirms both $d_F(m, t) \ge d_F(m-1, t)$ and **exact solving-degree equality** $d_F^{\text{Dickson}}(m, t) = d_F^{\text{Kummer}}(m, t)$ (because the top Grevlex leading monomial of $y_{i,j}^2 - y_{i,j+1} y_{i,j} + 1$ has the same quadratic degree-2 staircase as $y_{i,j}^2 - y_{i,j+1}$ when ordered properly, with `max_cols` matching within `< 3%` `[pcc:unverified]`):
- $t = 2$, $m = 2, 3, 4, 5$: $d_F = 4, 5, 6, 6$ on both Kummer (`cols = 16, 164, 1049, 5525`) and $T_2$-Dickson (`cols = 16, 164, 1080, 5525`) `[pcc:unverified]`.
- $t = 3$, $m = 2, 3, 4$: $d_F = 4, 6, 7$ on both Kummer (`cols = 56, 1186, 9452`) and $T_2$-Dickson (`cols = 56, 1184, 9451`) `[pcc:unverified]`.

---

## 4. Lead P2-5: Exact 256-Bit Aligned P-256 Auxiliary Curve (`w^2 = x^3 - 3x`) & Scaled $p \equiv 3 \pmod 4$ Towers

### 4.1 Exact 256-Bit NIST P-256 Auxiliary Base & Presentation Bakeoff (`RESULTS.json`)

Our native `[u64; 4]` NIST P-256 arithmetic and Vélu 2-isogeny tower (`src/p2_5_p256_aux.rs`) reproduces the exact SHA-256 factor-base identifiers of `research/p256_aux_smooth_20261114` (`FB1hfa6130558c05` at `d=1`, `FB1hb6856918fe35` at `d=2`, `FB1h1188227591e8` at `d=3`, `FB1h2704fe49495e` at `d=4`, and **`FB1h16882188194e` with `288` signed points at `d=8`** `[pcc:unverified]`), with `planted_witness_verified = true` across all 15 rows `[pcc:unverified]`:

| Depth | `fb_id` | Signed Pts | Presentation Arm | Vars | Eqs | Planted OK? | Degree Cap | Unplanted Refuted? | Max Step Deg | Max Cols | Full Muladds | Replay Muladds | Replay Speedup |
| ---: | --- | ---: | --- | ---: | ---: | :---: | ---: | :---: | ---: | ---: | ---: | ---: | ---: |
| 1 | `FB1hfa6130558c05` | 2 `[pcc:unverified]` | `dense` | 2 `[pcc:unverified]` | 3 `[pcc:unverified]` | `true` `[pcc:unverified]` | 7 | `true` `[pcc:unverified]` | 4 `[pcc:unverified]` | 9 `[pcc:unverified]` | 9 `[pcc:unverified]` | 9 `[pcc:unverified]` | `1.00x` `[pcc:unverified]` |
| 1 | `FB1hfa6130558c05` | 2 `[pcc:unverified]` | `tower` | 2 `[pcc:unverified]` | 3 `[pcc:unverified]` | `true` `[pcc:unverified]` | 7 | `true` `[pcc:unverified]` | 4 `[pcc:unverified]` | 9 `[pcc:unverified]` | 92 `[pcc:unverified]` | 75 `[pcc:unverified]` | **`1.23x`** `[pcc:unverified]` |
| 1 | `FB1hfa6130558c05` | 2 `[pcc:unverified]` | `paired` | 6 `[pcc:unverified]` | 7 `[pcc:unverified]` | `true` `[pcc:unverified]` | 7 | `false` `[pcc:unverified]` | 3 `[pcc:unverified]` | 23 `[pcc:unverified]` | 428 `[pcc:unverified]` | 428 `[pcc:unverified]` | `1.00x` `[pcc:unverified]` |
| 1 | `FB1hfa6130558c05` | 2 `[pcc:unverified]` | `oriented` | 6 `[pcc:unverified]` | 9 `[pcc:unverified]` | `true` `[pcc:unverified]` | 7 | `false` `[pcc:unverified]` | 3 `[pcc:unverified]` | 37 `[pcc:unverified]` | 770 `[pcc:unverified]` | 770 `[pcc:unverified]` | `1.00x` `[pcc:unverified]` |
| 2 | `FB1hb6856918fe35` | 4 `[pcc:unverified]` | `dense` | 2 `[pcc:unverified]` | 3 `[pcc:unverified]` | `true` `[pcc:unverified]` | 6 | `true` `[pcc:unverified]` | 4 `[pcc:unverified]` | 9 `[pcc:unverified]` | 88 `[pcc:unverified]` | 71 `[pcc:unverified]` | **`1.24x`** `[pcc:unverified]` |
| 2 | `FB1hb6856918fe35` | 4 `[pcc:unverified]` | `tower` | 4 `[pcc:unverified]` | 5 `[pcc:unverified]` | `true` `[pcc:unverified]` | 6 | `false` `[pcc:unverified]` | 4 `[pcc:unverified]` | 15 `[pcc:unverified]` | 99 `[pcc:unverified]` | 99 `[pcc:unverified]` | `1.00x` `[pcc:unverified]` |
| 2 | `FB1hb6856918fe35` | 4 `[pcc:unverified]` | `paired` | 8 `[pcc:unverified]` | 9 `[pcc:unverified]` | `true` `[pcc:unverified]` | 5 | `false` `[pcc:unverified]` | 3 `[pcc:unverified]` | 35 `[pcc:unverified]` | 918 `[pcc:unverified]` | 918 `[pcc:unverified]` | `1.00x` `[pcc:unverified]` |
| 2 | `FB1hb6856918fe35` | 4 `[pcc:unverified]` | `oriented` | 10 `[pcc:unverified]` | 13 `[pcc:unverified]` | `true` `[pcc:unverified]` | 5 | `false` `[pcc:unverified]` | 3 `[pcc:unverified]` | 81 `[pcc:unverified]` | 1,066 `[pcc:unverified]` | 1,066 `[pcc:unverified]` | `1.00x` `[pcc:unverified]` |
| 3 | `FB1h1188227591e8` | 6 `[pcc:unverified]` | `dense` | 2 `[pcc:unverified]` | 3 `[pcc:unverified]` | `true` `[pcc:unverified]` | 5 | `true` `[pcc:unverified]` | 5 `[pcc:unverified]` | 15 `[pcc:unverified]` | 478 `[pcc:unverified]` | 323 `[pcc:unverified]` | **`1.48x`** `[pcc:unverified]` |
| 3 | `FB1h1188227591e8` | 6 `[pcc:unverified]` | `tower` | 6 `[pcc:unverified]` | 7 `[pcc:unverified]` | `true` `[pcc:unverified]` | 5 | `false` `[pcc:unverified]` | 4 `[pcc:unverified]` | 15 `[pcc:unverified]` | 107 `[pcc:unverified]` | 107 `[pcc:unverified]` | `1.00x` `[pcc:unverified]` |
| 3 | `FB1h1188227591e8` | 6 `[pcc:unverified]` | `paired` | 10 `[pcc:unverified]` | 11 `[pcc:unverified]` | `true` `[pcc:unverified]` | 5 | `false` `[pcc:unverified]` | 3 `[pcc:unverified]` | 23 `[pcc:unverified]` | 370 `[pcc:unverified]` | 370 `[pcc:unverified]` | `1.00x` `[pcc:unverified]` |
| 4 | `FB1h2704fe49495e` | 18 `[pcc:unverified]` | `tower` | 8 `[pcc:unverified]` | 9 `[pcc:unverified]` | `true` `[pcc:unverified]` | 4 | `false` `[pcc:unverified]` | 4 `[pcc:unverified]` | 10 `[pcc:unverified]` | 112 `[pcc:unverified]` | 112 `[pcc:unverified]` | `1.00x` `[pcc:unverified]` |
| 8 | `FB1h16882188194e` | 288 `[pcc:unverified]` | `tower (deg<=3)` | 16 `[pcc:unverified]` | 17 `[pcc:unverified]` | `true` `[pcc:unverified]` | 4 | `false` `[pcc:unverified]` | 0 `[pcc:unverified]` | 0 `[pcc:unverified]` | 56 `[pcc:unverified]` | 56 `[pcc:unverified]` | `1.00x` `[pcc:unverified]` |
| 8 | `FB1h16882188194e` | 288 `[pcc:unverified]` | `paired (deg<=3)` | 20 `[pcc:unverified]` | 21 `[pcc:unverified]` | `true` `[pcc:unverified]` | 4 | `false` `[pcc:unverified]` | 3 `[pcc:unverified]` | 23 `[pcc:unverified]` | 410 `[pcc:unverified]` | 410 `[pcc:unverified]` | `1.00x` `[pcc:unverified]` |
| 8 | `FB1h16882188194e` | 288 `[pcc:unverified]` | `oriented (deg<=3)` | 34 `[pcc:unverified]` | 37 `[pcc:unverified]` | `true` `[pcc:unverified]` | 4 | `false` `[pcc:unverified]` | 3 `[pcc:unverified]` | 145 `[pcc:unverified]` | 906 `[pcc:unverified]` | 906 `[pcc:unverified]` | `1.00x` `[pcc:unverified]` |

### 4.2 Scaled $p \equiv 3 \pmod 4$ Prime-Order Curves (`M19`, `M31`) with Aligned $w^2 = x^3 - 3x$ Auxiliary Towers

On `M19` ($p = 2^{19}-1$) and `M31` ($p = 2^{31}-1$), `#E_aux(F_p) = p + 1 = 2^k` provides full rational 2-isogeny towers sharing the $a = -3$ linear term (`y^2 = x^3 - 3x + B`), evaluated via `PkmSemaevOracle::new_isogeny` (`32` natural UNSAT targets + `1` planted $m$-sum control, `planted_hit_verified = true` on all 6 rows `[pcc:unverified]`):

| Curve (`a = -3`) | Depth | $m$ | FB Cols | Solving Deg $D$ | Max Cols | Scratch Ops/Call | Trace Ops/Call | Ops Speedup | Wall (`Scratch -> Trace`) | Wall Speedup |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- | ---: |
| `M19` ($p = 524{,}287$) | 3 | 2 | 5 `[pcc:unverified]` | 5 `[pcc:unverified]` | 63 `[pcc:unverified]` | `42,523.1` `[pcc:unverified]` | `22,209.8` `[pcc:unverified]` | **1.91x** `[pcc:unverified]` | `50.8ms -> 6.5ms` `[pcc:unverified]` | **7.87x** `[pcc:unverified]` |
| `M19` ($p = 524{,}287$) | 4 | 2 | 13 `[pcc:unverified]` | 5 `[pcc:unverified]` | 206 `[pcc:unverified]` | `525,964.0` `[pcc:unverified]` | `248,728.3` `[pcc:unverified]` | **2.11x** `[pcc:unverified]` | `119.8ms -> 19.2ms` `[pcc:unverified]` | **6.25x** `[pcc:unverified]` |
| `M19` ($p = 524{,}287$) | 3 | 3 | 5 `[pcc:unverified]` | 6 `[pcc:unverified]` | 1,541 `[pcc:unverified]` | `262.94M` `[pcc:unverified]` | `77.86M` `[pcc:unverified]` | **3.38x** `[pcc:unverified]` | `1,579.3ms -> 763.5ms` `[pcc:unverified]` | **2.07x** `[pcc:unverified]` |
| `M31` ($p = 2^{31}-1$) | 3 | 2 | 4 `[pcc:unverified]` | 5 `[pcc:unverified]` | 63 `[pcc:unverified]` | `42,523.1` `[pcc:unverified]` | `22,209.8` `[pcc:unverified]` | **1.91x** `[pcc:unverified]` | `46.8ms -> 5.6ms` `[pcc:unverified]` | **8.41x** `[pcc:unverified]` |
| `M31` ($p = 2^{31}-1$) | 4 | 2 | 7 `[pcc:unverified]` | 5 `[pcc:unverified]` | 201 `[pcc:unverified]` | `525,965.1` `[pcc:unverified]` | `248,711.7` `[pcc:unverified]` | **2.11x** `[pcc:unverified]` | `105.2ms -> 18.3ms` `[pcc:unverified]` | **5.76x** `[pcc:unverified]` |
| `M31` ($p = 2^{31}-1$) | 3 | 3 | 4 `[pcc:unverified]` | 6 `[pcc:unverified]` | 1,528 `[pcc:unverified]` | `262.94M` `[pcc:unverified]` | `77.86M` `[pcc:unverified]` | **3.38x** `[pcc:unverified]` | `1,614.6ms -> 832.4ms` `[pcc:unverified]` | **1.94x** `[pcc:unverified]` |

---

## 5. `CONTINUATION_PROTOCOL.md` Panel A: Empirical Scaling $\hat{\gamma}$ ($\le 4\text{ GiB}$ RAM)

Charging the complete per-invocation cost floor (target specialization + F4 / trace replay + algebraic root descent + curve lift) across feasible rungs within $\le 4\text{ GiB}$ RAM (`24` natural hidden targets `R = [a]G + [b]Q` plus `4` genuine positive $m$-sums per rung, `planted_control_passed = true` on all 14 rungs `[pcc:unverified]`):

| Family | $m$ | Rungs $t$ ($B = 2^t$) | $\hat{\gamma}_{\text{pos}}$ (Positive Calls) | 95% CI $[\gamma_{\text{LCB}}, \gamma_{\text{UCB}}]$ | $\hat{\gamma}_{\text{stream}}$ (`Before -> After`) | Passes $\gamma < 3.5$ Gate? | Protocol Verdict |
| --- | ---: | --- | ---: | --- | --- | :---: | --- |
| Kummer ($p = 786{,}433$) | 2 | `[2, 3, 4, 5]` (`B = 4..32`) | `3.884` `[pcc:unverified]` | `[3.487, 4.281]` `[pcc:unverified]` | `3.976 -> 3.822` `[pcc:unverified]` | `false` `[pcc:unverified]` | `STRADDLES_3_5_GATE_AT_SMALL_T` `[pcc:unverified]` |
| **Kummer ($p = 786{,}433$)** | **3** | **`[1, 2, 3]` (`B = 2..8`)** | **`7.669`** `[pcc:unverified]` | **`[6.102, 9.237]`** `[pcc:unverified]` | **`7.758 -> 7.569`** `[pcc:unverified]` | **`false`** `[pcc:unverified]` | **`KILL_AT_MEASURED_SCALE` ($\gamma_{\text{LCB}} \ge 3.5$)** `[pcc:unverified]` |
| $T_2$-Dickson ($p = 131{,}071$) | 2 | `[2, 3, 4, 5]` (`B = 4..32`) | `3.858` `[pcc:unverified]` | `[3.490, 4.226]` `[pcc:unverified]` | `3.961 -> 3.772` `[pcc:unverified]` | `false` `[pcc:unverified]` | `STRADDLES_3_5_GATE_AT_SMALL_T` `[pcc:unverified]` |
| **$T_2$-Dickson ($p = 131{,}071$)** | **3** | **`[1, 2, 3]` (`B = 2..8`)** | **`7.660`** `[pcc:unverified]` | **`[6.402, 8.918]`** `[pcc:unverified]` | **`7.769 -> 7.514`** `[pcc:unverified]` | **`false`** `[pcc:unverified]` | **`KILL_AT_MEASURED_SCALE` ($\gamma_{\text{LCB}} \ge 3.5$)** `[pcc:unverified]` |

---

## 6. Reproducibility Commands

```bash
export CARGO_TARGET_DIR=/Volumes/SSD990-2/wt-3858/target-crypto-pkm
export CARGO_PROFILE_RELEASE_DEBUG=0
export CARGO_PROFILE_RELEASE_STRIP=symbols

# 1. Unit tests (ecbench + research crate, 0 warnings)
cargo +1.99.1 test --offline --release --bin ecbench
cargo +1.99.1 test --offline --release \
  --manifest-path research/prime_pkm_aux_chase_20261119/Cargo.toml

# 2. Full 4-suite benchmark regenerating RESULTS.json
cargo +1.99.1 run --offline --release \
  --manifest-path research/prime_pkm_aux_chase_20261119/Cargo.toml -- \
  --suites all research/prime_pkm_aux_chase_20261119/RESULTS.json
```
