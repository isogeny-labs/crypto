# Appendix A: inventory of cover-attack work already in this repository (2026-10-18)

Compiled by a read-only survey agent for the gap search in the parent directory.
Every row cites the directory it was read from. Nothing here is a new measurement.

## 1. Binary and Koblitz covers (GHS / Hess / Weil descent in characteristic 2)

| dir | what | curve / size | verdict | key number | stated open question |
|---|---|---|---|---|---|
| `research/cover_route_prime_binary_20261008` | GHS magic number `m` for every deployed binary curve, plus RFC 2409 Oakley groups 3 and 4 | sect113r1…sect571r1; Koblitz sect*k1; Oakley 3 (2^155), Oakley 4 (2^185) | Negative for deployed curves: Koblitz curves have m=1 (degenerate), random-b curves have m=n. Positive by formula only for Oakley 3/4 | Oakley: m=5, genus 15–16 over GF(2^31) / GF(2^37); corrected to +1.1 and +10.9 bits | "genus-16 hyperelliptic index calculus over F_{2^31}… not started"; "which classes over F_{q^3} have an explicit small-degree cover… remains the live part" |
| `research/jacobian_transfer_scan_20261010`, `docs/curves/JACOBIAN_TRANSFERS.md` | Formula screen over every registry curve and route: GHS, Galbraith–Hess–Smart isogeny extension, same-field covers, base-change descent, `cover_descent` | about 2,100 curves | All deployed prime-field and prime-degree binary curves are boundary. Formula advantage only for Oakley 4 (+10.9), c2pnb368w1 (+10.1), c2pnb176w1 (+23.6), c2pnb272w1 (+69 to +72) | 598 formula_advantage routes; 596 `proposed` (no cover exhibited) | Not enumerated: Hess's generalised Artin–Schreier covers beyond the classical GHS family, pairing transfers, G2 of pairing curves |
| `research/jacobian_transfer_lemma6_20261013`, `docs/audits/GHS_MAGIC_NUMBER_MMT_CORRECTION_20261013.md` | MMT Lemma 6 and the magic-number definition correction (dim span{(1, σ^i √b)}) | X9.62 c2pnb176/272/304w1 | Correction | c2pnb176w1: m=8, genus 128 over GF(4), +23.64 bits | — |
| `research/ghs_counterfactual_n130_20261013` | Counterfactual N=130 instead of 131 | abstract F_{2^130}/F_2 tower | Model-positive only: m=9, type I, genus 255 | 2^55.60 against rho 2^60.31 (−4.71 bits) | cover, norm–conorm map, subgroup preservation and Jacobian order not constructed; cost unit unmeasured |
| `research/notes/ecc2k130/RESEARCH_ECC2K130_HYPERELLIPTIC.md`, `RESEARCH_ECC2K130_CURVE_CONSTRUCTION.md` | Covers carrying the ECC2K-130 subgroup | F_{2^131}, r ≈ 2^129 | GHS genus only 1, 2^130−1 or 2^130. A genus-130 curve carrying A would cost 2^37.17 (−23.6 bits vs rho). Best explicit construction: modular curve X_H(3²·7²·263²), genus 2^28.92. Cyclic covers of genus ≤ 1 bases need genus ≥ 1300 | window: genus 130 to about 290–300 | §6: "Is A isogenous to a Jacobian, by an isogeny anyone can evaluate?" — open |
| `research/notes/ecc2k130/RESEARCH_ECC2K130_ROUTES.md` | Joux–Vitse cover-and-decomposition for ECC2K-130 | F_{2^131} | "Closed — do not reopen": needs a composite tower; embedding into F_{2^131m} gives genus ≈ 2^130 | — | — |
| `research/ghs_curve_comparison_20261009` | `ghs_screen` / `ghs_transport` against rho and BSGS on small models, plus sect113r1/r2 | degrees 7, 9, 13, 15; 113 | Null. Two degree-15 non-Koblitz curves have m=3, genus 3 ("structural candidate"); sect113: m=113 | 80/80 verified | genus-3 degree-15 models "next characteristic-two construction lead" |
| `research/ghs-c2pnb/ghs_poc.py` (legacy Python) | X9.62 c2pnb*w1 magic numbers with simulated Hess shift | c2pnb176–368w1 | Direct m=1; m′=5 instance judged infeasible to find (MMT) | earlier "~2^57 BROKEN" claim withdrawn 2026-10-09 | — |
| `src/cryptanalysis/ghs_full_attack.rs`, `ghs_descent.rs` | GHS pipeline code | toy | m=1 end to end; m=2 symbolic only | — | smooth-model conversion, Diem/Gaudry–Thériault IC, Vélu in char 2, genus-16 case: not implemented |
| `research/weak_isogeny_classes_20261013` | Fraction of binary curves whose isogeny class contains a GHS-weak member; 3-isogeny walk reach | exhaustive, N ≤ 24 | Class closure 1.8–1543× the directly weak set; the walk reaches only part | N=20, n=4, M=3: closure 0.236 vs walk 0.063 | cost of the isogeny beyond 3-steps, genus, Jacobian order, transfer, rho comparison: not established |
| `research/twist_walk_blocked_towers_20261014` | 3-isogeny walk escaping MMT-blocked towers | 13 towers | l=1 never escapes (0/2000); relative-degree towers 34–77% | — | — |
| `research/sect113r1-weak-curve-20261006` | sect113r1 audit | GF(2^113) | GHS row not computed | ord_113(2)=28 | transfer and DLP advantage untested |
| `research/cover_transfer_assessment_20261013`, `research/hyperelliptic_cover_infrastructure_20261006` | Same-field binary genus-3 degree-3 covers `v²+u²v=u⁷+au⁴+du` | 160 maps, 134 EC1 bindings | geometry supported; generator-level transfer passes; arbitrary-class pushforward only for one GF(8) fixture | 486 classes / 236,196 pairs | `problems/binary-cover-arbitrary-jacobian-norm-missing.md` open |
| `koblitz_isogeny_descent_37_*`, `koblitz-isogeny/reports` | Degree-73 isogeny to a different model then Boolean Semaev | 2^37; n = 17, 19, 31 | registered criterion failed (4 of 8 cells) | — | (not a cover) |

