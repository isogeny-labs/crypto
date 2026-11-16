# R132: degree-11 and degree-22 base fibers

The two remaining finite lambda-discriminant x-points from R100 have complete, squarefree fibers in the R51 degree-seven **base** cover. Over the degree-11 point, the specialized y-polynomial has two linear factors and one irreducible quintic. Over the degree-22 point, it has irreducible factors of degrees one, two and four. Each point therefore has three base places, all with ramification index one in the base cover.

| x-point | y factor degrees | F2 residue degrees | weighted degree |
| --- | --- | --- | ---: |
| P11 | 1, 1, 5 | 11, 11, 55 | 77 |
| P22 | 1, 2, 4 | 22, 44, 88 | 154 |

`result.json` is the exact factor record. The producer uses the pinned R51 equation and independently verified R100 x-factors. `verify132.py` separately evaluates the equation in carryless finite fields and checks the specialized coefficients, derivative gcd, factor product, and Frobenius irreducibility tests. It reports `BASE132_INDEPENDENT_FACTOR_DEGREES_PASS`. A fresh producer run yielded byte-identical JSON with SHA-256 `8848edb1a7f1abfed1070260fd013d3e8194ba7bb998f936f7a6bbdb3667a43c`. A one-exponent change to the P11 input was rejected with `AssertionError: bad_factors`, before result creation. `lean/BaseFibers132.lean` checks the integer degree sums with `native_decide`; these Lean statements are arithmetic consequences of the separately checked factorization, not a formal factorization proof. Local exact work used no cloud resources.

The six places exhaust these two base fibers because the monic degree-seven specializations are squarefree and the checked factors multiply to the full polynomial. The [Henselian simple-root criterion](https://stacks.math.columbia.edu/tag/04GE) supplies the local lifting interpretation. These x-points are on the lambda discriminant: lambda normalization, exceptional branches, and N51 valuations at their lifts remain to be computed. The x-infinity fiber is a separate outstanding support.

Source and protocol were pushed before the result at `5acea4258`. The degree chart in `publication/fibers132.svg` and `publication/fibers132.pdf` records the residue-degree decomposition, not measured performance.
