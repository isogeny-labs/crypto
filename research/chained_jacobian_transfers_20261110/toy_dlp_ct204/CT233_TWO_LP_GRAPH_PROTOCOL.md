# CT233 — preregistered two-large-prime graph with exact odd-characteristic elimination

Date: 2026-11-19 UTC. Existing CT204 draft lane: crypto PR #4715. Status **protocol only**, [pcc:unverified]. This document is committed **before** writing the native implementation or collecting native CT233 measurements.

## Source, literature, and non-duplication

Reuse the unchanged F101 cubic cover and its verified native arithmetic from CT226 and CT231; keep 54-variable control and CT232 negatives. Check crypto #4587's already implemented *generic HEC* weighted 2LP (do not recopy it), #4693's *prime-order genus-4 Jacobian* cost benchmark (different group), and ongoing #4697 transfer infrastructure. Classical double-large-prime variants are established: Gaudry–Thomé–Thériault–Diem (Math. Comp. 76, 2007; DOI 10.1090/S0025-5718-06-01900-4). KB MCP unavailable in this runtime; do not claim novelty. Unique question here is whether true two-LP support after the exact *source elliptic norm* improves the complete cold composite-order solve.

## Frozen model and experiment

* Curve C/F101: y²=u⁹+u³+46; E/F101: y²=x³+x+46. Source generator G=(2,37), subgroup order 109. Public targets Q37=(64,19) and **fresh fixed Q73=(19,37)**; the numerical answers are reserved for post-solve verification, not supplied to the solver's relation, matrix, or recovery stages.
* Signed lexicographic prefix bases **24,36,48,54** of the same full 54-point rational roster. Same 60 ordered offsets and k=0..108 (≤6,540 candidates per size); stop relation collection at full B-rank. Control at B=54 MUST be (rank, full, candidates)=(54,62,1421). Failing smaller arms remain failures without made-up solving ratios.
* Accept only reduced degree-four *squarefree fully rational* supports with 0, 1 or 2 distinct out-of-base signed roots; discard supports with 3–4 outside roots and all nonsplit/lower-degree cases. Retain all observations, including failures, discarded classes, topology, and memory.
* Represent a relation as r_B·l_B + s_i·l_i [+s_j·l_j]=3k mod 109, with all fixed-offset coordinates subtracted from r_B; s_i,s_j in {−1,+1}. One-LP relations are edges to a sentinel node l_0=0; two-LP are genuine weighted edges. Record edge count, graph components/cycles (topology diagnostic only).
* **Do not assume every graph cycle yields a B-only relation.** In odd characteristic a signed triangle with rows l_1+l_2, l_2+l_3, l_3+l_1 has determinant 2, not 0. Weighted elimination must distinguish dependencies from full-rank LP equations. Use exact F109 LP pivot elimination with each pivot retaining its B vector and RHS; feed to the original B eliminator only when *all LP coefficients are zero*. This is equivalent to a sparse graph-based Schur complement, avoids false relations and does not count an LP pivot as B rank.
* At full B-rank, back-substitute **only uniquely recoverable** LP logs from the triangular LP system; no free variable is silently assigned an asserted logarithm. Verify every recovered B and LP log by an independent elliptic scalar multiplication. Verify each input relation for which all involved variables are determined; if a relation has an undetermined LP, retain it explicitly instead of treating it as verified.
* Target recovery scans the same capped offset schedule and source shifts on Q37 and Q73, requiring 0/1/2 out-of-base roots and all corresponding LP logs uniquely known. Independently verify [a]G=Q for every reported answer. No precomputed source DLP enters the solver. Record target failures per target.
* Per arm, measure cold setup/transfer, relation collection including graph/pivots, B linear algebra, independent log verification, target recovery and all attempted work. Report actual #Cantor adds, LP memory upper bound/peak logical allocation, and process-wide VmHWM when readable; report missing fields explicitly. Alternate inline same-run direct BSGS ordering; do not infer calibrated rho/frontier IC, uncertainty intervals or PCC from these small tests.

## Correctness gates and cheap falsifiers

1. Synthetic F109 weighted triangle test: the three all-plus edges have LP matrix determinant 2 mod 109 and MUST **not** emit a fake B-only cycle relation. A sign-balanced triangle that is LP-dependent MUST emit the exact B row/RHS, including offset signs.
2. One-LP parallel-edge test reproduces CT231 cancellation algebra for all sign patterns, with direct substitution and nonzero residual mutation controls.
3. If the B=54 control drifts, fail the whole run. Every completed target must match its public elliptic target, and every solved norm log must match E/F101. A rank-deficient or target-failed arm has no complete-solve ratio.
4. If no reduced arm beats full-base CT231 control on the same paired host, record the negative result and move to different cover/collector. Compare against optimized rho, BSGS and frontier IC **only after all methods are actually run or appropriate measured, matched records are available**. Do not mix runner timings across days.
5. No paid EKS/Spot, S3 credential probing, protected merge, ledger rewrite or PCC signature assertion. Original 695/701 authenticated norm-reuse and physical C169/C13a/C13b/N2_actual/D pricing remain separate gated tasks.

## Reproduction target (after implementation and native CI)

```sh
cargo test --locked --release --test ct226_native_genus4_transferred_ic ct233_ -- --nocapture
```

Expected output: exactly 4 structured cold native CT233 arm records, 2 target dispositions per arm, and explicit failures. Repeated CI grep echoes do not count as independent runs.
