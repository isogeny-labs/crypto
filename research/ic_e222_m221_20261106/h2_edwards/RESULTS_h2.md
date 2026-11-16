# H2: Edwards unified addition on the real E-222, priced in counted field operations

**Status:** unvalidated (pre-gate). Pure-Python operation counts on the registry curve `other/E-222`; no wall time, no isolation, no replay certificate, no validator report, no §5a band. Not a leaderboard row, no rho comparison.

**Curve.** p = 2^222 - 117, Edwards x^2 + y^2 = 1 + d x^2y^2, d = 160102 (non-square, so the unified formulas are complete), cofactor 4, registry generator and order. Weierstrass image through Edwards -> Montgomery (A, B) -> short Weierstrass, as in `gen_instances.py`.

**Correctness checks (agree = same Edwards-affine result).** [r]G = identity on all four implementations: edwards_affine True, edwards_ext True, weier_affine True, weier_jacobian True. 20/20 random scalars give equal results across eight scalar-multiplication variants (Edwards affine, extended binary mixed/unified, extended 4-bit window, Weierstrass affine, Jacobian binary mixed/general, Jacobian window).

**Op classes.** M full multiplication (a full-size constant such as the Weierstrass `a` counts as M); S squaring; D multiplication by the 18-bit constant d; A add/sub/neg/shift-by-add (x2 = 1A, x3 = 2A, x4 = 2A, x8 = 3A); I inversion. Formulas: Hisil-Wong-Carter-Dawson 2008 (`add-2008-hwcd` with a = 1, `dbl-2008-hwcd`); Bernstein-Lange 2007 Jacobian (`add-2007-bl`, `madd-2007-bl`, `dbl-2007-bl`).

**Pricing.** M-equivalent = M + 0.8 S + 0.1 D + 0 A + (I/M) I, with I/M in {1, 20, 80, 100}. D = 0.1 M is an assumption (18-bit by 222-bit product), A = 0 in the headline (a 0.05 sensitivity is in the scalar table). I/M must be measured on the target implementation; the Python ratio here is a diagnostic only: 38.0x (inversion by `pow(x, -1, p)` against `x*y % p`).


## Single group operations (exact counts)

| operation | M | S | D | A | I | M-eq @ I=1M | M-eq @ I=20M | M-eq @ I=80M | M-eq @ I=100M |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| Edwards affine add | 10 | 0 | 1 | 4 | 1 | 11.1 | 30.1 | 90.1 | 110.1 |
| Edwards extended unified add (HWCD) | 9 | 0 | 1 | 7 | 0 | 9.1 | 9.1 | 9.1 | 9.1 |
| Edwards extended mixed add (Z2=1) | 8 | 0 | 1 | 7 | 0 | 8.1 | 8.1 | 8.1 | 8.1 |
| Edwards extended dedicated doubling | 4 | 4 | 0 | 7 | 0 | 7.2 | 7.2 | 7.2 | 7.2 |
| Weierstrass affine add | 2 | 1 | 0 | 6 | 1 | 3.8 | 22.8 | 82.8 | 102.8 |
| Weierstrass affine doubling | 2 | 2 | 0 | 8 | 1 | 4.6 | 23.6 | 83.6 | 103.6 |
| Weierstrass Jacobian add (add-2007-bl) | 11 | 5 | 0 | 13 | 0 | 15.0 | 15.0 | 15.0 | 15.0 |
| Weierstrass Jacobian mixed add (madd-2007-bl) | 7 | 4 | 0 | 14 | 0 | 10.2 | 10.2 | 10.2 | 10.2 |
| Weierstrass Jacobian doubling (dbl-2007-bl) | 2 | 8 | 0 | 17 | 0 | 8.4 | 8.4 | 8.4 | 8.4 |

## One scalar multiplication, mean over 200 random 220-bit scalars (final conversion to affine included)

