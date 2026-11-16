#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:9pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let dv(v)=if v==none {[unknown]} else if type(v)==bool {repr(v)} else {str(v)}
= Quadratic extension remainders in graph IC
16 November 2026 · Trial40 · Rust relation-collection component on Runpod

Source #raw(s.source_revision.slice(0,9)). Fixed public p=1009 curve from39,
new seed379, degree9, full Fp factor base. Two cold producer processes; no
planted scalar supplied. Both insert actual homogeneous rows and stop at
rank base_size−1 only after a complete1024B batch, or at the fixed B/source
budget. One cold round, no warmups/retries. [pcc:unverified]

#table(columns:(127pt,77pt,72pt,70pt,70pt),inset:4pt,align:left,
 [Arm],[B enumerated],[Full rows],[Partial rows],[Collisions],
 ..s.outcomes.map(r=>([#dv(r.mode)],[#dv(r.counts.b_count)],[#dv(r.counts.full_rows)],[#dv(r.counts.partial_rows)],[#dv(r.counts.collisions)])).flatten()
)
#image("rank-and-cost.svg",width:100%)

The candidate retains seven-point principal relations plus a verified
quadratic remainder D=(U,V) over Fp², with U irreducible over Fp and
V=−A/C modU. Its exact certificate checks F=A²−C²H, distinct supported
roots, point ordinates and denominator coprimality. Signed canonical D/−D
classes form graph vertices; repeated classes produce small-only rows
with two-original-source cancellation witnesses. Full rows still receive
the existing Jacobian check. [pcc:unverified]

The copied qualifier checks every original using independent Cantor group
addition, every principal/source binding, every class cancellation and small
projection, and every modular rank/product prefix using u128 arithmetic
and Fermat inverses. Both cold model/base inputs agree; matched complete
line-cohort prefixes: #dv(s.common_complete_batch_prefixes).
#for r in s.outcomes [#linebreak()#dv(r.mode): rank #dv(r.counts.rank) (target
#dv(r.rank_target)), #dv(r.counts.vertices) remainder vertices,
#dv(r.verified_originals) originals and #dv(r.verified_projections)
projections checked. Endpoint: #dv(r.stop). ]
[pcc:unverified]

Counts are measured; rank curves copy recorded complete-batch checkpoints.
Primitive subtotals are incomplete lower bounds, with native container,
control/scan/PRNG/fixed-factor and hashing/serialization gaps. Complete
costs, pricing bands and ratios stay unknown. Extra partial rows alone
establish no pipeline gain. This component recovers no new discrete log.
[pcc:unverified]

4vCPU/8GiB CPU at0.12USD/hour, actual pinned CPU #dv(e.pinned_cpu),
300s/worker,700s whole,900s qualifier,1800s lifetime. Builds finish before
timing; recorded wall is practicality only. Source/binaries/protocol/input
precede the sole dispatch. Exact raw/argv/flow/status/qualifier/hash custody:
#raw(e.run_path). Collection precedes owned-pod deletion and independent
absence check. Native export claims no unavailable canonical-schema or
independent signed admission. [pcc:unverified]
