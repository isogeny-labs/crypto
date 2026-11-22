# L2 protocol: coordinate types as decomposition variables (wave 7, 2026-11-22)

Status: **Phase 1, design only. No data taken.** Frozen at the time this file was written,
before the smoke test. The smoke test (`smoke/`) is labelled "smoke, not data" and
decides nothing. Every result sentence written later carries [pcc:unverified].

Brief: `../BRIEF.md`. Lead text: the lead message of 2026-11-22 (L2).

## 0. Exposure log (read before judging blindness)

While the code was being built, two functional checks ran before this file existed.
Their numbers were seen:

1. The fibre-size table at k = 16 for all eight functions (`common.py` self-test).
   - Max rational fibre: x 2, v 3, q 3, uv 5, xeye 4, jac6 6, ye 2, xe 2.
   - Points per value: about 1 for every function.
2. A census of K = 50 frames at k = 16, t = 5. Mean covered for each arm, against
   cols-matched x-frames:

   | arm | covered | x-frames |
   |:--|--:|--:|
   | v | 262.8 | 255.4 |
   | xeye | 77.9 | 279.0 |
   | even | 507.6 | 259.4 |
   | rand | 260.3 | 256.3 |
   | xAA | 272.6 | 266.8 |

Consequences for blindness:

- **xeye at k = 16 is post-hoc.** The xeye deficit led to the translation-collapse
  mechanism (§2, M3). Its prediction at k = 16 (1/4) is post-hoc, and those cells are
  flagged. Its predictions at k = 18 and 20 are blind. There A + 2 is a non-square (a
  curve fact, computed), so the rational part of the collapse group has order 2, not 4.
- **xe is blind.** Its prediction (H = {O, T0}, coverage 1/2 per column, S^xe of degree 4
  per variable) was derived from the same mechanism. xe was never run.
- **Seen, not post-hoc.** The k = 16, t = 5 numbers for v, even, rand and xAA agree with the
  predictions below, which were derived from theory. They are still seen data. Each
  verdict reports with and without that cell; at K = 2000 the cell is re-drawn anyway,
  with a different seed.

## 1. Question

Do the rational functions that the coordinate systems give help index calculus?

Projective, Jacobian, XZ, inverted and extended coordinates are not functions on points,
because they scale. The functions they give are the weight-0 ratios, rational functions
on E. The question has two parts:

- Does a degree ≥ 3 such function give an m = 2 factor base that covers more order-r
  targets than an x-frame with the same number of columns?
- Is its summation polynomial cheap enough to solve?

## 2. Desk part

All functions are on the Montgomery model v² = u³ + A u² + u, with B = 1. Short Weierstrass:
x = u + A/3, y = v. Twisted Edwards (a = A + 2, d = A − 2): x_E = u/v, y_E = (u − 1)/(u + 1).
Ed25519 multiplies x_E by the constant √(−(A + 2)). A constant changes neither the degree nor the
stabiliser.

Notation:

- T0 = (0, 0) is the rational 2-torsion point.
- ±Q₁ are the points with u = 1 and ±Q₋₁ the points with u = −1. All four are 4-torsion, with
  2Q = T0. Q₁ is rational iff A + 2 is a square: true at k = 16, false at k = 18 and 20, where
  Q₋₁ is the rational one.
- T1 and T2 are the two irrational 2-torsion points.

