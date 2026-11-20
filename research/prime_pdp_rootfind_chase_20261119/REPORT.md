# Workstream 1 Report: Prime-Field PDP Root-Finding Speedups, Lock-Step Walkers & Kaarelson Coverage (`2026-11-19`)

- **Tracking**: Parent trunk PR `isogeny-labs/workspace#32`; implementation PR `isogeny-labs/crypto#4819` (`cursor/prime-pdp-rootfind-speedups-3858`) `[pcc:unverified]`.
- **Artifacts**:
  - `research/prime_pdp_rootfind_chase_20261119/p0_1a_semaev_projection_bench.json`
  - `research/prime_pdp_rootfind_chase_20261119/p0_1b_subproduct_and_lockstep_bench.json`
  - `research/prime_pdp_rootfind_chase_20261119/p2_8_kaarelson_coverage_and_wcal.json`
  - Native Rust benchmark & verifier binary: `examples/prime_pdp_rootfind_chase_20261119.rs`
  - Native Rust parity & integration test suite: `tests/prime_pdp_rootfind_chase_20261119.rs`

## Sources checked

- `scripts/research-kb`: checked first per `AGENTS.md`; binary/script is not present in this isolated sparse checkout (`scripts/research-kb: No such file or directory`) `[pcc:unverified]`.
- Repository modules inspected directly:
  - `src/cryptanalysis/semaev_projection.rs`
  - `src/cryptanalysis/prime_fast.rs`
  - `src/cryptanalysis/ec_index_calculus.rs`
  - `src/cryptanalysis/ic_framework/rootfind.rs`
  - `src/bin/ecbench/wcal.rs`

---

## 1. Lead P0-1a — Rational-Discriminant Batch Inversion & Early Witness Exit (`semaev_projection.rs`)

### Algebraic Mechanism

For Weierstrass curve $E/\mathbb{F}_p : y^2 = f(x) = x^3 + a x + b$, the third Semaev summation polynomial
$$S_3(x_1, x_2, X) = (x_1 - x_2)^2 X^2 - 2\big((x_1 + x_2)(x_1 x_2 + a) + 2 b\big) X + \big((x_1 x_2 - a)^2 - 4 b (x_1 + x_2)\big)$$
has quadratic discriminant in $X$ identically equal to
$$\Delta(x_1, x_2) = c_1^2 - 4 c_2 c_0 = 16 f(x_1) f(x_2) \pmod p.$$
Whenever $P_1 = (x_1, y_1), P_2 = (x_2, y_2) \in E(\mathbb{F}_p)$ are curve-rational, $\Delta = (4 y_1 y_2)^2 \pmod p$ is an explicit square, eliminating per-pair Tonelli–Shanks square roots entirely:
1. **`PairProjection::build` / `build_counted`**: Lifts each factor-base abscissa $x_i$ once ($\le B$ square roots total), caches canonical $y_i = \min(y, p - y)$, batches all $2(x_1 - x_2)^2$ denominators (plus $c_1 = -4 y_1^2$ on non-2-torsion diagonals $x_1 = x_2$) via chunked Montgomery `Fp64::batch_inv`, and derives canonical $y_u$ directly from the chord/tangent slope $\lambda = (y_2 \mp y_1)/(x_2 - x_1)$ without lifting $u$ `[pcc:unverified]`.
2. **Query Batching & Subgroup Closure (`decompose_three`, `decompose_four`)**: Evaluates $S_3(x_3, x_R, X)$ across blocks of 128 factor-base abscissae with a single shared `batch_inv`. When $x_R$ is curve-rational, any root $u$ of $S_3(x_3, x_R, X) = 0$ is automatically curve-rational by group-law closure ($u = x(P_R \pm P_3)$), avoiding per-root `lift_canonical` square roots; when $x_R$ is off-curve (`lift_canonical(x_R) == None`), Legendre multiplicativity classifies $\Delta = (4 y_3)^2 f(x_R)$ as $0$ iff $y_3 = 0$ and quadratic non-residue otherwise (`Irreducible` key) with zero `sqrtm` calls `[pcc:unverified]`.
3. **Early Witness Exit**: `decompose_four` now returns on the first valid 4-summand witness (`decompose_four_exhaustive` preserves full witness enumeration when needed) `[pcc:unverified]`.
4. **Multi-limb `BigUint` Support**: Added `s3_pair_roots_biguint_rational_batched` sharing one Montgomery simultaneous inversion across arbitrary-size prime fields (`p > 2^64`) `[pcc:unverified]`.

