# RESULTS: KM3-SPEC-LADDER (wave 2, lab B), 2026-11-15 [pcc:unverified]

PROTOCOL.md sha256 (frozen before any data): edecadfdd6d49a51343162fcbc489e1c2b21b5e6606e6d1c011dd850fa0864f1

Every number is [pcc:unverified]: from `spec_ladder.py` (no replay certificate, no pcc claim id).

## Route and deviations from PROTOCOL.md
- The Rust pilot could not be built here (the build did not finish within the harness limits; the coordinator reports
  `cargo` cannot build the crate offline, missing `aws-credential-types`). The `--target-xr` / `--spec-ladder` edit of
  `examples/pkm_tower_pilot.rs` is saved as `target-xr.patch` and is **untested** (never compiled). The file was
  restored with `git checkout`.
- Measurements use lab A's independent Python HKY closure (`../KM3-LQR-CLOSURE/hky_closure.py`, validated by lab A against
  committed Rust d_F values) and lab N1's instance generator (`ns_degree.make_instance`). So the instances are N1's draws,
  not the pilot's seed 0x504B4D54 draws; the prime P1 = 2013265921 is **not** covered (the closure uses float64 mod-p
  arithmetic, p < 2^16 only). Primes: 40961 and 12289, 2 draws per cell.
- Preregistered cells measured: m = 3 at t = 4, 5 (cmax 8) and t = 6 were **not completed**. m = 3 at t = 2, 3 complete
  (2 draws x 2 primes). m = 3, t = 4, p = 40961, draw 0: c = 2..5 do not contain 1 (cols 93, 392, 1186, 2772), the c = 6
  closure was still running after 48 min and was killed. So d_F(3,4) >= 6 on that one draw; the other draws and t >= 5
  have no m = 3 value. The specialised m = 2 arm was measured at t = 2..5 on all draws.
- No Rust muladds, no secondary hybrid readout.

## Table (primes 40961, 12289; 2 draws each; all four cells of a row agree)
| t | d_F(m=3) | n distinct rho per draw | d_F(m=2, target rho), all rho | Delta = d_F(3) - max_rho d_F(2) | lemma violations |
|---|---|---|---|---|---|
| 2 | 5 (4/4) | 4, 6, 6, 8 | 4 (all 24 rho) | 1 | 0 / 24 |
| 3 | 6 (4/4) | 4, 8, 10, 4 | 4 (all 26 rho) | 2 | 0 / 26 |
| 4 | >= 6 (1 draw, c=6 unfinished); others not run | 16, 12, 16, 20 | 4 (all 64 rho) | not computed | not checkable beyond 1 draw (vacuous: bound 4 <= 6) |
| 5 | not run | 36, 36, 32, 30 | 4 (all 134 rho) | not computed | not checkable |

"n distinct rho" counts distinct F_p-roots rho of S3(u, x3, x_R) over grid x3 (the m = 2 system depends only on rho).
Per draw, the grid x3 with no F_p root (discriminant a non-residue) number 1..17 and are not covered by the numerics
(see the F_{p^2} remark in the proof). No specialised system had a grid zero (all inconsistent), consistent with every
measured m = 3 system having finite d_F.

## Prediction check
- Lemma P: held on 50/50 (draw, rho) pairs with a finite m = 3 value (t = 2, 3). Not falsified. At t = 4, 5 the
  m = 2 side was measured but the m = 3 side was not, so those pairs were not tested.
- H_inherit (d_F(3,4) = d_F(3,5) = 6, Delta = 2): **undecided**. d_F(3,4) >= 6 on one draw (consistent, not decisive);
  nothing at t = 5. Note Delta = 1 at t = 2 and 2 at t = 3 (m = 2 reads 4 in both).
- Specialised m = 2 d_F is 4 for every rho at t <= 5, independent of rho; it equals the committed m = 2 value 4.

## Controls
- Positive: the engine reproduces the committed Rust d_F values: m = 2 d_F = 4 (t <= 5) and m = 3 d_F = 5, 6 at t = 2, 3
  (`../../prime_ic_leads_20261110/devcheck/m2_t2-10.jsonl`, `m3_t2-3.jsonl`). It checks the engine, not instance identity.
- Negative / power: the reversed inequality d_F(3) <= d_F(2; rho) is flagged on every pair at t = 2, 3 (50/50), so the
  comparison can fail. Out-of-hypothesis arm rho + 1 (not a root) also reads 4 everywhere, so the specialised value does
  not distinguish roots from non-roots.
