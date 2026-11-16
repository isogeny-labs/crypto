# Target Frobenius checkpoint: distinction and forced backtrack

Producer cryptanalysis63cf80560b8339b9ea67629746131f5c6f722299,
branch research/chained-transfer-target-frobenius-20261111. Source-linked
[account](https://github.com/isogeny-labs/cryptanalysis/blob/63cf80560b8339b9ea67629746131f5c6f722299/experiments/chained-transfer-reference/TARGET_FROBENIUS.md),
[diagram/inspected two-page PDF](https://github.com/isogeny-labs/cryptanalysis/blob/63cf80560b8339b9ea67629746131f5c6f722299/experiments/chained-transfer-reference/target-frobenius-report.pdf).
Prior shifted source767c945a0 passes exact-head focused43022364261:
native25/reference46,1 old benchmark ignored,formatting,strict Clippy and
source guards(PCC46a880). PR1479 merged3e76a274f at12:48:32Z.
Current CT16 full47-test focused replay remains pending. [pcc:unverified]

## Exact mathematical outcomes

For both accepted targets,p17/shifted101,all three target H discriminants
are nonsquare in the checked cubic field. Actual six-root splitting in Fp⁶
gives Frobenius[1,2,3,4,5,0]. Every15 unordered branch-pair class is checked
valid,distinct,nonzero and double-zero in each source/target Jacobian.
Together with identity these exhaust geometric2-torsion. Source/target
Frobenius³-fixed cardinalities are16/4; all are Frobenius⁶-fixed.
Completed branch fibers have kernel4/image4; all256 torsion additions and
Frobenius-equivariant branch-image checks pass. Discovery/regression/Clippy
receipts and PCC954795/4823b5 remain unsigned. [pcc:unverified]

In odd characteristic genus2 has16 geometric2-torsion points,with15 unique
nonzero unordered branch-pair descriptions. This written standard geometric
input identifies the computed fixed sets with2-torsion over Fp³. Any
unpolarized Fp-abelian-variety isomorphism induces a Frobenius-equivariant
bijection on these fixed sets,contradicting16≠4. Thus both accepted maps
reach Fp-nonisomorphic Jacobians. This strengthens the earlier curve-level
distinction; it is not an extension-field nonisomorphism assertion.
[pcc:unverified]

All15 target perfect matchings have exactly one Frobenius-stable member:
opposite pairs(03),(14),(25),also directly identified from the three H
factors as the previous-dual kernel. A nonopposite edge has six translates,
too many for a three-edge matching. Thus a next rational Richelot step is
forced to backtrack,and its composition is[2]. Preserve this exact-model
negative; it excludes neither other degrees,products,extension kernels nor
other starting covers. No further nonbacktracking hop accepted.
[pcc:unverified]

Seven TargetFrobenius.lean lemmas pass gate7/PCC8c5ee5: finite cube/fixed-pair/
stable-matching certificates and generic commuting-function/right-inverse
obstruction. Total55 written lemmas,gates22+8+7+12+6+5+7+4+7.
Finite fields,Jacobians and the branch-pair geometry remain written inputs;
local proof pass is not signed scientific standing. [pcc:unverified]

## Next route and cost obligations

CT16 is closed for these rational Richelot neighbors. CT17 next derives an
explicit full rational-group decomposition into the two elliptic quotients
for the accepted odd-order bielliptic sources. Test reciprocal coordinate
completion,all class/dual and source-scalar controls. This can expose direct
cost controls and prevent mistaking ambient Jacobian size for an advantage;
no full split implementation is accepted yet. CT14 production/catalog/large
fields stays open. Benchmark/solver comparison remains blocked by S3/live
allowance. KB/vault and crypto ledger/archive constraints are unchanged;
no canonical cost edge promoted. [pcc:unverified]
