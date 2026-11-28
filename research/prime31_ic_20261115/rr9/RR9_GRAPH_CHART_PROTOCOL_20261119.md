# Preregistered experiment: Prime31 RR9 Grassmann graph-chart contraction
Date: 2026-11-19 UTC. Draft PR #4780, stacked on #4776. Before candidate code and measurements. [pcc:unverified].

## Scope
Original unchanged ordinary cofactor-one E/Fp, p=2147483647, a=847514846, b=2087444724, n=2147446831, G=(625037715,2100627251). Reuse the original Rust Arithmetic via existing `research/prime31_ic_20261115/native/examples/prime31_rr9_exterior4.rs`, pinned arithmetic 1df714645aa683cf975bc76fcacb9842cb4780fd. Candidate receives public target R, public first-four-x B4 signed supports, and no secret label. Retain weights 1,3,5,7,11,13,17,19. Do not use EC group additions, scalar multiplication or EC group-sum dictionaries inside the algebraic candidate. Independent grader may do so out of band.

## Exact algebraic hypothesis
For four RR9 rows H_i=[A_i|B_i], each 4x4, with det(A_L),det(A_R) nonzero, set M_L=A_L^{-1}B_L and M_R=A_R^{-1}B_R. Then
 det([A_L B_L; A_R B_R]) = det(A_L)det(A_R)det(M_R-M_L).
In the canonical RR9 rows A_i=(x_i-r)[1,x_i,x_i²,x_i³], B_i=(y_i+s)[1,x_i,x_i²,x_i³]. For four distinct x_i, det(A) = product_i(x_i-r) * Vandermonde(x_0,...,x_3), so the graph chart is valid iff all x_i != r. The test is exact only on these admissible chart fibres; any singular chart is a declared failure, not a false no-relation.

Compute det(M_R-M_L) by 2+2 Laplace expansion: six signed products of two 2x2 minors, or exactly (6*2*2 + 6) = **30 finite-field multiplications per pair**, compared with 70 for the previous 70-coordinate Plucker contraction. Both scan all half-pair assignments, and the graph method is *nonlinear in 16 stored coordinates*; this does not contradict a full 70-dimensional linear span in PR #4779.

## Frozen tests before seeing results
- Original-curve B4 planted all-zero signed word and natural R=G exhaustive no-hit target, same as #4776.
- Synthetic 8x8 matrix exact determinant identity, if left and right first-four-column blocks invertible; original-curve sample pairs exact residues, not only zero/nonzero.
- Run full B4 scan if 4096 half-graphs per side succeed, with all 16,777,216 pair checks on a no-hit.
- Independently replay the original Python BigInt grader `verify_prime31_rr9_exterior4.py` on the candidate output with an equivalent JSON contract. External grader is not included in candidate elapsed.
- Compare same-runner B4 graph chart and previous #4776 exterior-4 example, including setup, per-query chart precomputation, full scan, total finite-field operation counts and both memory footprints.
- Do not run naive B8 exhaustive search unless a **new** subquadratic selector appears. Arithmetic B8 4,294,967,296 pairs and 128,849,018,880 candidate products alone; 1024 full scans are not competitive.

## Stop/go / inference limits
A successful reduction of component operations or memory does **not** establish a faster complete non-generic ECDLP solver: both algorithms still scan B^8 pairs; chart construction includes inversions. If verified, deliver it as algebraic primitive for PR #4692 only if a nonenumerative matching algorithm can use it. Advance to the four-window rank31 and 1024-target Rust pipeline of #4728 only after sealed natural B8 46-positive/18-negative public-only unknown-index extraction succeeds; compare fully charged costs on the same runner to optimized shared BSGS, mixed IC, and rho.

## Related work checked
RR9 classical Hermite #4466, public-only exterior-4 #4776, target-only F4 owner #4692, RR9 formal and signed-determinant #4728, gradient #4772, prefix2 #4773, matched shared-BSGS #4774 and contemporaneous Plucker linear-span #4779. Autoresearcher SA-04 and SA-18 negative determinant/norm lanes. Published prime-field index calculus: Petit–Kosters–Messeng, PKC 2016, DOI 10.1007/978-3-662-49387-8_1; Amadori–Pintore–Sala, FFA 2018, DOI 10.1016/j.ffa.2018.01.009; Kudo et al. 2018 on hybrid elimination and symmetry. Shared research KB unavailable in this runtime; bounded code/PR search is not an exhaustive novelty audit. Graph-chart determinant identity is elementary Schur complement, no claim of original theorem.
