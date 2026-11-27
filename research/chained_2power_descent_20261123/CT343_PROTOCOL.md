# CT343 preregistered protocol — cyclic even-order toy-DLP lifts

Date: 2026-11-23 UTC. Frozen parent: main at `da7777094392d3ccd37db78dcfc761c32e7e7813`, merged CT342 PR #5060, CT341 PR #5045, CT339 canonical Python SHA-256 `c798c3201da743091be50e1a3c3cd63200f2e995e72d17991518db3ca5345695`. Producer-only finite positive control, **[pcc:unverified]**. This protocol must be published before executing CT343's implementation or scientific run. The original authenticated 695/701 model, physical norm-point costs and cold solver comparisons belong to other owners; do not touch them.

## Specific hypothesis and known-answer cohort

Use the **unchanged** CT339 Python genus-2 Cantor/elliptic quotient arithmetic over F103 for
`C:y²=prod_{a=1}^6(x-a)`, with basepoint the Weierstrass point W1=(1,0), and the already checked dual map `Phi`. Set `H23=[100]((10,15)-W1)` from the frozen CT339 known-answer row (order 23). Set `K=W6-W1=(6,0)-W1`. Demand `K != 0, [2]K=0, Phi(K)=0, H23 != 0, [23]H23=0`. Set `G46=H23+K`; demand `ord(G46)=46, ord(Phi(G46))=23`.

For a larger compositional check, enumerate rational affine source points in deterministic lexicographic order x from 2 to 102, y from 0 to 102, using the original sextic equation. Select the **first** point P for which `H25=[368](P-W1)` is nonzero with `[25]H25=0` and `[5]H25!=0`. If none exists, fail and retain the negative; do not enlarge the search. Require `H575=H23+H25` has exact order 575, `G1150=H575+K` has exact order 1150, and `Phi(G1150)` has exact order 575. This is a predeclared deterministic source-point selection, not target scalar selection.

For each public scalar `k=0,...,45` and `k=0,...,1149`, construct `Q=[k]G` using the Jacobian law. Supply only `Phi(G),Phi(Q),m=ord(Phi(G))` to the elliptic-product BSGS, **not** k. Demand it return the unique `r in [0,m)`, `r=k mod m`. Check exactly two candidate source scalars `r` and `r+m` (both less than 2m): exactly one reproduces the public source class Q. This is a scalar-blind algorithmic check with known-answer correctness after solving, **not** evidence of hardness or a speed improvement. Verify `Phi(Q)=r Phi(G)` and the false source lift really fails; do not infer injectivity from cardinalities alone. Compare to direct source BSGS only in operation counters if appropriate, with no clock-based claim or unmatched resource ranking.

## Fail-closed guards and immutable source identities

Require normal and Python optimized runs to emit byte-identical canonical JSON with schema `ct343-even-cyclic-lift/v1`, original CT339 canonical source SHA-256, explicit witnesses and counts, primitive counters, and no private data. The source hash must match before loading. Require exact cohort sizes **46 and 1150** and distinct source targets. Require three independent changed-source failures:
(1) drop K (must violate order46 gate);
(2) poison image of one target by nonzero rational 2-torsion (destination BSGS cannot recover it);
(3) skip the source scalar verification and assert two source lifts both valid (must be rejected by the equality guard).
Rejections must be explicit raised failures, never ordinary `assert` disabled under -O. Preserve failed attempts and append any amendments **before** changing the protocol if the hypothesis is wrong.

## Scope, ownership and acceptance

One coordinator and up to three native subagents if available; this connector-only runtime has no provisioned native child-agent spawn, and no Rust compiler. Mathematics: cyclic exact-sequence and subgroup orders; implementation: a distinct CT343 script importing *unaltered* CT339; validation: known-answer oracle, independently review source path and three deliberate mutations. Distinct from CT342's standalone elliptic group census, CT341's complete 9200-J census, the F3/E7 native lane, and the original EKS-only norm-reuse/C169-C13-D benchmark. Research-KB/Conductor not exposed; use CT340–342 and existing Magma Richelot/finite-field-isogeny references as fallback; no novelty claim.

Accept only source-bound frozen finite exact results at this stated producer-only level. Rust/Cantor replay, signed independent PCC, formal group-scheme proof, real ordinary-prime-field ECDLP transfer and fully charged optimized rho/BSGS/IC remain separate. No paid compute, benchmark reservation collision, new automation or protected merge.
