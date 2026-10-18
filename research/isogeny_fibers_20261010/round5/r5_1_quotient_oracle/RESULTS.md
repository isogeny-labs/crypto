# Lead R5.1: a quotient-coordinate oracle removes the kernel-saturation penalty in characteristic 2 (round 5, 2026-10-12)

Example: `examples/r5_1_quotient_oracle.rs`. Raw outputs are the `*.json` / `*.txt` files in this
directory. The pooled cross-seed statistics in `summary.json` were computed by the example's
`--exp summarize` mode (Rust). Graph: `ratios.svg`/`.png`, made by `plot_ratios.py` from
`summary.json` and `a1_m31.json`. Report: `report.pdf`. All instances are toy Koblitz curves,
named by ledger slug. Nothing here is a sub-rho result or a statement about a deployed curve.
Wall times are contended.

## Question, hypothesis, falsifier

Round 4 (§17.4, odd characteristic, ℓ = 3) found that a kernel-saturated base pays the
|M|^{m−2} cost of Prop. 15.2 only when the oracle enumerates F. An oracle in the quotient
coordinate (enumerating F/M) paid about 1. §17.7 Thm 7 explains this as the image law on E/M,
transported back. This lead asks whether the same holds in characteristic 2, on Koblitz curves
`y² + xy = x³ + ax² + 1` over F_{2^n} with the 2-torsion kernel M = ⟨T⟩, T = (0, 1).

* **Hypothesis:** (c) quotient oracle ≈ 1× the plain base (a) at m = 3 (not 2×), and (c) ≈ (d)
  up to the index factor 2.
* **Falsifier:** (c) still pays ≈ 2^{m−2}. That would mean the transport of Thm 7 gives no free
  oracle in char 2, for example through inseparability or because x(T) = 0.

## The 2-isogeny (derived here, validated by `--exp validate`)

Vélu for the 2-torsion point Q = (0, s), s = √b, with a₁ = 1, a₃ = a₄ = 0. In characteristic 2:
g^x_Q = y_Q = s and g^y_Q = x_Q = 0, so t_Q = s, u_Q = 0, w = 0. Then a₄' = s and a₆' = b + s.
The substitution y ↦ y + s clears the x-term:

```
φ(x, y) = ( x + s/x ,  y + s(x + y + s)/x² + s ),     E/⟨T⟩ : y² + xy = x³ + ax² + s.
```

The library's `binary_velu` covers odd ℓ only, so this map is written in the example (`phi`).
On a Koblitz curve (b = 1) the codomain is E itself, and the quotient coordinate is
**x' = x + 1/x**.

Validation, all measured (`validate.json`). The runs cover the Koblitz curves with an instance
at n ∈ {11, 13, 15, 17}: (11,1), (13,0), (15,0), (15,1), (17,1). Each also has two random
b ∉ F₂, for 15 curves in total. On every curve:

* every image point lies on the codomain;
* 4000/4000 random pairs satisfy φ(P+Q) = φ(P) + φ(Q);
* the kernel is exactly {O, T};
* #E' = #E (independent point count);
* the image has index exactly 2.

The image equals {Q : [N/2]Q = O}, with 0 disagreements. The halving trace criterion
Tr(x) = Tr(a) also agrees, except at the single point x = 0 on a = 0. There T ∈ 2E, and the
test in the example excluded x = 0.

On b = 1 the map φ equals π − 1 on every point for a = 1, and −(π + 1) for a = 0, so φ is the
Verschiebung. Its dual is the Frobenius, which is purely inseparable, so φ̂(E') = E has index 1.
The index-2 image base is therefore 2E = φ(E), and arm (d) uses it.

## Arms (equal |F|; every base is a union of signed Frobenius orbits)

| arm | base | oracle (library `FrobeniusPairTable` + `decompose_mitm_frobenius`) |
|:--|:--|:--|
| a | 2K random orbits | sweep all of F (m = 3), all pairs i ≤ j (m = 4) |
| b | K random orbits saturated by the library's `saturate_factor_base_two_torsion` (x ↦ 1/x) | same, sweeping all of F |
| c1 | F' = φ(F_b) on E/⟨T⟩ = E (|F|/2 points, table keyed on x'), target φ(R), relations lifted back through a preimage map | same oracle on F'; φ(R) is charged as one addition |
| c2 | the base of b | the pair table of b, sweeping one representative per {P, P+T} |
| d | 2K random orbits inside 2E (checked by [N/2]P = O) | sweep all of F |

