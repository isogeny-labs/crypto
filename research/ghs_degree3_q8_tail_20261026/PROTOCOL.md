# GF(8) remaining cubic degree-three cover screen

26 October 2026. Conductor T-5421. Extend the frozen exact screen for
orders 502 and 514 in the [GF(8) leading-class report](../ghs_degree3_q8_screen_20261026/REPORT.md)
to source orders `526=2*263` and `542=2*271`. The earlier
[GF(8) quartic report](../ghs_binary_cubic_isogeny_20261026/REPORT.md)
contains exact nonhyperelliptic Jacobian witnesses in these two classes.
The [GHS paper](https://www.kantmath.de/publications/papers/weil_descent.pdf)
and [Menezes--Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf)
cover the known direct binary descent and isogeny walks;
[Tian's cubic isogeny construction](https://arxiv.org/pdf/2012.07173)
requires geometric two-torsion data that differs in this ordinary binary
setting. This exact screen is a route bound, not a full transfer attack.

Extract both exact isogeny-class rows from the hash-verified local LMFDB
archive. For every listed genus-three hyperelliptic equation, check the
smooth genus and four genuine split branch points over Conway `GF(512)`.
Then test every source/model pair for a degree-two elliptic quotient and
both complete degree-three branch-support patterns via the characteristic-two
Artin--Schreier equation. Retain all skips, all hits with rational-map
witnesses, positive controls for both patterns, exact counts and input hashes.
The first run decides whether these listed models admit a separable
degree-two or degree-three curve map to a normalized source in the same
isogeny class. It does not assess unlisted curves, degree-four maps,
correspondences, or full attack cost.