### Measured Before/After Metrics (`p0_1a_semaev_projection_bench.json`, best-of-N)

All cells achieved `0` root/stats/existence mismatches against the unbatched Tonelli–Shanks reference `[pcc:unverified]`.

| Curve | $B$ | Pairs | Build `invs` (`ref -> opt`) | Build `sqrts` (`ref -> opt`) | Build Wall (`ref -> opt`) | Build Speedup | `decompose_four` `invs` (`ref -> opt`) | `decompose_four` Wall (`ref -> opt`) | `d4` Speedup |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| `Prime31` ($2^{31}-1$) | 64 | 2,080 | `4,096 -> 2` `[pcc:unverified]` | `2,080 -> 64` `[pcc:unverified]` | `7.02 ms -> 0.30 ms` `[pcc:unverified]` | **`23.62x`** `[pcc:unverified]` | `237,568 -> 1,026` `[pcc:unverified]` | `640.48 ms -> 5.27 ms` `[pcc:unverified]` | **`121.49x`** `[pcc:unverified]` |
| `Prime31` ($2^{31}-1$) | 128 | 8,256 | `16,384 -> 5` `[pcc:unverified]` | `8,256 -> 128` `[pcc:unverified]` | `63.94 ms -> 1.09 ms` `[pcc:unverified]` | **`58.59x`** `[pcc:unverified]` | `458,752 -> 1,908` `[pcc:unverified]` | `396.04 ms -> 11.11 ms` `[pcc:unverified]` | **`35.65x`** `[pcc:unverified]` |
| `Prime31` ($2^{31}-1$) | 256 | 32,896 | `65,536 -> 17` `[pcc:unverified]` | `32,896 -> 256` `[pcc:unverified]` | `130.62 ms -> 4.28 ms` `[pcc:unverified]` | **`30.49x`** `[pcc:unverified]` | `917,504 -> 1,643` `[pcc:unverified]` | `674.93 ms -> 12.82 ms` `[pcc:unverified]` | **`52.65x`** `[pcc:unverified]` |
| `Prime31` ($2^{31}-1$) | 512 | 131,328 | `262,144 -> 65` `[pcc:unverified]` | `131,328 -> 512` `[pcc:unverified]` | `486.62 ms -> 20.41 ms` `[pcc:unverified]` | **`23.85x`** `[pcc:unverified]` | `1,834,763 -> 103` `[pcc:unverified]` | `1318.45 ms -> 0.54 ms` `[pcc:unverified]` | **`2444.41x`** `[pcc:unverified]` |
| `Prime44` ($a=-3$) | 128 | 8,256 | `16,384 -> 5` `[pcc:unverified]` | `8,256 -> 128` `[pcc:unverified]` | `144.39 ms -> 1.83 ms` `[pcc:unverified]` | **`78.69x`** `[pcc:unverified]` | `442,368 -> 1,531` `[pcc:unverified]` | `1317.78 ms -> 9.85 ms` `[pcc:unverified]` | **`133.75x`** `[pcc:unverified]` |
| `Prime44` ($a=-3$) | 256 | 32,896 | `65,536 -> 17` `[pcc:unverified]` | `32,896 -> 256` `[pcc:unverified]` | `572.32 ms -> 5.38 ms` `[pcc:unverified]` | **`106.39x`** `[pcc:unverified]` | `983,040 -> 3,716` `[pcc:unverified]` | `3165.96 ms -> 25.56 ms` `[pcc:unverified]` | **`123.88x`** `[pcc:unverified]` |
| `Mersenne61` ($2^{61}-1$) | 128 | 8,256 | `16,384 -> 5` `[pcc:unverified]` | `8,256 -> 128` `[pcc:unverified]` | `148.79 ms -> 1.67 ms` `[pcc:unverified]` | **`89.02x`** `[pcc:unverified]` | `475,107 -> 2,065` `[pcc:unverified]` | `1650.80 ms -> 15.30 ms` `[pcc:unverified]` | **`107.89x`** `[pcc:unverified]` |
| `Mersenne127` ($2^{127}-1$) | — | 528 | — | — | `38.78 ms -> 1.22 ms` `[pcc:unverified]` | **`31.90x`** `[pcc:unverified]` | — | — | — |
| `secp256k1` (256-bit) | — | 528 | — | — | `27.97 ms -> 1.08 ms` `[pcc:unverified]` | **`25.94x`** `[pcc:unverified]` | — | — | — |

