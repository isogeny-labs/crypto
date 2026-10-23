# E29 finite projection and E23 helper replay, version 1

Freeze date: 2026-10-23. Commit the native source, protocol and supporting
checks before the panel. This is a bounded correctness pilot. Formal
coordinate/group bridges, complete cost and the full E23/E29 proposals
remain open.

For E29 enumerate all extension points on p=59 and the three original
even/composite controls. Verify native conjugation against actual p-th
powers of both coordinates, Frobenius squared, the standard twist embedding
(x/d, y*w/d^2), every pair in the rational twist's group law, every
rational/embedded-twist mixed pair, and the trace equations. Where the two
group orders are coprime and the source order is odd, check the CRT source
projector on every extension point against half its trace. Use a checked
integer modular inverse, including the composite-order control; Fermat
inversion is not valid modulo a composite group order. Even controls must
exhibit a nonzero common two-torsion mixed relation. Reject the wrong
Frobenius sign and the missing trace factor two; classify noninvertible CRT
denominators explicitly. Independently replay the twist embeddings and
every twist-addition pair in PARI/GP. Use the native hash/error adjudicator.

For E23 consume the committed E21 census. Freeze its two training-selected
configurations, linear-l1-h4 and linear-l3-h4, before generating any E23
target. These choices are informed by E21 training outcomes, not by E23
outcomes. Preserve E21's ten fixtures, B=8,12,16 and base tuple arities 2,3;
inspect helper caps 16,32,64. Share identical projected profiles explicitly.
Retain insufficient bases/helpers and noncoprime projectors as unsupported.
Use E23-domain targets with E21's exact hash/rejection rule: labels 0–15 on
structural/rep-0 fixtures and 16–31 on rep-1. Do not generate final labels
32–63 or provide generation scalars to collection.

Choose distinct canonical rational helpers H_i outside the base in lexical
order, and the first nonzero embedded rational twist point U. Verify its
source projection is zero, and the projection of H_i+U is H_i. Reject a
fake nonzero twist-log column. Compare four arms: no helper; the cancelling
pure pair U,−U; general extension pairs (H_i+U),(H_j−U); and rational pairs
H_i,H_j, with i<=j in lexical order. Helper columns are their exact source
projections. The last two arms use the identical projected graph, queries
and stopping rule. They append two helpers to the declared base tuple
arity; no claim of equal total arity with the zero-helper arm is made.

Enumerate the signed base tuple index, charge its full cold construction
separately, and query in order the identity, public generator and target.
Verify each accepted equation in the whole extension group before matrix
insertion. Reduce verified projected rows over the source prime. Stop when
the target unit vector is in their span; verify the recovered scalar and
independently replay every resulting echelon equation in the source group.
Store raw projected equations, concrete base/helper identities, row hashes,
graph edges/components, echelon rows, rank, modular row operations, failure
status and scalar verification. Exact row hashes and scalar outcomes must
match between general-extension/rational helpers and between no-helper/pure
twist. The pure arm uses one representative cancelling pair after merging
its zero source projection; this is not an exhaustive search of twist pairs.

Each arm/target has a 120-second unranked fail-safe, at most 65,536 stored
partial equations and 2^20 counted collection group additions, including
binary scalar-multiplication additions used in row replay. Record any cap
as an unresolved span. Count final-echelon replay separately. The signed
tuple index, helper construction/projectors, field work, allocation and
complete end-to-end accounting are separate and still incomplete; unpriced
reference calls are not group-equivalent cost. A global 3,600-second command
cap preserves every completed checkpoint and any partial batch. Run one
thread with a 4-GiB address-space limit. No speed, novelty, NIST-DLP or
cross-method comparison is admitted.
