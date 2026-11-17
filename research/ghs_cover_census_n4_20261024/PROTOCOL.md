# Protocol: GHS cover census for `n = 4` (`E / F_{q^4}`, odd characteristic)

2026-10-24. Frozen before `cover_census_n4.rs` was written and before any
run. **Structural stage diagnostic on toy fields.** It counts covers by
Riemann-Hurwitz genus. No curve over `F_q` is constructed, no transfer map is
evaluated, no discrete logarithm is solved, and no speedup is claimed. No
claim about any deployed curve. Conductor is not installed in this session
and was skipped. Implementation language: Rust for every computed number;
Python only lays out recorded values (plots, PDF), per AGENTS.md "Implementation
language: no Python". Every number below is labelled **derived** (proved here),
**measured** (written by the tool) or **model** (asymptotic formula, not
measured).

Extends [`../ghs_cover_census_n5_20261013/`](../ghs_cover_census_n5_20261013/)
(`n = 3, 5`, prime `n`; its README, PROTOCOL and `cover_census_n.rs` are the
baseline and are not edited).

## Question

`K = F_{q^4}`, `E: y^2 = f(x)` over `K`, `B = x(E[2]) U {inf}` (4 points of
`P^1(K-bar)`). For each `F_q`-structure `tau` on `P^1_K` (`tau = M sigma M^-1`,
one per coset `M PGL2(F_q)`; `sigma` = `q`-Frobenius; `tau^4 = id`) let
`B' = M^-1 B`, `v = 1_{B'}`, `m = dim span{sigma^i v}`, `U = U_i sigma^i B'`,
`r = |U|`. The compositum of the four conjugate double covers has group
`(Z/2)^m` and genus `g = 2^(m-2)(r-4)+1` (every point of `U` has inertia of
order 2). **What is the minimal genus over `tau`, per 2-torsion class, and
which structures are admissible GHS descents?**

## What must change relative to `cover_census_n.rs` (n prime)

`n = 4` is composite, so the `sigma`-orbit of `B'` can have length 1, 2 or 4
(for prime `n` only 1 or `n`, which is why the old tool tested only `m = 1`).

1. **Disjointness fast path up to `k <= n/2`.** `sigma^i B'` and `sigma^j B'`
   meet iff `B'` meets `sigma^(j-i) B'`, and `sigma^(n-k)` is equivalent to
   `sigma^k`; so pairwise disjointness of all `n` sets is `gcd(B', sigma^k B')=1`
   for `1 <= k <= floor(n/2)`. The old tool used `(n-1)/2`, which equals
   `floor(n/2)` for odd `n` (so `n = 3, 5` results are unchanged) but would miss
   `k = 2` at `n = 4`. For `n = 4` the test covers `k = 1, 2`; disjoint means
   `m = 4`, `r = 16`, `g = 49` (**derived**).
2. **Orbit length of `B'`.** `d = min{k >= 1 : sigma^k B' = B'}` (`d | n`). `d = 1`:
   `m = 1`, recorded and excluded as before. **`1 < d < n` (here `d = 2`, the
   `sigma^2`-orbit case): recorded as *degenerate* (`deg`) and excluded from
   `min_genus`**, with its own genus histogram `deg_by_genus` and `min_genus_incl_deg`.
   `d = n`: *valid*.
3. **Transversal of `PGL2(F_q)\PGL2(K)` with non-free action.** The old tool relied
   on `PGL2(F_q)` acting freely on `K \ F_q`; for `n = 4` a point `a` of
   `F_{q^2} \ F_q` has a stabiliser `S_a` of order `q+1`. The tool now takes
   orbit-minimal `a`, computes `S_a`, and keeps a pair `(b, c)` only if it is
   lexicographically minimal under `S_a` (the action of `S_a` on pairs of
   distinct points other than `a` is free: a non-identity `g in S_a` fixes only
   `a` and its conjugate, so it cannot fix both `b != c`). The total number of
   cosets is asserted equal to `(Q^3-Q)/(q^3-q)` (`Q = q^4`). Code path for odd
   `n` is value-identical (empty `S_a`).
4. New derived-statement checks (below) and `n = 4` extra output fields
   (`deg_*`, `min_genus_incl_deg`, `j_in_Fq2`); output for `n` odd is
   byte-identical to the baseline tool apart from timing fields.

## Derived statements (proved here before the runs; a violation means a bug or an error in the proof)

Notation: `e` = size of the `sigma`-orbit of a point, `d_K` = its degree over
`K`. `e / gcd(e,4) = d_K`, hence `d_K=1: e in {1,2,4}`; `d_K=2: e = 8` only;
`d_K=3: e in {3,6,12}`. For an orbit `O` and `S = B' n O`,
`mult(P) = #{i<4 : sigma^-i P in S}`.