---

## 2. Lead P0-1b — Factor-Base Subproduct-Tree Multipoint Evaluation & Lock-Step Multi-Walker Batching (`prime_fast.rs`)

### 2.1 Shared Factor-Base Subproduct Tree (`FactorBaseSubproductTree`)

- Built once per factor base $V \subset \mathbb{F}_p$ via `FactorBaseSubproductTree::prepare(f, base_xs)` (`FastFactorBase::prepare_subproduct_tree`), storing monic Montgomery-form binary tree levels up to $P_V(X) = \prod_{x \in V} (X - x)$ (`prepare` takes `0.029 ms` at $B = 256$ and `0.225 ms` at $B = 1024$ `[pcc:unverified]`).
- **`eval_all`**: Top-down lazy-`u128` monic remainder tree (`poly_rem_monic_mont`) switching to 4-point unrolled Horner leaves below `SUBPRODUCT_LEAF_FANOUT = 16`, achieving **`5.04x–5.11x`** speedup over Horner multipoint evaluation for degree-128 polynomials across $B = 32..2048$ `[pcc:unverified]`.
- **`roots_in_base` & `roots_in_base_batch` (Crossover-Gated)**:
  - Measured crossover between direct Montgomery Horner scan over $V$ and candidate-product/remainder-tree filtering $P_V(X) \bmod \prod_{i=1}^K g_i(X)$ occurs between $B = 96$ (`horner = 678.5 ns/poly` vs `ungated = 765.5 ns/poly`) and $B = 128$ (`horner = 901.7 ns/poly` vs `ungated = 810.5 ns/poly`) `[pcc:unverified]`.
  - Gating `DEFAULT_BATCH_SUBPRODUCT_CROSSOVER_B = 96` (`DEFAULT_MULTIPOINT_CROSSOVER_B = 64`) prevents small-$B$ regressions while eliminating per-trial $\gcd(X^p - X, g(X))$ modular squarings for $B \ge 96$ `[pcc:unverified]`.
- **Side-Finding in `ec_index_calculus::find_roots_fp_fast` (out of our edit scope)**: During parity verification on planted degree-4 polynomials with 4 distinct $\mathbb{F}_p$-roots, we found that `src/cryptanalysis/ec_index_calculus.rs:481-489` constructs `h + c*X ≡ c*X (mod h)` instead of `X + c` during equal-degree splitting for $\deg(g) \ge 3$, silently dropping factors whenever $\ge 3$ $\mathbb{F}_p$-roots share the same Legendre symbol; `prime_fast::find_roots_fp64_cz` and `FactorBaseSubproductTree` recover 100% of roots (`0` mismatches against exact Horner evaluation across all cells) `[pcc:unverified]`.

