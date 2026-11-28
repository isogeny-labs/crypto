# Sparse dependency operator and target witness qualify

Pushed freeze f030dee3d7a, single invocation exits0. All frozen hashes pass.
18panels:6unchanged incidence workloads times seed-orbit counts2,ceil(sqrt(r)),r.
252complete basis columns,54adjoint checks, RHS and every original admitted
equation under allbasis assignments/affine offset agree exactly. Materialized
SchurSparse, originalSparse and nativeFast original matrix agree on every
target answer; all reconstructed scalars verify in original Jacobian.

Every panel obtains a target-only row-space witness w via reference dense
elimination. Matrix-free transpose independently verifies S^T w=e_k and
w^T rhs matches target scalar. No production black-box witness finder is
implemented or benchmarked. This certifies the operator/verification path,
not a faster end-to-end solve or full-rank theorem.

For p601/source1/intermediate9seed-orbit panel, the operator stores7811terms
versus13444nonzeros in expanded matrix. Source17 has7978versus10904. Small
two-seed panels can have FEWER explicit dense nonzeros than dependency terms;
no universal memory or field-operation saving follows. Counts are structure,
not measured native operations/CPU. Full tables retained in CONTROL.tsv.

The exact operator applies chronological creation dependencies and reverses
them for transpose, keeping cost linear in graphterms rather than expanding
all point labels at each application. All retained supports and true action
are supplied freely in this qualification; no new producer is demonstrated.
The larger seed layouts are prescribed controls, not success-tuned pools.

THEORY.md distinguishes exact Schur algebra from uniform-incidence growth
heuristics. Under that unproved adaptive mean-field model, scanning enough
anchors to double a pool at the classical size recovers q^(2-2/g)work,
not a new exponent. Classical progressive factor bases and black-box methods
are prior art. A10xclaim still requires a real producer, useful rank, efficient
witness generation, charged setup/CRT and final verification on original
genus2+targets; genus3/4 implementations remain unqualified.
