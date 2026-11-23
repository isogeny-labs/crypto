# Explicit 3-torsion over F_(127^13): frozen correctness gate

Full mathematical10xgenus2/3/4objective unchanged. Previous goal turn made
progress: exact20Weil polynomials and odd-prime kernels/projectors. No process
live. This step constructs actual genus3kernel coordinates for the already
positive p127family0input; it does not replace other genera or assert a gain.

## Sources and method
KB ready lexical revision9141e386254340a9bdba3036f2bb706a,
refresh2026-11-14T21:44:48Zpredates studies. Queries Jacobian/torsion/cofactor,
Cantor/extension,Couveignes/torsionreturned unrelated or explicitly recalled
notes, not validated construction evidence. No absence/novelty inference.
Primary Tian https://arxiv.org/html/2007.03172Section4/Remark4.1read again:
kernel input is required, quotient must be an appropriate Jacobian. Magma
hyperelliptic rational-points/group-order handbook read. Existing native
crates/hec-native-ic/src/jac.rsCantor formulas read in full and reused over
an extension field. This is established arithmetic/cofactor extraction and
Chinese-remainder module projection, not a new isogeny algorithm.
ecdbread remains unavailable as recorded in prior study; pinned fixture and
certified Weil polynomial are authoritative inputs, no new curve search.

Input: ../hec_kernel_admission_20261115/fixtures/select-g3-p127-f0.out,
L=[1,1,-43,-1224,-5461,16129,2048383], certified prior study sourceda1f554b0c.
Use first c1..126for which M=X13+X+c is irreducible overF127. Rabin criterion
for prime degree13: x^(127^13)=xmodM and gcd(x^127−x,M)=1. If none, cap and
retain failure; do not select a new field family after observing an outcome.
Fixed13coefficient arithmetic, exact Fp arithmetic, field inverses crosschecked
against exponentiation. q=127^13fitsu128, q=3mod4so sqrt(a)=a^((q+1)/4)
with mandatory square verification. Cantor composition/reduction in unique
Mumford form. Cached Rustnum-bigint(or offline Cargo build)for group-order
integers only; no scientific Python/cloud/remote worker.

Compute #J(Fq)=product(1−alpha_i^13)from Lby Newton power sums and reciprocal
identities. Independently compute det(I−companion(chi)^13)by fraction-free
Bareiss; require exact equality and exact divisions. Verify #J(F127)=2057786,
large prime target order1028893divides extension order, and3valuation>=6.
Write N=3^v*h with3notdividingh. All public integer values retained.

## Frozen controls and construction
1. Field64deterministic nonzero elements: inverse via base-polynomial xgcd
versus exponentiation, Frobenius13identity, square-root verification, field
laws. Fixed seed2026111501300. No scalar/DLPanswer used.
2. Compare lifted publicbase-field Cantor arithmetic with existing native
Fpcurve on128deterministic scalar pairs (addition, negation, doubling),
including identity/opposites. Check Mumford invariants on every new output.
3. At most16random curve points with seed2026111501301; at most32xtrials
for a square per point, each xnon-base-field. Require [N]R=0, extract
Q=[h]R, if nonzero descend its3power order to a nonzero3torsion point.
Apply BOTHcertified projectors E0=1+2pi+2pi²+pi⁴andE1=pi+pi²+2pi⁴ONLYtoQ.
Retain attempt outcomes; stop on first attempt with both nonzero images.
If cap or mismatch, preserve partial output and do not claim kernels.
4. For each image P, use P,piP,pi²Pas basis, enumerate all27combinations.
Verify every divisor invariant, [3]P=0, exact distinct cardinality, closure,
Frobenius stability and annihilation by its respective irreducible cubic
f=X³+2X+2or g=X³+X²+2. The non-self-dual primary-module theorem supplies
isotropy; do not claim a numerical Miller/Weil-pairing computation.
Check intersection is identity and all729cross sums distinct, exhausting J[3].
5. Emit public field/curve/cofactor/sample and all54Mumford records. Separate
replay mode consumes only this certificate, not generator RNG or expected
coordinates; shares arithmetic engine, so is replay rather than independent
controlled validation. Negative controls reject a nonmonic point, duplicate,
wrong-primary assignment, and embedded D1in place of3torsion. No source
point generation repeated for replay or failure controls.

Native process aggregate600secondscap. Stop on any field/group/order mismatch;
retain stderr/errors. This is a correctness/constructive gate, no whole-solve
time ratio or10xprediction, benchmark or admitted-standing change. Initial
setup includes field construction, extension order, point generation, cofactor
extraction, projection and verification; any later timing must charge them.
Commit/checkpoint source before execution, keep raw/source/dependency hashes.
Native reportPDF/SVGand existingPRupdate, checkpoint and actualCI. Next
successful dependency: explicit suitable same-field quotient/divisor map and
complete same-target cost. Full genus2/4work remains part of the objective.