**L1 (module, composite case).** `span{sigma^i v}` is the cyclic `F_2[x]`-module
of `v`, annihilated by `x^4 - 1 = (x+1)^4` over `F_2`. So `m in {1,2,3,4}` with
`m = 1` iff `sigma B' = B'`; `m = 2` iff `sigma^2 B' = B' != sigma B'`;
`m = 3` iff `(x+1)^3 v = sum_i sigma^i v = 0` and `sigma^2 B' != B'`; `m = 4`
otherwise. `sum_i sigma^i v = 0` iff every point of `U` has even `mult`.
On an orbit of size `e | 4` the sum is `(4/e)|S| 1_O`, which is nonzero mod 2
only for `e = 4`, `|S|` odd.

**L2 (admissibility).** The four elliptic quotients of the cover are the
characters `sigma^i v` (support 4). By Kani-Rosen for `G = (Z/2)^m` acting on a
curve with `C/G = P^1`, `Jac(C) ~ prod over nonzero characters chi of Jac(C_chi)`
(this is also the genus identity `g = sum (|supp chi| - 2)/2` the tool
cross-checks). The conorm map `Res_{K/F_q} E -> Jac(C)` therefore has finite
kernel iff the four `sigma^i B'` are pairwise distinct iff `d = 4` iff `m >= 3`.
Structures with `d < 4` are not GHS descents to `F_q` (the `d = 2` case only
descends `E` to a Weil restriction `F_{q^4} -> F_{q^2}`, an `n = 2` problem
over `F_{q^2}`, whose model exponent `2 - 2/g'` in `q^2` against rho's `q^2`
never beats rho for `g' >= 2`). Admissible = valid (`m in {3,4}`), **derived**.

**L3 (global bounds).** For valid structures `g = 2r - 7` (`m = 3`) or `4r - 15`
(`m = 4`), so `g` is odd. (a) `m = 4` needs `r >= 5` (`r = 4` forces `sigma B' = B'`),
giving `g >= 5`. (b) `m = 3`, `r = 5` is impossible: mults even, `2 t2 + 4 t4 = 16`,
`t2 + t4 = 5` gives `t4 = 3`; the three points of mult 4 form a `sigma`-stable
`Z subset B'`, `B' = Z + {a}` with `mult(a) = 2`, i.e. `e_a = 2`, so
`sigma^2 B' = B'`: degenerate. Hence `m = 3` needs `r >= 6`, `g >= 5`.
**Valid genus is odd and >= 5**; no genus 2, 3, 4 from any valid structure.
(c) `g = 5` forces all four points of `B'` to be `K`-rational (full 2-torsion):
`m = 4, r = 5` is `B' = (O \ {p}) + {w}` with `O` a 4-orbit and `w` fixed; `m = 3, r = 6`
needs a `sigma`-stable `Z4` of size 2 inside `B'`, which cannot contain a
`K`-quadratic pair (that pair has `e = 8`).

**T-Z (`E(K)[2] = Z/2`: one `K`-point `a` of `B'`, one `K`-quadratic pair).**
The pair has `e = 8`, `S = {0, 4}` in `Z/8`, `mult = 1` at all 8 points
(odd), so **every structure is valid, `m = 4`, `r = 8 + e_a`**, `g = 17 + 4 e_a`:
`e_a = 1, 2, 4` give **`g = 21, 25, 33`**, where `a' = N(inf)` and `N = M^-1`.
No `m = 1` or `d = 2` structure exists. Counting cosets by `e_a`
(`C1 = Q(Q-1)/(q(q-1))` with `a' in P^1(F_q)`, `C2 = Q(Q-1)/(q+1)` with
`a' in F_{q^2} \ F_q`): **the histogram of structures is exactly
`{21: C1, 25: C2, 33: rest}`**, same for every such curve. So the class
minimum is **21** for *every* `E(K)[2] = Z/2` curve, and **the mod-4 refinement
does not matter at `n = 4`** (`#E == 2 mod 4` and `4 | #E` with `E(K)[2]=Z/2`
have the same floor). At `q = 3`: `C1 = 1080`, `C2 = 1620`, 22 140 structures.

