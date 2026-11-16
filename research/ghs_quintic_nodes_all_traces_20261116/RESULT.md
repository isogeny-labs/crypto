# All-trace census of five-node plane-sextic genus-five carriers over F3

## Claim boundary

This is an exhaustive census **inside one specified plane-sextic stratum and four-count filter**, not a classification of all genus-five curves or all curves vulnerable to GHS. It gives 19 elliptic-source-eligible Weil classes with explicit genus-five Jacobian representatives absent from the LMFDB model snapshot of 2026-11-16. An explicit evaluable subgroup transfer and complete GHS-versus-rho measurement remain open. Accordingly, these are candidate new class realizations, not established new vulnerabilities.

## Exact finite search and bounds

The two PGL3(F3) node-position orbits are the complete degree-five-orbit configuration with no four collinear points, as certified in the preceding quintic-node search. For each position, the sextic linear system has dimension 13 over F3, hence exactly (3^13-1)/(3-1)=797161 projective coefficient vectors. The earlier full enumeration retained 947 and 1024 equations satisfying N1=4, N2=10, N3=28, N4=82. This frozen classifier examined all 1971 rows: 850 genus-five carriers, 1008 of different genus, 61 with a nonordinary intended node, and 52 outside the genus-five trace bound. This count is of searched equations, not isomorphism classes.

For a genus-five curve C/F3 to have the Weil polynomial of Res_{F243/F3}(E), its first four counts must be 3^r+1 and its fifth count must satisfy `N5(C)=244-5t`, where `t=244-#E(F243)`. Hasse gives the exact integer bound `|t|<=31`. Waterhouse’s elliptic trace criterion specialized to F243 leaves `3∤t` or `t in {0,±27}`. The classifier found 50 trace classes; 35 pass this source-existence filter. Of these, 19 have zero stored curve models in the dated LMFDB query. The 19 classes contain 236 of the 850 searched equations. The other 378 source-eligible equations lie in 16 classes with recorded models.

The direct replay independently evaluated all monomials over F3, F9, F27, F81, and F243 for one representative of each of the 19 classes, checked five ordinary nodes and the split correction at degree five, and recomputed the normalized counts. Sage/Singular checked geometric genus five. Factoring all 850 equations over F729 rules out the six-conjugate-line case omitted by the earlier F9/F27 factor checks. `TraceLedger.lean` checked the finite trace, order, and fifth-count arithmetic with Lean 4.35.0; it does not formalize genus, point counting, Waterhouse, or Tate.

## Candidate classes and exact elliptic sources

The field is `F243=F3[u]/(u^5+2u+1)`. In the table, ordinary source equations are `y^2=x^3+a2*x^2+a6`; for trace -27 the source is `y^2=x^3+2*x+u^4`. The full sextic equations appear below.

| trace t | equations in stratum | #E(F243)=#J(F3) | LMFDB class | a2 | a6 | sextic case:index |
|---:|---:|---:|---|---|---|---|
| -29 | 5 | 273 | 5.3.a_a_a_a_bd | `1` | `u^3 + 2*u^2 + u + 1` | 0:780 |
| -28 | 18 | 272 | 5.3.a_a_a_a_bc | `2` | `u^3 + u^2 + u` | 1:151 |
| -27 | 47 | 271 | 5.3.a_a_a_a_bb | `0` | `u^4` | 1:704 |
| -25 | 23 | 269 | 5.3.a_a_a_a_z | `2` | `u^4 + 2*u` | 0:582 |
| -23 | 11 | 267 | 5.3.a_a_a_a_x | `1` | `u^3 + u + 2` | 1:568 |
| -19 | 20 | 263 | 5.3.a_a_a_a_t | `2` | `u^4 + 2*u^3 + 2*u + 1` | 0:16 |
| -17 | 8 | 261 | 5.3.a_a_a_a_r | `1` | `u^3 + u^2` | 0:887 |
| -13 | 3 | 257 | 5.3.a_a_a_a_n | `2` | `u^4 + 2*u^3 + u^2 + u + 2` | 0:481 |
| -11 | 35 | 255 | 5.3.a_a_a_a_l | `1` | `u^4 + u^3 + u + 2` | 1:111 |
| -7 | 7 | 251 | 5.3.a_a_a_a_h | `2` | `u^4 + 2*u^3 + u^2 + 2*u` | 1:193 |
| -5 | 9 | 249 | 5.3.a_a_a_a_f | `1` | `2*u^2 + 2*u + 2` | 0:226 |
| -1 | 17 | 245 | 5.3.a_a_a_a_b | `2` | `2*u^4 + 2*u^2 + u + 2` | 1:789 |
| 1 | 18 | 243 | 5.3.a_a_a_a_ab | `1` | `u` | 0:327 |
| 7 | 6 | 237 | 5.3.a_a_a_a_ah | `1` | `u^4 + u^2 + 2*u + 2` | 1:401 |
| 13 | 1 | 231 | 5.3.a_a_a_a_an | `1` | `2*u^2 + u + 2` | 0:446 |
| 17 | 5 | 227 | 5.3.a_a_a_a_ar | `2` | `u^4 + u^3 + u + 1` | 1:937 |
| 19 | 1 | 225 | 5.3.a_a_a_a_at | `1` | `u^3 + u^2 + u` | 1:279 |
| 23 | 1 | 221 | 5.3.a_a_a_a_ax | `2` | `u^4 + 2*u^3 + 2*u + 2` | 1:553 |
| 28 | 1 | 216 | 5.3.a_a_a_a_abc | `1` | `u^3 + 2*u^2` | 1:801 |

