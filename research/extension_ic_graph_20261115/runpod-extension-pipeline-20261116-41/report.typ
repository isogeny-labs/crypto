#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:9pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let dv(v)=if v==none {[unknown]} else if type(v)==bool {repr(v)} else {str(v)}
#let pc(r,name)=r.accounting.phases.find(p=>p.phase==name).diagnostic_priced_bitops
= Quadratic remainder graphs in Rust IC
16 November 2026 · Trial41 · Fresh full-pipeline Runpod execution

Source #raw(s.source_revision.slice(0,9)), p=#dv(s.p), seed=#dv(s.seed),
subgroup order #dv(s.l). Seven public-input cold workers independently
recover #dv(s.actual_scalar), followed by elliptic verification. The fixture
holds out its planted answer and worker Spec.d is0. One cold round,
zero warmups/retries. [pcc:unverified]

#image("primitive-subtotals.svg",width:100%)

The optional production path retains seven-point principal relations plus
an exact quadratic remainder over Fp². Signed canonical Mumford classes
form graph vertices; repeated classes produce small-only relations with
original-source cancellation witnesses. Actual homogeneous rank at
complete1024B batches supplies the checked normalized kernel. Existing
Nagao decomposition and verified six-point descent feed protected
(kernel scale,target) affine coordinates, then actual elliptic replay.
[pcc:unverified]

#table(columns:(123pt,101pt,101pt,91pt),inset:4pt,align:left,
 [IC method],[Relations bitops],[Descent bitops],[LA bitops],
 ..s.outcomes.slice(0,3).map(r=>([#dv(r.mode)],[#dv(pc(r,"relations"))],[#dv(pc(r,"descent"))],[#dv(pc(r,"linear_algebra"))])).flatten()
)

The paired homogeneous rank is #dv(s.outcomes.at(2).collection.counts.rank),
at #dv(s.outcomes.at(1).collection.counts.b_count) B polynomials for full
smooth and #dv(s.outcomes.at(2).collection.counts.b_count) for remainders.
Candidate class collisions: #dv(s.outcomes.at(2).collection.counts.collisions).

The two new producer arms have identical normalized kernel, descents,
affine rows and post-collection LA primitives. Every other observed
primitive phase apart from relations and collection LA also matches:
#dv(s.paired_pipeline.all_other_primitive_phases_identical).
Each original principal/source/Cantor relation, class-cancellation witness,
rank/product prefix, descent equation/residual label, kernel/projection
and affine target prefix is checked independently. An unreduced full-matrix
solve and elliptic replay agree; all observed LA primitive charges are
conserved. Four registered native rho configurations bind executed options,
calibration, phase counts and target verification. [pcc:unverified]

Original IC retains its existing random-stream recipe; paired producer
arms share their separate base/enumeration/extraction/descent streams.
Costs shown are measured incomplete primitive subtotals. Missing native
container/control/scan/PRNG/fixed-factor/hash/serialization counts keep
all arms unrankable, with null complete costs, bands and ratios. The
four-configuration reference scope establishes no global rho optimum.
This planted validation is not an unknown-log T1 sample. [pcc:unverified]

CPU4vCPU/8GiB at0.12USD/hour, actual pinned CPU #dv(e.pinned_cpu).
Limits60s fixture/600s worker/4500s whole/900s qualifier/7200s runner.
Builds finish before timing; wall is practicality only. Source/spec/binary
custody precedes the sole dispatch, and collection/hash checks precede
owned-pod deletion and independent absence. Exact originals:
#raw(e.run_path). Native export claims no unavailable canonical-schema or
independent signed admission. [pcc:unverified]
