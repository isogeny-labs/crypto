# Theorems A and B under a first-fall-degree / F4 model (f4-model follow-up, 2026-11-15)

Standing: every statement here is `[pcc:unverified]`. Paper arguments only; nothing is machine-checked.
Notation follows `finish/theorem/PROOF.md`: m = arity, (U) base f(x) squarefree of degree d,
S_R = S_{m+1}(x_1..x_m, x(R)), Δ = deg S_R, A = k[x_1..x_m]/(x_1^d, ..., x_m^d).

## 0. Two solver models

- **M1 (first-fall-degree model).** The solver is a Macaulay-type method (XL, MutantXL, F4 with full
  Macaulay rows) that works at a degree D at least the first fall degree D_ff of the system, the
  usual heuristic "solving degree ≈ D_ff". An attempt that reaches degree D is charged rank Mac_D
  (as axiom A1 of PROOF.md, with D_ff in place of the XL degree).
- **M2 (F4 as implemented, msolve 0.6.5).** Cost is what msolve actually builds: rows, rows reduced,
  matrix entries (nnz), time. There is no theorem in M2; only measurements (`out/tables.md`).

## 1. Theorem A (m = 2) is solver-independent

PROOF.md states Theorem A for "any base, any solver" with attempt cost ≥ 1. Its proof uses only
π ≤ |B|²/n (Lemma 2a), Lemma 9 and `master_sq`. Neither D_XL nor the XL rank appears.
**It holds verbatim under M1 and M2.** The two hidden assumptions the checker found are
solver-independent and unchanged:
1. "≤ 1 relation per attempt" can be replaced by "an attempt costs at least the number of relations it
   outputs" (output size). That is true for any solver, F4 included, since each relation is written.
   With that replacement the c|B| branch goes through. (Desk argument, not checked.)
2. A3 (h ≥ c|B|) fails for known-log bases such as B = {iP}; F4 does not change this.

## 2. Lemma FF (exact first fall degree of the (U) system)

**Lemma TF (top form).** For m = 2, 3, 4 the homogeneous top-degree part of S_R in x_1..x_m is exactly
∏_{i≤m} x_i^{2^{m-1}}, with coefficient 1 over Z, for every a, b, x(R). So Δ = m·2^{m-1}.
*Evidence:* read off the generic S_3, S_4, S_5 over Z[a,b,x_1..x_{m+1}] (`semaev_polys.pkl`, built
by resultants and checked positive 30/30 (m=2,3) and 10/10 (m=4) at every prime 1021, 4093, 65521,
1048573): the coefficient of ∏x_i^{2^{m-1}} is 1, and no other monomial has x-degree Δ
(`code/topform_check.py`, `out/topform_check.json`, also 40/40 or 10/10 specialised targets
per prime and m). For m ≥ 5 this is a conjecture.

**Lemma FF.** For (U) with deg f = d and m ≤ 4, the first fall degree (least degree of a non-Koszul
syzygy of the top forms x_1^d, ..., x_m^d, S_R^h) is

  D_ff = Δ + max(0, d − 2^{m-1}) = (m−1)·2^{m-1} + d   for d ≥ 2^{m-1}.

*Proof.* x_1^d..x_m^d is a regular sequence, so a non-Koszul syzygy in degree D is a nonzero
u ∈ A_{D−Δ} with S_R^h·u = 0 in A. S_R^h is a monomial (Lemma TF), so multiplication by it sends
the monomial basis of A to monomials or to 0, injectively on the monomials it does not kill.
Its kernel is therefore spanned by monomials u with u_i + 2^{m-1} ≥ d for some i. The least degree of
such a u is max(0, d − 2^{m-1}). ∎

In particular D_ff ≥ d always: no degree fall exists below the base degree.

*Measured:* D_ff computed by exact rank (independent of the lemma) and msolve's maximal F4 pair degree
D_F4 both equal the formula in every (U)/interval cell with status ok (count in §4).