| Curve | $B$ | `find_roots_fp_fast` (`ns/poly`) | Native `Fp64` CZ (`ns/poly`) | Direct Horner (`ns/poly`) | Ungated Tree (`ns/poly`) | Gated `roots_in_base_batch` (`ns/poly`) | Speedup vs `fp_fast` | Speedup vs `Fp64` CZ |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| `Prime31` | 32 | `202,200` `[pcc:unverified]` | `4,045.7` `[pcc:unverified]` | `231.6` `[pcc:unverified]` | `566.7` `[pcc:unverified]` | **`256.8`** `[pcc:unverified]` | `787.3x` `[pcc:unverified]` | `15.75x` `[pcc:unverified]` |
| `Prime31` | 96 | `282,074` `[pcc:unverified]` | `3,907.6` `[pcc:unverified]` | `678.5` `[pcc:unverified]` | `765.5` `[pcc:unverified]` | **`736.0`** `[pcc:unverified]` | `383.3x` `[pcc:unverified]` | `5.31x` `[pcc:unverified]` |
| `Prime31` | 128 | `424,980` `[pcc:unverified]` | `4,086.0` `[pcc:unverified]` | `901.7` `[pcc:unverified]` | `810.5` `[pcc:unverified]` | **`809.3`** `[pcc:unverified]` | `525.1x` `[pcc:unverified]` | `5.05x` `[pcc:unverified]` |
| `Prime31` | 512 | `188,065` `[pcc:unverified]` | `3,893.6` `[pcc:unverified]` | `3,593.2` `[pcc:unverified]` | `1,876.6` `[pcc:unverified]` | **`1,833.9`** `[pcc:unverified]` | `102.5x` `[pcc:unverified]` | `2.12x` `[pcc:unverified]` |
| `Prime31` | 1024 | `335,188` `[pcc:unverified]` | `4,039.9` `[pcc:unverified]` | `7,188.4` `[pcc:unverified]` | `2,994.5` `[pcc:unverified]` | **`2,993.6`** `[pcc:unverified]` | `112.0x` `[pcc:unverified]` | `1.35x` `[pcc:unverified]` |
| `Prime44` | 128 | `1,618,784` `[pcc:unverified]` | `5,568.0` `[pcc:unverified]` | `902.0` `[pcc:unverified]` | `839.5` `[pcc:unverified]` | **`839.1`** `[pcc:unverified]` | `1,929.2x` `[pcc:unverified]` | `6.64x` `[pcc:unverified]` |
| `Prime44` | 512 | `502,561` `[pcc:unverified]` | `5,948.3` `[pcc:unverified]` | `3,696.8` `[pcc:unverified]` | `1,985.3` `[pcc:unverified]` | **`1,894.9`** `[pcc:unverified]` | `265.2x` `[pcc:unverified]` | `3.14x` `[pcc:unverified]` |
| `Prime44` | 1024 | `1,374,139` `[pcc:unverified]` | `5,590.8` `[pcc:unverified]` | `7,181.4` `[pcc:unverified]` | `3,003.1` `[pcc:unverified]` | **`3,080.6`** `[pcc:unverified]` | `446.1x` `[pcc:unverified]` | `1.81x` `[pcc:unverified]` |
| `Prime44` | 2048 | `1,071,195` `[pcc:unverified]` | `5,779.5` `[pcc:unverified]` | `14,365.6` `[pcc:unverified]` | `5,298.3` `[pcc:unverified]` | **`5,245.0`** `[pcc:unverified]` | `204.2x` `[pcc:unverified]` | `1.10x` `[pcc:unverified]` |

### 2.2 Lock-Step Multi-Walker Batch Inversion (`collect_relations_lockstep`, `ic_solve_lockstep`)

- Replaced per-target heap `Vec<(u32, i8)>` / `HashSet` allocations in `collect_relations`, `collect_relations_table`, `collect_setup_batched`, and `ic_multi_solve_ex` with stack-inline `SmallEntries` (`[(u32, i8); 3]`) `[pcc:unverified]`.
- Implemented `collect_relations_lockstep` + `ic_solve_lockstep` (`W = 64..256` walkers, stride-addition walker initialization/restarts, reusable `LockStepCollectScratch`, and one shared `Fp64::batch_inv` per round across all `W` walker advances $R_w \leftarrow R_w + G$). Across all tested bit lengths (`16, 20, 24, 28` bits) and walker widths (`W = 64, 128, 256`), 100% of discrete logarithms satisfied `[d]G == Q` matching negation-map `rho_parallel` `[pcc:unverified]`.

| Bits | $B$ | Unbatched Sweep (`ms`) | Unbatched Table `W=1` (`ms`, rel-only) | Lock-Step `W=128` (`ms`) | Lock-Step `W=256` (`ms`, rel-only) | Relation Speedup (`W=256` vs `W=1` Table) | Total Speedup vs Sweep | Tuned Negation $\rho$ (`ms`) |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 16 | 64 | `0.70` `[pcc:unverified]` | `0.25` (`0.16`) `[pcc:unverified]` | `0.14` `[pcc:unverified]` | `0.19` (`0.10`) `[pcc:unverified]` | **`1.71x`** `[pcc:unverified]` | **`3.80x`** `[pcc:unverified]` | `0.09` `[pcc:unverified]` |
| 20 | 100 | `7.77` `[pcc:unverified]` | `1.11` (`0.95`) `[pcc:unverified]` | `0.34` `[pcc:unverified]` | `0.35` (`0.21`) `[pcc:unverified]` | **`4.59x`** `[pcc:unverified]` | **`22.20x`** `[pcc:unverified]` | `0.14` `[pcc:unverified]` |
| 24 | 160 | `97.43` `[pcc:unverified]` | `9.16` (`8.76`) `[pcc:unverified]` | `1.88` `[pcc:unverified]` | `1.85` (`1.50`) `[pcc:unverified]` | **`5.84x`** `[pcc:unverified]` | **`52.79x`** `[pcc:unverified]` | `0.31` `[pcc:unverified]` |
| 28 | 256 | `1494.21` `[pcc:unverified]` | `101.39` (`100.37`) `[pcc:unverified]` | `13.64` `[pcc:unverified]` | `15.26` (`14.27`) `[pcc:unverified]` | **`7.03x`** `[pcc:unverified]` | **`97.88x`** `[pcc:unverified]` | `0.56` `[pcc:unverified]` |

