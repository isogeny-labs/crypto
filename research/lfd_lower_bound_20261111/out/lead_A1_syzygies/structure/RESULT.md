# Structure of the extra selector syzygies at e = e_min (task: syzygy-structure), 2026-11-12

All result statements below are `[pcc:unverified]`.

## Preregistration (frozen 2026-11-12, before any computation in this directory)

Notation as in `../RESULT.md`. Instance `(n, k=(n+1)/2, V, z, a6)`, unknown `lambda: V -> GF(2^n)`
with ANF coordinate degree `<= e`; `RM(d)` = Boolean functions on `V` of degree `<= d`;
`zeta = z^-2`, `ell_0(C) = Tr(C) + Tr(a6 zeta)`; `T_e = zeta (1+ell_0) RM(e-1)`.

- `I_e` = sections satisfying (I) only: `Tr(lambda(C) Phi*_C(v)) = 0` for all `v in V`, all `C`.
- `rho: I_e -> {functions on V}`, `rho(lambda)(C) = Tr(lambda(C) h_C)` (the c-only consequence
  `sum_i lambda_i f_i = rho(lambda)(c)` of the Weil-descent equations), `R_e = rho(I_e)`.
- Then `K_e = ker rho`, the feasible selectors of Prop. S are `rho^{-1}(1)`, and
  `dim K_e = dim I_e - dim R_e` (rank-nullity, a tautology).

Cells: `n = 15` seeds 1, 6 (`e_min = 3`); `n = 17` seeds 1, 8 (`e_min = 3`); `n = 19` seeds 4, 7
(`e_min = 4`); each at `e = e_min - 1` and `e = e_min` (h = 0/1, 0/1, 1/2).

**Hypotheses (each falsified by one cell that contradicts it):**

