# Exact fixed-sum and adaptive pair cells

The fixed-sum hypothesis passed its declared calibration. On GF(2^83), d18, 1237 positive-dimensional cells were certified empty among 1800 configurations. All 100440 point-fiber calls recovered complete ordered pair sets. The corpus has 75144 distinct (p,s) points, of which 67626 belong to certified empty cells, and 11 distinct recovered (x,y,s) tuples. Overlapping configurations are not added as coverage.

Adaptive directions change the tested assumption: instead of arbitrary low-coordinate directions, construct the nullspace of the point certificate. Registered exposed-training replication produced 16 empty affine cells, each of dimension 65 (maximum 65). Each cell contains 2^65 auxiliary values for its fixed p, proved by a full basis and dual replay. Their volumes are not a union. Even their volume sum is at most2^69, a2^-32 fraction of the full2^101 (p,s) grid; this is an upper bound, not observed global coverage. The independent exhaustive small oracle checked 16924 cell members across 1275 certificates over GF8/GF16/GF32/GF128.

## Output contract and proof

For p=x+y in D, q_i=e_i^2+p e_i and S3=0 is exactly sum x_i(q_i^2+s q_i)=1+s^2 p^2. Recovering the full binary kernel recovers every ordered pair, including zero and repeated sources. An empty point dual lambda defines V by lambda(v q_i)=0 and lambda(v^2 p^2)=0. Thus the same lambda certifies every s in s0+V empty. The constraint rank is at most d, including p0, so dim V>=n-d=65. See the native sources and PROOF.md for exact hypotheses and bounds. Odd characteristic has a separate two-quadratic chart, tested only on small fields; the binary proof does not transfer to odd fields.

## Evidence boundaries

The unconditioned predecessor had zero positive-dimensional certificates. The changed guarded chord-trace hard run returned UNKNOWN with zero witnesses; its raw result is retained. The new cell results certify the declared first-pair components, not the original339-variable five-source hard system. The full outer grid is2^101 (p,s) pairs. A universal controller, all requested prime/extension and binary/Koblitz sizes, matched genuine full all-roots F4/F5/F6 baselines, novelty review and the immutable performance gate remain open. No qualified 2x,5x or8x end-to-end gain is claimed. Times in REPORT.json are single cold supervisor diagnostics on a shared host, including process setup and serialization; no A/A or isolation qualification. Training was exposed, not a holdout.

## Next bounded experiment

Deduplicate/intersect the emitted adaptive cells at each p and count the exact union by affine-subspace methods. Freeze a disjoint residual partition with a hard state/output cap; compare every small-field residual against exhaustive literal S3. Then test a coverage-preserving controller on partial original-chain states, paying all pair sums, other auxiliaries and final verification. Do not rerun the unchanged failed hard formula. Implement the separate odd two-quadratic chart over the NIST fields next; do not call the small odd checks full-width support.

## Requirements

| Requirement | Status |
|---|---|
| Exact declared binary pair fibers and empty cells | Verified calibration |
| Complete339-coordinate original hard roots | Unresolved |
| All prime, extension, binary and Koblitz families/sizes | Partial; wide new component only GF(2^83) |
| Genuine matched F4/F5/F6,20 hard inputs,3 cold repetitions | Not completed |
| Qualified>=2x floor; requested stronger gains | Not established |
| Novel algorithm priority | Not established |
| Research PR and paper addendum | This publication; no merge |
