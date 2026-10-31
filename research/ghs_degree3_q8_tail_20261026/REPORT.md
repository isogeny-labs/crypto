# GF(8) cubic cover bound for orders 526 and 542

26 October 2026. The [frozen protocol](PROTOCOL.md), [catalog extractor](extract_catalog.py),
[archive rows](catalog_models_raw.json), [exact screen](screen.py),
[certificate](certificate.json), and [Lean counts](Counts.lean) extend the
[orders-502/514 screen](../ghs_degree3_q8_screen_20261026/REPORT.md)
to the other two leading class-C groups from the earlier
[GF(8) quartic study](../ghs_binary_cubic_isogeny_20261026/REPORT.md).

## Exact cohort and finite lower bound

Use Conway `k=GF(8)` and `K=GF(512)`. Every normalized ordinary source
in these groups has `E(1,b):v²+uv=u³+u²+b`:

| Source order | Source `b` encodings | Listed hyperelliptic models | Catalog class |
| ---: | :--- | ---: | :--- |
| `526=2*263` | `19,48,56,106,126,129,151,159,160,203,207,213,226,236,238,246,252,261,267,290,292,354,356,399,424,459,510` | 27 | [`3.8.a_a_n`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_n) |
| `542=2*271` | `44,52,73,91,114,131,188,195,201,248,306,312,323,422,430,461,475,482` | 15 | [`3.8.a_a_bd`](https://www.lmfdb.org/Variety/Abelian/Fq/3/8/a_a_bd) |

The hash-verified LMFDB archive supplies all 42 displayed-class
hyperelliptic equations. Every equation passes the Sage genus-three
check and the four distinct genuine `K`-rational branch-point gate.
For each of the 999 exact source/model pairs, the complete degree-two
lift test finds no separable elliptic quotient and the degree-three
Artin--Schreier test finds no map to the corresponding source.

| Audit | Count |
| :--- | ---: |
| Source equations | 45 |
| Listed genus-three hyperelliptic models | 42 |
| Source/model pairs, skipped pairs | 999, 0 |
| Four-support rational functions | 23,976 |
| Five-support rational functions | 4,067,928 |
| Total tested rational functions, maps | 4,091,904, 0 |

The screen's constructed genus-three covers pass for both complete
degree-three support patterns, including recovery of a hidden double
point. The simple-pole residue fixes the scalar in each pattern; the
certificate records exact equations, every pair, source and script
hashes, and any skip or map witness. The degree-two and degree-three
argument is the same as in the [leading-class report](../ghs_degree3_q8_screen_20261026/REPORT.md).
For these 42 **listed models** and normalized source classes, any
separable curve map has degree at least four.

Across all four leading GF(8) order classes, the two certificates now
cover **66 listed hyperelliptic models, 1,431 exact pairs, and 5,861,376
degree-three rational functions**, with zero skips or maps and zero
degree-two elliptic quotients. This is a finite route bound, not a
classification of all curves or all GHS transfer mechanisms.

## Prior work and surviving route

The isogeny classes and listed equations are prior LMFDB data. The
[original GHS work](https://www.kantmath.de/publications/papers/weil_descent.pdf)
and [Menezes--Teske](https://cacr.uwaterloo.ca/techreports/2004/corr2004-25.pdf)
give the direct binary cubic descent and isogeny-walk context.
[Tian](https://arxiv.org/pdf/2012.07173) gives an evaluated cubic
Weil-restriction isogeny in another setting; its displayed formulas
use three nonzero geometric two-torsion points, whereas an ordinary
characteristic-two elliptic curve has one. This screen adds a finite
negative calculation within the project; publication novelty has not
been established.

The earlier GF(8) study records exact smooth **plane quartics** in
both classes with the matching Weil polynomial and nonempty source-prime
rational-point part. Those curves are outside this hyperelliptic-model
screen and remain the principal constructive lead. No explicit
source-to-quartic subgroup map or complete transfer/target-decomposition
benchmark is supplied here. The number of newly demonstrated vulnerable
families is zero. Unlisted models, alternative polarizations, degree-four
or higher curve maps, and non-cover correspondences remain open.

`lean Counts.lean` checks enumeration arithmetic and subgroup-order
coprimality with three. It does not formalize the geometric completeness
or provide an isogeny certificate.