## 2. Prime-field curves via extension or isogeny

| dir | what | curve / size | verdict | key number | stated open question |
|---|---|---|---|---|---|
| `research/cover_route_prime_binary_20261008` Part A | F_p has no subfield; base change to F_{p^k} and descent | all deployed F_p curves, pairing G2 | Negative (formula): at best p^{4/3} | — | — |
| `research/p192_extension_cover_20261012` | P-192 over F_{p^6}, F_{p^3}, F_{p^2}: Joux–Vitse family; Type I non-hyperelliptic genus-3 cover (Momose–Chao); Scholten genus-2; compact-base relation probes | NIST P-192; P-224, P-256 conorm classes | Direct JV family excluded for P-192; Type I cover constructed (14-term quartic), norm class of exact order r; every relation probe gives 0 relations | hit probability < 2^-176 per 64 queries | "A scalable P-192 relation method remains the principal implementation gate" |
| `research/notes/cm-isogeny/RESEARCH_P256_ISOGENY_COVER.md`, `docs/guides/WHAT_IS_THE_NN_COVER_ATTACK.md` | (N,N)-split genus-2 Jacobian isogenous to E × E^twist | P-256 and all prime-order F_p curves | Negative (structural): Frob mod 2 = (T²+T+1)² not attained by genus-2 Jacobians (Howe; Maisner–Nart) | density 2^-858 to 2^-181 at 255 bits | Phases 8a–8c and 15 proposed, not built |
| `RESEARCH_AUTOLAB_LOG.md` (B6′), `PAPER_STRUCTURAL_COMPLETENESS.md` | secp256k1: base change then Diem; Howe-glued genus-2; CHLRS | secp256k1 | Negative: Weil-descent circularity | Gaudry IC cost grows with k | Mestre reconstruction, CHLRS blocked |
| `research/notes/cm-isogeny/RESEARCH_MESTRE_HOWE.md` | Explicit Howe-glued cover via Mestre | secp256k1 pairs | Blocked | — | — |
| Same-field covers (`docs/curves/COVERS.md`) | Odd-characteristic `v²=F(u²+c)`, genus 2 | 1,940 maps | Geometry only | −2.0 to −2.1 bits vs rho (formula) | `problems/prime-cover-balanced-jacobian-and-induced-maps-missing.md` |
| `research/isogeny_fibers_20261010` (H6) | Covers through the fibre of an isogeny | — | Negative (Riemann–Hurwitz) | — | falsification run for F_{2^31} curves not reported |

## 3. Hyperelliptic, cubic and quartic covers (odd characteristic)