## Exact plane sextics

For each trace, `C` is the normalization of the displayed homogeneous plane sextic over F3. The row `case:index` identifies its full machine receipt in `ANALYSIS.json` and `VERIFIED_UNMODELED.json`. Coefficients `-1` mean `2` in F3.

- **t=-29, case 0 index 780**: `x^6 - x^4*y^2 + x^2*y^4 + x*y^5 + x^5*z + x^4*y*z + x*y^4*z - x^3*y*z^2 - y^4*z^2 + x^3*z^3 - x^2*y*z^3 - x*y^2*z^3 + y^3*z^3 - x^2*z^4 - x*y*z^4 + x*z^5 - y*z^5 + z^6 = 0`; normalized counts `[4, 10, 28, 82, 389]`.
- **t=-28, case 1 index 151**: `-x^6 - x^5*y + x^2*y^4 - x*y^5 + y^6 - x^5*z + x*y^4*z + x^3*y*z^2 + x*y^3*z^2 + y^4*z^2 + x^3*z^3 - y^3*z^3 + x^2*z^4 + y^2*z^4 + x*z^5 - y*z^5 + z^6 = 0`; normalized counts `[4, 10, 28, 82, 384]`.
- **t=-27, case 1 index 704**: `x^6 - x^2*y^4 + x*y^5 - y^5*z - x^3*y*z^2 - x^2*y^2*z^2 + x*y^3*z^2 - x^3*z^3 + x^2*z^4 + x*z^5 + y*z^5 = 0`; normalized counts `[4, 10, 28, 82, 379]`.
- **t=-25, case 0 index 582**: `-x^5*y + x^4*y^2 + x^2*y^4 - x*y^5 + y^6 + x^4*z^2 + x^2*y^2*z^2 - x*y^3*z^2 - y^4*z^2 + x^2*z^4 - x*z^5 - y*z^5 + z^6 = 0`; normalized counts `[4, 10, 28, 82, 369]`.
- **t=-23, case 1 index 568**: `-x^6 + x^3*y^3 - x^2*y^4 + x^5*z - x*y^4*z - y^5*z - x^4*z^2 - x^3*y*z^2 + x^2*y^2*z^2 + y^4*z^2 - x^3*z^3 + x*y^2*z^3 - x^2*z^4 - x*y*z^4 + z^6 = 0`; normalized counts `[4, 10, 28, 82, 359]`.
- **t=-19, case 0 index 16**: `-x^6 - x^4*y^2 + x^3*y^3 + y^6 + x^5*z - x^4*y*z + x^2*y^3*z - x*y^4*z + x^4*z^2 + x^3*y*z^2 + x^3*z^3 + x*y^2*z^3 + y*z^5 = 0`; normalized counts `[4, 10, 28, 82, 339]`.
- **t=-17, case 0 index 887**: `x^6 + x^5*y + x^2*y^4 - x*y^5 + y^6 + x^4*y*z + x^2*y^3*z - x*y^4*z + y^5*z + x*y^3*z^2 + y^4*z^2 - x^2*z^4 + x*y*z^4 - y^2*z^4 + x*z^5 + z^6 = 0`; normalized counts `[4, 10, 28, 82, 329]`.
- **t=-13, case 0 index 481**: `-x^5*y - x^3*y^3 + x*y^5 - y^6 + x^4*y*z + x^2*y^3*z - x*y^4*z + y^5*z + x^3*y*z^2 - x*y^3*z^2 + x^3*z^3 + y^3*z^3 + x^2*z^4 + x*z^5 - y*z^5 + z^6 = 0`; normalized counts `[4, 10, 28, 82, 309]`.
- **t=-11, case 1 index 111**: `x^6 - x^5*y + x^4*y^2 - x^3*y^3 - x^2*y^4 - x*y^5 - y^6 + x^5*z + x^4*z^2 - x*y^3*z^2 - y^4*z^2 - x*y^2*z^3 + y^3*z^3 - x^2*z^4 - x*y*z^4 - x*z^5 + z^6 = 0`; normalized counts `[4, 10, 28, 82, 299]`.
- **t=-7, case 1 index 193**: `-x^5*y + x^2*y^4 - x*y^5 - y^6 - x^5*z + x^3*y^2*z + x*y^4*z + x*y^3*z^2 + y^4*z^2 - x^3*z^3 + x^2*y*z^3 - x*y^2*z^3 - x*y*z^4 + x*z^5 + y*z^5 + z^6 = 0`; normalized counts `[4, 10, 28, 82, 279]`.
- **t=-5, case 0 index 226**: `x^6 + x^5*y - x^4*y^2 + x^3*y^3 - x*y^5 + y^6 - x^5*z - x^3*y^2*z + x*y^4*z - x^4*z^2 + x^2*y*z^3 - x*y*z^4 - x*z^5 = 0`; normalized counts `[4, 10, 28, 82, 269]`.
- **t=-1, case 1 index 789**: `x^6 - x^5*y - x^4*y^2 - x*y^5 + y^6 - x^5*z - x^3*y^2*z + x*y^4*z - y^5*z + x^3*y*z^2 - x^2*y^2*z^2 + y^4*z^2 - x^2*y*z^3 + x^2*z^4 + x*y*z^4 + y*z^5 = 0`; normalized counts `[4, 10, 28, 82, 249]`.
- **t=1, case 0 index 327**: `x^6 + x^5*y - x^2*y^4 + y^6 + x^5*z - x^3*y^2*z - x*y^4*z + x^4*z^2 + x^2*y^2*z^2 + x^2*y*z^3 + x*y^2*z^3 - y^3*z^3 + x^2*z^4 - y^2*z^4 - y*z^5 + z^6 = 0`; normalized counts `[4, 10, 28, 82, 239]`.
- **t=7, case 1 index 401**: `x^5*y + x^4*y^2 - x^3*y^3 + x*y^5 + y^6 - x^5*z + x^4*y*z - x^2*y^3*z + y^5*z + x^4*z^2 + x^3*y*z^2 + x^2*y^2*z^2 + x*y^3*z^2 - x*y^2*z^3 + x^2*z^4 + x*y*z^4 + z^6 = 0`; normalized counts `[4, 10, 28, 82, 209]`.
- **t=13, case 0 index 446**: `x^6 + x^5*y + x^4*y^2 - x^3*y^3 + x*y^5 - y^6 + x^5*z - x^2*y^3*z - x*y^4*z + y^5*z - x^4*z^2 + x^3*y*z^2 + x^2*y^2*z^2 - x^2*y*z^3 - x*y^2*z^3 - y^3*z^3 + x^2*z^4 + x*y*z^4 - y^2*z^4 - x*z^5 + y*z^5 = 0`; normalized counts `[4, 10, 28, 82, 179]`.
- **t=17, case 1 index 937**: `-x^6 + x^5*y + x^4*y^2 - x^2*y^4 + x*y^5 - x^3*y^2*z - x*y^4*z + y^5*z - x^2*y^2*z^2 + y^4*z^2 + x^3*z^3 - x*y^2*z^3 - y^3*z^3 - x^2*z^4 - x*y*z^4 - y^2*z^4 - x*z^5 + y*z^5 + z^6 = 0`; normalized counts `[4, 10, 28, 82, 159]`.
- **t=19, case 1 index 279**: `-x^5*y + x^4*y^2 + x^3*y^3 - x^2*y^4 - x*y^5 - x^5*z + x^4*y*z + x^3*y^2*z - x*y^4*z - y^5*z - x^3*y*z^2 + x*y^3*z^2 - y^4*z^2 - x^3*z^3 - x^2*y*z^3 + x*y^2*z^3 - y^3*z^3 - x^2*z^4 + x*y*z^4 + y^2*z^4 - x*z^5 = 0`; normalized counts `[4, 10, 28, 82, 149]`.
- **t=23, case 1 index 553**: `-x^5*y + x^4*y^2 - x^3*y^3 + x^2*y^4 - y^6 + x^4*y*z - x^3*y^2*z - x*y^4*z + y^5*z - x^4*z^2 + x^3*y*z^2 + x^2*y^2*z^2 + y^4*z^2 + x^3*z^3 + x^2*y*z^3 - y^3*z^3 + x^2*z^4 + x*y*z^4 - x*z^5 - y*z^5 + z^6 = 0`; normalized counts `[4, 10, 28, 82, 129]`.
- **t=28, case 1 index 801**: `x^6 - x^5*y + x^4*y^2 + x^3*y^3 + y^6 - x^5*z + x^2*y^3*z + x*y^4*z + x^4*z^2 - x^3*y*z^2 - x^2*y^2*z^2 + y^4*z^2 - x^3*z^3 + x*y^2*z^3 + x*z^5 = 0`; normalized counts `[4, 10, 28, 82, 104]`.

