# Symmetric normalized kernel Weil sets and theta sections

Both explicit genus3 maximal isotropic3kernels now have symmetric normalized
Weil sets and nonzero invariant theta sections. The construction stays in
K=F(127^13); normalization uses square/cube compatibility without extracting
roots or materializing the degree156 branch/kernel compositum. Each section
is Frobenius compatible and has nonzero base-field evaluations at every one
of the four registered regular points. This is a constructive theta-group
step, without quotient equation, target transport or runtime comparison.

## Construction and argument
The pinned evaluator gives eta_P; the Weil convention is f_P=eta_(-P).
For each27element kernel, the exact addition table and inverses are computed
from the stored Mumford classes. Define

    C(P,Q)=f_P(X) f_Q(X+P) / f_(P+Q)(X).

The previously certified divisors of the Weil functions imply that this
ratio has divisor0, hence is a nonzero constant. Values at four regular X
are checked for agreement rather than assuming the implementation obeys the
geometric identity. Commutator C(P,Q)/C(Q,P)=1 on each kernel, consistent
with the prior primary-module isotropy certificate. This is not a check of
pairings between the two complementary kernels.

For odd level3 and prescribed within-kernel pairing1, write

    s_P = f_(-P)(X) / f_P(-X),
    A3 = 1 / (C(P,P) C(P,-P)),
    A2 = s_P / C(P,-P),
    alpha_P = A3/A2.

The symmetric odd-level normalization fixes the scalar uniquely. Every
computed scalar satisfies alpha³=A3 and alpha²=A2. Thus no cube root or
extra field is required. All four X give exactly the same alpha vector.
For every pair, alpha_P alpha_Q C(P,Q)=alpha_(P+Q). The normalized functions
F_P=alpha_P f_P satisfy F_P(X)F_Q(X+P)=F_(P+Q)(X), and their symmetry and
Frobenius propagation are explicitly checked.

Set Theta=sum_(P in V) F_P. Reindexing the finite group gives

    F_P(X) Theta(X+P) = Theta(X).

This is invariance under the theta-group action, not ordinary translation
invariance of the rational function Theta. The global identity follows from
the certified Weil divisors, constant cocycle and normalization construction;
finite evaluations additionally check the implementation. A nonzero value
certifies the sum is not identically zero. No quotient curve or a Jacobian
over F127 follows solely from this section: that is still a reconstruction
and descent question.

## Registered outcome
Protocol2d193a800a preceded scientific source932ead8b52db and checkpoint
20261115T142338Z-932ead8b52db. Original curve, publicD1and both kernels are
unchanged. Candidate nD1are considered ascending, with cap32. n1is outside
the evaluator chart; n2,3,4,5are fully regular for both complete kernels,
including negative points and all translates. All attempt outcomes retained.

| Exact implementation check | Recorded count | Result |
|---|---:|---|
| Cocycles compared beyond first evaluation point |4374|All equal|
| Ordered within-kernel commutator checks |1458|All trivial|
| Associativity triples |39366|All equal|
| Normalization pair checks across four points |5832|All equal|
| Symmetry checks |216|All equal|
| Frobenius-compatible normalization scalars |54|All equal|
| Theta-group section-action checks |216|All equal|
| Zero theta values |0 of8|Both sums nonzero|
| Corrupted-alpha normality failures |152|Negative control detected|
| Corrupted-alpha action failures |208|Negative control detected|

Actual values in F127:

| Public multiple n | Kernel0 Theta | Kernel1 Theta |
|---:|---:|---:|
|2|2|20|
|3|15|18|
|4|37|4|
|5|82|29|

These values are scalars in a chosen normalization, not times, speed ratios
or a comparison of kernel quality. Both outcomes retained. RUN.txt also
stores all54alpha vectors, sample rejection reason and terminal counts.
The one rejection is part of the frozen bounded chart search, not a rerun.
There are no confidence intervals for this deterministic correctness gate.
Arithmetic shares the previous finite-field/Cantor/evaluator implementation;
this is not independent external validation. No cross-kernel nontrivial
pairing control, quotient equation or Serre invariant is supplied here.

## Reproduce and retain

```sh
export PATH=/Users/adamburan/.rustup/toolchains/stable-aarch64-apple-darwin/bin:$PATH
export CARGO_TARGET_DIR=/Users/adamburan/pq-work/hec-theta13-target
cargo build --offline --locked --release --manifest-path research/hec_theta13_20261115/Cargo.toml
$CARGO_TARGET_DIR/release/theta13 \
 research/hec_torsion13_20261115/CERTIFICATE.txt \
 research/hec_kernel_admission_20261115/fixtures/select-g3-p127-f0.out
```

Original execution is terminal, exit0, empty stderr and no cap. Source and
binary/shared implementation/certificate/lock hashes in SOURCE.txt and
hashes.sha256. Clean isolated macOS ARM release build. No scientific Python,
cloud worker, performance benchmark, closed-record/default/leaderboard change.
No timing inference from process metadata. Existing native CI does not execute
this standalone gate. Report and diagram rendered and visually inspected.

Primary source: Tian https://arxiv.org/html/2007.03172 Section2.2, Eq2.3/2.4,
odd symmetric normalization, and Section4 theta-group section sum. Prior
root-free and torsion13 reports supply exact divisor arithmetic. KB ready
lexical revision9141e386254340a9bdba3036f2bb706a refresh2026-11-14T21:44:48Z;
two queries gave adjacent internal material, not validation of this study.
This uses established theory; no novelty assertion. Next: quotient functions
and equation, same-field/Serre test and divisor transport, followed by a
complete-cost comparison with all setup charged. The full mathematical10x
genus2/3/4objective remains active.
