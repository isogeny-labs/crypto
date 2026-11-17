# R6.6 — Genus 2: the affine law under a Nagao-style degree-1 oracle, with a Richelot (2,2) image arm

Toy instances only (`y² = x⁵ + b` over `F_p`, `p ≤ 4111`). No sub-rho claim, no claim about any deployed curve.

## Question
Round 4 (16.6) proved that no degree-1 divisor base is translation-closed and used the subgroup
`[2]J` as a stand-in for an isogeny image. Does the affine law hold (a) for an algebraic
decomposition oracle restricted to degree-1 base divisors, and (b) for the image of a genuine
Richelot (2,2)-isogeny?

## Hypothesis and falsifier (PROGRAM.md R6.6)
* A-closed bases (ζ₅-orbits of the x-set, both signs) gain |A| = 10 in unknowns at unchanged relation yield.
* The Richelot index-4 base gains ×4 in class factor.
* Saturation is unavailable to the oracle.
* **Falsifier:** A-gain outside [8, 12], image gain outside [3, 5] (bootstrap 95%), or a usable saturated degree-1 base.

## What was built (`examples/r6_6_genus2_oracle.rs`, uses `genus2_fp`)
* Oracle (a): exact enumeration of m = 2, 3 sums of degree-1 base divisors (census numbers).
* Oracle (b): Nagao m = 2 solve — `D = [P₁−∞]+[P₂−∞]` iff Mumford `u(D)` splits with both roots in the x-set;
  counted in F_p multiplications / inversions / square roots, checked against (a) on random targets.
  An m = 3 λ-sweep solve is run on 40 targets per base as a consistency check.
* Bases: plain (random x), A (ζ₅-closed), R (degree-1 points in `H₄ = φ̂(J′(F_p))`, cut out by the
  (2,2)-descent map), G (degree-1 points in the prime-order subgroup, index h), and on `J′` the literal
  image bases `φ(F_plain)`, `φ(F_A)` against a plain degree-1 base on `C′` (`plainP`).
* Richelot: `f = G₁G₂G₃` (one linear, two quadratic factors from the five rational roots), `H_i = [G_j, G_k]`,
  `C′` moved to a quintic model, correspondence `G₁(x)H₁(X)+G₂(x)H₂(X)=0`, φ evaluated in `F_p[x]/(u)`.
* Translation closure: `|(C−∞) ∩ (C−∞−t)|` for every rational 2-torsion t and random t.

## Commands
```
export CARGO_TARGET_DIR=…/scratchpad/r6target CARGO_BUILD_JOBS=1
cargo +1.98 build --release --example r6_6_genus2_oracle
sh run_all.sh          # 6 runs, |F| = 80, 40 bases per cell, 2000 bootstrap resamples
python plot_r6_6.py    # plot_data.json, measured_vs_predicted.svg, oracle_cost.svg
```
C2 curves (f splits, 15 rational 2-torsion points, h = 16, Richelot available): (p, b) = (1051, 1042), (2131, 32), (4111, 2172), seed 1.
C1 curves (b a square, f no rational root, h = 5, no rational (2,2)-kernel): (1051, 49), (2131, 4), (4111, 3984), seed 2.

## Richelot validation (measured, all three C2 curves)
`#J′ = #J` (988016, 4382416, 16434896); homomorphism failures 0/300; kernel: the three (2,2) generators map to 0 (3/3),
the other 12 nonzero 2-torsion points do not (12/12); degree-1 closed-form vs generic evaluation agree 134/134, 140/140, 154/154;
ψ-density of `φ̂(J′)` in J 0.2507, 0.2465, 0.2463 (index 4 predicts 0.25); 0 invalid images.

## Results (measured; ratio to the plain arm on the same curve, mean over 40 bases, bootstrap 95%)

Unknowns gain |F|/U (A predicted 10):

| curve | p | A | R (pred 2) | φ_plain / φ_A on J′ (pred 2) |
|---|---|---|---|---|
| C2 | 1051 | 10.00 [10.00, 10.00] | 2.47 [2.42, 2.53] | 2.00 / 2.00 |
| C2 | 2131 | 10.00 [10.00, 10.00] | 2.21 [2.17, 2.25] | 2.00 / 2.00 |
| C2 | 4111 | 10.00 [10.00, 10.00] | 2.13 [2.11, 2.15] | 2.00 / 2.00 |
| C1 | 1051 / 2131 / 4111 | 10.00 each | — | — |