**T-O (`#E(K)` odd: cubic irreducible over `K`).** `B'` = three conjugate degree-3
points `theta` (`e_theta in {3,6,12}`) plus `a'` (`e_a in {1,2,4}`).
`e_theta = 12`: `S={0,4,8}`, mult 1 everywhere, `m = 4`, `r = 12 + e_a`:
**g = 37, 41, 49**. `e_theta = 6`: `S = {0,2,4}`, mult 2, `sigma^2 S = S`; with
`e_a in {1,2}` `sigma^2 B' = B'` (degenerate, `m = 2`, `r = 7, 8`, `g = 4, 5`);
`e_a = 4`: `m = 4`, `r = 10`, **g = 25**. `e_theta = 3`: `S = O`, mult 4; `e_a = 1`:
`m = 1`; `e_a = 2`: degenerate (`r = 5`, `g = 2`); `e_a = 4`: `m = 4`, `r = 7`,
**g = 13**. `PGL2(K)` acts simply transitively on degree-3 points (`Q^3 - Q` each),
so `N <-> theta' = N theta_1` and, with `PGL2(F_q)` free on those points:
exactly **1** coset has `e_theta = 3`, **`q^3 + q - 1`** have `e_theta = 6`, all
others `e_theta = 12`. Hence per odd curve (**exact identities**):
`m1 + deg + by_genus[13] + by_genus[25] = q^3 + q`;
`by_genus[37] + m1 + deg_by_genus[4] = C1`;
`by_genus[41] + deg_by_genus[2] + deg_by_genus[5] = C2`;
`by_genus[49] + by_genus[13] + by_genus[25] = total - C1 - C2`
(`q = 3`: 30 special cosets). Floor in `{13, 25, 37, 41, 49}`;
**if `e_a in {1,2}` for the `e_theta = 3` coset then `B'` is `sigma^2`-stable and
`j in F_{q^2}`, so odd and `j not in F_{q^2}` implies the floor is exactly 13.**

**T-S (`E(K)[2] = (Z/2)^2`: all four points `K`-rational).** Valid genus is odd
and `>= 5` (L3). `g = 5` occurs iff some coset has `B' = (O\{p}) + {w}` or the
`(m=3, r=6)` form; the generic structure is 4 points in 4 distinct 4-orbits,
`g = 49`. No upper-bound theorem: whether 5 is attained depends on the cross-ratio
of the particular curve.

## Preregistered predictions (set before any run)

| class | predicted class minimum of min-genus over structures | basis |
| --- | --- | --- |
| `#E` odd | **13** for every curve with `j not in F_{q^2}` (derived); otherwise 13 or 25; I predict the curve minima are in `{13, 25}` only, with the fraction at 25 about `(q^2+1)/q^4` (model: `a'` of the `e_theta=3` coset behaves as a random point of `P^1(K)`; ~12% at `q=3`, ~4% at `q=5`) | T-O |
| `#E == 2 mod 4` | **21** on every curve, histogram `{21: C1, 25: C2, 33: rest}` exactly | T-Z |
| `E(K)[2] = Z/2`, `4 | #E` | **21** on every curve, same histogram | T-Z |
| `E(K)[2] = (Z/2)^2` | class minimum **5** (attained by at least one curve at `q = 3` and at `q = 5`); not every curve attains 5; all curve minima odd and >= 5 | L3, T-S |

Model exponent `2 - 2/g` against rho's `2` (both in powers of `q`; constants
~g! and descent costs not included): `g=5: 1.600`, `13: 1.846`, `21: 1.905`,
`25: 1.920`, `33: 1.939`, `49: 1.959`.
Further preregistered statements: no valid structure has genus 2, 3, 4 or any
even genus; no `m = 2` valid structure; `m1`, `deg` are 0 for `Z/2` curves;
`(Z/2)^2` curves can have `deg > 0`.

## Controls and checks (frozen)

- **R1 (reproduction control).** `cover_census_n4.rs` run as `n = 3`,
  `q = 5, 7`, exhaustive, `cosets`: every `curve`, `group`, `mr` and
  `derived_checks` record equal, byte for byte, to the committed
  `research/ghs_cover_census_n5_20261013/results/run-20261014T0050Z/R1-n3-q{5,7}.jsonl`
  (the `field` and `totals` records differ only in the timing fields `setup_s`,
  `wall_s`, which are removed before comparison).
  Extras if cheap: `n = 3, q = 3` against the committed pilot; `n = 5, q = 3`
  against committed panel D (verifies the transversal change is a no-op for odd `n`).
- **R4 (new-enumeration control).** `n = 4`, `q = 3`, exhaustive: `cosets` and
  `cocycles` (all `A in PGL2(K)` with `A A^s A^{s^2} A^{s^3}` scalar) give
  identical `curve` records except `mismatch` scope.
- **Internal checks per curve**: the genus-formula against the sum over
  quadratic subcovers (`mismatch = 0`); the derived statements above
  (`violations`). A violation is a bug or a proof error and is reported as such.

## Panels (frozen; one process at a time, 4 threads, shared 4-core host)

Curves: every `K`-isomorphism class (short Weierstrass for `p > 3`, general
form for `p = 3`, same representatives as the baseline tool). Total compute
budget about 2 h.