| method | M | S | D | A | I | M-eq @ I=1M | M-eq @ I=20M | M-eq @ I=80M | M-eq @ I=100M | M-eq @ I=100M, A=0.05 |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| Edwards affine, binary | 3264.8 | 0.0 | 326.48 | 1305.92 | 326.48 | 3623.9 | 9827.0 | 29415.8 | 35945.4 | 36010.7 |
| Edwards extended, binary, mixed add | 1743.06 | 872.34 | 108.59 | 2286.725 | 1.0 | 2452.8 | 2471.8 | 2531.8 | 2551.8 | 2666.1 |
| Edwards extended, binary, unified add | 1853.66 | 872.28 | 108.82 | 2288.23 | 1.0 | 2563.4 | 2582.4 | 2642.4 | 2662.4 | 2776.8 |
| Edwards extended, 4-bit window | 1427.835 | 867.2 | 63.515 | 1962.205 | 1.0 | 2128.9 | 2147.9 | 2207.9 | 2227.9 | 2326.1 |
| Weierstrass affine, binary | 655.35 | 545.755 | 0.0 | 2402.21 | 327.675 | 1419.6 | 7645.5 | 27306.0 | 33859.5 | 33979.6 |
| Weierstrass Jacobian, binary, mixed add (producer path) | 1207.99 | 2185.28 | 0.0 | 5245.605 | 1.0 | 2957.2 | 2976.2 | 3036.2 | 3056.2 | 3318.5 |
| Weierstrass Jacobian, binary, general add | 1636.405 | 2288.495 | 0.0 | 5119.505 | 1.0 | 3468.2 | 3487.2 | 3547.2 | 3567.2 | 3823.2 |
| Weierstrass Jacobian, 4-bit window | 1087.405 | 2041.715 | 0.0 | 4528.895 | 1.0 | 2721.8 | 2740.8 | 2800.8 | 2820.8 | 3047.2 |

