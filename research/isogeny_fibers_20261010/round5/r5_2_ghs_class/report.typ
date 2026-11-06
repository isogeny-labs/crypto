#set page(paper: "a4", margin: 2cm)
#set text(size: 10pt)
#set par(justify: true)
#show heading: set block(above: 1.1em, below: 0.6em)

= R5.2: how much of a binary isogeny class is within GHS reach?
_Round 5, lead R5.2. 2026-10-18. Example `examples/r5_2_ghs_class.rs`. Records:
`census.json`, `walk.json`, `walk_20.json`, `selftest.json` (this directory). Toy fields
$F_(2^N)$, $N in {12,14,15,16,18,20}$, exhaustive over $b in K^*$ and both twists. No
deployed-curve claim. Prime $N$ is out of scope: GHS needs $N = l n$ with $n >= 2$._

== Verdict
- *Cost model P* (min of Gaudry $g! dot q^(2-2/g)$ and Enge–Gaudry $L_(q^g)[1/2, sqrt(2)]$,
  against rho $sqrt(pi r\/4)$): *0 weak curves* out of 2 858 996 at every N (derived
  verdicts on measured $g$, $r$). The best class is 1.65–2.42 bits above rho. The
  hypothesis, read as a cost statement, is *inconclusive*. At these sizes it is refuted
  only trivially, because no curve is weak at all.
- *Structural low-genus reach* (LG: applicable descent, transfer possible, $g <= 2n$,
  $n >= 5$): a small nonzero share, 0.09 %–3.1 % of curves where it exists. Every class
  containing such a curve is mixed (median in-class share 3.6 %–25 %). With
  $ell in {3,5,7}$, 66 %–100 % of the strong curves in those classes reach one. The mean
  distance is 1.3–3.0 steps (max 15), and a search expands on average 1.5–8.4 curves (max 53).
  *Confirmed structurally*, with 0 %–34 % of strong curves unreachable via $ell <= 7$.
- The constant-free exponent model M1 and the fixed-$n$ exponent model A are degenerate.
  Over $F_2$ and $F_4$, $q^(2-2/g) <= 16$ whatever $g$ is; and every curve with $n >= 4$
  passes A.
- _Derived from recalled exponents:_ at fixed $n$, transfer needs $g >= n$, so the GHS
  exponent $2 - 2/g$ is never below Gaudry's decomposition exponent $2 - 2/n$, which applies
  to every curve. Exponent separation inside a class needs $n$ to grow.
- _Library:_ `ec_trapdoor::audit_curve` disagrees with the derived $(m, g)$ on 1 248 of
  1 500 sampled N = 12 rows; the √b-orbit rank itself agrees on 1 500/1 500.

== Formula implemented
$sigma(x) = x^(2^l)$, $c_i = sigma^i (sqrt(b))$, $U = "Span"_(F_2) {(c_i, 1)}$, $m = dim U$,
$m' = deg f$ with $f = "Ord"_sigma (sqrt(b))$. Then $m = m' + [f(1) != 0]$ (checked on every
curve: 0 mismatches). The genus is $g = 2^(m-1) - 1$ if $(x+1) divides.not f$, else
$2^(m-1)$. This follows from summing the genera $[c_u != 0 and d_u != 0]$ of the quadratic
subfields. Hand-checked cases: $b = 1$ gives $(1,1)$; $sqrt(b) = c + sigma(c)$ at $(l,n) = (4,3)$
gives $(3,3)$; generic $b$ gives $(3,4)$ at $n = 3$ and $(4,8)$ at $n = 4$. Point counts come from
one Walsh–Hadamard transform. Validation: 1 979/1 979 against a naive count and 80/80
against the library. 2-isogenies are Galois conjugations and preserve $m$ (1 000/1 000).

#figure(image("fig1_low_genus_share.svg", width: 100%),
  caption: [Share of curves (blue) and of isogeny classes (orange) with a low-genus
  descent, $g <= 2n$, per factorisation with $n >= 3$. Measured counts from `census.json`.
  Rows with $n = 3, 4$ cover almost all curves; rows with $n >= 5$ are small or zero.])

#figure(image("fig2_model_p_margin.svg", width: 100%),
  caption: [Model P: $log_2$ of the IC cost over the rho cost, for the best curve in each
  class (best factorisation per curve). Range, median and minimum over the classes for each N.
  Derived from measured $g$ and $r$. No class reaches 0.])

#figure(image("fig3_walk_distance.svg", width: 100%),
  caption: [Distance in $ell$-isogeny steps ($ell in {3,5,7}$, Elkies kernels from $psi_ell$,
  binary Vélu codomain) from each strong curve to the nearest LG curve with $n >= 5$, in mixed
  classes. Curve-weighted and exhaustive, from `walk.json`. Validation: 0 codomain-trace
  mismatches and 0 asymmetric edges.])

== Limits
No descent was constructed and no Jacobian order was computed: transfer is the Hasse–Weil
size test. The Menezes–Qu applicability condition, the genus-sum formula and the IC
constants are recalled. P and LG were defined after the first census showed that M1 and A
are degenerate. Only $ell <= 7$. Toy $N <= 20$. Full tables, commands and the library
comparison are in `RESULTS.md`.

== References (recalled where marked in RESULTS.md)
Gaudry, Hess, Smart, J. Cryptology 15 (2002). Galbraith, Hess, Smart, EUROCRYPT 2002.
Menezes, Qu, CT-RSA 2001. Menezes, Teske, Weng, CT-RSA 2004. Gaudry, J. Symb. Comput. 44
(2009). Enge, Gaudry, Acta Arith. 102 (2002).