| panel | `q` | coverage |
| --- | --- | --- |
| P1 | 3 | exhaustive (all classes), 22 140 structures per curve |
| P2 | 5 | exhaustive (all classes), 2 031 000-ish structures per curve (`(Q^3-Q)/(q^3-q)`); **fallback**: if one timed curve implies more than 75 min wall for all classes at 4 threads, run `sample:120` (baseline seed stream, tag `100q+n`, plus 2-isogeny neighbours) instead and say so |
| P3 | 7 | `sample:12` + neighbours (seed stream as above), only if the timed per-curve cost implies at most 30 min; every structure of each sampled model; no class minima |

Stop rule: a panel is killed at 2x its estimate and reported incomplete;
nothing is rerun with a different sample after seeing results. Fractions in
sampled panels are sample estimates with their count `n`, no interval claimed.

## Reporting

README.md with the genus-floor table per class (measured, with n curves),
predictions vs observed, exponent table (model), genus histogram per class
(SVG + PNG), PDF; every failure recorded with its reason.

---

## Amendment A1 (appended 2026-10-24 after the first `n = 4`, `q = 3` run; the text above is unchanged)

What happened. The first `n = 4`, `q = 3` cosets run (kept as
`results/run-20261024/R4-firstrun-n4-q3-cosets-WRONG-CHECKS.jsonl`) reported
violations of two derived statements. Both were errors in this protocol's
proofs or check code, not in the enumeration:

1. **T-Z was wrong (preregistered prediction "21 for `E(K)[2] = Z/2`" failed).**
   `E(K)[2] = Z/2` means `B` has **two** `K`-rational points (`inf` and one
   root) plus a `K`-quadratic pair, not one. Corrected, **derived**: `B' = {a, b}`
   + pair; the pair has an 8-point orbit with mult 1 (so `m = 4`, all
   structures valid); `g = 17 + 4 r_rat`, `r_rat = |U_i sigma^i {a,b}|`, so
   `g in {25, 29, 33, 37, 41, 49}` and **the floor is 25, not 21**, attained
   when `{a,b}` is `sigma`-stable (both in `P^1(F_q)`, or an `F_{q^2}`-conjugate
   pair). With `P1 = q+1`, `P2 = q^2-q`, `P4 = Q-q^2` and `Q-1` matrices `N` per
   ordered pair `(N inf, N e) = (a, b)`, the number of cosets of genus `g` is
   `N_g (Q-1)/(q^3-q)` with `N_25 = P1(P1-1)+P2 = 2q^2`, `N_29 = 2 P1 P2`,
   `N_33 = P2(P2-2)+3 P4`, `N_37 = 2 P1 P4`, `N_41 = 2 P2 P4`,
   `N_49 = P4(P4-4)` (sum `= Q(Q+1)`). At `q = 3`: 60 / 160 / 800 / 1920 / 2880 /
   16 320. Still independent of `#E mod 4`.
2. **T-O refined.** The `q^3+q` cosets with `e_theta in {3,6}` are all
   `sigma^2`-stable (`m = 1` or degenerate) when `j in F_{q^2}`: the
   `K`-rational Moebius `A` with `A B' = sigma^2 B'`, `A a' = sigma^2 a'`
   permutes the cubic set `S` cyclically, so `A` has order 1 or 3, and
   `sigma^4 a' = a'` gives `A^2 a' = a'`, hence `A a' = a'`, `sigma^2 a' = a'`.
   Conversely `j not in F_{q^2}` makes all of them valid with `e_a = 4`.
   **Derived**: odd and `j not in F_{q^2}` gives histogram
   `{13: 1, 25: q^3+q-1, 37: C1, 41: C2, 49: rest}`, floor 13; odd and
   `j in F_{q^2}` gives floor 37 (`C1 - s1 > 0` since `s1 <= q^3+q < C1`), with
   no 13 or 25. The odd-class prediction "curve minima in {13, 25}" is therefore
   replaced by **"13 iff `j not in F_{q^2}`, else 37; never 25"**.
3. **Check bug.** `L1_valid_m_3_4` also counted the recorded `m = 1`
   structures (they are in `structures_by_m_r` but excluded from the valid
   set). Fixed to ignore `m = 1`.

The tool change is confined to the derived-check function (`violations`);
enumeration and output fields are untouched. The `q = 3` data were seen before
this amendment, so the corrected statements are **post-hoc for `q = 3`**;
`q = 5` and `q = 7` are out of sample for them. Preregistered predictions that
stand unchanged: `(Z/2)^2` class minimum 5; valid genera odd and >= 5; no
valid `m = 2`; genus 5 only for full 2-torsion. Also recorded as observed, not
derived (check `OBS_m1_iff_jFq`): `m = 1` occurs iff `j in F_q`.