Columns are the projected signed Frobenius orbits (`ColumnFold::ProjectedSignedFrobeniusOrbit`,
the `projected=1` fold of §7.A1-R). The cost model is §15.2: cost = U · L_m · r / D_m, with
L₂ = 1, L₃ = |swept set| and L₄ = |swept|(|swept|+1)/2. The prediction at equal |F|, relative to
(a), is (b) 2^{m−2}, (c) (1/2)·2^{m−1}·2^{−(m−2)} = 1, and (d) 1/2.

* **Census (exact):** every multiset of m base points whose sum is in G \ {O}. The sums are
  bucketed by cofactor class, so this covers every m-decomposition of every target. It gives D_m
  exactly, along with the distinct projected rows and the representations per row.
* **Oracle (measured):** a shared list of random targets R = [k]G. The counted work W is group
  additions plus table lookups per target, and H is the hit rate. Total = U·W/H. Every returned
  relation was re-summed: 0 verification failures in all runs. Lifted c1 relations sum to R or
  R + T. Canonicalisations equal lookups and are recorded separately.

## Commands (single-threaded; contended wall times in the JSON)

```
B=target/release/examples/r5_1_quotient_oracle; D=research/isogeny_fibers_20261010/round5/r5_1_quotient_oracle
$B --exp validate --curves 11:0,11:1,13:0,13:1,15:0,15:1,17:0,17:1 --json $D/validate.json
$B --exp grid --curves 13:0,17:1,19:0,19:1 --seeds 20 --fill 0.1 --targets 4000 --targets4 600 --boot 1000 --seed 20261012 --json $D/grid_small.json
$B --exp grid --curves 23:0,23:1 --seeds 12 --fill 0.1 --targets 4000 --targets4 600 --boot 1000 --seed 20261013 --json $D/grid_n23.json
$B --exp grid --curves 31:0 --ms 2,3,4 --seeds 6 --fill 0.05 --targets 6000 --targets4 1500 --boot 1000 --seed 20261014 --json $D/grid_n31.json
$B --exp grid --curves 37:0 --ms 2,3 --seeds 5 --fill 0.02 --targets 6000 --boot 1000 --seed 20261015 --json $D/grid_n37_m23.json
$B --exp grid --curves 37:0 --ms 4 --seeds 3 --fill 0.02 --targets4 1500 --boot 1000 --seed 20261016 --json $D/grid_n37_m4.json
$B --exp a1 --ms 2,3 --targets 4000 --boot 1000 --seed 123212651130 --json $D/a1_m31.json
$B --exp summarize --inputs $D/grid_small.json,$D/grid_n23.json,$D/grid_n31.json,$D/grid_n37_m23.json,$D/grid_n37_m4.json --boot 2000 --seed 7 --json $D/summary.json
```

`--fill` sets |F| so that arm (a) has an expected hit rate of about `fill`. |F| is rounded to a
multiple of 4n. Koblitz instances exist (`KoblitzCurve::new`) only for (n, a) ∈ {(11,1), (13,0),
(15,0), (15,1), (17,1), (19,·), (23,·), (29,1), (31,0), (37,0)} in this range. n = 15 (r = 751 / 211)
and n = 29 (r = 42 457, h = 12 646) were left out of the grid as degenerate.

## Measured: the main grid (n = 31 and 37, a = 0; many orbits per base)

The census columns give the pooled ratio to (a), with a 95% bootstrap over base seeds. The
oracle column excludes the φ(R) charge; it is quoted with that charge below.