| chart (stored coords) | weight-0 functions | on (u, v) | deg | name here |
|:--|:--|:--|--:|:--|
| Montgomery XZ ladder (X:Z) | X/Z | u | 2 | (x, up to + A/3) |
| Montgomery affine / projective (X:Y:Z) | X/Z, Y/Z, **X/Y**, Z/X, Z/Y, Y/X, **XY/Z²** | u, v, **u/v**, 1/u, 1/v, v/u, **uv** | 2, 3, **2**, 2, 3, 2, **5** | x, v, **xe**, –, –, –, **uv** |
| short Weierstrass affine / projective | X/Z, Y/Z, **X/Y** | x, y, **x/y = (u + A/3)/v** | 2, 3, **3** | x, v, **q** |
| Jacobian / Chudnovsky / modified Jacobian, weights (2, 3, 1) | X/Z², Y/Z³, XZ/Y, Y²/X³ | x, y, x/y, y²/x³ = (x³ + a₄x + a₆)/x³ | 2, 3, 3, 6 | x, v, q, **jac6** |
| twisted Edwards affine / projective (X:Y:Z) | X/Z, Y/Z, X/Y | x_E = u/v, y_E = (u − 1)/(u + 1), x_E/y_E = u(u + 1)/(v(u − 1)) | 2, 2, 4 | xe, **ye**, – |
| extended Edwards (X:Y:Z:T), T = XY/Z | T/Z, T/X, T/Y, X/Y | **x_E y_E = u(u − 1)/(v(u + 1))**, y_E, x_E, x_E/y_E | 4, 2, 2, 4 | **xeye** |
| inverted Edwards (X:Y:Z), x_E = Z/X | Z/X, Z/Y, X/Y | x_E, y_E, y_E/x_E | 2, 2, 4 | – |
| completed Edwards ((X:Z), (Y:T)) | X/Z, Y/T | x_E, y_E | 2, 2 | – |
| Jacobi quartic | its coordinate | a Möbius frame of x∘τ_T | 2 | (frame search) |
| Hessian, twisted Hessian, Jacobi intersection, Huff | not defined over F_p (#E = 8r with cyclic Z/8) | | | (frame search) |

Degrees come from divisors:

- div u = 2(T0) − 2(O) and div v = (T0) + (T1) + (T2) − 3(O).
- **u/v**: div = (T0) + (O) − (T1) − (T2), degree **2**. Montgomery X/Y is Edwards x, a degree-2 function.
- **Weierstrass x/y**: div = (P_x) + (−P_x) + (O) − (T0) − (T1) − (T2), where x(P_x) = 0. Degree **3**.
- **x_E y_E**: zeros {T0, ±Q₁, O}, poles {±Q₋₁, T1, T2}, degree 4.
- **uv**: div = 3(T0) + (T1) + (T2) − 5(O), degree 5.
- **jac6** = h∘x with deg h = 3, degree 6.

Signed stabiliser: σ : P ↦ εP + Q with f∘σ = ±f. Aut(E) = {±1}, since j ≠ 0, 1728.

| f | deg | f∘(−1) | exact (+) stabiliser | H: translation parts of the + stabiliser, rational | predicted per-variable degree of S^f |
|:--|--:|:--|:--|:--|--:|
| x | 2 | + | {1, −1} | {O} | 2 |
| ye | 2 | + | {1, −1} (τ_T0 gives −ye) | {O} | 2 |
| xe | 2 | − | {1, P ↦ T0 − P} | **{O, T0}** | **4** |
| v | 3 | − | {1} | {O} | 9 |
| q | 3 | − | {1} | {O} | 9 |
| xeye | 4 | − | {1, τ_T0, P ↦ ±Q₁ − P} (Klein) | **{O, T0}** (k = 18, 20); **⟨Q₁⟩, order 4** (k = 16) | 4 |
| jac6 | 6 | + | {1, −1} | {O} | 18 |
| uv | 5 | − | {1} | {O} | 25 |

Mechanisms, used for the predictions:

- **M1, points per value.** A degree-d function has fibres of up to d points, but over F_p
  the mean number of rational points per value is 1 for every degree. This is Chebotarev:
  the mean number of fixed points of the fibre's Galois group is 1. So a base of |V|
  values holds about |V| points, not d|V|.
  - For v and q (Galois group S₃), P(3, 1, 0 points) = 1/6, 1/2, 1/3.
  - For uv (S₅), P(5) = 1/120.
- **M2, columns per value: an accounting effect.** An even f puts ±P in one fibre, which
  gives about |V|/2 classes. An odd f gives about |V| classes, because −P lies in the fibre
  of −c. The difference is one sign bit. The decomposition must search a ∈ ±V, so the
  sign bit is paid for as a Boolean variable. The per-value gain of about 4× in coverage
  for odd f is accounting, not an improvement (CLAUDE.md success criterion).
- **M3, translation collapse.** If the + stabiliser contains P ↦ ±P + h with h ≠ O
  rational, each fibre carries the classes of P and P + h. The signed pair sums of two
  fibres then repeat with offsets in H. The coverage of a set of columns is about 1/|H|
  of a generic set with the same number of columns. Predictions: **xe 1/2**; **xeye 1/2 at
  k = 18 and 20, 1/4 at k = 16**; 1 for the rest.
- **M4, coverage is generic otherwise.** With H trivial, the cofactor cosets of base
  points behave like those of an x-frame. Coverage per column then equals the x-frame's.
  This holds for v, q, uv and jac6, and for ye.
- **M5, degree of S^f in one variable.** It is the number of distinct values
  f(−(A_i + B_j)), where A_i runs over the fibre of a and B_j over the fibre of b. It is d²
  for a trivial stabiliser (v, q: 9; uv: 25). It is 2e² for h∘x (jac6: 18). For xeye it is
  4, the four G-orbits of the 16 fibre sums.
  - **Scope note on the frame theorem** (`curve25519_frame_search_20261103`). A degree-2
    function is a Möbius frame of x∘τ_Q. When 2Q ≠ O, the sums of fibre points take 4 values
    of x∘τ_Q, so S has degree 4 per variable, not 2. That frame search measured only Möbius
    frames of x itself, which have degree 2. xe = Möbius(x∘τ_{Q₄}) with 2Q₄ = T0 is the
    blind test of this.
- **M6, dense Boolean system.** In the engine's `bits` form, a summand of b Boolean variables
  enters S through powers a^i. Reduced mod z² = z, these have degree min(i, b), plus one for
  the sign bit when i is odd. When the per-variable degree is at least b, the reduced
  equation has degree n = 2b, the whole cube. The degree-D Macaulay closure has no rows
  below D = n, so D_ref = n. This is predicted for v, q, uv and jac6 at every T ≤ 6, and for
  xe and xeye at T = 4. For x and ye the reduced degree is 4.

## 3. Instrument (exact commands, one core)

```
cd research/curve25519_wave7_untried_20261122/L2_highdeg_coords
bash scripts/run_all.sh        # = the four steps below, then sha256sum and analyse
python3 scripts/sumpoly.py  --out data/sumpoly.json                            # (b1) desk + S^f
python3 scripts/census.py   --K 2000 --pc1 --out data                          # (a) census
python3 scripts/macaulay.py --sumpoly data/sumpoly.json --out data/macaulay.json   # (b2) proxy
OUT=data bash scripts/engine_calib.sh                                          # (C2) engine F4, x only
python3 scripts/analyse.py --data data --out analysis
```

`OPENBLAS_NUM_THREADS=1` is set by `run_all.sh`. Python 3.13 with numpy; sympy is not used.

- **(b1) `sumpoly.py`**, per curve and f:
  - the exact fibre-size table over all of E(F_p);
  - the signed symmetries over E(F_p)[8] (16 maps, 4,000 points);
  - a split-fibre count of distinct f(−(A_i + B_j)), 20 pairs;
  - interpolation in the symmetric orbit-sum basis (odd f: both total-degree parities).
    The box degree D is raised from 1 until the kernel over F_p is non-zero, with
    #orbits + 40 sampled zeros (f(P1), f(P2), f(P3)) where P1 + P2 + P3 = O;
  - checks on 1,000 fresh zeros (all must vanish) and 1,000 random triples (non-vanishing
    rate);
  - for f = x, proportionality to the explicit Weierstrass S₃ (PC3).
- **(a) `census.py`**, cells k ∈ {16, 18, 20} × t ∈ {4, 5, 6, 7} × arm, K = 2000 frames per
  cell. A frame of arm f is V = [L, L + 2^t) mod p with L uniform; its base is
  {P : f(P) ∈ V}. Per frame it records the f-base, a cols-matched x-frame (xm) and a
  same-|V| x-frame (xv): values, points, cols, covered, J. `covered` is the frame-search
  instrument: exact distinct R ≠ O in E[r] with R = ±P_i ± P_j, i = j allowed.
- **(b2) `macaulay.py`**, the counted proxy (see its docstring):
  - the engine's bits system, m = 2. Even f: T magnitude bits. Odd f: T − 1 bits plus a
    sign bit. Both give n = 2T Booleans and about 2^(T−1) classes per summand;
  - Ŝ_R exactly, by evaluating S^f on the grid and applying the Möbius transform: e, terms,
    Boolean roots, true (re-added) decompositions, and agreement with the census;
  - **M1**, plain Macaulay: the exact refutation multiplier degree, deg(1/Ŝ);
  - **M2**, the degree-D Macaulay closure with degree falls (MutantXL rule; V_n is the full
    ideal): D_ref for refutable targets, D_root for unique-root targets, and per D the
    active columns, rank and counted mul-adds.

  Cells: k ∈ {16, 18, 20}; f ∈ {x, ye, xe, v, q, xeye, jac6, uv}; T ∈ {4, 5, 6}. M2 runs
  only at T ∈ {4, 5}; at T = 6, e, terms and M1 only. Targets: the same 10 random order-r
  targets per k for every f and T, plus 3 targets covered by that f's base per (k, f, T).
- **(C2) `engine_calib.sh`**: `curve25519_family-5abc70ba f4cost k bits T 0 10 seed 900 2 full 0 30000000`
  for k ∈ {16, 18, 20}, T ∈ {4, 5}. The engine cannot take f ≠ x; this only calibrates
  M2 for x.

**Seeds** (derived rule): `common.dseed(parts) = int(sha256('wave7-L2|' + '|'.join(parts))[:12], 16)`.

| use | parts |
|:--|:--|
| census | (k, arm, t) |
| targets | (k, 'targets') |
| covered targets | (k, f, T, 'covered') |
| interpolation | (k, 'sumpoly', f) |
| verification | (k, 'verify', f), (k, 'nonzero', f) |
| bootstrap | ('boot', k, arm, t, 'col' / 'val') |
| engine | 2026112200 + 10k + T |

**Curves** (each fact verified by computation in `common.Curve._verify`):

| k | p | A | #E | r | twist order |
|--:|--:|--:|--:|--:|--:|
| 16 | 65437 | 1058 | 65672 = 8 · 8209 | 8209 | 65204 = 4 · 16301 |
| 18 | 262133 | 78 | 263096 = 8 · 32887 | 32887 | 4r′ |
| 20 | 1048573 | 926 | 1049624 = 8 · 131203 | 131203 | 4r′ |

The checks: p prime, p ≡ 5 (mod 8), A² − 4 a non-square, the exact count by Euler's
criterion over every u, r prime, twist order 4 × prime, and a generator of order exactly 8r.

## 4. Counted units and estimators

- **Census.** R_col = mean covered(f) ÷ mean covered(xm), and R_val = mean covered(f) ÷
  mean covered(xv). Each has a 95% percentile bootstrap interval: B = 2000, frames
  resampled independently in each arm. Also reported: points per value, cols per value,
  and the selection gain (argmax-J frame ÷ mean) for f and for xm.
- **S^f.** Exact degrees per variable and in total, terms, kernel dimension, and
  verification counts.
- **Proxy.** Exact e, terms and D values. Counted M2 mul-adds per target (median and
  min–max over targets). Cost per relation = median M2 mul-adds ÷ base coverage, where the
  base coverage is the exact covered ÷ r of the same [0, 2^t) base.
- **No timings are used.**

## 5. Predictions (numeric, frozen; a failed prediction stays failed)

- **P1, no per-column gain** (v, q, uv, jac6). R_col ∈ [0.85, 1.15] in ≥ 90% of the cells
  with mean xm covered ≥ 50, and no cell has the R_col interval's lower bound > 1.15.
- **P1x, translation collapse** (xe, xeye). R_col ∈ [0.8/|H|, 1.2/|H|] in every cell.
  - xe: |H| = 2 on every k.
  - xeye: |H| = 2 at k = 18 and 20 (blind), |H| = 4 at k = 16 (post-hoc, §0).
  - The verdict is also reported on the blind cells alone.
- **P2, per value (accounting).** R_val ∈ [3.0, 5.0] for v, q and uv; ∈ [0.75, 1.33] for
  jac6; ∈ [0.75, 1.33] × 4/|H| for xe and xeye. This must hold in ≥ 90% of cells.
- **P3, points per value.** The exact points per value over all of F_p lies in
  [0.99, 1.01] for every f and k (M1).
- **P4, desk facts** (exact, every k):
  - Max rational fibre = deg f. For xeye it is 4 at k = 16 and 2 at k = 18 and 20.
  - The rational signed symmetries equal the table in §2. `analyse.py` holds them as
    SYM_PRED, and adds (±1, ±Q₁) for xeye at k = 16.
  - Per-variable degree of S^f: x 2, ye 2, **xe 4**, v 9, q 9, xeye 4, jac6 18, uv 25. The
    kernel dimension is 1.
  - Total degree: x = 4; at most 2 × the per-variable degree for every other f.
  - All 1,000 fresh zeros vanish, and ≥ 99% of random triples do not.
  - The split-fibre count equals the per-variable degree wherever fully rational fibres
    exist.
- **P5, proxy:**
  - **P5a.** e = n for v, q, uv and jac6 at every T; e = 8 for xe and xeye at T = 4, 5, 6
    (that is e = n at T = 4); e = 4 for x and ye.
  - **P5b.** D_ref = n for v, q, uv and jac6 on every refutable target.
  - **P5c.** For x, D_ref ≤ n − 2 on ≥ 90% of refutable targets: ≤ 6 at T = 4 and ≤ 8 at T = 5.
  - **P5d.** Median M2 mul-adds per refutable target, v ÷ x and q ÷ x, ≥ 1.0 at T = 4 and
    T = 5 on every k. The expected point value is 1.5–3. M2 at n ≤ 10 touches about 80% or
    more of the cube even for x, so the proxy has little power to show a large gap at these
    sizes. The structural result is P5a–P5c.
  - **P5e.** Median D_ref(ye) is within 1 of median D_ref(x) at T = 5 on every k.
  - **P5f.** Median D_ref(xe) and D_ref(xeye) ≥ median D_ref(x) + 1 at T = 5 on every k.
  - **P5g.** M1 (plain Macaulay) needs the full multiplier space, d_mult = n, on ≥ 90% of
    refutable targets in all arms: plain Macaulay does not discriminate.
- **P6, cost per relation.** At T = 5, (M2 mul-adds ÷ base coverage) for v and q ÷ the same
  for x ≥ 1.0 on every k.
- **Consistency (must hold, else the instrument is wrong):**
  - every target the census covers has a true decomposition among the Boolean roots, and
    the converse;
  - rank V_n = 2^n − |Z| whenever M2 reaches D = n;
  - Ŝ_x has terms 121, 256 and 484 at T = 4, 5 and 6 (C1). 256 at T = 5 is the engine's
    `poly_terms_per_specialisation` at G = 0 (frame search, k = 20).
- **C2, engine calibration.** The engine's F4 `solving_degree_max` (x, G = 0) and the M2
  median D_ref(x) agree within ±1 at each (k, T ∈ {4, 5}). If they do not, M2 is reported as
  an upper bound and the gap is stated.

## 6. Controls

- **PC1, the brief's positive control.** The census must reproduce the published
  frame-search counts at k = 20 exactly: x ∈ [0, 2^t) gives 42 (t = 5) and 162 (t = 6);
  L = 976503 (t = 5) gives 198; L = 908172 (t = 6) gives 536.
  **PC1b.** Within the random x-frames of size 2^5 and 2^6 (arm xAA, t = 4 and 5) at k = 20,
  the argmax-J frame ÷ the mean is ≥ 1.5. The frame search measured 2.84 and 1.98. This shows
  the census sees a coset-selection effect of about 2–3×.
- **PC2, the estimator's positive control.** An x-frame restricted to P ∈ 2E (even log, the
  torsion-leads square-bits effect) has R_col ∈ [1.6, 2.4] in every cell. The matched-column
  ratio detects a 2× effect.
