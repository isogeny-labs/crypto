# Rational trigonal gate: both eligible pair kernels fail base-field rationality

The complete20curve kernel census produced two genus3tractable candidates.
Both fail the next known Smith rational-trigonal-map prerequisite over their
existing base fields. Neither curve was replaced, no selector or timing rerun.

| Prime/family | Nullity | Q(u+t v), low-to-high coefficients | Discriminant | Legendre | Rational projective roots | Valid maps |
|---|---:|---|---:|---:|---:|---:|
|127/1|2|71,25,30|106|-1|0|0|
|1009/1|2|556,129,20|413|-1|0|0|

Both leading quadratic coefficients are nonzero, so infinity is not a missing
projective root. Discriminants are exactly c1²−4c0c2; nonsquare means no root
in the base field. Complete enumeration of p+1projective points independently
agrees. Raw nullspace bases and all counts are in CHECKS.txt.

## Construction and exact scope

For each even-degree irreducible factorF, let tau=x^(p^(e/2))modF. On these
unique pair kernels, tau exchanges the paired roots. Involutivity and
nontriviality are checked. The numerator/denominator coefficient minors
p_ij=n_i*d_j−n_j*d_i satisfy sum p_ij(x^i*tau^j−x^j*tau^i)=0modF.
Expanding coefficients produces equations directly over Fp. The rational
root/infinity pair gives the remaining equation. This realizes the rational
constraint space without extension-field root coordinates; the earlier
admission handoff's list of such coordinates was sufficient, not necessary.

For lexicographic minor order01,02,03,12,13,23, decomposability is
p01*p23−p02*p13+p03*p12=0. Rank-four linear equations give the displayed
projective line; every decomposable point would reconstruct a pair of cubic
coefficient vectors. The reconstruction checks all six minors, coprimality,
map degree and original paired-value polynomial identities whenever a map
exists. No real map exists here: real-map identity assertions are vacuous,
not a positive divisor-transport or curve-map verification. In particular,
stdout all_pair_identities=true with valid_maps0does not claim a real map.

Sixty-four constructed wedge controls over5/7/11/17all satisfy the quadric
and reconstruct exactly. Three planted x³maps over13/19/31survive the same
nullspace/projective-line machinery and satisfy original rational pair values.
No curve or smoothness assertion is made for these polynomial controls.
All controls pass; scientific stderrempty. No mismatch, retry or cap.

This is established Smith/Grassmannian geometry with quotient-ring Frobenius
constraints, not a novelty or speed claim. Direct rational-trigonal construction
fails for these two pair kernels. It does not establish absence of a rational
Jacobian isogeny: Smith Remark5discusses quadratic descent, and p127family1
also has two non-pair kernels outside the tractable formula. A rational quotient
of principally polarized abelian varieties does not itself guarantee a suitable
same-field curve/divisor map. No base-field extension cost is silently omitted.

## Custody and replay

Protocol d9fa1ed2eb preceded source8c6fad97c906 and checkpoint
20261115T125024Z-8c6fad97c906. Input fixtures are the exact copied archive
members in ../hec_kernel_admission_20261115/fixtures, hashes retained.
Build check.rs with stable Rust and cached hec_native_icrlib as in the kernel
study, then invoke it with that fixture directory. No scientific Python,
cloud worker, clock study or benchmark record. Preserve raw coefficients,
source/binary hashes and reportPDF/SVG. Existing native CI does not execute
this standalone gate. Full genus2/3/4mathematical10xobjective remains active.

Primary source: Smith arXiv0806.2995v2Section5(Plucker quadric, rational
nullspace and discriminant criterion), Section9and Remark5(field/descent
limits). Current-state modern isogeny formulas have not been surveyed by this
gate; no absence claim about them. A next structural attempt needs explicit
non-pair-kernel or descent formulas and a same-target complete-cost case,
rather than relabelling these zero-map outcomes as successful transfers.