| n, m | seeds | \|F\| | U_a | (b) census | (c1) census | (d) census | (b) oracle | (c1) oracle | (c2) oracle | (d) oracle |
|:--|--:|--:|--:|:--|:--|:--|:--|:--|:--|:--|
| 31, 2 | 6 | 14 632 | 235 | 1.00 [0.98, 1.02] | 1.00 [0.98, 1.02] | 0.50 [0.49, 0.51] | 1.02 [0.92, 1.13] | 1.02 | 1.02 [0.92, 1.13] | 0.54 [0.51, 0.58] |
| 31, 3 | 6 | 868 | 14 | 1.90 [1.69, 2.11] | **0.95 [0.85, 1.05]** | 0.53 [0.50, 0.56] | 2.04 [1.73, 2.54] | **1.02** | 1.02 [0.87, 1.24] | 0.51 [0.47, 0.55] |
| 31, 4 | 6 | 248 | 4 | 4.02 [2.77, 6.09] | **1.01 [0.69, 1.53]** | 0.54 [0.39, 0.78] | 5.29 [4.14, 6.64] | **1.33** | 1.33 [1.04, 1.68] | 0.54 [0.39, 0.76] |
| 37, 2 | 5 | 74 148 | 1002 | 0.99 [0.99, 1.00] | 0.99 [0.99, 1.00] | 0.50 [0.50, 0.51] | 1.01 [0.84, 1.21] | 1.01 | 1.01 [0.84, 1.24] | 0.54 [0.49, 0.59] |
| 37, 3 | 5 | 2 516 | 34 | 1.99 [1.86, 2.10] | **0.99 [0.93, 1.05]** | 0.52 [0.50, 0.53] | 1.90 [1.58, 2.42] | **0.95** | 0.95 [0.79, 1.18] | 0.53 [0.51, 0.55] |
| 37, 4 | 3 | 444 | 6 | 3.69 [2.22, 5.93] | **0.92 [0.56, 1.49]** | 0.57 [0.50, 0.69] | 2.67 [1.25, 8.05] | **0.67** | 0.67 [0.32, 2.02] | 0.57 [0.46, 0.78] |
| prediction | | | | 2^{m−2} = 1, 2, 4 | 1 | 1/2 | 1, 2, 4 | 1 | 1 | 1/2 |

Oracle c1 intervals: [0.92, 1.13], [0.86, 1.27], [1.02, 1.68], [0.84, 1.24], [0.79, 1.23] and
[0.32, 2.02] (`summary.json`). The rows-based census ratios agree with the D-based ones to
within 4% in every cell. The φ(R) charge (one addition per target) is a constant per target.
It doubles the m = 2 oracle total for c1 (2.01, 2.02), because the m = 2 probe is a single
lookup and target generation itself is not counted. At m ≥ 3 the charge shifts c1 by less than
0.2%.

Structural facts (measured in every one of the 343 grid cells):

* **Pushforward identity of Thm 7:** D_m(c1 on E/⟨T⟩) = D_m(b on E), and the distinct rows are
  equal as well, exactly, in 343 of 343 cells.
* **Multiplicity:** representations per distinct row on the saturated base were 2.00–2.01 (m = 2),
  4.03–4.17 (m = 3) and 9.4–10.5 (m = 4) at n ≥ 31. The law gives 2^{m−1} = 2, 4, 8. The m = 4
  excess comes from coincident sums on 2–3-column bases. On the quotient base it is 1.0–1.1.