**Degree gap.** With the generic interpolation degree δ(1/S_R) = m(d−1) (Lemma 4 of PROOF.md),
D_XL − D_ff = (m−1)d + 2^{m-1} − m: d at m = 2, 2d+1 at m = 3, 3d+4 at m = 4.
The F4 degree is the base degree plus a constant, while the XL degree is m times the base degree.
In proof-complexity terms D_XL is the Nullstellensatz degree of the refutation and D_F4 is the
polynomial-calculus degree. The checker's "F4 d+2 vs XL 2d+2 (m=2), d+8 vs 3d+9 (m=3)" is this
lemma.

## 3. Theorem B under M1

**Theorem B-FF.** In model M1, for (U) with deg f = d and m ≤ 4, every schedule satisfies

  E[total] ≥ n · C(m + d − 2^{m-1}, m) / C(2d + m − 1, m)  →  2^{−m}·n   (d → ∞).

*Proof.*
- Below D_ff the Macaulay spaces have no degree falls. Every top-form syzygy is Koszul, and Koszul
  syzygies of the top forms lift to exact syzygies of the full generators, so
  Mac_D ∩ k[x]_{<D} = Mac_{D−1} for D < D_ff.
- So below D_ff: (i) 1 ∉ Mac_D; (ii) Mac_D contains no block-1 univariate u ∉ J_1. Any element of
  degree < d lies in Mac_{d−1}, whose top forms are multiples of ∏x_i^{2^{m-1}} and so are not
  univariate. (iii) The rank of Mac_D is the Hilbert function of the target-independent monomial ideal
  (x_i^d, ∏x_i^{2^{m-1}}), so the observations do not depend on R.
- Hence attempts that stop below D_ff never succeed, and their stopping cannot depend on R.
  Every other attempt reaches D_ff and pays rank Mac_{D_ff} ≥ dim S_R·k[x]_{≤ D_ff − Δ}
  = C(m + d − 2^{m-1}, m).
  (Multiplication by S_R is injective on polynomials; for this count the f-rows are not needed.)
- N_dec ≤ C(2d+m−1, m) and Lemma 9 with h ≥ 1 conclude, exactly as in PROOF.md. ∎

The constant is better than Theorem B's (2^{m-1} < Δ in the binomial). **Theorem B's closures of A5
and A6-APS therefore survive unchanged under M1, for m ≤ 4.** This is close to a tautology: M1 charges
the full Macaulay rank at a degree ≥ d, and that rank is already ~d^m/m!. It says nothing about
an F4 that does not build the full Macaulay matrix. That is model M2, and only measurements cover it.

**Graph (Γ) family under M1: not established.** (Γ) is not an H-basis: the base ideal itself has
degree falls, so "first fall degree" must be defined relative to J. That was not done here.

**Boolean bits (β) / towers (T):** Theorem C territory. M1 adds nothing without hypothesis G.

## 4. M2 (msolve F4): what the measurements say

Frozen rule: `PRESET.txt` (sha256 a9bcaffb…), written after a 5-cell smoke test and before the sweep.
Sweep: 267 cells, 258 with msolve status ok. Primes: 1021 (12 cells), 4093 (22), 65521 (112) and
1048573 (112). Curves have prime order. Targets are uniform. Two seeds per size. msolve 0.6.5 was run
with `-g 2 -t 1`, 4 GB RLIMIT_AS (6 GB for the bits m=4 rerun), and a 900 s timeout.
Tables: `out/tables.md`; fits: `out/summary.json`; local slopes: `out/local_slopes.txt`; figure:
`out/f4_vs_xl.{svg,png,pdf}`.

**Integrity checks.**
- msolve's verdict ("GB = {1}" / not) agrees with the grid-zero count V_R on 247/247 ok cells
  where V_R was computed. 10 of the targets were decomposable.
- Semaev S_3, S_4 and S_5 vanish on actual decompositions in 30/30 (m=2,3) and 10/10 (m=4)
  trials at every prime.

**Degrees (no-aux families: uni and interval).**
- D_F4 = D_ff = Δ + max(0, d − 2^{m-1}) on 158/158 ok cells. The rank-based D_ff was computed on
  every one of those cells.
- D_XL − D_F4 = (m−1)d + 2^{m-1} − m on 128/128 cells with d ≥ 2^{m-1} where D_XL was computed
  (V_R empty).
