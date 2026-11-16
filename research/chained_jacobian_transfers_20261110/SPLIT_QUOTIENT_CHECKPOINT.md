# Explicit split-quotient checkpoint and direct controls

Producer cryptanalysisda66ab9d953d2ba96c4ac9d6e8379c3478b3f277,
branch research/chained-transfer-split-quotients-20261111. Source digests:

|Source|SHA-256|
|---|---|
|src/balanced.rs|3be16c2a1e2f75cc00f28751c0d956b950f2c74fb6b49641820e5988e42bb6c7|
|src/nonsplit_map.rs|ea5146b957cfdb4f5016dce8cc179fe1661e17329f92abf3e3b8b47d6d4c4bce|

[Report](https://github.com/isogeny-labs/cryptanalysis/blob/da66ab9d953d2ba96c4ac9d6e8379c3478b3f277/experiments/chained-transfer-reference/SPLIT_QUOTIENTS.md),
[diagram/inspected two-page PDF](https://github.com/isogeny-labs/cryptanalysis/blob/da66ab9d953d2ba96c4ac9d6e8379c3478b3f277/experiments/chained-transfer-reference/split-quotients-report.pdf).
All standing unsigned. Prior63cf80560 focused43024621372 passes native25/
reference47,1 old benchmark ignored,formatting/strict Clippy/guards;PR1480
merged57ab3ad05 at13:05:29Z. Current full49-test replay pending.
[pcc:unverified]

## Complete reciprocal maps and quotient identities

For C:y²=x6+a2*x4+a4*x²+b,b=s² nonzero,the actual Fp reciprocal
T=s/x,Y=b*y/x³ gives monic cover coefficients[b²,0,a2*b,0,a4,0,1].
Inverse x=s/T,y=s*Y/T³. Both old infinities map to finite(T0,Y±b);
zero support maps to target infinity with its proper branch/multiplicity.
K maps to a hyperelliptic fiber. The complete balanced divisor map retains
all support and infinity coefficients; no implicit twist. [pcc:unverified]

Every285 F17 and12,317 shiftedF101 class passes reciprocal inverses,
bijection,paired additions,zero multiplicities0/1/2,both elliptic norm
additions,cross-norm zero and sum of pull/norms=[2]. Explicit elliptic
counts are19/15 and109/113; full norm-pair images exhaust those product
groups. Halving the odd components and pulling reconstructs every class.
The API checks supplied odd annihilators on each input point; they are not
silently treated as group-order certificates. PCC036d81 binds the full
targeted test,96b1e2 the source-component controls/Clippy. [pcc:unverified]

Written geometric bridge: the two quotient involutions are sigma and
iota*sigma,where hyperelliptic iota induces−1 on the Jacobian. Pull/norm
operators are1±sigma,whose sum is[2]. Norm/pull on each elliptic factor
is[2],and cross-norms vanish. Thus N=(N1,N2) and P=P1+P2 satisfy
PN=NP=[2]. These are explicit rational-group isomorphisms on the fixed
odd groups; no abelian-variety product isomorphism is inferred.
[pcc:unverified]

For accepted R and dual Rhat,set M=N*Rhat,S=R*P. Then MS=SM=[4],
so quartering gives rational inverses. All19/109 source scalars have
coordinates([4]P,0),with inverse5/82 recovery and reversed-class split
reconstruction. No map reads unknown scalars. The second elliptic factor
supplies no extra DLP equation for this subgroup. A larger ambient group
alone gives no generic-query gain; no non-generic solver or cost advantage
has been demonstrated. [pcc:unverified]

Seven SplitQuotients.lean lemmas pass gate7/localPCCfffa35: reciprocal
cleared curve/inverse,complementary sum,generic odd halving,conditional
reconstruction and target components. Total62 written lemmas,gates
22+8+7+12+6+5+7+4+7+7. Two failed broad tactics retained; explicit power
identities pass. Finite fields,group orders and Picard geometry remain
written primary inputs; local proof pass is not signed standing.
[pcc:unverified]

## Next and operating state

CT18 next assesses rational odd-prime maximal-isotropic kernels as a route
beyond the closed rational Richelot neighbors. Freeze inputs,read primary
odd-prime polarization/Frobenius results and preserve negative outcomes;
no odd-prime map or benefit pre-accepted. CT14 production/catalog/large
fields and all solver-cost obligations stay open. [pcc:unverified]

Updated .resume/RESUME.md now satisfies exact checkpoint headings. A fresh
S3 gate reaches transport but fails:no AWS profile/checkpoint token.
Both first-format and transport failure receipts retained. No experiment,
benchmark,collector,solver or paid allocation ran. Live allowance unknown;
KB/vault and root frozen-ledger/archive blockers remain. [pcc:unverified]
