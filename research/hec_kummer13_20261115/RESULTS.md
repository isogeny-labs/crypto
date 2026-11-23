# Explicit base-field level-two quotient coordinates

Both existing genus3 3-kernels now have eight explicit descended coordinate
functions spanning the complete level-two section space of the polarized
quotient. Their arithmetic uses only F(127^13), with coefficients descended
to F127. The implementation does not split source2torsion, construct its
branch field or materialize the degree156 compositum. This is an explicit
coordinate map; equations of the image, quotient Jacobian/Serre testing and
divisor transport are not supplied by this gate. No speed ratio is measured.

## Derivation and scope of the rank certificate
Let V be either kernel and F_P its pinned symmetric normalized Weil functions.
Use the existing base3 kernel coordinates, verified against all group sums.
The first nontrivial cube root in F127 is zeta=19. For each of the27characters
chi_a(P)=zeta^(a dot P), put Phi_a=sum_P chi_a(P)F_P. The theta-group law gives

    F_P(X) Phi_a(X+P) = chi_a(-P) Phi_a(X).

Hence, with Theta=Phi_0,

    G_a = Phi_a Phi_(-a) / Theta²

is invariant under kernel translations and negation. There are14a modulo
plus/minus. The products are invariant sections of L² for L=O(3W)=psi*M,
so descend to sections of2M on A=J/V. Division by Theta² fixes a rational
frame on its nonvanishing chart. Riemann-Roch gives h0(A,2M)=2³=8.
Thus eight independent evaluations prove eight independent global sections,
and the dimension upper bound proves a complete basis, not just an observed
sample rank. No projective-normality theorem is needed for this argument.
No assertion that A is geometrically indecomposable or a Jacobian follows.
The full level-two map is the relevant quotient/Kummer coordinate map;
image geometry still needs investigation, especially if A is decomposable.

Frobenius permutes characters by inverse pullback of its action on V.
Let sigma be the induced permutation of the14opposite pairs. With field
basis w_j=z^j, j0..12, define coefficient traces

    D_(a,j) = sum_(t=0..12) w_j^(127^t) G_(sigma^t(a)).

These functions are fixed by coefficient Frobenius. At base-field inputs
they equal field traces of w_j G_a(X). At non-base-field kernel translates,
that shortcut is invalid; the implementation uses the explicit character
permutation. Every selected descended output at those translates equals
the base output and is itself fixed by Frobenius.

## Frozen terminal result
Protocol61923beee0 preceded source26a74be6430e and checkpoint
20261115T143749Z-26a74be6430e. All original curve/kernel/alpha inputs retained.
The bounded ascending search rejects n1andn8for the original evaluator chart;
all other n2..18give16joint regular public multiples with Theta nonzero.
Training points n2..7andn9..14select the basis; holdouts n15..18do not enter
selection. No sample/field/kernel retuning or rerun.

| Recorded check | Count / result |
|---|---:|
| Character product laws |19683pass|
| Nonzero character sections across both kernels |54of54|
| Character eigenactions |23328pass|
| Opposite-product translation invariance |12096pass|
| Coefficient trace versus field trace at base points |5824agree|
| Opposite-product symmetry |448pass|
| Descended evaluation rank |8and8|
| Selected coordinate translation checks |6912pass|
| Selected coordinate negation checks |256pass|
| Duplicate basis descriptor rank |7as required|
| Broken-character eigenaction failures |832detected|

Both lexicographic basis selections give the same descriptors:

    (a,j) = (0,0), (1,0), (1,1), (1,2), (1,3), (1,4), (1,5), (1,6).

D_(0,0)=13is a nonzero constant in this frame. For example at publicmultiple2,
the two projective vectors are

    kernel0: [13,113,9,83,81,43,38,75]
    kernel1: [13,20,28,10,12,107,29,90].

All16vectors per kernel, full16x182candidate tables, source-scalars provenance,
attempt outcomes and terminal checks are retained in RUN.txt. Both outcomes
remain; these coordinates are not kernel-quality scores. The four holdouts
check identities outside basis selection, not independent arithmetic or a
statistical generalization claim. Global membership/descent and finite rank
are combined explicitly above; finite sampled identities alone would not
prove the global quotient construction. All arithmetic shares the pinned
finite-field/Cantor/Weil implementation; no external independent validation.

## Reproduce and custody

```sh
export PATH=/Users/adamburan/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH
export CARGO_TARGET_DIR=/Users/adamburan/pq-work/hec-kummer13-target
cargo build --offline --locked --release --manifest-path research/hec_kummer13_20261115/Cargo.toml
$CARGO_TARGET_DIR/release/kummer13 \
 research/hec_torsion13_20261115/CERTIFICATE.txt \
 research/hec_kernel_admission_20261115/fixtures/select-g3-p127-f0.out \
 research/hec_theta13_20261115/RUN.txt
```

Original run terminal: exit0, empty stderr, no cap. Clean locked offline
macOS ARM release build. Source/dependency/binary/certificate/alpha/shared
implementation hashes pinned. No scientific Python/cloudworker, performance
benchmark or historical record/default/leaderboard mutation. No timing claim
from process metadata. Existing native CI does not execute this standalone
scientific gate. Report and diagram rendered and visually inspected.

Sources: prior normalized/theta and root-free reports; Tian
https://arxiv.org/html/2007.03172 Sections3/4 for theta-group actions, quotient
line-bundle descent and level-two coordinate dimension. KB ready lexical at
revision9141e386254340a9bdba3036f2bb706a, lastrefresh2026-11-14T21:44:48Z;
two queries returned adjacent internal notes, not study validation. A web
search found Koizumi projective-normality statements, but characteristic
hypotheses were not verified and that theorem is not used. Established finite
character projections/trace/descent, no novelty or speed assertion.

Next: compute relations/equations of both coordinate images, test quotient
geometry and same-field/Serre conditions, construct divisor transport, then
charge all setup and solve work in complete comparisons. The mathematical10x
genus2/3/4objective stays active; other genera are not replaced by this gate.
