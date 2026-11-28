# Frozen hypothesis: D4/S5 coefficient-span-adapted lattice

Date: 2026-11-18. Class: CONJECTURE; novelty_unverified, previously identified but not tested as a lattice construction. This does not reopen MP-lattices-4's invalid small-x-invariant bound. The Y-coordinate base is mandatory.

## Sources and overlap
- Existing closure: research/ic_mega_pass_20261031/brief/BRIEF.md, report/REPORT.md (generic group wall; D4 is constant-factor at best).
- Direct predecessor: experiments/N2-d4-s5-lattice/{README,AUDIT,build_h.py,lattice.py,rank_test.py}; observed coefficient-family rank 59/105 for varying (A,c,xR), 9/105 at fixed curve varying xR. The audit explicitly leaves the rank-59 *lattice construction* untested. Existing 35-37-bit first-vector shortfall is NOT evidence of causation.
- Crosschecked repo searches for rank-59 S5, D4 corrected x-box, parameter target F4. Literature: Semaev ePrint 2004/031, Petit–Quisquater ASIACRYPT 2012, Semaev complexity bounds (2020). Generic-group non-genericity criterion from closure brief.
- Research-KB and Conductor services not exposed in this runtime. Checked GitHub source/PR overlap; no overlapping PR found for this exact coefficient-span falsifier. Scope limited to this directory, plus docs/ACTIVE.md tracking.

## Hypothesis
For the D4-invariant 4-summand S5 polynomial H(s1,s2,s3,t), with genuine Y_i=(x_i-c)/(x_i+c) integer bounds, there exists a provably root-preserving transformation using the 46 coefficient-family dependencies that reduces the 105-coordinate modular lattice's **median CPU time to produce four independent, integer-vanishing root equations by >=1.20x**, at matched planted roots and resources, versus the fastest unadapted D4 lattice. A rank-59 coefficient family by itself is not a rank-59 ideal lattice: the map Q -> QH is injective. Any row-space compression needs an explicit correct injection or saturation proof.

## Conditional prediction, not a theorem
If one could replace the 105 effective independent modular dimensions by 59 without changing determinant exponents or root obligations, the naive shortest-vector exponent difference would be 104/105 - 58/59 = 46/6195. At p≈2^6144 this equals about 45.62 bits. This is a **heuristic upper motivation only**: no such row-space projection is yet constructed, and 35-37 bits seen in the predecessor were measured against a determinant model, not a matched random/solver control. Previous empirical X reach at p≈2^6144 was between delta 0.00058 and 0.0011; the m=4 IC requirement is delta≈0.169. Even a small constant gain would not cross rho's group-operation cost sqrt(pi*r/2) (adjust for negation, CM or Frobenius folding).

## Cheapest disproof gate
1. Reconstruct H via the predecessor's build_h.py, p=nextprime(2^4096), 4 distinct planted |Y_i| in [2,6], with pairwise different Y_i^2, no denominator poles. Fixed seeds 2026111801..2026111804; target is exactly the sum of those 4 points. No postselection other than documented nondegeneracy.
2. Verify H(root)=0 mod p and check invariance under all 8 even sign flips and S4 on 4 coordinates. Derive the 46 universal linear relations on coefficients using independent training curves; verify them on all 4 holdouts.
3. Construct A: original valid 105-dimensional lattice, B: coefficient-span adapted lattice WITH A ROOT-PRESERVATION PROOF, C: synthetic 105-support D4 polynomial with the SAME planted root and bounds (random coefficients except one solved to enforce zero at that root), D: genuine non-symmetrized G(Y) Y-box with equal root and matched reducer budget. C is NOT a substitute for D; report both.
4. Preregister p, curve parameters, target, Y, all coefficients, basis matrices/hashes, reduced vectors and exact Z-valuations. Full LLL resource cap: 20 CPU-min / arm aggregate; record timeouts and missing arms as missing, not losses.
5. Primary falsifier: if B cannot preserve roots, or its per-root complete solver time fails the >=1.20x criterion on the 4 cells, reject this particular row-space-adapted advantage. Secondary: if its first-three-short-vectors median length is not <= 1/4 of A at equal p/X and both are valid, reject the proposed 2-bit phase shortening. The root equations must yield four independent algebraic constraints (not merely three coprime polynomials in four variables); recover a true signed point tuple and group-law check it before declaring success.
6. Report the strongest matched reference: A is best existing D4-S5 lattice, D the full Y-box at identical solver goal, plus full cold rho for the actual subgroup order r. Charge polynomial construction, misses, duplicate targets, LLL, root extraction, verification, relation rank, LA and descent. In absence of all phases, no total ECDLP speedup claim.

## Executed prerequisite control
A different exact S3 (not S5) toy curve calibration probes whether a coefficient-family rank deficiency creates an advantage after matching symmetry AND a planted root. This does not meet the D4 primary test. Run:
python research/unexplored_d4_coeffspan_20261118/s3_control.py --cases 96 --seed 20261118 --X 4 --output RESULTS-X4.json
python research/unexplored_d4_coeffspan_20261118/s3_control.py --cases 96 --seed 20261118 --X 8 --output RESULTS-X8.json