- **PC3.** The interpolated S^x is proportional to the explicit Weierstrass S₃.
- **NC1, A/A.** x-frames (|V| = 2^(t+1)) against cols-matched x-frames: R_col ∈ [0.95, 1.05]
  in ≥ 90% of cells.
- **NC2, random null.** Uniform random classes: R_col ∈ [0.9, 1.1] in every cell.
- **NC3, degree-2 negative control.** ye, a Möbius frame of x: R_col ∈ [0.9, 1.1] in every
  cell, per-variable degree 2, and P5e.

## 7. Failure, verdict and stopping rules

- **Instrument invalid** (no verdict on L2) if any of these fails: PC1 (exact replay), PC2,
  PC3, NC1, NC2, a consistency check, or the S^f verification.
- **L2 positive**, worth a Phase-3 engine implementation, if either holds:
  - some arm has R_col ≥ 1.5 with the interval's lower bound > 1.15 in ≥ 2/3 of its cells;
  - some f has M2 cost per relation ≤ 0.67 × x's at T = 5 on ≥ 2 of the 3 curves.
- **L2 negative** if P1 and P1x hold and P6 holds: degree ≥ 3 coordinate functions give no
  coverage per column and cost at least as much per relation. Higher-degree functions with
  a translation in their stabiliser (Edwards x, extended T/Z) cost coverage.
