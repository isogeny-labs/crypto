# New results: prepared-curve coverage fails, polynomial transfer works

**Historical stage.** The [four-track continuation](FOUR_TRACK_RESULTS.md) adds
a forced two-step source example, endomorphism-ring comparisons, stronger
transfer handling, and geometric target solvers. Its scope distinguishes one
total step from one extra step after preparation.

The stronger prepared-curve coverage claim has a counterexample. Separately,
the quotient-to-divisor stage is now implemented as a base-field polynomial
system, and actual target index calculus completes two small ECDLP reductions.

## A precise counterexample and its repair

Over F_(109³), with z³+z+103=0, the curve

    y² = x³ + (37z²+105z+29)x + (43z²+6z+66)

is ordinary, has j not in F_109, and has prime group order r=1293169. It meets
p=1 mod 3 and r=4 mod 9 and already has scalar -I action on E[3]. Nevertheless,
all 12 transverse quotient candidates produce curves whose base-field
Jacobians have order **1296891=3^7·593**. They cannot preserve an r-subgroup.

All twelve orders have independent exact certificates using N1=110, N2=11882,
explicit rational-point divisor orders, and a unique multiple in a Weil
interval. The source prime order also has an independent point-annihilator and
Hasse-interval certificate. This conclusion does not depend only on chi18.

Each of the four immediate 3-isogeny neighbors has usable candidates: 5,3,4,5
respectively. One repaired target Jacobian has independently certified order r.
See [the counterexample, equations, and scope](COUNTEREXAMPLE.md),
[the full audit](results/counterexample_audit.json), and
[explicit divisor witnesses](results/explicit_counterexample_witnesses.json).

**What is refuted:** every prepared curve immediately has a usable kernel, so
the procedure can stop once the torsion condition holds.

**What remains open:** one freely chosen additional isogeny always reaches
some usable prepared model. This example is repaired by such a step and does
not refute that weaker existential statement. No universal walk-length bound
is proved.

## Direct image inversion and actual target logarithms

The new driver uses a rational odd theta characteristic and rational tangent
chart. Eight symmetric sections are fitted from a fixed number of random curve
points, with rank 112 and eight exact symbolic diagonal identities checked.
The final seven-variable polynomial system is over F_p. Its solutions give
actual divisor classes; three images, for P,Q,P+Q, fix their relative signs.

The target logarithm is then recovered by a rational-point smooth-divisor
index-calculus solver. Every relation is checked in the actual Jacobian and
the answer is checked on the original elliptic curve. No target-group
enumeration, F_(p³) point scan, or known source logarithm is used by this driver.

| p | Source prime order | Recovered scalar | Three polynomial inversions | Target index calculus | Complete pipeline | Source BSGS |
|---|---:|---:|---:|---:|---:|---:|
| 7 | 373 | 218 | 2.90 s | 16.67 s | 45.64 s | 0.0011 s |
| 13 | 2137 | 803 | 3.48 s | 65.19 s | 92.88 s | 0.0022 s |

The pipeline timings include setup and checks. The BSGS measurements use the
same complete source subgroup. These small tests show **no speedup**. The
target solver is basic smooth-divisor index calculus, not Diem's optimized
line/large-prime method. No cryptographic-size DLP was solved.

The [derivation](POLYNOMIAL_INVERSION.md) explains the theta translation,
normalization cancellation, section construction, and polynomial recovery.
Raw records: [p=7](results/polynomial_inverse.json),
[p=13](results/polynomial_inverse_p13.json).

## Search scope and novelty

The counterexample was found in a 4430-case early-rejection Hesse-family search.
A separate 1000-case search and 49 closed classes in three targeted isogeny
components found no counterexample. These are different, potentially
overlapping exploratory datasets, not independent density estimates.

The work builds on [Tian's quotient and symmetric-section method](https://arxiv.org/abs/2012.07173),
known isogeny preprocessing, and established algebraic Serre-obstruction
techniques, including [Kirschmer–Narbonne–Ritzenthaler–Robert](https://arxiv.org/abs/2004.08315).
The odd-theta base-field specialization and this coverage counterexample have
not been certified novel. The counterexample refutes our proposed sufficient
condition, not a claimed theorem of Tian.

The remaining research work is optimized target index calculus, comprehensive
chart handling, larger transfer benchmarks, and the weaker coverage question.

## Reproduce

From the suite root with SageMath 10.6:

```sh
sage -python research_lead/counterexample_audit.py
sage -python research_lead/explicit_counterexample_witnesses.py
sage -python research_lead/polynomial_inverse.py --solve
sage -python research_lead/polynomial_inverse.py --solve --p 13
sage -python research_lead/finalize_transfer_results.py
python3 research_lead/next_report.py
```

The older exhaustive `complete_toy.py` inverse oracle is development evidence
and is not used in the polynomial runs. Saved code hashes and validation scope
are in [next_summary.json](results/next_summary.json).