- Examples:
  - m = 2, d = 512: F4 514 against XL 1026;
  - m = 3, d = 64: F4 72 against XL 201;
  - m = 4, d = 16: F4 40 against XL 92.

**Graph family (m = 3).** D_F4 grows by about 0.4–0.6 per unit of d_T over d_T ≤ 24, starting from
the S_4 constant 12. At d_T = 16 and 24 (graph2), D_F4 = d_T.

**Cost exponents k (log-log slope in s, over the largest 4 sizes; uni/interval s = d; graph s = |T|; bits s = 2^s):**

| route | k sum_rows | k rows_reduced | k nnz | k time | preset e_F4 | preset verdict | Thm B exp-1 (k ≥ m−1.1) |
|---|---|---|---|---|---|---|---|
| uni m=2 (A5) | 1.92 | 1.01 | 2.01 | – | 1.00 | SURVIVES | yes |
| uni m=3 (A5) | 4.46 | 2.07 | 4.96 | 5.02 | 1.00 | SURVIVES | yes |
| uni m=4 (A5) | 6.51 | 0.10 | 8.45 | 6.38 | 0.50 | INCONCLUSIVE | no (see below) |
| interval m=3 (A6-APS) | 3.68 | 1.58 | 5.06 | – | 0.86 | SURVIVES | no |
| interval m=4 (A6-APS) | 4.14 | 2.42 | 6.60 | 5.78 | 0.86 | SURVIVES | no |
| graph2 m=2 (A4) | 1.89 | 0.96 | 1.99 | – | 1.00 | SURVIVES | yes |
| graphr m=2 (A4) | 2.52 | 0.95 | 2.28 | – | 1.00 | SURVIVES | yes |
| graph2 m=3 (A4) | 2.75 | 0.97 | 3.94 | 4.46 | 0.67 | SURVIVES | no |
| graphr m=3 (A4) | 3.24 | 1.74 | 4.52 | – | 0.91 | SURVIVES | no |
| bits m=3 (A6 Boolean, Thm C) | 3.14 | 3.54 | 5.57 | – | 1.00 | SURVIVES (3 sizes only) | yes |
| bits m=4 | – | – | – | – | – | NO FIT: s=3,4 segfault at 6 GB | – |

**How to read the table.**
- The preset takes k = min over the proxies. That is always `rows_reduced`, msolve's count of rows
  sent to reduction, which is a count and not a work measure.
- The work msolve actually does is at least the matrix entries it writes (nnz). By that measure k is
  4–8.5 at m ≥ 3, above XL's own column growth (k_XLcols 2.4–2.7).
- **F4 reaches a degree about m times lower than XL, but builds more matrix entries per attempt than
  the XL^J rank and more than the d^m grid at every m ≥ 3 measured.**
  - Example: uni m=3, d=64 builds 9.2·10^9 entries against 1.4·10^6 XL columns and a grid of 2.6·10^5.

**uni m=4 is pre-asymptotic, and the preset's INCONCLUSIVE comes from that.**
- d ≤ 8 = 2^{m-1} sits on the D_ff = Δ = 32 plateau.
- The fit window [8, 10, 12, 16] straddles that regime change. `rows_reduced` is non-monotone there:
  32751, 12143, 23343, 28211.
- Post-preset, restricted to d > 8 (d = 10, 12, 16), the slopes are:
  - rows_reduced 1.69 (e = 0.67);
  - sum_rows 5.16, nnz 12.3 and time 11.4 (e = 1).
- At d ≤ 16, D_F4 = d + 24 is still dominated by the constant, so no m=4 asymptotic claim is made.

**Post-preset correction to the preset formula.** max((1+k)/m, 2/m) fixes |B| = n^{1/m}. With
linear algebra free, as in PROOF.md A3, the IC-optimal exponent is min(1, (1+k)/m), which can only
be lower. Every m ≥ 3 route except uni m=4 still reads ≥ 0.66 on the least favourable proxy (graph2
m=3 is lowest at 0.66), and every route reads 1.00 on nnz and time.