## Prior work and novelty

Kudo–Harashita (2023) introduced the five-node plane-sextic parameterization, including the two degree-five node configurations, but their reported search targeted at least 32 F9 points; these sparse classes all have N2=10. Their 2022 hyperelliptic/trigonal study also addressed high F9 point counts. Thus the geometric construction and broad family are known, while the exact low-point equations and the 19 catalog-absent Jacobian class realizations are plausible new results. The LMFDB `model_count=0` flag means that its snapshot has no stored curve equation; it does not establish publication priority or prove that the class has no Jacobian. A per-equation publication search and isomorphism comparison remain open. `ANALYSIS.json` includes a `candidate_prior_work_checks` row for every one of the 850 carriers and marks exact-equation priority unresolved.

The class equality `L_C(T)=1-t*T^5+243*T^10=L_{Res E}(T)` implies an isogeny by Tate’s theorem, but it does not supply an evaluable map on the chosen rational subgroup. No vulnerability or speedup is claimed. The strongest prime-order candidate here is t=-19: `#E(F243)=#J(C)(F3)=263`, with an exact source point and 7P in `TRACE_MINUS19.json`. Its rho reference is on a group of order 263; the entire transfer, decomposition, linear algebra, descent, and verification cost must be measured before any attack comparison.

## Primary sources

- Kudo and Harashita, *Representation of non-special curves of genus 5 as plane sextic curves* (2023): https://arxiv.org/pdf/2102.07270
- Kudo and Harashita, *Genus-five hyperelliptic or trigonal curves with many rational points* (2022): https://arxiv.org/pdf/2204.06805
- Waterhouse criterion as restated in Agathocleous, Joux and Taufer, Theorem 2.1 (2026): https://pure.mpg.de/pubman/item/item_3596867_6/component/file_3655796/Agathocleous-Joux-Taufer_Elliptic%2Bcurves%2Bover%2BHasse%2Bpairs_2026.pdf
- LMFDB finite-abelian-variety data queried 2026-11-16: exact SQL and rows in `LMFDB_SNAPSHOT.json`.

## Reproduction

The frozen full run is `sage -python classify.py FULL.jsonl`. The derived checks are `python3 analyze.py > ANALYSIS.json`, `sage -python verify_unmodeled.py > VERIFIED_UNMODELED.json`, `sage -python verify_absolute.py > VERIFIED_ABSOLUTE.json`, `sage -python build_elliptic_sources.py > ELLIPTIC_SOURCES.json`, and `lean TraceLedger.lean`. Hashes linking scripts and inputs appear in the receipts.