* **The quotient removes exactly 2^{m−2}:** census (c1)/(b) = 0.50 at m = 3 and 0.25 at m = 4,
  in every cell at every n, small n included. c1 and c2 give identical hits; c2 needs the full
  pair table (twice the entries of c1's), and c1 needs φ(R).

## Measured: small n (few orbits per base) — the law is not reached, the quotient factor is

| n | a | m | seeds | \|F\| | U_a | (b) census | (c1) census | (d) census | (b) / (c1) oracle |
|--:|--:|--:|--:|--:|--:|--:|--:|--:|:--|
| 13 | 0 | 2 | 20 | 52 | 2 | 0.92 | 0.92 | 0.61 | 0.86 / 0.86 |
| 13 | 0 | 3 | 20 | 52 | 2 | 0.85 | 0.43 | 0.78 | 1.15 / 0.60 |
| 13 | 0 | 4 | 20 | 52 | 2 | 0.51 | 0.13 | 1.00 | 3.48 / 1.59 |
| 17 | 1 | 2 | 20 | 136 | 4 | 1.27 | 1.27 | 0.64 | 1.20 / 1.20 |
| 17 | 1 | 3 | 20 | 68 | 2 | 2.62 | 1.31 | 0.61 | 3.24 / 1.63 |
| 17 | 1 | 4 | 20 | 68 | 2 | 2.08 | 0.53 | 0.98 | 16.76 / 4.43 |
| 19 | 0 | 2 | 20 | 304 | 8 | 0.98 | 0.98 | 0.55 | 0.92 / 0.92 |
| 19 | 0 | 3 | 20 | 76 | 2 | 3.03 | 1.52 | 0.57 | 3.14 / 1.57 |
| 19 | 0 | 4 | 20 | 76 | 2 | 2.09 | 0.53 | 0.92 | 8.07 / 2.13 |
| 19 | 1 | 2 | 20 | 304 | 8 | 1.08 | 1.08 | 0.57 | 1.05 / 1.05 |
| 19 | 1 | 3 | 20 | 76 | 2 | 3.01 | 1.51 | 0.63 | 3.17 / 1.59 |
| 19 | 1 | 4 | 20 | 76 | 2 | 4.16 | 1.05 | 0.94 | 16.27 / 4.22 |
| 23 | 0 | 2 | 12 | 1288 | 28 | 0.94 | 0.94 | 0.53 | 0.93 / 0.93 |
| 23 | 0 | 3 | 12 | 184 | 4 | 1.79 | 0.90 | 0.47 | 1.80 / 0.90 |
| 23 | 0 | 4 | 12 | 92 | 2 | 4.43 | 1.12 | 0.61 | 6.30 / 1.61 |
| 23 | 1 | 2 | 12 | 1288 | 28 | 0.99 | 0.99 | 0.53 | 0.96 / 0.96 |
| 23 | 1 | 3 | 12 | 184 | 4 | 2.15 | 1.08 | 0.54 | 2.24 / 1.12 |
| 23 | 1 | 4 | 12 | 92 | 2 | 7.02 | 1.77 | 0.68 | 9.33 / 2.39 |

At n ≤ 19 a Frobenius-closed base of the size the m = 3 and m = 4 census needs is only K = 1
saturated orbit pair (U = 1–2 columns). Its m-sums are then the τ-adic combinations
±λ^i ± λ^j ± λ^k of one or two generators, and are not random. For example, on n = 19, a = 1,
m = 3 the saturated base gives D = 5358 and 38 038 representations on every seed, a
seed-independent structural count. The random-sum heuristic behind the class factor does not
hold at this size. (b) then lands anywhere in 0.85–3.0 at m = 3 instead of 2. At m = 4, arm (a)
saturates (hit rate ≈ 0.9–1.0), which compresses its D. **The ratio (c1)/(b) is still exactly
1/2 at m = 3 and 1/4 at m = 4 in every small-n cell.** The quotient oracle always removes the
full 2^{m−2}; only the baseline it is compared with is distorted. These rows are reported, not
used for the verdict.

## Measured: the §7.A1-R comparison point (`icv1-f2m31-tm90707-c95f16f5`, divisor 0;1;2)

Bases: x = `Divisor[0,1,2]` (2049 points, 33 projected columns); u = the symmetrised base on the
same V (2357 points, 19 columns, +T-closed except T itself); tz = `Divisor[1,2]` (1179 points,
19 columns); c1 = φ(F_u) (1178 points, 19 columns). The census is exact over all r = 1 439 393
targets. The oracle used 200 000 targets at m = 2 and 4000 at m = 3. The interval is a paired
bootstrap over targets, and the φ(R) charge is included.

| m | arm | swept | D_m (exact) | cost_D / x | hit rate | W per relation | total U·W/H / x [95%] | table entries |
|--:|:--|--:|--:|--:|--:|--:|:--|--:|
| 3 | x | 2049 | 702 522 | 1 | 0.490 | 5 136 | 1 | 33 740 |
| 3 | u, sweep F (the §7.A1-R u-arm) | 2357 | 320 478 | 1.45 | 0.220 | 17 448 | **1.96 [1.78, 2.14]** | 22 307 |
| 3 | **u, c1 quotient** | 1178 | 318 618 | 0.73 | 0.220 | 8 908 | **1.00 [0.91, 1.09]** | 11 134 |
| 3 | u, c2 half-sweep | 1179 | 320 478 | 0.73 | 0.220 | 8 970 | 1.01 [0.92, 1.09] | 22 307 |
| 3 | tz | 1179 | 318 618 | 0.73 | 0.224 | 8 716 | 0.98 [0.92, 1.03] | 11 153 |
| 2 | x | 1 | 2 728 | 1 | 0.0019 | 516 | 1 | 33 740 |
| 2 | u, sweep F | 1 | 2 356 | 0.67 | 0.0015 | 650 | 0.73 [0.63, 0.84] | 22 307 |
| 2 | u, c1 quotient | 1 | 2 294 | 0.69 | 0.0015 | 1 326 | 1.48 [1.29, 1.70] (φ charge doubles W) | 11 134 |
| 2 | tz | 1 | 2 356 | 0.67 | 0.0017 | 596 | 0.66 [0.64, 0.69] | 11 153 |

* The §7.A1-R loss is reproduced. With unit W = adds + lookups, u/x is 3.40× per relation and
  1.96× in total collection cost. §7.A1-R reported 2.95× per relation in GAE, a different unit
  that includes converted canonicalisations.
* **With the quotient oracle the loss disappears:** 1.00 [0.91, 1.09] in total cost. Per
  relation c1 is still 1.73× x. That is offset exactly by the halved unknowns (19 against 33).
* **The quotient base is the trace-zero base, translated by T.** All 589 abscissae of φ(F_u) are
  inverses of trace-zero abscissae (x ↦ 1/x is +T). The one tz abscissa left over pairs with the
  T excluded from φ(F_u). So D_3(c1) = D_3(tz) = 318 618 exactly. The §7.A1-R reading 3, "the
  trace-zero base dominates the quotient base", is Thm 7 itself: the u-arm swept on its quotient
  is the trace-zero arm. D_3(u) − D_3(c1) = 1 860 comes from sums that use T ∈ F_u, which has
  no image in F'.

## Verdict

**Confirmed** at n = 31 and 37, the sizes where each base holds 4–1000 columns. The falsifier is
not met.

* (c) costs 0.95–1.02 at m = 3 (census and oracle; every 95% interval contains 1 and excludes 2)
  and 0.67–1.33 at m = 4. At m = 4 both census intervals contain 1. The n = 31 oracle interval
  [1.02, 1.68] lies just above 1 and excludes 4. The n = 37 oracle interval [0.32, 2.02] is wide,
  because about 3 hits per seed were recorded.
* (b) costs 1.90–2.04 at m = 3 and 2.7–5.3 at m = 4, against the law's 2 and 4.
* (d) costs 0.50–0.57 throughout, so (c)/(d) ≈ 2 = the index factor.
* At the §7.A1-R cell, the 2.95× u-frame loss is a loss of the F-sweeping oracle. The quotient
  oracle prices the same base at 1.00 [0.91, 1.09] of the x-arm.

Neither inseparability nor x(T) = 0 interferes. φ is the separable Verschiebung; only its dual,
the Frobenius, is inseparable. That makes φ̂(E') all of E, so the image control has to be
2E = φ(E).

At small n (U ≤ 4 columns) the class-factor law itself fails because the bases are
structurally non-random. The quotient still removes exactly 2^{m−2} relative to (b) there.

## Limits

* Toy curves only. Exact census needs r ≤ 2.3·10⁸, so the base sizes per m are fixed by `fill`.
  At m = 4 the bases have 2–3 saturated orbit pairs even at n = 37, and the oracle intervals are
  wide (n = 37, m = 4: 3 seeds and 1500 targets, about 3 hits per seed for (b) and (c)).
* The cost unit is counted additions plus table lookups. Canonicalisations (equal in number to
  lookups) are not priced. The φ(R) evaluation is priced as one addition, and target generation
  is not counted. The pair-table build is not in the total: the table entries are listed, and
  c1's table has half the entries of b's.
* Linear algebra, full-rank streams and logarithm recovery were not run. Totals use "relations
  needed ≈ U".
* The m = 4 oracle (pairs i ≤ j, then the library `probe`) and the c2 half-sweep are loops in the
  example around the library table. They are not library oracles.
* No algebraic (Semaev or symmetrised) quotient oracle was run. The quotient arm here is
  combinatorial.

## Next step and what should move into `src/`

* A `binary_velu` entry for ℓ = 2 (`phi` above, codomain b' = √b), with the identity φ = ±(π ∓ 1)
  on Koblitz models. Done in this example.
* An `ic` oracle option `mitm-frobenius-quotient` for `+T`-closed bases. It would build the table
  on φ(F), probe φ(R) and lift through a preimage map. Equivalently it could sweep F/⟨T⟩ (c2),
  which needs no map but keeps the double-size table.
* Restate Prop. 15.2's translation term per oracle in char 2 as well: 2^{m−2} for enumeration over
  F, 1 over F/⟨T⟩. Restate the §14.6 design rule accordingly: closing a base under ⟨T⟩ is neutral
  when the oracle works on the quotient, and the quotient of the u-frame base is the trace-zero
  base translated by T.