| dir | what | curve / size | verdict | key number | stated open question |
|---|---|---|---|---|---|
| `research/ghs_cubic_cover_census_20261012` | Minimal GHS genus over F_q for E/F_{q^3} | q = 5, 7 exhaustive; 11 sample | min genus set by #E mod 4: 13 / 5 / 3 | 390/390 with 4∣#E reach genus 3 | trigonal, n = 5, 7, char 2 not covered |
| `research/ghs_cover_census_n5_20261013` | n=3 at larger q; n=5 census | q ≤ 17; n=5: q = 3, 5 | P3 falsified at q=13; n=5 floors 33/33/25/5 | genus 5 at n=5 only with split 2-torsion | exhaustive split census at n=5, q=5 |
| `research/ghs_cubic_genus3_build_20261012` | Explicit m=2 plane-quartic cover | q = 5, 7, 11 | Positive: #Jac(C)(F_q) = #E0(K) | — | — |
| `research/ghs_cubic_hyperelliptic_genus3_20261013` | m=3, r=5 hyperelliptic genus 3; transfer with Cantor | 20 instances, q ≤ 13 | φ injective on the ℓ-part | 6000 checks | conjugate arm not implemented |
| `research/ghs_cubic_quartic_ic_20261012` | IC on the GHS plane quartic | q = 5, 7, 11 | 90/90 correct; no rho ratio | — | cost scaling vs rho |
| `research/ic_next10_20261014/07-cubic-ghs-genus3` | Cubic GHS + genus-3 double-large-prime IC priced end to end | q = 2^8–2^15 | Positive on exponent (toy): IC q^1.30–1.38 vs rho q^1.5; still 1.62× rho model at 2^15; crossover extrapolated 2^17–2^19 (2^25 with LA pricing) | — | plane-quartic branch BLOCKED; conjugate arm not covered |
| `research/odd_char_descent_genus3_20261014` | Momose–Chao, Joux–Vitse, Diem, Smith, Tian; minimum genus n ≤ 14 | p ≤ 31 | Genus 3 only at n = 2, 3; min genus 9 (n=6), 49 (n=10), 41 (n=14) | — | Smith Props 2, 4 unchecked; Tian disagreement unresolved; Diem 2003, Thériault, Diem–Thomé not read |
| `research/cover_descent_genus_floor_20261015` | Re-prices `cover_descent` rows at the GHS floor | k = 6, 10, 14 | Correction | 848/50/1201 → 601/67/1431 | rows stay `proposed` |
| `research/weak_subfield_hyperelliptic_decomposition_20261012` | Bhati–Singh weak genus-2 curves; Kani descent | (7,3)…(7,4) | No: factor base stays in J_C0(F_{q^gcd(2,n)}) | 15/15 collapse | — |
| `research/smith_*` (6 dirs), `problems/smith-kernel-isogeny-class-walk.md` | Smith Prop 2/4 natively; class-level success; sign law | up to 10^6 curves | Reproduces Smith 0.18638; class share 0.43–0.475; ~24% of the gap open | — | explicit hyperelliptic isogenies and walk distance unchecked |
| `research/hyperelliptic_cover_infrastructure_20261006`, `research/bielliptic_quartic_20261005` | Xarles construction audit; bielliptic quartic norm projection | small p | refuted as full-quartic relation matrix | — | 10 open obligations |
| `research/weak_curve_conjectures_20261012` | 7 open conjectures on class-support laws | p=7; 192–252-bit | Open | — | WCQ1 entries |

## 4. Joux–Vitse

| dir | what | curve / size | verdict | key number | stated open question |
|---|---|---|---|---|---|
| `research/notes/index-calculus/RESEARCH_COVER_DECOMPOSITION_LEDGER.md`, `src/cryptanalysis/jv_cover.rs`, `jv_sieve.rs`, `jv_isogeny_walk.rs` | JV cover + Nagao 6-point decomposition and sieve on E(F_{p^6}) → genus-3 over F_{p^2} | p up to 1823 (ℓ ≈ 2^63) | POSITIVE on the weak class (the only measured below-rho transfer in the repo); reproduction of a published route | S/rho ∝ p^-1.92; with sieve crossover at p ≈ 430; 0.003–0.005× rho at p ≈ 1800 | deg h = 2 not built; even characteristic not started; k ≥ 3 unmeasurable |
| `research/jv_cover_e2e_30bit_20261009` | Full route from a non-weak curve | p = 251…1511 | Positive at p ≥ 1009: 0.058 and 0.011 of pooled rho | — | pooled rho not a matched run |
| `research/jv_weak_class_law_20261007` | Which F_{p^6} classes contain a weak curve | toy p | depth-1 never; 86–95% of depth ≥ 2 | — | — |
| `research/jv_weak_generalization_20261008` | Genus-3 covers of any degree | q = 13 | 95% of traces have some genus-3 cover | — | degree-3/4 covers constructed; Tian (ℓ,ℓ,ℓ) ℓ ≤ 7 |
| `research/iso1_weak_classes_20261007` | Norm-one conductor theorem; trace census | p = 7…113 | Necessary, not sufficient | — | p = 53–200 census, sufficient criterion |
| `research/joux_vitse_n1_20261015` | (n−1)-decomposition at n=5 | q = 11, 31 | No: 4.5·10^7–1.8·10^8× rho | crossover q ≈ 2^58.2 | n = 6 not attempted |