Class factor c (A predicted 1; R, φ predicted 4; G predicted h):

| curve | p | A c₂ | A c₃ | R c₂ | R c₃ | φ_plain c₃ | φ_A c₃ | G c₃ |
|---|---|---|---|---|---|---|---|---|
| C2 | 1051 | 1.11 [0.94, 1.31] | 1.04 [0.96, 1.15] | 3.85 [3.68, 4.01] | 3.53 [3.47, 3.60] | 3.60 [3.56, 3.65] | 3.65 [3.56, 3.74] | skipped |
| C2 | 2131 | 1.02 [0.94, 1.09] | 0.99 [0.96, 1.01] | 4.20 [4.02, 4.38] | 3.89 [3.83, 3.96] | 3.90 [3.84, 3.95] | 3.86 [3.80, 3.92] | 14.02 (h=16) |
| C2 | 4111 | 0.91 [0.84, 1.00] | 0.95 [0.91, 0.98] | 3.99 [3.83, 4.19] | 3.96 [3.87, 4.04] | 4.07 [4.01, 4.13] | 4.01 [3.94, 4.07] | 15.15 (h=16) |
| C1 | 1051 | 1.17 [1.11, 1.24] | 0.98 [0.94, 1.01] | — | — | — | — | 4.37 (h=5) |
| C1 | 2131 | 1.23 [1.15, 1.32] | 0.99 [0.94, 1.02] | — | — | — | — | 4.83 (h=5) |
| C1 | 4111 | 1.18 [1.13, 1.24] | 1.02 [0.99, 1.05] | — | — | — | — | 4.95 (h=5) |

(c₂ is the distinct-summand count. "skipped": the G arm at C2 p = 1051 had too few degree-1 points in the order-r subgroup for |F| = 80; recorded in the JSON `skipped` field.)

Derived m = 3 cost ratio (nominal reference columns; A predicted 0.2, R/φ 0.25): A 0.192–0.212; R 0.229–0.237; φ 0.246–0.277.

Nagao m = 2 oracle (measured): 49–58 counted F_p multiplications per call (grows with log p via the square root);
0 disagreements with the exact enumeration on every base and curve; m = 3 sweep 0/40 disagreements per cell.

Translation closure (measured): for every rational 2-torsion t and 400 random t, `|(C−∞) ∩ (C−∞−t)| ≤ 2` on all six curves
(histograms in the JSON). A degree-1 base is never closed under a nonzero translation: saturation is unavailable.

## Verdict: **confirmed (falsifier did not fire)**
* A-gain in unknowns = 10.00 (exact, interval [10.00, 10.00]) on all six curves, inside [8, 12]; relation yield
  unchanged within ~±20% at m = 2 and within the interval of 1 at m = 3 on 5/6 curves (C2 p = 4111: 0.95 [0.91, 0.98]).
* Image gain (class factor) 3.53–4.20 for R and 3.60–4.07 for the literal φ images, all intervals inside [3, 5].
  It approaches 4 as p grows (m = 3 at p = 1051 is ≈ 3.5–3.65, below 4 with intervals excluding 4: a small-p
  saturation effect of the census, not a law violation under the stated falsifier).
* Max translation overlap 2: no usable saturated degree-1 base exists.

## Limits
* |F| = 80 only, one curve per (family, p); the G arm is a control and deviates in U (fewer x-orbits fit), so its
  derived cost is not interpreted.
* The R arm's unknowns gain exceeds 2 (2.1–2.5) because points in H₄ share x less often than plain points; it converges to 2 with p.
* Wall times are contended and not used. The Nagao cost is a counted-multiplication model, not a timing.
* Cleanup after the runs (removed an unused helper, a `mut`, and a constant `if`; rustfmt) changed no behaviour; the
  binary that produced the data was built from the pre-cleanup source.

## Next step
Move the Richelot construction (validated here) into `src/cryptanalysis/genus2_fp.rs`; repeat at |F| ∈ {160, 320} and p ≈ 10⁴ to see the m = 3 image gain close to 4.