- Planted (consistent) m = 3 target never reaching 1: not rerun here; lab A's `../KM3-LQR-CLOSURE/RESULTS.md` has it (passed).
- A/A: not run.

## Commands (cwd = scratch dir; OMP_NUM_THREADS=1 OPENBLAS_NUM_THREADS=1)
```
SL_OUT=raw python3 spec_ladder.py P T IDX 8 7        # P in 40961 12289, T in 2 3, IDX in 0 1   (run_t23.log)
SL_OUT=raw_spec python3 spec_ladder.py P T IDX 1 7   # T in 4 5, specialised arm only (cmax3 = 1)  (run_spec45.log)
SL_OUT=raw_m3t4 python3 spec_ladder.py 40961 4 0 7 7 # m = 3 t = 4, killed in c = 6 (m3t4_i0.log)
```
Runtime: t = 2 under 1 s; t = 3 about 58 s per cell (m = 3 closure); t = 4 spec arm about 10 s; t = 5 spec arm 150-195 s;
m = 3 t = 4 c = 6 over 48 min, unfinished. Four labs shared the host.

## What is and is not shown
Shown (small cases, Python closure): d_F(3,2) = 5, d_F(3,3) = 6 on 4 draws; the m = 2 specialisations all read 4 and the
inequality of Lemma P held on 50/50 pairs; the m = 2 values at t = 4, 5 are 4 for 198 rho.
Not shown: any m = 3 value at t >= 4 beyond d_F >= 6 on one draw; P1; the Rust route (patch untested); the F_{p^2} case
numerically; any bound growing in t (specialised m = 2 d_F is flat at 4, so Lemma P gives only 4 here).

## Lemma P (partial specialisation, m = 3) [pcc:unverified]

Prose proof in the style of D6 of `research/prime_fall_degree_20261027/DERIVED.md`. It is an
unverified assertion; it has no pcc claim id and no Lean proof.

**Setting.** `K = 2^t`. `R_3 = F_p[y_1, y_2, y_3, u]/T` is the m = 3 tower quotient: three
Kummer towers `y_{i,j}` (`j < t`, same `g`, same relations `T_i`) and one free unknown `u`.
The chain is `F_1 = S3(x_1, x_2, u)`, `F_2 = S3(u, x_3, x_R)`, with `x_i = g y_{i,0}`.
`R_2 = F_p[y_1, y_2]/T` is the m = 2 quotient. `f_1, f_2` are the normal forms of `F_1, F_2`
in `R_3`, and `V_{F,c}` is the HKY closure of D6's convention (smallest subspace of `R_{<=c}`
containing the generators of degree `<= c` and closed under `g -> y g` and `g -> u g`
(rewritten to normal form) for `deg g < c`).

**Hypotheses.**
1. (H1) `x_3 ∈ V`, i.e. `x_3` is a grid point; write `a_j = x_3^{2^j}/g^{2^j}`-type constants,
   precisely the values of the tower variables `y_{3,j}` at `x_3` (D1, D2).
2. (H2) `ρ ∈ F_p` and `F_2(ρ, x_3, x_R) = S3(ρ, x_3, x_R) = 0`. The polynomial
   `S3(u, x_3, x_R)` is quadratic in `u` with leading coefficient `(x_3 − x_R)^2`. So it has a
   root in `F_p` iff its discriminant is a square in `F_p`, which holds for about half of the
   `x_3`. If `x_3 = x_R`, it has degree `<= 1` in `u`; the hypothesis `x_R ∉ V` excludes this.
3. (H3) `d_F(F_3) < ∞`, i.e. `1 ∈ V_{F_3,c}` for some `c` (otherwise the inequality is
   `∞ >= d_F(F_2(ρ))` and says nothing; the contrapositive is that a finite `d_F(F_3)` forces
   `F_2(ρ)` to be inconsistent over `F̄_p`).

**Statement.** Under H1, H2: `d_F(F_3) >= d_F(F_2(ρ))`, where `F_2(ρ) = {S3(x_1, x_2, ρ)}` is the
m = 2 system with target `ρ` on `R_2`. In particular `1 ∈ V_{F_3,c}` implies `1 ∈ V_{F_2(ρ),c}`.

