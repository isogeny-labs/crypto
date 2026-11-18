# Odd-prime kernel and independent Frobenius checkpoint

Producer cryptanalysis75e5064d1d089b917b068e7ca15b124ec0bcae37,
ready PR1482,branch research/chained-transfer-odd-kernels-20261111.
[Report](https://github.com/isogeny-labs/cryptanalysis/blob/75e5064d1d089b917b068e7ca15b124ec0bcae37/experiments/chained-transfer-reference/ODD_KERNELS.md),
[diagram/inspected two-page PDF](https://github.com/isogeny-labs/cryptanalysis/blob/75e5064d1d089b917b068e7ca15b124ec0bcae37/experiments/chained-transfer-reference/odd-kernels-report.pdf).
Source odd_kernels.rs SHA256434c23fd3a5242e755b68d50a9007bb5ee3c0a52e888e1487ca16dc9384a1217;
independent integration SHA2560f048b82d0c2f2be4e0caf9abdc6145fc7bd82bd0dc53cfcf18e1f0e034188e1.
All standing unsigned. [pcc:unverified]

## Actual-module negatives and candidate route

The accepted elliptic factors have traces−1/3 at17 and−7/−11 at101.
Their quadratics fi=X²−tiX+p satisfy
(X−t2)f1−(X−t1)f2=p(t1−t2). Since t1−t2=±4,the fixedell3/5/7
factors are coprime. CRT projections are polynomials inFrobenius,so any
invariant2-plane splits across the two elliptic blocks. If one quadratic
is irreducible,the only invariant2-planes are whole nonisotropic blocks.
Thus rational maximal-isotropic3/5 kernels are excluded for both accepted
examples,and7 for17. This argument does not assume the actual repeated
Jordan type of the other block at101/5. [pcc:unverified]

At101/7 both blocks are squarefree split,with eigenvalues2/5 and4/6.
Their four mixed eigenline sums are abstract rational kernel candidates.
No actual geometric kernel polynomial,codomain,odd-prime map or benefit
is accepted yet. These exact negatives do not exclude other degrees,
unpolarized isogenies,extension-defined kernels or other covers.
[pcc:unverified]

Complete finite controls enumerate130/806/2850 RREF2-planes,40/156/400
isotropic planes and every nonzero component pairing-scale ratio. The
companion matrices are controls,not assumed actual repeated representations.
Actual torsion/pairing transport through the prime-toell split/Richelot
isogenies and the invariant-subspace argument remain written inputs.
PCC034bed/afecb5 bind discovery/regression/Clippy. [pcc:unverified]

Six OddKernelProjectors.lean lemmas pass gate6/localPCC8b67e3: Bezout/unit
coprime/projector/idempotent algebra and fixed root lists. Total68 written
lemmas,gates22+8+7+12+6+5+7+4+7+7+6. No actual finite-field,torsion,
polarization or isogeny geometry formalized;no new axioms. [pcc:unverified]

Primary reviewed:J.S.Milne,Abelian Varieties,Version2.0,2008-03-16,
https://www.jmilne.org/math/CourseNotes/AV.pdf,
SHA256f5ca4e63e5092a4b102daad1470e4cbed5fe8f82115e3a28c8881e3f67f6aaef;
I§13/Prop13.2 pairing/Galois/functoriality,IProp10.20 Tate characteristic
polynomial,IIThm1.1 Frobenius/point counts,I Remark8.12 quotient isogenies.
Scope supports module inputs,not an unconstructed positive explicit map.

## Separately authored branch oracle,not a whole-map verdict

Closed cryptoPR3918/source3a972adb7aaaf1ee10dde605dc48f2c9822dbda5
provided a separately authored std-only Rust modular-polynomial oracle.
Original SHA256284e4d72af09fa0729bfa14a7c06249ce8fe78cb6194f01cc0008bf63ab2b04a
is preserved verbatim and compiles/passes unchanged underRust1.99.1.
Formatting-only consumer integration also passes Cargo/strict Clippy;
no algorithm/fixture/lint exemption changed. It imports no producer field,
Richelot or Cantor code and confirms both cubic source factorizations,
both reflected sextic target Rabin irreducibility,Frobenius orbits and
pair/matching fixed counts at degrees1/2/3/6. PCCed9aff binds source
custody/original/receipts. No closed PR reopened or frozen lineage imported.
This independently cross-checks bounded branch data,not whole Jacobian
maps or signed scientific standing. [pcc:unverified]

Current targeted controls/Clippy pass;full native25/reference50-unit-plus1-
integration CI pending,one old benchmark ignored. Prior da66ab9d9 exact-head
43026964101 passes native25/reference49/guards(PCCf2de08);PR1481 merged
ee3f177b4 at13:21:37Z. Native runtime pin75a737b8 remains unchanged.
[pcc:unverified]

## Next construction

CT19 constructs the fixed101 degree7 isogenies on both elliptic factors
using existing isogeny_algos division-polynomial/Kohel code. Require actual
kernel polynomials,exact cleared curve identities,duals/global x-compositions,
all109/113 group/scalar/recovery checks and separate existing-kernel verifier.
See CT19_PLAN.md. CT20 then considers compatible genus2 gluing and a genuine
Jacobian7-hop;no map is pre-accepted. Additional CRT scalar controls from
PR3877 comment remain unexecuted:17 projectors210/76,101 projectors9266/3052.
No solver/benchmark/paid job/cross-prime descent. S3/live-budget,KB/vault and
crypto frozen-ledger/archive gates remain. [pcc:unverified]
