# C132: exact precision-filtration test; a narrower next direction

Hypothesis: a parameter direction preserving all coefficients below precision n while toggling the current Sylvester row gives an exact unit derivative of a necessary Hensel correction. Pairing theta with theta+v then yields one zero-correction representative per pair. Reconstruct the original selector before final product and original-equation checks; this is not an all-roots solver by itself. PROOF_AND_REVIEW.md proves the conditional bijection and reviews its limits.

Actual evidence:24 independent small affine families,72 exhaustive rank oracles,224 unit-identity checks,112 selector-reconstruction checks and12 positive certificates passed. A rank-equality counterexample passed: F=Z^2(Z+1)^3+x*u*Z has second correction x^2=x, despite no direction preserving its lower coefficients. Thus failure of this sufficient test cannot establish absence of all unit derivatives.68 directional controls passed in17 coefficient fields, including the requested five NIST primes and GF(2^83); these are component controls, not full original-system coverage.

On the frozen actual63-bit k3 chart, the lower-coefficient map has rank63 already at precision18 and remains rank63 through25. All eight sufficient tests have zero-dimensional kernels and no witness. No selector was eliminated and no hard solve was repeated. Supervisor wall time and peak process RSS:

{
  "status": "ok",
  "wall_seconds": 0.8089514930034056,
  "peak_process_rss_kib": 751388
}

What changed: whole-polynomial ZDD construction previously reached work caps. This cycle tested an exact linear-algebra route without expanding those polynomials and ruled it out on this chart. The full lower-rank ladder is retained: rank60 at precision17 leaves a3-dimensional delayed-perturbation space, whereas rank63 at18 leaves none.

New falsifiable lead: allow the3 directions preserving coefficients below17 to alter the precision17 coefficient. For target precision18, two such perturbations first interact at34, so the directional recurrence through18 should be a tangent recurrence. Its dependence on the base should use only coefficients through degree1. The affine family has rank1 through degree1, so the proposed next test has just two reachable early states, rather than a63-variable ANF. Prove this statement for the degree-clipped recurrence first; enumerate all7 nonzero directions and both reachable early states; verify any candidate against the complete Boolean correction circuit. A discovered unit would support exact selector reconstruction; a nonconstant difference yields an explicit counterexample. No tangent certificate or gain is claimed in this checkpoint.

One scientific experiment executed once with90s outer,4GiB,CPU0 limits. Source/input hashes and frozen current manifest are preserved. The prior manifest is authenticated by PREDECESSOR_MANIFEST_RECEIPT.json. An exclusive-write setup attempt encountered the already-existing pointer snapshot; no file was overwritten and no scientific execution was retried. The controller checkpoint began at638s,38s beyond the600s checkpoint target; the900s session limit remains binding.

The original source/U/ghost/outer branches, complete ordered original root sets, independent verification, matched genuine F4/F5/F6,20 preregistered completed hard instances, repeated cold timings and memory comparisons remain active requirements. The release gate is unpassed; neither2x nor the user's50x stretch target is established. Rank tests and17-field components cannot certify those claims.