**Proof.**
- *The ring map.* Define `φ: R_3 -> R_2` on `F_p[y_1, y_2, y_3, u]` by `y_{1,j} -> y_{1,j}`,
  `y_{2,j} -> y_{2,j}`, `y_{3,j} -> c_j` (the tower values at `x_3`, constants), `u -> ρ`. It
  descends to `R_3 -> R_2`: `T_1, T_2` map to themselves, and `T_3` maps to `0` because `x_3` is
  a grid point (the tower relations hold at the values of a grid point, D1).
- *Degree.* A normal-form monomial of `R_3` is a squarefree product of `y_{i,j}` and a power
  `u^e`. Its image is a constant times a squarefree product of the surviving `y_{1,j}, y_{2,j}`.
  This is again a normal-form monomial of `R_2`, of degree at most the original. So `φ` maps
  `R_{3,<=c}` into `R_{2,<=c}` with no rewriting needed, and it commutes with normal-form
  reduction (it is a ring map and sends the Gröbner basis `T` into `T`'s ideal).
- *Generators.* `φ(f_2)` is the normal form of `S3(ρ, x_3, x_R)` (a constant), which is `0` by H2.
  `φ(f_1)` is the normal form of `S3(x_1, x_2, ρ)`, i.e. the sole generator of `F_2(ρ)`.
- *Induction on the closure* (D6 verbatim, with `u` also a scalar image). Claim:
  `φ(V_{F_3,c}) ⊆ V_{F_2(ρ),c}`.
  - Base: a generator of degree `<= c` goes to `0` or to the generator of `F_2(ρ)`; its degree does
    not rise.
  - Step: take `g ∈ V_{F_3,c}` obtained at an earlier stage with `deg g < c`, and suppose
    `φ(g) ∈ V_{F_2(ρ),c}` with `deg φ(g) < c` (true since `deg φ(g) <= deg g`). For a variable
    `y_{1,j}, y_{2,j}`: `φ(y g) = y φ(g)`, which is the closure step in `R_2`. For `y_{3,j}`:
    `φ(y_{3,j} g) = c_j φ(g)`, a scalar multiple of an element already in the subspace. For `u`:
    `φ(u g) = ρ φ(g)`, likewise. Linear combinations are preserved because `φ` is `F_p`-linear.
- *Conclusion.* If `1 ∈ V_{F_3,c}` then `1 = φ(1) ∈ V_{F_2(ρ),c}`. So the least such `c` for
  `F_2(ρ)` is at most that of `F_3`. ∎

**What the hypotheses are doing.**
- `ρ ∈ F_p` is needed so that `φ` is a map of `F_p`-algebras and the m = 2 side is a system over
  the same field, the one the engine computes with. It is not needed for the inequality itself.
- If `S3(u, x_3, x_R)` has no root in `F_p` (discriminant a non-residue), pass to `F_{p^2}`.
  `V_{F,c}` is the span of explicit linear combinations, so it commutes with base change: its
  dimension and membership of `1` are the same over `F_p` and over `F_{p^2}` (rank of a matrix
  with entries in `F_p` does not change under a field extension). `φ` is then a map
  `R_3 ⊗ F_{p^2} -> R_2 ⊗ F_{p^2}`, with `ρ ∈ F_{p^2}`, and the proof goes through verbatim:
  `d_F(F_3) >= d_F^{F_{p^2}}(F_2(ρ))`. That right-hand side is not computed by the F_p engine,
  so the numerical check below covers only the F_p roots; the `F_{p^2}` case is argued, not measured.
- `ρ` can be taken to be any element of `F̄_p` for which `S3(ρ, x_3, x_R) = 0`, and the same proof applies over
  `F_p(ρ)`. Over `F̄_p` a root always exists (degree 2, leading coefficient non-zero).
- The grid-point hypothesis on `x_3` is essential: for `x_3 ∉ V` the substitution of `T_3` is not
  a ring map (D6 has the same condition).
- Lemma P gives a *lower* bound on `d_F(F_3)` only through what is known about `d_F(F_2(ρ))`.
  `d_F(F_2(ρ))` is an m = 2 value for a *particular* target `ρ` that need not be a random point's
  abscissa on the curve; it is whatever the engine reads, and a Kummer m = 2 lower bound that holds for all
  inconsistent targets (hypothesis of the lead) is needed to make the bound grow.
- Generalisation to any `m >= 3`: specialise the last tower and `u_{m-2} -> ρ`; `F_{m-1} -> 0`, the others
  keep their form and the last link's partner is `F_{m-1}(ρ)` with target `ρ`. Same proof.
