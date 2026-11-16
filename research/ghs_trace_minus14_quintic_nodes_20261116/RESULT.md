# Degree-five node orbit: explicit genus-five carriers and a finite bound

## Result and scope

The normalization of **62 projectively inequivalent plane sextics over F3** has genus five and L-polynomial `1-t*T^5+243*T^10` with `t` among `±10,±14,±22,±26`. All 62 lie in the geometric stratum whose five singular points form one degree-five Frobenius orbit of ordinary nodes in general position. Thirty have `t=-14`, hence L-polynomial `1+14*T^5+243*T^10` and normalization counts `(4,10,28,82,314)` over F3 through F243. The five nodes have no collinear triples. The blowup/canonical-embedding argument for a plane sextic with five nodes in general position shows that the normalizations are nonhyperelliptic; see [Examples of genus 5 curves, Example 2.8](https://pi.math.cornell.edu/~ckfok/Examples%20of%20genus%205%20curves.pdf). These are explicit carrier curves in **previously known isogeny classes**. They are not an established GHS vulnerability: an evaluable subgroup-preserving source-to-Jacobian map, complete decomposition, and matched rho cost have not been obtained.

The main search source `search.py` (SHA-256 `ec003d36d4257c16f20169820f43a59576ce0d54c7cf34592683c75b7eea94b1`) was committed before the counted runs. Sage uses `GF(3^5,'zeta')` with modulus `X^5+2X+1`. The two node representatives are `(1:zeta^127:zeta^143)` and `(1:zeta^218:zeta^72)`. Each yields 15 independent F3-linear singularity conditions on 28 sextic monomials and a 13-dimensional kernel. The counted run for each position traversed all `3^13-1=1,594,322` nonzero vectors, or `797,161` projective coefficient classes. The first four point counts use direct plane counts because the degree-five nodes are absent over those extensions; the fifth count applies the exact `+5` or `-5` ordinary-node tangent correction.

| Node position | N1=4 | N2=10 | N3=28 | N4=82 | Ordinary N4 survivors | Eight-class matches |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 0 | 184,512 | 33,993 | 5,520 | 947 | 916 | 32 |
| 1 | 184,512 | 34,225 | 5,650 | 1,024 | 994 | 30 |

The raw receipts are `FULL_0.json` and `FULL_1.json` (SHA-256 `4e6578490826752d5280156238c530e183d57ae3c95571ea9c424b1170c14749` and `e735e143a9dc5a57a2f40a634e19901a717090895e51df0ce973aaacdd00f2f6`). They report all 1,971 fourth-count survivors, including 61 with nonordinary intended nodes, and give the exact sextic equation for each of the 62 class matches.

## Independent verification and coverage

`verify_hits.py` (SHA-256 `68e0da5d51da3809c2cd92d1c60c7813453f4ab5e39b2066c2f24b660a58fac1`) independently evaluates all 28 sextic monomials at every projective point over F3, F9, F27, F81, and F243 for each of the 62 equations. It checks the fifth-extension tangent correction, irreducibility over F3, irreducibility after base change to F9 and F27, and Sage/Singular geometric genus five. Geometric reducibility of an F3-irreducible degree-six polynomial would have 2, 3, or 6 conjugate components and would factor over F9 or F27, so these checks certify absolute irreducibility. The receipt `VERIFIED_HITS.json` (SHA-256 `cbfa850f1b9aa4d53723d3cf2a0f66006595cf093a08c30b009c37a31a235a3d`) records all 62 exact coefficient vectors and counts. It completed with all assertions true.

`verify_position_orbits.py` (SHA-256 `bac18275bad4b601e3349e67d07fb008b611c66f74ca24f4df1b93850de0503b`) independently enumerates all `11,856` degree-five Frobenius orbits of P²(F243), retains the `11,232` with no four points collinear, and enumerates PGL3(F3) of order `5,616`. The two searched configurations form disjoint full-size orbits of `5,616` each and cover all `11,232`; their five nodes also have no collinear triple. Its receipt is `POSITION_ORBITS.json` (SHA-256 `92f15d2f84a3f88222396573d334252996cc5569b651374598dc9f53a67b949d`). The full-size orbit also makes each fixed node configuration's projective stabilizer trivial. Because the 62 verified sextics have exactly the five ordinary nodes, the 62 distinct equations are pairwise projectively inequivalent as plane sextics. This does **not** establish 62 nonisomorphic normalizations.

For example, case 0 hit 7 gives the following exact equation over F3, interpreted modulo 3:

```text
-x^6 + x^4*y^2 - x^3*y^3 - x^2*y^4 - x*y^5 - y^6
- x^4*y*z + x^3*y^2*z + x^3*y*z^2 + x^2*y^2*z^2
- x^3*z^3 + x*y^2*z^3 + y^3*z^3 - y*z^5 + z^6 = 0.
```

Its direct plane counts are `(4,10,28,82,319)`; the five nonsplit nodes contribute `-5` at degree five, giving normalization counts `(4,10,28,82,314)`. Its geometric genus is five. Its Jacobian has `#J(F3)=L(1)=258=2*3*43`. The point-count and genus checks show its Jacobian lies in the known isogeny class `5.3.a_a_a_a_o`.

## Prior work and novelty boundary

Kudo--Harashita's [plane-sextic classification](https://arxiv.org/pdf/2102.07270), Section 4.1, supplies the two position patterns, while their reported computation in Section 4.2 imposed `#C(F9) >= 32`. Every curve here has `#C(F9)=10`. Their [hyperelliptic/trigonal study](https://arxiv.org/pdf/2204.06805) likewise concerns large F9 point counts and does not list these plane-sextic equations. The read-only LMFDB `av_fq_isog` query on 2026-11-16 showed that all six represented trace-isogeny classes already contain hyperelliptic equations. `PRIOR_WORK.json` (generated by `prior_work_check.py`) assigns one of these known classes and the published search scope to **each of the 62 candidates**. It marks exact-equation publication priority unresolved. The six represented trace values and their counts are:

| `t` | -26 | -22 | -14 | -10 | 10 | 14 |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| Equations | 9 | 7 | 30 | 2 | 5 | 9 |

The finding is a reproducible finite classification of a specific genus-five carrier stratum, with explicit equations in known isogeny classes. It can support a search for new *carrier geometry* or an explicit correspondence. An equation absent from the bounded paper/database checks is only a potentially new exact equation; it does not establish a new eligible family, a new GHS method, or a vulnerability. The next falsifier is an explicit, efficiently evaluable nonzero subgroup homomorphism from the source elliptic curve to one of these Jacobians, followed by same-target full attack and rho measurements.