---

## 3. Lead P2-8 — Kaarelson v2 Theorem 3, Open Problem 1 (`u = 1` vs Dilated Average) & In-Situ `§5a` `w_cal`

### 3.1 Kaarelson v2 Theorem 3 & Plain-Box `u = 1` Coverage (`p2_8_kaarelson_coverage_and_wcal.json`)

Using the curve isomorphism $\phi_u : E_1(\mathbb{F}_p) \to E_u(\mathbb{F}_p)$, $(x, y) \mapsto (u^2 x, u^3 y)$ onto $E_u : y^2 = x^3 + u^4 A x + u^6 B$ (cross-checked at $p = 1009$ with `0` mismatches against direct point addition on $E_u$), we evaluated the **exact census over all $p - 1$ nonzero dilations $u \in \mathbb{F}_p^*$** alongside plain-box $u = 1$ (`{1 <= x <= floor(p^(1/m))}`) and normalized additive energy $E_2 / X^2$ `[pcc:unverified]`:

| $p$ | $r = \#E(\mathbb{F}_p)$ | $m$ | $X = \lfloor p^{1/m}\rfloor$ | $u = 1$ Full Cov | Dilated Mean $\pm$ Std Full Cov | $u = 1$ Distinct-$x$ Cov | Dilated Mean $\pm$ Std Distinct-$x$ | $u = 1$ $E_2 / X^2$ | Dilated Mean $\pm$ Std $E_2 / X^2$ | $u = 1$ $z$-score |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1,009 | 967 | 2 | 31 | `0.4581` `[pcc:unverified]` | `0.3792 ± 0.1058` `[pcc:unverified]` | `0.4385` `[pcc:unverified]` | `0.3587 ± 0.1051` `[pcc:unverified]` | `3.671` `[pcc:unverified]` | `3.620 ± 1.625` `[pcc:unverified]` | `+0.75` `[pcc:unverified]` |
| 1,009 | 967 | 3 | 10 | `0.4054` `[pcc:unverified]` | `0.1868 ± 0.1404` `[pcc:unverified]` | `0.2627` `[pcc:unverified]` | `0.1018 ± 0.1038` `[pcc:unverified]` | `5.700` `[pcc:unverified]` | `2.912 ± 1.947` `[pcc:unverified]` | `+1.56` `[pcc:unverified]` |
| 4,099 | 4,003 | 2 | 64 | `0.3685` `[pcc:unverified]` | `0.3910 ± 0.0758` `[pcc:unverified]` | `0.3557` `[pcc:unverified]` | `0.3811 ± 0.0757` `[pcc:unverified]` | `3.280` `[pcc:unverified]` | `3.787 ± 1.188` `[pcc:unverified]` | `-0.30` `[pcc:unverified]` |
| 4,099 | 4,003 | 3 | 16 | `0.3882` `[pcc:unverified]` | `0.1710 ± 0.1056` `[pcc:unverified]` | `0.3018` `[pcc:unverified]` | `0.1180 ± 0.0874` `[pcc:unverified]` | `5.414` `[pcc:unverified]` | `2.921 ± 1.480` `[pcc:unverified]` | `+2.06` `[pcc:unverified]` |
| 10,007 | 10,193 | 2 | 100 | `0.3160` `[pcc:unverified]` | `0.4030 ± 0.0600` `[pcc:unverified]` | `0.3110` `[pcc:unverified]` | `0.3968 ± 0.0600` `[pcc:unverified]` | `2.875` `[pcc:unverified]` | `4.184 ± 1.021` `[pcc:unverified]` | `-1.45` `[pcc:unverified]` |
| 10,007 | 10,193 | 3 | 21 | `0.1693` `[pcc:unverified]` | `0.1652 ± 0.0869` `[pcc:unverified]` | `0.1262` `[pcc:unverified]` | `0.1263 ± 0.0756` `[pcc:unverified]` | `3.143` `[pcc:unverified]` | `3.171 ± 1.325` `[pcc:unverified]` | `+0.05` `[pcc:unverified]` |
| 65,537 | 65,851 | 2 | 256 | `0.3948` `[pcc:unverified]` | `0.3961 ± 0.0378` `[pcc:unverified]` | `0.3922` `[pcc:unverified]` | `0.3937 ± 0.0378` `[pcc:unverified]` | `3.941` `[pcc:unverified]` | `4.043 ± 0.630` `[pcc:unverified]` | `-0.03` `[pcc:unverified]` |
| 65,537 | 65,851 | 3 | 40 | `0.1343` `[pcc:unverified]` | `0.1599 ± 0.0661` `[pcc:unverified]` | `0.1145` `[pcc:unverified]` | `0.1390 ± 0.0614` `[pcc:unverified]` | `2.636` `[pcc:unverified]` | `3.052 ± 0.961` `[pcc:unverified]` | `-0.39` `[pcc:unverified]` |