## 5. Trace-zero / Weil restriction

| item | what | verdict |
|---|---|---|
| `RESEARCH_ECC2K130_HYPERELLIPTIC.md` §6 | A = 130-dim trace-zero part of Res E. Isogenous to a Jacobian? | Open; genus-130 IC would cost 2^37.17 |
| `research/ic_next10_20261014/10-torelli-toy` | Torelli rarity heuristic on A_n | n=5 negative (low power); n=7 carriers at genus 7 (1,536 models), apparently GHS m=4 structure; heuristic under-predicts structured classes by up to 2^19 |
| `research/ic_next10_20261014/08-quasi-subfield` | Trace-zero hyperplane pre-filter | ~50% refuted at degree 2; not costed end to end |
| `research/isogeny_fibers_20261010/parts/p6b_hyp.md` H8 | Gorla–Massierer trace-zero | negative by argument; run not reported |

## 6. Other related work

- Gaudry decomposition over F_{p^4} with D₄ symmetry (`gaudry_quartic_d4_*`): C₄ cut 48× to 481×; crossover extrapolated 2^68 (char 2) to 2^135.
- `nagao_relations`: engineering diagnostics only.
- `cd_attack`: Castryck–Decru SIDH reconstruction; not ECDLP.
- `src/cryptanalysis/diem_descent.rs`: toy Diem.

## 7. Explicitly stated gaps found in the repository (cover side)

1. Genus-16 hyperelliptic IC over F_{2^31} for Oakley group 3: not started.
2. Which classes over F_{q^3} have an explicit small-degree cover: "the live part of the cover line".
3. N=130 counterfactual: cover, norm–conorm, subgroup preservation, Jacobian order not constructed; cost unit unmeasured.
4. Genus-3 degree-15 binary models: construction lead; transport, certificate, end-to-end IC open.
5. `ghs_full_attack.rs`: m=2 smooth model, Diem/Gaudry–Thériault IC, Vélu in char 2, genus-16 case: not implemented.
6. `JACOBIAN_TRANSFERS.md`: Hess's generalised Artin–Schreier covers beyond classical GHS not enumerated; isogeny-extension rows never exhibit an isogenous curve.
7. Weak isogeny classes: cost of the isogeny to the weak member beyond 3-steps, genus, transfer, rho comparison not established.
8. ECC2K-130 §6 open: is A isogenous to a Jacobian; class number of the degree-260 CM field not computed.
9. Toy ladder at n=5 for non-hyperelliptic genus 5 and genus ≥ 6: not done (partly done in `ic_next10/10`).
10. Even-characteristic Joux–Vitse cover construction: not started.
11. Threads 4 (trace-zero × Frobenius) and 10 (partial Weil descent n=ab) of the Koblitz queue: no completion records.
12. P-192 extension cover: scalable relation method is the principal gate; Type I branch needs a relation method.
13. P-256 (N,N)-cover phases 8a–8c, 15 (genus-3 Jacobian arithmetic): future work.
14. secp256k1 (N,N)-split cover "genuinely open" for j = 0.
15. Non-GHS constructions and (ℓ,…,ℓ)-isogenies (Tian) not ruled out for k = 6, 10, 14.
16. Cubic census: trigonal, n = 5, 7, characteristic 2 not covered.
17. `ic_next10/07`: plane-quartic (Diem) branch blocked; conjugate arm not covered; no crossover measured.
18. Smith: Props 2 and 4 unchecked on the family; explicit isogenies and walk distance unchecked; ~24% of the gap open.
19. Joux–Vitse: deg h = 2; k ≥ 3; even characteristic; necessary-and-sufficient weak-class formula; n = 6 (20 GB).
20. Trace-zero: Gorla–Massierer yield comparison not run; trace-zero pre-filter not costed.
21. Gaudry D₄: F4 with weighted sugar, Wiedemann on Macaulay, Magma comparison, double large primes stacked arm: not run.
