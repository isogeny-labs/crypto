# Exact formal scope

Lean4.35.0; CryptoLean148f5dc389b341401e06d76abe7783c4ccf42f2e;
Mathlibcfc55fc58714a59dc06e6e79edc6d1d95140d3c2. Libraries CauchyNormalizer and
KernelNodes use the unchanged prior source directory. Final producer checks
are recorded in VERIFICATION.md; all claim standing is unsigned pending trusted
PCC signature. [pcc:unverified]

| Claim/identity | Lean declaration | Exact scope |
|---|---|---|
| Actual R node value | CommonScalar.normalizer_node | m<2^(a+1), rational values |
| Actual W node preservation | CommonScalar.node_preserved | k,m in grid; PCC-20261108-02347a |
| Center unit and kernel | CommonScalar.normalizer_origin; kernel_origin | all natural a,k |
| Matrix identity at0 | CommonScalar.matrix_origin | x,y<2^a; PCC-20261108-54f993 |
| Jet origin obstruction | CommonScalar.jet_origin_obstruction | C(0)=0 explicit |
| Exact finite squared series | CommonScalar.squared_geometric_sum; squared_geometric_remainder | rational x; remainder requires1-x!=0 |
| Coarse precision inclusion | CommonScalar.coarse_budget_inclusion | finite natural multiplicities, new budget2q; PCC-20261108-3bd7df |
| Actual rational denominator nodes | RationalScalar.denominator_node; denominator_node_unit | m in grid; rho nonzero/unit comes from parent theorem |
| Rational node preservation | RationalScalar.node_preserved | k,m in grid; PCC-20261108-75f0ff |
| Rational origin | RationalScalar.denominator_origin; kernel_origin | all a,k |
| Rational point valuation | RationalScalar.kernel_valuation | base nonzero, denominator nonzero and valuation0 explicitly assumed |
| Clear one product | RationalScalar.clear_product | d!=0 |
| Clear homogeneous sum | RationalScalar.clear_homogeneous_sum | d!=0, exactly M slots per term; PCC-20261108-83608b |
| Scalar relation | RationalScalar.scalar_relation | D(T)!=0; no automatic unit assumption |
| Cleared square-product degree | RationalScalar.squared_product_degree | polynomial f_j, natDegree(f_j)<=d; actual B-polynomial bridge still open |
| Three-level margin | RationalScalar.three_level_margin | rational h>0,u>=3,hu>=3M; PCC-20261108-61a4c4; numeric lemma only |

Not yet formal: D=C'/C'(0) for the actual explicit polynomial representation;
coefficientwise chart integrality/inversion; Smith/Newton common-unit invariance;
actual recursive-program homogeneity; finite-precision recovery and its costs;
all-level uniform mode bound; resulting prime-field ECDLP algorithm. REPORT.md
contains the written chart argument and PLAN.md the concrete closure sequence.

No finite arithmetic certificate is substituted for a quantified theorem.
Failed source/build attempts are retained, never accepted via sorryAx.