- Mixed outcomes are reported as they fall. Any failed P is reported as failed, with no
  retuning.

## 8. Claim scope and distance to rho

- This is a mechanism measurement on the algebraic track (`pkm-semaev`-type m = 2 systems)
  at k ≤ 20. It is not a leaderboard table A–C row and not a benchmark row. Nothing is
  extrapolated to 255 bits.
- Distance to rho is imported, not measured here (`curve25519_frame_search_20261103`
  README, "Costs and scope"):
  - at k = 20 a rho solve takes 547–936 group operations;
  - the best algebraic arm measured 18× (w = 0) to 5.9·10⁶× (w = 1) behind rho-neg;
  - the generic baseline is 7.8× rho at k = 20.
  - A coverage or cost factor of a few × cannot change any order across the §5a band.
  - If L2 is positive, the distance remaining is the imported figure divided by the measured
    factor, stated as a projection.

## 9. Runtime and memory estimate

See `smoke/SMOKE.md`, which is written after the smoke test. The budget is ≤ 3 h CPU on one
core and ≤ 4 GB.

## 10. Sources checked (repository files read)

- `research/curve25519_wave7_untried_20261122/BRIEF.md`
- `research/curve25519_frame_search_20261103/{README.md, PROTOCOL.md, scripts/frame_census.py, scripts/analyse.py, scripts/run_confirm.sh, scripts/run_confirm_rulefix.sh, scripts/run_engine.sh, analysis/summary.json}`
- `research/curve25519_t0_symmetry_20261102/README.md` (head): the T0-invariant degree-4 w is the
  x-coordinate of E/⟨T0⟩, so S^w has degree 2. This is why T0-invariant functions are not
  re-tested here.
- `research/curve25519_torsion_leads_20261103/README.md` (grep): u(P) square ⟺ P ∈ 2E, the source of PC2.
- `research/lead_sweep_20261110/LEAD-EXO-curve-model-representation/RESULT.md`: y, x/y and x + y
  were left unscored ("injective on F_p-points"). M1 and P3 here supersede that.
- `research/lead_sweep_20261110/LEAD-EXO-coordinate-revealing-oracles/RESULT.md`
- `research/prime_field_ic_closure_20261112/closure-ledger.md` (grep):
  - A4, rational-map image bases x = φ(t), is a different construction: the image of small
    parameters, not a preimage under a chart function.
  - A5's K1 counting argument is consistent with M1.
- `src/bin/curve25519_family.rs` (`support`, `f4cost`, `build_miniature`) and
  `src/cryptanalysis/ic_framework/pkm_semaev.rs`: the `Membership::Bits` formulation
  x = lo + Σ 2^j b_j with b_j² = b_j, which the proxy reproduces.
- The research-kb and the lead atlas are unreachable in this container (BRIEF).