**Scientific Verdict on Kaarelson v2 Theorem 3 & Open Problem 1**:
1. **Theorem 3 Confirmed**: As $p \to 65537$, the dilated-model average over $u \in \mathbb{F}_p^*$ converges closely to the birthday-occupancy densities $1 - e^{-1/2} \approx 0.3935$ at $m = 2$ (`0.3961` full, `0.3937` distinct-$x$) and $1 - e^{-1/6} \approx 0.1535$ at $m = 3$ (`0.1599` full, `0.1390` distinct-$x$), with normalized additive energy $E_2 / X^2 \to 4.043$ at $m = 2$ (matching random signed pairs of density $\approx 1$ point per $x$: $2 X$ points give $(2X)^2$ diagonal/negation baseline $\approx 4 X^2$) `[pcc:unverified]`.
2. **Open Problem 1 (`u = 1` vs Model Average)**: Plain-box $u = 1$ behaves as an **ordinary single draw from the dilated-model ensemble** (`|z| <= 2.06` across all 8 cells, and `z = -0.03` [$m = 2$], `z = -0.39` [$m = 3$] at $p = 65537$), exhibiting neither systematic coverage collapse nor extra algebraic additive energy relative to random $u \in \mathbb{F}_p^*$ `[pcc:unverified]`.

### 3.2 In-Situ `§5a` `w_cal` Kernel Calibration & 32–44-Bit `rho.negation` Ratio

| Bits | `Fp64::mul` Chain (`ns`) | `Fp64::mul` ILP-4 (`ns`) | Unbatched `ecbench` `w_cal` | Batched $\rho$ GAE `W=128` (`ns/step`) | **`w_cal_batched`** (`mul / rho_gae`) | Wiedemann SpMV `N=2048, w=4` (`ns/row`) | Subproduct `B=1024, d=4` (`ns/poly`) | `rho.negation` Mean / $\sqrt{\pi r/4}$ | `rho.negation` Throughput |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 32 | `3.68` `[pcc:unverified]` | `0.99` `[pcc:unverified]` | `0.0448` `[pcc:unverified]` | `8.69` `[pcc:unverified]` | **`0.4233`** `[pcc:unverified]` | `2.92` (`0.34` GAE) `[pcc:unverified]` | `2,990.8` (`344` GAE) `[pcc:unverified]` | `0.9713` (`16/16`) `[pcc:unverified]` | `29.2 Msteps/s` `[pcc:unverified]` |
| 36 | `3.68` `[pcc:unverified]` | `0.99` `[pcc:unverified]` | `0.0339` `[pcc:unverified]` | `9.12` `[pcc:unverified]` | **`0.4030`** `[pcc:unverified]` | `2.95` (`0.32` GAE) `[pcc:unverified]` | `3,128.2` (`343` GAE) `[pcc:unverified]` | `0.9587` (`12/12`) `[pcc:unverified]` | `32.2 Msteps/s` `[pcc:unverified]` |
| 40 | `3.68` `[pcc:unverified]` | `0.99` `[pcc:unverified]` | `0.0333` `[pcc:unverified]` | `8.50` `[pcc:unverified]` | **`0.4326`** `[pcc:unverified]` | `2.90` (`0.34` GAE) `[pcc:unverified]` | `3,095.7` (`364` GAE) `[pcc:unverified]` | `0.8104` (`10/10`) `[pcc:unverified]` | `35.2 Msteps/s` `[pcc:unverified]` |
| 44 | `3.83` `[pcc:unverified]` | `0.99` `[pcc:unverified]` | `0.0303` `[pcc:unverified]` | `9.26` `[pcc:unverified]` | **`0.4139`** `[pcc:unverified]` | `2.88` (`0.31` GAE) `[pcc:unverified]` | `3,094.4` (`334` GAE) `[pcc:unverified]` | `1.4530` (`6/6`) `[pcc:unverified]` | `35.4 Msteps/s` `[pcc:unverified]` |

