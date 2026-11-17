# Explicit quotients after one 3-isogeny: results

**Historical stage report.** See [the current counterexample and completed
small-field polynomial transfers](NEXT_RESULTS.md).

The construction stage succeeded on all **23 newly eligible inputs** from the
earlier screen. Their 276 kernels produced **276 smooth plane quartics over the
base field**. Of the corresponding polarized quotients, **152 are base-field
Jacobians and 124 have the quadratic twist obstruction**. Every input has at
least one usable quotient; the observed range is 2–11 out of 12.

This is a working construction/coverage result, not a new end-to-end ECDLP
solver. The remaining implementation step is to invert general quotient Weil
coordinates into target divisors, recover signs consistently, and run target
index calculus with all costs included.

## Exact small-field certificates

| p | Newly eligible inputs | Smooth quartics | Usable Jacobians | Obstructed | Usable per input |
|---|---:|---:|---:|---:|---:|
| 7 | 3 | 36 | 33 | 3 | 11–11 |
| 13 | 3 | 36 | 16 | 20 | 3–8 |
| 19 | 5 | 60 | 26 | 34 | 2–8 |
| 31 | 5 | 60 | 32 | 28 | 5–8 |
| 37 | 4 | 48 | 27 | 21 | 4–9 |
| 43 | 3 | 36 | 18 | 18 | 5–8 |
| Total | 23 | 276 | 152 | 124 | 2–11 |

For every quartic, all 28 odd-gradient bitangents and global smoothness were
checked. Every coefficient was verified fixed by p-Frobenius. Independent
Jacobian-order certificates combine exact counts over F_p and F_(p²), exact
divisor-class orders, and a Weil interval containing just one possible multiple.
The [raw records](results/quotient_sweep.json) contain the equations and certificates.

These are exploratory inputs from the earlier sample; Frobenius conjugates and
curves in the same isogeny class occur. These counts are not independent samples
or a population-density estimate. No claim is made about all cubic curves.

## The more useful implementation lead

The same 27-coefficient tangent-line tensor supplies odd theta gradients. They
give both a quartic in the Weil-restriction differential coordinates and a direct
evaluation of the known chi_18 obstruction criterion. This avoids reconstructing
from Dixmier-Ohno invariants and avoids Jacobian arithmetic during kernel
selection. The chi_18 predictions match **all 276 independently certified
quartics**, identified by equation rather than arbitrary kernel labels.

The [method and derivation](QUOTIENT_METHOD.md) distinguish this specialization
from established isogeny, theta-gradient, and Serre-obstruction techniques.
The latter have substantial prior art, including
[Kirschmer–Narbonne–Ritzenthaler–Robert](https://arxiv.org/abs/2004.08315).
Novelty of the complete specialization is unverified; no new attack exponent is
claimed.

## A point map was evaluated on a newly constructed quotient

On the p=7, r=373 example, the new quotient is evaluated in 64 projective
level-two Weil coordinates. All 27 kernel translations leave the image unchanged.
Enumerating the 373-point subgroup yields exactly 187 images, as required when
opposite points are identified. The associated quartic's Jacobian order is
independently certified as 373. The new source/target test pair has scalar 218.

This validates quotient-point evaluation modulo sign. It does not convert those
coordinates into divisors or solve a DLP on this new target. See
[the exact image audit](results/theta_point_probe.json).

## Larger construction tests

- p=2147483857, prime r=9903523205810560391850992323 (94 bits): 9/12 predicted usable; one smooth quartic reconstructed and all 28 bitangents checked. Total observed time 2.40 s, including source generation.
- p=38685626227668133590608551, prime r=57896044618658097711834515899297219474986235450897067015382916193432141113279 (256 bits): 5/12 predicted usable; one smooth quartic reconstructed and all 28 bitangents checked. Total observed time 17.93 s, including source generation.

CM discriminant -59 supplies candidate orders, but the source prime orders are
independently certified: a nonzero point is killed by the prime r, and its Hasse
interval contains only one multiple of r. One descending 3-isogeny supplies an
input that fails the direct torsion screen, and its unique ascending step reaches
the prepared curve. Large-p smoothness uses exact Jacobian-ideal Gröbner bases
equal to [1] in all three affine charts.

The larger tests use chi_18 for the obstruction. Their Jacobian orders were not
independently point-counted. Their timings include different checks from the
small-field certification sweep and are not DLP performance comparisons.

## Bounded search for an obstruction counterexample

The additional search examined 200 distinct prepared (j,r)
pairs across p=13,19,31,37,43,61. It found 0 inputs with all 12
quotients obstructed. The minimum observed usable count was 1.
The [records](results/coverage_hunt.json) retain all chi_18 predictions. Only an
all-obstructed hit triggers independent full quotient-order certification in
this extra search; the other records are obstruction predictions.

Whether at least one usable quotient always exists under the stated torsion
conditions remains open here. A proof or an independently certified
counterexample would be the next mathematical contribution.

## Reproduce

From the suite root with SageMath 10.6:

```sh
sage -python research_lead/quotient_sweep.py
sage -python research_lead/obstruction_probe.py
sage -python research_lead/theta_point_probe.py
sage -python research_lead/zero_test_regression.py
sage -python research_lead/large_instance.py --bits 32
sage -python research_lead/large_instance.py --bits 86
sage -python research_lead/coverage_hunt.py --count 72 --max-draws-per-prime 100000
sage -python research_lead/coverage_hunt.py --count 200 --resume --max-draws-per-prime 3000
python3 research_lead/quotient_report.py
```

The original eligibility outputs remain in this folder. The current arithmetic
results are in [quotient_summary.json](results/quotient_summary.json). The recovered
Sage zero-test issue is documented by an independent three-extension point-count
[regression](results/zero_test_regression.json); production certificates use class
equality with G.zero(). Development failures are excluded from the current report.
