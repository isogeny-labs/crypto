#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(justify:true)
= Replaying both transfer kernels with a second group engine
2026-11-12 · exact F101 models · scientific standing unsigned

The exported Psi and Psi2 kernels now pass group-law replay with the
pre-existing generic Cantor implementation from jv_cover.rs. All49 classes
in each corpus are converted to a checked monic quintic chart. The second
engine agrees on all2,401 sums per group, all7-annihilators, inverses,
validity and distinctness. Complete standalone03 passes8 tests and strict
Clippy;PCC9ebcf7. This is correctness evidence without a timing claim.
[pcc:unverified]

#image("replay.svg",width:100%)

== The missing evidence this adds
The earlier separate flat-field replay checked divisor validity and Frobenius,
while additions used the producer's balanced arithmetic. This new replay
uses an existing second polynomial/Cantor implementation with a thin field
trait adapter. It imports no producer tower, balanced addition, Richelot
map or elliptic kernel generator. The fixed input curves are
T0=[31,75,74,54,71,55,1] andT1=[95,26,19,27,94,91,1]. [pcc:unverified]

== Source custody and a retained failure
The engine comes from crypto:src/cryptanalysis/jv_cover.rs, last changed
at cfa4ef08bccfb509c89f8d0bb0c6c5baf662ff11. The full source SHA256 is
1ecd62c4282402e3411635e0487d0bb36a5d57e15a547f352a7cab1fda7309cc.
Retained source ranges40–52 and464–690 contain the generic field trait,
polynomials and odd-model Cantor operations. Initial byte-exact source01
and its two-test receipt42b1b7 remain frozen. [pcc:unverified]

StrictClippy02 rejected the old extended-gcd tuple return signature's
type complexity. Final03 uses the identical tuple through a Bezout alias;
every arithmetic body stays unchanged. An exact transformation guard
permits only that alias declaration and signature substitution. Both
source hashes are checked in CI. No lint exemption or reduced test is used;
failed02 and full03 are retained,PCC647a5a/9ebcf7. [pcc:unverified]

#pagebreak()
= Why the coordinate bridge preserves divisor classes

== A monic quintic over the existing field
For y²=F(x), choose the first sorted branch a avoiding every displayed
support and putL=F'(a). Smoothness makesL nonzero. Set
H(z)=z⁶F(a+1/z),w=L/(x-a),V=yw³/L andG(w)=L⁴H(w/L).
ThenH has degree5 and leadingL,soG is monic degree5 without an ordinate
square-root assumption. The inverse isx=a+L/w,y=VL/w³.
Native checks cover every denominator, degree, leading coefficient and
squarefreeness. [pcc:unverified]

Foru=x²+b x+c andA=u(a), the transformed support is
U=w²+L(2a+b)w/A+L²/A. The ordinate is
v1 w²+v(a)w³/L moduloU. Exact polynomial identities prove these formulas
in the whole finite support algebra, including multiplicities;no support
points are individually chosen. Four KernelChart Lean lemmas pass the
kernel/axiom gate and statement bindings,PCC2026a6. Their inverse and
chart hypotheses are explicit. [pcc:unverified]

Every nonidentity exported class has degree2 and balanced-plus0, hence
isD_finite-K. The two old infinities map to(w0,V=±L²). The functionw
has divisorBplus+Bminus-2W, whereW is the quintic's unique Weierstrass
infinity. ThusK maps to2W up to linear equivalence, and the ordinary
Mumford representative has exactly the correct baseline. Projective
completion and Picard functoriality remain written in
INDEPENDENT_GROUP_REPLAY_V2.md/PCC40a568. [pcc:unverified]

== Limits and next work
The replay covers the two exact49-class corpora and identity. It does
not independently evaluate either isogeny, establish map-kernel membership,
replay every arbitrary Jacobian class, or supply a signed scientific verdict.
Kernel identification still uses the written global commuting square and
the producer's actual elliptic-map killing controls. Prior limited replay
claims stay unchanged. [pcc:unverified]

The full consumer CT24 guard separately passes25 native/80 reference unit
and five integration tests,one pre-existing ignored benchmark,PCC5383d3.
All partial Lean source modules now contain125 written declarations;
geometric dependencies remain explicit. CT25 prepares direct C2 norms;
its seven cleared/conditional lemmas pass,c2aaaa,while runtime integration
requires complete generic comparisons. No measured computational advantage
is established. Admission/livebudget and vault/ledger constraints persist.
[pcc:unverified]

Reproduction:successor-preflight/legacy.sha256,src/kernel_group_replay.rs,
source extraction guard,fixtures/explicit-psi-kernel-07.tsv and
explicit-psi2-kernel-12.tsv. Raw receipts and source snapshots are in
evidence/second-cantor-*. Existing group-engine source and the written
chart/Picard proof state all assumptions and remaining scope.
