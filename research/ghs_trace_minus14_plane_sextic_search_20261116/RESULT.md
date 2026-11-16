# Five rational ordinary nodes: exact finite exclusion

## Scope and prior work

Kudo and Harashita, *Representation of non-special curves of genus 5 as plane sextic curves and its application to finding curves with many rational points* (2023), Section 4.1, classify the two PGL3(F3) positions of five rational singular points when no four are collinear. Their Section 4.2 computes curves with at least 32 points over F9. Our required count is 10, so that published computation does not settle this slice. Source: https://arxiv.org/pdf/2102.07270 .

An irreducible sextic with four double points on one line would meet that line at least eight times, contrary to Bezout. Hence the two positions cover the five-rational-point configuration for irreducible sextics. This screen requires all five points to be **ordinary nodes**. It excludes repeated-tangent double points and all other Frobenius-orbit patterns; it does not classify all genus-five curves.

## Frozen computation

The first source `search.py` (SHA-256 `bb55f548a990e9611d48bd9359df4575a68df49ca1d83813189dccb4f0686383`) and the second `search_case2.py` (SHA-256 `e9aef41158ed7fee738435717730951fc9dcc127ce820803f887d500939217b2`) were committed before their counted runs. Each 28-monomial sextic system has 15 independent node/derivative conditions and a 13-dimensional F3 kernel. Each run enumerated all 1,594,322 nonzero vectors and retained one representative for each of 797,161 projective coefficient classes. Both report one direct F3/F9 point-evaluation control. Raw receipts are `FULL.json` and `FULL_CASE2.json`; both commands exited 0.

| Fixed rational nodes, in addition to `(0:0:1),(0:1:0),(1:0:0),(1:1:0)` | Five ordinary nodes | Normalized N1=4 | N1=4, N2=10 | N1=4, N2=10, N3=28 |
| --- | ---: | ---: | ---: | ---: |
| `(0:1:1)` | 104,976 | 4,543 | 4 | 0 |
| `(1:2:1)` | 104,976 | 3,159 | 0 | 0 |

At an ordinary rational node, the normalization contributes two points over odd extensions if its tangent cone splits, zero if it does not, and two over even extensions. Thus the correction to the plane point count is `2*s-5` for odd extension degree and `+5` for even degree, where `s` is the number of split tangent cones among the five nodes. This is the correction used in both screens.

The four first-case F9 survivors were extracted with `extract_n2.py` into `N2_SURVIVORS.json` and replayed by direct Sage polynomial evaluation over F3, F9, and F27 in `direct_replay.py`. The replay receipt is `DIRECT_REPLAY.json`. All four have normalized counts `(4,10,34)`, so all miss the required F27 count 28. The direct replay source SHA-256 is `ac152c2e30a67b36ee9713de41774cbaf81d3081a328279abadb3c6c9d719ac4` and the captured-input SHA-256 is `19189556c9aa1e686ca37aedb5ded163e2d3a99dcafe5612053c8b9439fe8cb1`.

For any L-polynomial `1-t*T^5+3^5*T^10`, the first four power sums of Frobenius roots vanish. Thus every one of the eight trace-isogeny candidates with `t=±10,±14,±22,±26` requires `(N1,N2,N3,N4)=(4,10,28,82)`. The first three counts alone exclude all five-rational-ordinary-node sextics in both positions from those eight classes.

This is a finite exclusion for one geometric stratum. It neither produces a curve in an eligible isogeny class nor establishes a subgroup transfer or an end-to-end attack cost. Next search targets the `(1,1,1,2)` and other Frobenius-orbit patterns of the five nodes, then repeated-tangent double points.
