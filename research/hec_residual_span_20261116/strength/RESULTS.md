# Subgroup-strength screen: five accepted fields

The once-only run at pushed checkpoint `8e706cdb2f6` exited 0. It classified all eight fixed fields and passed both positive controls. Five models passed the original algebraic checks and the additional integer criterion

`4 ell >= U`, where `U = (ceil(sqrt(p)) + 1)^(2g)`.

Hasse's bound gives `#J <= U`. Since the verified source has prime order ell, ell divides #J. Consequently the whole-Jacobian cofactor is at most `floor(U/ell) <= 4`. This does not assume that the point-order LCM equals #J.

| Genus | p | Selected c | ell | U | Whole-cofactor upper bound | Point columns | Orbit columns |
|---|---:|---:|---:|---:|---:|---:|---:|
| 2 | 101 | 2 | 7,481 | 20,736 | 2 | 35 | 7 |
| 2 | 251 | 18 | 67,931 | 83,521 | 1 | 135 | 27 |
| 2 | 401 | none | none | 234,256 | — | — | — |
| 2 | 601 | 7 | 411,491 | 456,976 | 1 | 340 | 68 |
| 2 | 1021 | none | none | 1,185,921 | — | — | — |
| 3 | 113 | 3 | 1,247,429 | 2,985,984 | 2 | 49 | 7 |
| 3 | 211 | none | none | 16,777,216 | — | — | — |
| 3 | 421 | 2 | 77,152,909 | 113,379,904 | 1 | 217 | 31 |

“None” means that no model passed within the registered scan c=1..32. It is not a nonexistence claim for other constants or families. All rejections remain in FIXTURES.tsv. No fields were replaced, and no runtime result influenced selection. This screen constructed no target DLP instances and measured no speedup.

The accepted branch polynomials are irreducible of odd degree 2g+1. Frobenius therefore has one fixed branch point at infinity and one odd orbit containing the other branch points. Rational Jacobian 2-torsion corresponds to invariant even subsets modulo complements. A subset cannot be sent to its complement because infinity is fixed; the only invariant even subsets are empty and full. Thus rational 2-torsion is trivial and #J is odd. Every selected cofactor bound is actually one or two, so #J must equal the odd prime ell. A separate, prospectively frozen solve gate adds an integer Frobenius-determinant check of that order.

GENUS4-ROUTE.md separately derives the geometric decomposition forced by tame odd automorphisms in genus four and lists proof obligations for an order-four alternative. It does not certify a replacement curve or an attack.

All raw hashes verified. Git custody replaces unavailable S3; no upload, retry, field replacement or threshold change is claimed. The broader tenfold objective remains open.
