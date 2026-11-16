#set page(paper: "a4", margin: 1.6cm, flipped: false)
#set text(size: 9.5pt)
#set heading(numbering: "1.")

= Lead R5.1: the quotient-coordinate oracle in characteristic 2

Round 5, 2026-10-12. Example `examples/r5_1_quotient_oracle.rs`. Raw records in this directory: `validate.json`, `grid_small.json`, `grid_n23.json`, `grid_n31.json`, `grid_n37_m23.json`, `grid_n37_m4.json`, `a1_m31.json`, and the Rust-computed pooled statistics `summary.json`. Full tables, commands and limits are in `RESULTS.md`. Toy Koblitz curves only. No sub-rho claim and no claim about deployed curves. Wall times are contended.

== Question

On $y^2 + x y = x^3 + a x^2 + 1$ over $FF_(2^n)$, the 2-torsion kernel $M = ⟨T⟩$ has $T = (0,1)$. Prop. 15.2 charges a $+T$-saturated base $2^(m-2)$ for the mitm oracle. Round 4 (§17.4, ℓ = 3, odd characteristic) found this charge only for oracles that enumerate $F$. Hypothesis: a quotient oracle (arm c) costs about 1 at $m = 3$, and (c) ≈ (d) up to the index factor 2. Falsifier: (c) still pays about $2^(m-2)$.

== The 2-isogeny (derived; validated)

Vélu for the 2-torsion point $(0, s)$, $s = sqrt(b)$, gives
$phi(x,y) = (x + s\/x, med y + s(x + y + s)\/x^2 + s)$ onto $y^2 + x y = x^3 + a x^2 + s$.
For $b = 1$ the codomain is $E$ itself, $phi = plus.minus(pi minus.plus 1)$ is the Verschiebung, and the quotient coordinate is $x' = x + 1\/x$.

Measured on 15 curves (5 Koblitz curves and 10 random $b in.not FF_2$; $n <= 17$):
- 0 points off the codomain;
- 4000/4000 homomorphism checks;
- kernel exactly ${O, T}$;
- $\#E' = \#E$;
- image index exactly 2, equal to ${Q : [N\/2]Q = O}$.

The dual is the Frobenius, which is inseparable, so $hat(phi)(E') = E$. The index-2 image control is therefore $2E = phi(E)$.

== Results

#figure(
  image("ratios.svg", width: 100%),
  caption: [Recorded total relation-collection cost relative to the plain base (a), log₂ scale. Short bars mark the Prop. 15.2 / Thm 7 predictions: (b) $2^(m-2)$, (c) 1, (d) 1/2. Panels 1–2 show `icv1-f2m31-tm90707-c95f16f5` (6 base seeds) and `icv1-f2m37-tm534059-32aad96b` (5 seeds; 3 at $m=4$), equal $|F|$ per cell. Panel 1 is the exact census over all of $G$. Panel 2 is the library `FrobeniusPairTable` / `decompose_mitm_frobenius` on 6000 shared targets (1500 at $m=4$), with 95% bootstrap over seeds. Panel 3 is the §7.A1-R cell (divisor 0;1;2 bases), with paired bootstrap over 200 000 / 4000 targets. Sources: `summary.json`, `a1_m31.json`.],
)

#table(
  columns: (auto, auto, auto, auto, auto),
  align: (left, center, left, left, left),
  table.header([*cell*], [*law (b), (c), (d)*], [*(b) census / oracle*], [*(c1) census / oracle*], [*(d) census / oracle*]),
  [n=31, m=2], [1, 1, 1/2], [1.00 / 1.02], [1.00 / 1.02], [0.50 / 0.54],
  [n=31, m=3], [2, 1, 1/2], [1.90 / 2.04], [*0.95 / 1.02*], [0.53 / 0.51],
  [n=31, m=4], [4, 1, 1/2], [4.02 / 5.29], [*1.01 / 1.33*], [0.54 / 0.54],
  [n=37, m=2], [1, 1, 1/2], [0.99 / 1.01], [0.99 / 1.01], [0.50 / 0.54],
  [n=37, m=3], [2, 1, 1/2], [1.99 / 1.90], [*0.99 / 0.95*], [0.52 / 0.53],
  [n=37, m=4], [4, 1, 1/2], [3.69 / 2.67], [*0.92 / 0.67*], [0.57 / 0.57],
)

The (c1) oracle values exclude the φ(R) charge of one addition per target. With the charge, the $m = 2$ oracle value becomes 2.0, because the $m = 2$ probe is a single lookup; at $m >= 3$ the charge changes the value by less than 0.2%. The 95% intervals are in `RESULTS.md`. At $m = 3$ every (c) interval contains 1 and excludes 2.

The pushforward identity $D_m(phi(F_b)) = D_m(F_b)$ held exactly in 343/343 cells. In every cell, small $n$ included, census (c1)/(b) is exactly 1/2 at $m = 3$ and 1/4 at $m = 4$. At $n <= 23$ the bases hold only 1–4 Frobenius columns and their sums are structured τ-adic combinations, not random. There the baseline (b)/(a) ranges over 0.5–7 and the class-factor law is not reached. Those cells are reported and not used for the verdict.

== The §7.A1-R point (`icv1-f2m31-tm90707-c95f16f5`, $r = 1 439 393$)

- At $m = 3$ the u-arm sweeping $F_u$ costs 1.96× the x-arm in total ([1.78, 2.14]) and 3.40× per relation. This reproduces the §7.A1-R loss, in a different unit.
- The same base with the quotient oracle costs *1.00× [0.91, 1.09]*. The half-sweep on $E$ costs 1.01×. The trace-zero arm costs 0.98×.
- $phi(F_u)$ is exactly the trace-zero base translated by $T$: 589 of 589 abscissae are inverses of trace-zero abscissae. Hence $D_3$ = 318 618 for both.

== Verdict

*Confirmed.* The falsifier is not met. In characteristic 2, an oracle on $F\/⟨T⟩$ removes the full $2^(m-2)$ kernel-saturation penalty. (c) ≈ (a) and (c) ≈ 2·(d) at $n = 31, 37$. The §7.A1-R u-frame loss disappears under the quotient oracle. Neither inseparability nor $x(T) = 0$ interferes: φ is the separable Verschiebung.

Limits:
- toy sizes;
- cost unit = counted additions + table lookups, with canonicalisations not priced and the table build not in the total;
- no linear algebra or full-rank streams;
- $m = 4$ oracle intervals wide;
- no algebraic quotient oracle.