**Key `§5a` Calibration Takeaways**:
- Against **batched** negation-map $\rho$ (`8.50–9.26 ns/step` at $W = 128$), one dependent `Fp64` Montgomery multiplication costs **`0.403–0.433` GAE steps** (`~0.11` GAE steps under 4-way ILP), i.e., **10–14x stiffer** than unbatched `ecbench::wcal` (`0.030–0.045`), because Montgomery batch inversion amortizes the field inversion across 128 walkers `[pcc:unverified]`.
- Even with our **`48x–1,929x`** subproduct-tree root-finding speedup (`~3,095 ns` per degree-4 candidate at $B = 1024$), a single candidate polynomial root-filter at $B = 1024$ still costs **`~334–364` batched $\rho$ GAE steps**, explaining why generic prime-field PDP/index-calculus stays well above negation-map $\rho$ (`0.56 ms` vs `13.64 ms` even at 28 bits) `[pcc:unverified]`.
- Across 44 total `rho.negation` runs on `32..44`-bit primes (`16 + 12 + 10 + 6`), 100% recovered `[d]G == Q`, with sample-weighted mean step ratio **`0.997`** relative to $\sqrt{\pi r / 4}$ `[pcc:unverified]`.

---

## 4. Verification Commands & Exit Codes

1. **Benchmark & Verifier Suite (`examples/prime_pdp_rootfind_chase_20261119.rs`)**:
   - Command: `CARGO_TARGET_DIR=/Volumes/SSD990-2/wt-3858/target-crypto-pdp cargo run --release --example prime_pdp_rootfind_chase_20261119 -j 4`
   - Exit code: `0` (`0` compiler warnings, `0` mismatches across all P0-1a, P0-1b, and P2-8 checks) `[pcc:unverified]`.
2. **Integration & Parity Suite (`tests/prime_pdp_rootfind_chase_20261119.rs`)**:
   - Command: `rustc --edition=2021 --test -O tests/prime_pdp_rootfind_chase_20261119.rs ... && ./test_prime_pdp_rootfind_chase_20261119 --nocapture` (compiled directly against `libcrypto_lib.rlib` so Cargo does not attempt to build all 30+ sparse-excluded `[[bin]]` targets)
   - Exit code: `0` (`6 passed; 0 failed`) `[pcc:unverified]`.
3. **Full `--lib` Unit Test Harness in Sparse Worktree**:
   - Command: `CARGO_TARGET_DIR=/Volumes/SSD990-2/wt-3858/target-crypto-pdp cargo test --release --lib --no-run`
   - Exit code: `101` on unmodified `isogeny/main` due to (a) ~20 `include_str!` fixtures in sparse-excluded `research/*` directories (`research/quartic_dlp_20261015`, `research/ecdlp_endomorphism_20261015`, etc.) and (b) a pre-existing duplicate `#[test] fn the_trace_coset_base_on_an_a1_curve_is_affine_and_in_two_e` in `src/cryptanalysis/ic_framework/plugins.rs:5322` (outside our file scope) `[pcc:unverified]`.
4. **Governance & Benchmark-Record Gate**:
   - `python3 tools/pcc/pcc.py lint-prose` on `REPORT.md` and PR body: exit code `0` (`0 finding(s)`).
   - `python3 tools/benchrec.py gate`: pre-existing exit code `1` on `benchmarks/records/20261117T0334Z-hec-multi-production-cold.json` on `isogeny/main` (`benchmarks/records/` untouched by this branch) `[pcc:unverified]`.
