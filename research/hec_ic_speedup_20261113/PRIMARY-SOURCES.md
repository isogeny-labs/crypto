# Primary-source addendum, 14 November 2026

After the run report was closed, an earlier primary draft became accessible:
[Gaudry–Thériault–Thomé, 16 February 2005](https://cacr.uwaterloo.ca/techreports/2004/corr2004-29.pdf).
This is the three-author draft, distinct from the four-author 2007 publication.

Sections 2.3 and 3.1 describe signed large-prime graph updates using union-find. A cycle outside
the distinguished vertex need not give a full relation: it can instead produce a one-large-prime
relation. This supports retaining coefficient-aware elimination rather than unsigned cycle counting.

Section 6.2 and Table 2 give C/C++ genus-3 timings on a 1.7 GHz Pentium-M:

| field size (approximately) | relation search | linear algebra | total | rho estimate |
|---|---:|---:|---:|---:|
| 2^24 | 0.6 days | 0.2 days | 0.8 days | 3.5 days |
| 2^27 | 9 days | 5.8 days | 14.8 days | 79 days |

The stated arithmetic cost is 5 microseconds per Jacobian operation and 20 microseconds per
relation trial including smoothness; linear algebra uses block Wiedemann. These are published,
not reproduced, and are not same-input or same-host comparisons with this study.