Ratios Edwards extended / Weierstrass Jacobian (same algorithm), M-eq at I = 1M, 20M, 80M, 100M:
- binary, mixed add (the producer's algorithm): 0.829, 0.831, 0.834, 0.835
- 4-bit window: 0.782, 0.784, 0.788, 0.790

The affine rows pay one inversion per group operation and are two orders of magnitude above the projective ones at any I/M of 20 or more.

## IC relation-trial model (`find_one_relation_counted_with_rng`)

Per trial the producer does two variable-time scalar multiplications (Weierstrass Jacobian, binary, mixed add, each ending in `to_affine`), one affine addition `aG + bQ`, then for each of `fb_size` factor-base elements: the Semaev S3 coefficients, the discriminant, `sqrt_mod_p` (Euler test `n^((p-1)/2)` always; for p = 3 mod 4 a second power `n^((p+1)/4)` only if the discriminant is a residue), then one inversion `1/(2 qa)` and two candidate products only on that branch. Hash lookups are not counted.

- **Model A (task statement):** one exponentiation per element, 222 S + 111 M, plus about 6 M, 3 S, 12 A and 0.5 I of other work.
- **Model B (source-exact, counted by running it):** Euler exponent (p-1)/2 has 221 bits and popcount 217; root exponent (p+1)/4 has 220 bits and popcount 217. For this p both are almost all ones, so plain square-and-multiply costs about 220 S and 216 M each, not 111 M. Residue fraction observed 0.507. Counted mean per element: M 330.63, S 334.14, D 0.0, A 17.05, I 0.51. (The Rust `modpow` may use a window; the count above is the plain binary model, so Model B is an upper estimate for the exponentiation share.)

**What H2 replaces.** Only the part before the factor-base loop: two Edwards extended binary mixed-add scalar multiplications without the final affine conversion, one unified extended addition, and the conversion of R to the Weierstrass x used by Semaev (`u = (Z+Y)/(Z-Y)`, `x_W = B u + AB/3`: 1 I, 2 M, 3 A). Replacement part counts: M 3490.4, S 1743.8, D 217.95, A 4580.3, I 1.0. The factor-base loop is unchanged. The base-point `T = x y` is a one-time cost per relation and is not counted.

| model | fb_size | I/M | scalar-mult part, producer (M-eq) | trial, producer (M-eq) | scalar-mult share | scalar-mult part, H2 | trial, H2 | H2 / producer |
|:--|--:|--:|--:|--:|--:|--:|--:|--:|
| A | 8 | 1 | 5918.2 | 8298.2 | 71.3% | 4908.2 | 7288.2 | 0.878 |
| A | 8 | 20 | 5975.2 | 8431.2 | 70.9% | 4927.2 | 7383.2 | 0.876 |
| A | 8 | 80 | 6155.2 | 8851.2 | 69.5% | 4987.2 | 7683.2 | 0.868 |
| A | 8 | 100 | 6215.2 | 8991.2 | 69.1% | 5007.2 | 7783.2 | 0.866 |
| A | 32 | 1 | 5918.2 | 15438.2 | 38.3% | 4908.2 | 14428.2 | 0.935 |
| A | 32 | 20 | 5975.2 | 15799.2 | 37.8% | 4927.2 | 14751.2 | 0.934 |
| A | 32 | 80 | 6155.2 | 16939.2 | 36.3% | 4987.2 | 15771.2 | 0.931 |
| A | 32 | 100 | 6215.2 | 17319.2 | 35.9% | 5007.2 | 16111.2 | 0.930 |
| B | 8 | 1 | 5918.2 | 10705.9 | 55.3% | 4908.2 | 9695.9 | 0.906 |
| B | 8 | 20 | 5975.2 | 10840.0 | 55.1% | 4927.2 | 9792.0 | 0.903 |
| B | 8 | 80 | 6155.2 | 11263.6 | 54.6% | 4987.2 | 10095.6 | 0.896 |
| B | 8 | 100 | 6215.2 | 11404.8 | 54.5% | 5007.2 | 10196.8 | 0.894 |
| B | 32 | 1 | 5918.2 | 25068.8 | 23.6% | 4908.2 | 24058.8 | 0.960 |
| B | 32 | 20 | 5975.2 | 25434.4 | 23.5% | 4927.2 | 24386.4 | 0.959 |
| B | 32 | 80 | 6155.2 | 26588.8 | 23.2% | 4987.2 | 25420.8 | 0.956 |
| B | 32 | 100 | 6215.2 | 26973.6 | 23.0% | 5007.2 | 25765.6 | 0.955 |

## Reading

- On the scalar multiplications H2 is a real saving in counted operations: about 17% fewer M-equivalents than the Weierstrass Jacobian path with the producer's algorithm, and about 21% fewer with a 4-bit window, at I/M = 80 (see the ratio lines; the saving comes from the doubling, 4M+4S against 2M+8S, and from the mixed add, 8M+1D against 7M+4S).
- The whole trial is dominated by the factor-base loop once `fb_size` is not small: the two scalar multiplications are 55% (Model B) to 70% (Model A) of a trial at `fb_size` 8 and 23% to 38% at `fb_size` 32. H2 therefore changes a trial by about 10% to 13% at `fb_size` 8 and about 4% to 7% at `fb_size` 32.
- H1 (cofactor projection) reduces the number of trials per relation by the cofactor (4 on E-222); H2 reduces the cost per trial by at most the figures above. They compose multiplicatively: this note prices only the per-trial factor.
- The factor-base loop (two exponentiations of 220-bit-shaped exponents per residue element) is the larger lever at `fb_size` 32. Pricing it was out of scope here.

## Limits

Operation counts in Python, not wall time; the constants A (0) and D (0.1 M) are assumptions; Rust `BigUint` multiplication does not have the 1 M : 0.8 S shape assumed; the Rust producer's relation search on the registry curve has not been run (its field is capped at 10^6, so H1's ladder used 10 to 17 bit groups); H2 requires carrying the Edwards model next to the Weierstrass model the producer uses; no replay, validator or matched-rho pricing. Unvalidated, pre-gate.

## Commands

```
cd research/ic_e222_m221_20261106/h2_edwards
python3 -I h2_price.py --params /home/user/crypto/docs/curves/standards/parameters.json --out .
python3 -I make_report.py .
```
