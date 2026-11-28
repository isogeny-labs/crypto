# Prime-field T20 four-cover norms

Complete native full43/85436f implements the actual M=N_C20 R20† directly
on all12317 rational balanced T20 inputs with F101 arithmetic and no
input-support roots. Both actual adjoint compositions equal[4];public9238
recovery,group/Frobenius and all109+113 scalar images/exact orders pass.
The two explicit curve covers have degree4 and the induced norm pair
has global degree16. Scientific standing is unsigned. [pcc:unverified]

[Written proofbfa422](../../T20_DIRECT_NORMS_PROOF.md) gives the exact
F101 functions,characteristic101 equations,explicit Bezout witnesses,
function-field degree argument,positive-adjoint sign,coherent two-torsion
translation descent,complete balanced pole/infinity/jet treatment and
faithful formal boundary. [Protocolc05a75](../../T20_DIRECT_NORM_PROTOCOL.md)
was pinned before implementation;[prospective derivation](../../T20_DIRECT_NORM_DERIVATION.md)
and primary/actual KB127 receipts are retained. [pcc:unverified]

For raw Y²=76z⁶+62z⁵+52z⁴+63z³+69z²+3z+25,the covers are
X=n/q²,Y_E=Y*m/q³. E2(59,27) hasq=z²+38,
n=10z⁴+54z²+52z+53,m=2z³+55z²+47z+44,infinity+(10,100).
F0(44,21) hasq=z²+46,n=71z⁴+96z³+92z²+49,
m=5z³+53z²+17z+87,infinity+(71,48). The raw ordinate normalization
is Y=50*y_monic,a square coordinate change,not a retwist.
[pcc:unverified]

The runtime evaluator contains Zp,Vec<u64>,Curve<u64>,Pt<u64>;it receives
no extension context,root cache,index or DLP label. Construction still
uses and must charge the existing F101^12 factory. The complete corpus
converts from actual classes by inverse Chart,with every coefficient
projected and every input roundtripped. This is a thin validated adapter,
not a copied balanced group engine. [pcc:unverified]

The exact input degree counts are[3,236,12078],repeated supports118,
nonsplit5116,pole inputs2 per cover and applicable omitted-infinity
mutants238 per cover. Wrong global sign disagrees12316 times and agrees
only at identity. Geometry58b615 checks120 rational point classes,six
branches,eight pole points,both signed infinities,128 actual repeated
doubles,Frobenius/coherent-translation invariance/six malformed mutants,
and16 actual kernel classes/all256 sums/all killed4/exactly4 killed2,
checking kernel type(4,4). Geometric reference controls do not establish
an arbitrary F101^12 Jacobian input domain. [pcc:unverified]

Static1bbfec,geometry58b615 and whole90601c precede complete43/85436f.
Full43 retains all old41 tests,reference+legacy guards,fmt/strictClippy,
all628 hashes and byte-identical old geometry6141/kernel24475 JSON outputs.
Its whole18-file original execution tree is one57652-byte standardtar.xz,
with raw/archiveSHA256,every member hash and exact roundtrip. Source
freeze ended only after original start/finish/exit0/afterguards/comparisons.
[pcc:unverified]

Full40-module final proofgate52bbc7 observes238 compiled/212 written/
0trusted/all44 hashes unchanged;all171 files and inputs form one39344-byte
standardtar.xz with exact hashes/roundtrip. Fourteen exact Lean statements
cover generic linear/quadratic/translation algebra,conditional norm4/
recovery/constants,both actual characteristic101 curve equations and
their explicit Bezout witnesses. The actual curve/Picard/polarization/
descent/jet/group-law bridges remain written/native. [pcc:unverified]

Algebra02/9214e7 passes24 tests(old23 retained),eleven outputs,all ten old
payloads byte-identical,24 guards and the new mutation rejection.
IDC1h6de045a64b58c1c8 checks the universal linear-image ordinate
specialization over fixed2^61−1;it does not certify the characteristic101
curve coefficients. Candidate01/4771dd and full40/236 proofgate4a3e09
remain immutable after adding the two Bezout lemmas. Identity ID stays
the same;only its final ProofRef payload changes. [pcc:unverified]

The all12317 C0 comparison preserves M R20 Phi49=
[2]diag(alpha49,id)N_C0. After public normalization2 on the odd rational
groups this is the existing direct norm/isogeny bridge. No additional
solving advantage is claimed from that equality. Full setup/conversion/
transfer/solve/recovery/verification costs and matched external references
are unmeasured;these local Linuxx86_64/Rust1.99.1/Lean4.35.0 runs are
correctness/development,not benchmark isolation. [pcc:unverified]

Evidence: [nativefull](../../evidence/t20-direct-full-01/),
[geometry](../../evidence/t20-direct-geometry-01/),
[whole](../../evidence/t20-direct-whole-01/),
[formal12 bindings](../../evidence/t20-direct-lean-02/),
[two degree bindings](../../evidence/t20-direct-degree-03/),
[finalproofgate](../../evidence/t20-direct-proof-full-02/),
[finalalgebra](../../evidence/t20-direct-algebra-02/),
[newIDC](../../evidence/t20-direct-linear-image-identity-02.json),
[retained PATH failure](../t20-direct-norm-failures/REPORT.md).

[Editable chain](chain.dot),[SVG](chain.svg),[recorded counts](counts.json),
[count graph](counts.svg),[renderer](render.py),[Typst](report.typ),[PDF](report.pdf).
Exact finite populations have no sampling uncertainty. Prior scientific
graphs/IC timeline/scoreboard were checked and remain unchanged because
this adds a local evaluator,not a measured cost/catalog class/leaderboard row.
The new actual norm is drawn here with full raw models. [pcc:unverified]

Sources:BD0902.3480§2 Lemma2.6/§7 Lemma7.5/§8;Galbraith corrected8.3.8/
8.3.10 with proof;proper720a59/geometryb24be8/DirectNorms/ClosedFibres;
actual existing native engines;CT50dd4077d1 primary,external scripts not
adopted. KB status/two searches/theorem query all127,not absence evidence.
Next:scoped commit/new own CI,then a distinct consumer source pin/replay
and independent prime-field checks. The old proper8072 CIb3c8f1 is settled
all4 success;it does not cover this new component. [pcc:unverified]
