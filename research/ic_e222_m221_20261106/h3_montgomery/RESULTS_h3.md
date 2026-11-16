# H3 priced on the real M-221: x-only Montgomery arithmetic for `x(aG + bQ)`

**Status:** unvalidated (pre-gate). Python, exact counted field operations, no wall time, no isolated host, no replay certificate. Not a leaderboard row.

**Command** (about 10 s): `cd research/ic_e222_m221_20261106/h3_montgomery && python3 -I h3_price.py --out results_h3.json` (defaults: seed `h3-v1`, 200 uniform pairs, 100 golden-selected pairs, 2000 candidates each). Full output: `results_h3.json`.

**Curve** (`other/M-221`, read from `docs/curves/standards/parameters.json`): p = 2^221 − 3, B y² = x³ + A x² + x, A = 117050, B = 1, (A+2)/4 = 29263, G = (4, 0x0f7acd…), r has 219 bits, cofactor 8. Scalars are uniform in [1, r−1]. Q is a fixed random multiple of G (its log is never an input to any route). The Weierstrass image comes from `mont_to_weier` in `gen_instances.py` (u = x + A/3, v = y).

## Checks (all agree)

- [r]G = O three ways: affine Montgomery, x-only ladder (final Z = 0), Weierstrass-Jacobian. The registry generator and order are consistent.
- Ladder x([k]G) equals the Weierstrass x under the map for 20 random scalars.
- Routes a0, a, c, b1 give the reference x(aG + bQ) on 200/200 random pairs. The Euclid chain gives it on 15/15 small pairs (16, 32, 64 bits) and 100/100 golden-selected full-size pairs.

## Counted operations per relation sample (mean over 200 pairs; M mul, S sqr, I inv, D small-constant mul)

Per-primitive counts measured by running them: xDBLADD (affine difference) 5M+4S+1D; dADD 4M+2S (3M+2S with affine difference); Jacobian double 2M+8S (full-size `a`); mixed add 7M+4S; Jacobian add 11M+5S.

| route | M | S | I | D | M-eq, I=20M | I=80M | I=100M |
|---|---:|---:|--:|--:|---:|---:|---:|
| (a0) producer shape: 2 scalar mults to affine + affine add | 2382 | 4321 | 3 | 0 | 5899 | 6079 | 6139 |
| (a) Jacobian: 2 scalar mults + 1 Jacobian add, x only | 2386 | 4324 | 1 | 0 | 5866 | 5926 | 5946 |
| (c) Jacobian Shamir interleaved, table G, Q, G+Q | 1575 | 2386 | 1 | 0 | 3504 | 3564 | 3584 |
| (b1) 2 x-only ladders + y-recovery + 1 affine add | 2186 | 1741 | 3 | 436 | 3682 | 3862 | 3922 |
| (b2) x-only Euclid differential chain, golden-selected pair | 1574 | 788 | 1 | 0 | 2225 | 2285 | 2305 |
| (b2u) same chain, uniform random pair (mean) | 8208 | 4104 | 1 | 0 | 11511 | 11571 | 11591 |

Pricing: M-eq = M + 0.8 S + 0.1 D + (I/M) · I. Additions and subtractions (A) are counted in the JSON and not priced. The (a0) row matches how `scalar_mul` and `add` in the producer are shaped (Jacobian double-and-add with mixed addition, one inversion per conversion to affine); this prices the shape, not the Rust code.

## What each route is

- **(b1)** Two Montgomery ladders (Montgomery 1987; xDBLADD as in Costello–Smith 2017), then the y-coordinate of each result recovered from the ladder pair (derived here from the addition law, checked numerically; this is not the optimised Okeya–Sakurai form, so the 8M+2S+1D+1I per recovery is an upper bound), then one affine add.
- **(b2)** Montgomery's Euclid-style Lucas chain (Montgomery 1992, *Evaluating recurrences of form X_{m+n} = f(X_m, X_n, X_{m−n}) via Lucas chains*; Bernstein 2006, *Differential addition chains*): state (P1, P2, P1−P2) and coefficients (a, b); each subtractive step replaces (a, b) by (a−b, b) with pair (P1, P1+P2), or (a, b−a) with pair (P1+P2, P2); one dADD per step; ends when one coefficient is 0 (needs gcd(a, b) = 1). It uses the known x(G−Q) and nothing else.
  - **Step counts.** The number of steps equals the sum of the partial quotients of a/b (checked: formula equals executed count on all 100 golden pairs). Uniform pairs: median 1210, mean 2052, max 40707 (heavy tail), against the lower bound log_φ(2^218) = 314 for any such chain.
  - **Golden selection.** For each random a, b is searched among 2000 candidates around a/φ for the fewest steps: mean 394, min 375, max 423. This changes the distribution of (a, b): R = aG + bQ is then drawn from a one-parameter family per a. Whether relation yield and the linear algebra tolerate that is not tested here.

## Reading

- Among routes that keep the uniform (a, b) of the current sampler, Shamir (c) is cheapest at 3504 to 3584 M-eq, 1.68× below the producer-shaped route (a0) at I = 20M; x-only ladders (b1) are close (3682 to 3922). The x-only routes gain through S being cheaper per bit (4S instead of 8S per doubling), and lose through two ladders instead of one interleaved pass.
- The x-only Euclid chain with golden-selected pairs prices at 2225 to 2305, 0.38× of (a0) and 0.63× of (c), at the cost of the restricted (a, b). With uniform pairs it is worse than every other route (mean 11511; median-step estimate about 6800).
- The M-eq model ignores A, memory traffic and the cost of selecting b (integer work, no field operations).

## The "factor 2" claim in the H3 row of the regimen note

The producer already has it. `build_factor_base` stores one entry per x (smaller of y, p−y), `find_one_relation_counted_with_rng` looks candidates up by x in `x_to_idx` and sweeps `S₃(x_R, x_F, X)` in x only, and `try_finalise` tries all four sign combinations ε_i F_i + ε_j F_j. So the ±F folding is in place, and H3 has no trial-count effect: only the per-operation price in the table above is open. The row's "factor of 2 in effective factor base at equal cost per relation" is not an available gain.

## Not done

Rust implementation and timing; the Semaev sweep over the factor base (a sqrt per factor-base element per trial), which this table does not price; the same table for E-222 (Edwards complete addition, H2); generating R by a random walk with one addition per new R instead of a fresh pair (a, b) (not examined; it changes the sampler, not the arithmetic).