- **H_a (generated from below; "products of lower-degree solutions with the Koszul/trivial
  family").** `K_e ⊆ G_e := span{ m * mu : mu in I_d, d < e, m monomial, deg m <= e - d }`
  at `e = e_min`, i.e. every extra syzygy is a Boolean-multiple combination of lower-degree
  (I)-sections in which the (II) parts cancel. Falsified if `dim(K_e ∩ G_e) < dim K_e`.
  - H_a' (product with a feasible selector): no feasible selector of degree `< e_min` exists by
    definition, so the only product family available is `ell_0 * q * (lambda_0 + zeta)`
    (in the kernel, degree `e_min + 1 + deg q`); it cannot contribute at degree `e_min` unless
    a degree drop occurs. Tested by: does any `lambda_0` in the affine solution set satisfy
    `deg(ell_0 (lambda_0 + zeta)) <= e_min`? (equivalently a linear condition on the affine set).
- **H_b (orbit of one element under a symmetry).** There is a nontrivial group of symmetries
  of `K_e` in the classes (i) constant F2-linear maps on the values, `lambda(C) -> M lambda(C)`,
  `M in Mat_n(F2)` (this class contains Frobenius `x -> x^2`, multiplication by constants and
  trace twists `x -> Tr(alpha x) gamma`), (ii) translations `lambda -> lambda(. + v)`, `v in V`,
  under which `K_e / T_e` is cyclic. Test: the value stabiliser algebra
  `S = {M : M K_e ⊆ K_e}` and the translation-stable subspaces `{mu in K : mu(.+v) in K}`.
  Falsified (for these classes) if `S = F2 * Id` (or acts on `K/T` only by scalars) and no
  translation preserves `K_e`.
- **H_c (the excess is the jump of the consequence space; relation to the inhomogeneous
  system).**
  - H_c0 (literal form, tautological): the affine solution set of Prop. S at `e_min` is
    `lambda_0 + K_e`, of dimension `dim K_e = M(k-1, e_min-1) + x_{e_min}`. Checked only as a
    sanity control (a feasible `lambda_0` is found and verified).
  - H_c1: below `e_min` the (I)-sections are only the universal ones:
    `I_{e_min - 1} = zeta * RM(e_min - 1)`, `dim = M(k, e_min - 1)`, so
    `R_{e_min-1} = ell_0 RM(e_min-1)`.
  - H_c2: at `e_min` every c-only consequence of degree `<= e_min + 1` is reachable:
    `R_{e_min} = RM(e_min + 1)` (note `R_e ⊆ RM(e+1)` always, since `h_C` is F2-affine in `C`).
  - If H_c1 and H_c2 hold, then `x_{e_min} = dim I_{e_min} - M(k, e_min+1) - M(k-1, e_min-1)`,
    i.e. the excess is the (I)-section count minus a full Reed–Muller space; the excess is then
    "the feasibility jump": `1 in R_e` happens together with `R_e` becoming all of `RM(e+1)`.
    Falsified if `R_{e_min} != RM(e_min+1)` or `I_{e_min-1} != zeta RM(e_min-1)` in any cell.

**Controls.** Kernel dimension must equal `../grid.jsonl` (and `syz` rebuilt in this directory)
to the digit in every cell; `T_e ⊆ K_e`; every basis vector re-evaluated on all points by an
independent pointwise check (field-arithmetic evaluation of `Tr(lambda(C) w)`, not the matrix rows);
a feasible `lambda_0` at `e_min` verified on all points; positive control for the stabiliser
computation: `S(T_e)` must contain the maps fixing `zeta`'s line (it is large), and
`S(all functions)` = `Mat_n`.

### Addendum A (written after the n = 15 seed 1 cell only, before any other cell finished and before any a6-variation run)

Observation at n = 15 seed 1: H_c1 and H_c2 both held; `I_d = zeta RM(d)` for `d < 3` and
`I_3` jumps (309 vs 93). Since (I) does not involve `a6`, `I_d` depends on `(V, z)` only. Define
`d_I(V, z)` = least `d` with `I_d != zeta RM(d)`. Below `d_I`, `R_d = ell_0 RM(d)`, which does not
contain the constant 1, so `e_min >= d_I(V, z)` holds by construction (a short argument, written out
in the reading below).

New out-of-sample predictions, frozen now:

- **P_A1 (a6-independence).** For fixed `(V, z)` of `n = 15` seeds 1, 6 and `n = 17` seeds 1, 8,
  and fresh `a6'` drawn as in `make_instance` (same `a2`, `Tr(z + a2 + a6'/z^2) = 0`; 8 draws per
  instance, `random.Random(1000*n + seed)`): every **unsatisfiable** `a6'` has `e_min = d_I(V, z)`
  and `R_{d_I} = RM(d_I + 1)`. Falsified by one unsatisfiable `a6'` with a different `e_min` or a
  proper `R_{d_I}`.
- **P_A2 (satisfiable control).** Every **satisfiable** `a6'` has `R_{d_I}` a proper subspace of
  `RM(d_I+1)` and no feasible selector at `d_I` (forced: `rho(lambda)` vanishes at the `C` of a
  solution). This is a control that the detector sees the difference, not a hypothesis.
### Addendum B (written after the n = 15, 17 cells and the n = 19 seed 4 degree-<=3 log lines, before any a6-variation run)

Seen so far: H_c1 fails at n = 15 seed 6 (`dim I_2 = 38 > 37`) and at n = 19 seed 4
(`dim I_3 = 374 > M(10,3) = 176`); so `d_I < e_min` there and the conjunct `e_min = d_I` of P_A1 is
already falsified by the preregistered cells. H_c2 held in the four finished cells. The a6 scan is
still run, with these predictions frozen now:

- **P_B1 (H_c2 out of sample).** For every unsatisfiable `a6'` in the scan,
  `R_{e_min(a6')} = RM(e_min(a6') + 1)`.
- **P_B2 (a6-independence of e_min).** For fixed `(V, z)`, `e_min(a6')` is the same for every
  unsatisfiable `a6'` in the scan.
- P_A2 (satisfiable control) unchanged.
### Addendum C (written after the first a6 scan, n = 15, 17; before the n = 19 scan and before computing Z)

Observed in the first scan (`a6scan_v1.jsonl`): P_B1, P_B2 held; for satisfiable `a6'` the codimension
of `R_{e_min}` in `RM(e_min+1)` equalled `solutions/2` (2 -> 1, 4 -> 2, 6 -> 3, 8 -> 4). Frozen now:

- **P_C1 (Nullstellensatz form).** Let `Z = {C in V : h_C in Phi*_C(V)}` (the `C = a + b` of the
  solutions). For every `a6'` (satisfiable or not) in every scanned instance, at `e = e_min(V, z)` of
  that instance: `R_e = {f in RM(e+1) : f|Z = 0}` (i.e. every function of `R_e` vanishes on `Z`, and
  `dim R_e = M(k, e+1) - rank(eval of RM(e+1) on Z)`).
- **P_C2.** n = 19 seed 4 with 6 fresh `a6'` (`random.Random(1000*19+4)`): every unsatisfiable `a6'`
  has `e_min = 4` and `R_4 = RM(5)`.

---

## What was run

Local only, single-threaded, no RunPod, no S3. All from this directory
(`research/lfd_lower_bound_20261111/out/lead_A1_syzygies/structure/`); source tree commit
`2003c4bc85a6db91e74bfe19522a517b44995ff4` (working tree of the shared checkout).

- `syz` rebuilt here from the unmodified `../syz.c`
  (`gcc -O2 -march=native -o syz ../syz.c -lm4ri -lm`; `syz.c` sha256 `0cccd25f…c0c2`,
  binary `939f3050…849a`). Kernel dims of the 12 cells (n = 15, 17, 19; two seeds; `e_min − 1` and
  `e_min`) in `syz_rebuild_control.txt`: identical to `../grid.jsonl`.
- `syzstruct.py N SEED E_MIN` (`run_cells.sh` → `cells.jsonl`, `cells.log`): an independent code path
  (own field arithmetic, own trace rows, Python-integer GF(2) elimination). Per degree `d <= e_min` it
  computes `I_d`, `R_d = rho(I_d)`, `K_d = ker rho`, feasibility (`1 in R_d`). At `e_min` it runs the
  H_a, H_a', H_b tests and the controls. Wall time 10–205 s per cell, under 1 GB.
- `a6scan.py N SEED DRAWS` (`run_a6.sh` → `a6scan.jsonl`; first pass without `Z` in
  `a6scan_v1.jsonl`, reproduced exactly by the second pass): fixed `(V, z)`, fresh `a6'`
  (8 draws at n = 15, 17; 6 at n = 19 seed 4). Satisfiability is taken from `../../../emin count`
  (independent C code). 43 instances in total, 16 unsatisfiable, 27 satisfiable.
- `plot.py` → `structure.svg`, `structure.png`.
- The opt-in `a6=` argument of `syzstruct.Sys` defaults to the instance's own `a6`; all `cells.jsonl`
  rows use the default.

## Controls (all pass) `[pcc:unverified]`

- Kernel dims at `e_min − 1` and `e_min`: syzstruct = rebuilt syz = `../grid.jsonl` in all 12 cells
  (146/147, 87/88, 1164/1165; 8, 9, 46 below).
- `T_e ⊆ K_e`, `rank T_e = M(k−1, e−1)` in all 6 cells.
- Pointwise field-arithmetic check `Tr(lambda(C) w) = 0` (no trace rows): all kernel vectors at
  n = 15, 17; 200 random kernel vectors at n = 19. All pass.
- A feasible `lambda_0` at `e_min` was found and verified on all points by both the trace rows and
  field arithmetic, in all 6 cells (H_c0 sanity: affine solution set `lambda_0 + K_e`).
- Positive control for the stabiliser: `S(T_e)` has dimension `n² − n + 1` (211 at n = 15,
  273 at n = 17) as predicted, so the detector finds a large stabiliser when one exists.
  Every stabiliser solution was verified exactly against every basis vector.
- `I_d` is identical for every `a6'` of the same `(V, z)` (it does not involve `a6`), in all 43 scans.
- Satisfiable control (P_A2): all 27 satisfiable `a6'` have no feasible selector at any `d <= 4` and a
  proper `R_{e}`.

## Results `[pcc:unverified]`

### Per cell at e = e_min

| n | seed | h | dim I_d, d = 0..e_min | dim R_d | M(k, d+1) | dim K_e | excess | dim I_e − M(k,e+1) − M(k−1,e−1) | dim K ∩ G_e | new top generators |
|--:|--:|--:|:--|:--|:--|--:|--:|--:|--:|--:|
| 15 | 1 | 0 | 1, 9, 37, 309 | 1, 8, 29, 163 | 9, 37, 93, 163 | 146 | 117 | 117 | 29 | 117 |
| 15 | 6 | 1 | 1, 9, **38**, 310 | 1, 8, 30, 163 | 9, 37, 93, 163 | 147 | 118 | 118 | 30 | 117 |
| 17 | 1 | 0 | 1, 10, 46, 343 | 1, 9, 37, 256 | 10, 46, 130, 256 | 87 | 50 | 50 | 37 | 50 |
| 17 | 8 | 1 | 1, 10, 46, 344 | 1, 9, 37, 256 | 10, 46, 130, 256 | 88 | 51 | 51 | 37 | 51 |
| 19 | 4 | 1 | 1, 11, 56, **374**, 1802 | 1, 10, 46, 328, 638 | 11, 56, 176, 386, 638 | 1164 | 1034 | 1034 | 1080 | 84 |
| 19 | 7 | 2 | 1, 11, 56, **375**, 1803 | 1, 10, 46, 329, 638 | 11, 56, 176, 386, 638 | 1165 | 1035 | 1035 | 1081 | 84 |

`G_e = I_{e−1} + Σ_i c_i I_{e−1}` (all Boolean-multiple combinations of lower-degree (I)-sections; it
contains every `m * mu` with `mu in I_d`, `d < e`). "New top generators" = `dim K_e − dim(K_e ∩ G_e)`.
Bold: `I_{e_min−1}` larger than the universal `zeta RM(e_min−1)` (M(k, e−1) = 37, 176).

Value stabiliser `S(K_e) = F2 · Id` (dimension 1) in all 6 cells. Translation by a basis vector `v`
of `V` keeps exactly `dim T_e` of `K_e` when `Tr(v) = 0` and 0 otherwise. No translation preserves
`K_e`.

### a6 scan (fixed (V, z), 43 instances)

| (n, seed) | e_min(V,z) | unsat a6' : e_min, R_e = RM(e+1) | sat a6' : (#solutions, abs Z, codim R_e) |
|:--|--:|:--|:--|
| (15, 1) | 3 | 4 / 4: e_min = 3, full | (2,1,1) ×3, (4,2,2) ×2 |
| (15, 6) | 3 | 4 / 4: e_min = 3, full | (2,1,1) ×4, (8,4,4) |
| (17, 1) | 3 | 2 / 2: e_min = 3, full | (2,1,1) ×6, (4,2,2) |
| (17, 8) | 3 | 4 / 4: e_min = 3, full | (2,1,1) ×3, (4,2,2), (6,3,3) |
| (19, 4) | 4 | 2 / 2: e_min = 4, full | (4,2,2) ×2, (2,1,1) ×2, (6,3,3) |

In all 43 instances, at `e = e_min(V, z)` (and at `e_min + 1` where computed), `R_e` is exactly
`{f in RM(e+1) : f|Z = 0}`. Below `e_min`, `R_d` is the same subspace for every `a6'`: it does not
see `a6` at all.

## Verdicts against the preregistration `[pcc:unverified]`

| Hypothesis | Verdict | Evidence |
|:--|:--|:--|
| **H_a** (extra kernel generated from lower-degree sections) | **refuted** (6/6 cells) | n = 15, 17: `K_e ∩ G_e = T_e` (plus 1 at n = 15 s6), so the whole excess is new top-degree generators. n = 19: 950 of 1034 extra dimensions *are* generated from `I_3`, but 84 are not, in both seeds. |
| **H_a'** (product of a feasible selector with ell_0) | **refuted** (6/6) | No `lambda` in `lambda_0 + K_e` has `deg ell_0 (lambda + zeta) <= e_min`. |
| **H_b** (orbit of one element under value-linear maps or translations) | **refuted** for these symmetry classes (6/6) | `S(K_e) = F2 · Id`; the positive control finds `S(T_e)` of the predicted dimension. Frobenius, constant multiples and trace twists are all inside `Mat_n(F2)`, so none of them acts. No translation preserves `K_e`. |
| H_c0 (literal: affine solution set) | tautology, sanity passes | `dim(lambda_0 + K_e) = dim K_e`; `lambda_0` verified. |
| **H_c1** (`I_{e_min−1} = zeta RM(e_min−1)`) | **refuted** | It holds at n = 15 s1, 17 s1, 17 s8. It fails at n = 15 s6 (38 vs 37) and at n = 19 (374/375 vs 176). |
| **H_c2** (`R_{e_min} = RM(e_min+1)`) | **held** (6/6 cells, plus all 16 unsatisfiable a6 instances) | Table above. |
| H_c formula `x = dim I_e − M(k,e+1) − M(k−1,e−1)` | **held** (6/6) | It follows from H_c2 by rank–nullity. |
| P_A1 (`e_min = d_I`, plus a6-independence) | **falsified** through its `d_I` conjunct | n = 15 s6: `d_I = 2`, `e_min = 3`; n = 19 s4: `d_I = 3`, `e_min = 4`. |
| P_B1 (H_c2 out of sample) | **held** (16/16 unsatisfiable instances, 11 of them fresh draws) | |
| P_B2 (e_min independent of a6') | **held** (5 (V,z), 16 unsatisfiable instances) | |
| P_C1 (Nullstellensatz form `R_e = I(Z) ∩ RM(e+1)` at `e_min(V,z)`) | **held** (43/43, satisfiable included) | |
| P_C2 (n = 19 s4 out of sample) | **held** (2 unsatisfiable draws: e_min = 4, `R_4` full) | |

## Reading: an algebraic description of the extra syzygies `[pcc:unverified]`

**1. The exact sequence (proved here, elementary).**
- (I) is pointwise linear in `lambda(C)`, so `I_e` is a space of sections of the bundle
  `C ↦ {y : Tr(y Phi*_C(v)) = 0 for all v in V}`.
- `rho(lambda) = Tr(lambda h)` is linear, and `rho(lambda)` has degree `<= e + 1`, because
  `h_C = z² C² + a6` is F2-affine in `C`.
- For `C in Z` (`h_C = Phi*_C(A)` with `A in V`), `rho(lambda)(C) = Tr(lambda(C) Phi*_C(A)) = 0`.
- So `0 → K_e → I_e → R_e → 0` with `R_e ⊆ I(Z) ∩ RM(e+1)`.
- `rho(zeta q) = ell_0 q` (F1/F2), so `ell_0 RM(e) ⊆ R_e`.
- A feasible selector exists iff `1 in R_e`. `Z = ∅` iff the instance is unsatisfiable.

**2. What is measured.**
- For unsatisfiable instances, `e_min` is exactly the degree at which `rho` becomes *onto*
  `RM(e+1)` (H_c2).
- More generally, for every `a6` it is the degree at which `R_e` reaches its ceiling `I(Z) ∩ RM(e+1)`
  (P_C1, 43/43).
- Below that degree, `R_d` is the same strictly smaller space for every `a6`. So in these instances
  `e_min` is a function of `(V, z)` alone, the saturation degree `s(V, z)` of the consequence map.
  `a6` only decides whether `1` lies in the ceiling (`Z = ∅`).

**3. The extra syzygies.**
- At `e_min`: `K_e = ker(rho: I_e →> RM(e_min+1))` and
  `x_{e_min} = dim I_{e_min} − M(k, e_min+1) − M(k−1, e_min−1)`.
- The excess is therefore a property of the (I)-only bundle, which does not involve `a6` or `h`,
  together with the surjectivity of `rho`.
- The `+h` of `../RESULT.md` point 7 sits in `dim I_e`: 309/310, 343/344, 1802/1803.
- Below `e_min`, `rho` is injective modulo `T` (excess 0) even where `I_{e−1}` is much larger than
  `zeta RM` (n = 19: 198 non-universal sections at `d = 3`, all with distinct consequences).
- Hence: the extra syzygies are not a structured family (H_a, H_b refuted). They are the kernel of
  a surjection that appears exactly when the consequence space fills up.

**4. Generation degree.**
- At n = 15, 17 all of the excess consists of new degree-`e_min` generators.
- At n = 19, 950 of the 1034 extra dimensions are `c_i`-multiples of degree-3 (I)-sections whose
  consequences cancel. Only 84 are new generators (84 = C(9,3); a coincidence check is not done).

**What this does and does not give for a lower bound.**
- `e_min >= s(V, z)` holds by definition of `s` for unsatisfiable instances.
- `s(V, z)` is now an `a6`-free, purely homogeneous linear-algebra quantity.
- A lower bound on `e_min` reduces to showing that `rho(I_d)` misses `I(Z) ∩ RM(d+1)` for
  `d < d_0`. This needs only the (I)-bundle and the Reed–Muller structure, not the inhomogeneous
  row.
- That reduction is a reformulation (Prop. S* is its dual), not a proof of growth.

## What is blocked / not done

- Only n = 15, 17, 19, two seeds each, plus 43 a6 variants at those sizes. No n >= 21, no T-type
  (`V ⊆ ker Tr`) instances, no planted controls.
- 84 = C(9,3) at n = 19 is noted, not tested.
- No closed form for `dim I_e` or for `s(V, z)`.
- research-kb and ecdb were not queried from this subagent. No novelty claim is made.
- No RunPod was used (the task said local only). Nothing about deployed curves.

## Next step

1. Test P_C1/P_B2 at n = 21, 23 (seed pairs from `../grid.jsonl`), using M4RI. It is a small
   extension of `syz`: add the `rho`-image rank as an opt-in mode.
2. Characterise `s(V, z)` through the dual of `rho`: the functionals on `RM(e+1)` vanishing on
   `R_e` are the Prop. S* pseudo-solutions with `Z = ∅`. Count them as a function of `d`, to get a
   counting formula for `e_min` that does not involve `a6`.
3. Lean obligation: the exact sequence `0 → K_e → I_e → I(Z) ∩ RM(e+1)` and `rho(zeta q) = ell_0 q`.
