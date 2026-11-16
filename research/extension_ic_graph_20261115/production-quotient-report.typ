#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:9pt)
#let s=json("presentation.json")
#let e=json("local-evidence.json")
#let dv(v)=if v==none {[unknown]} else if type(v)==bool {repr(v)} else {str(v)}
= Homogeneous-basis reuse in Rust IC
16 November 2026 · Trial39 · Fresh full-pipeline Runpod execution

Source #raw(s.source_revision.slice(0,9)), p=#dv(s.p), seed=#dv(s.seed),
scalar modulus #dv(s.l). Three public-input workers must independently
recover #dv(s.actual_scalar). The fixture generator holds out its planted
answer; solver placeholders are0. One cold round, zero warmups/retries.
[pcc:unverified]

#table(columns:(150pt,260pt),inset:5pt,align:left,
 [Arm],[Observed incomplete primitive subtotal, bitops],
 ..s.outcomes.map(r=>([#dv(r.arm)],[#dv(r.observed_primitive_subtotal_bitops)])).flatten()
)
#image("phase-subtotals.svg",width:100%)

Graph arms use85%small base, rank stride32 and the same structural gate.
The candidate derives a checked codimension-one kernel from the actual ready
homogeneous RREF. It substitutes factor logs x=h·t and solves affine rows in
two coordinates(kernel scale,target), while retaining original relations,
descent equations, affine combination certificates and elliptic replay.
All kernel/projection algebra and known state events are charged; explicit
rank/product scope separates quotient coordinates from the full matrix.
[pcc:unverified]

The copied Rust qualifier independently checks affine certificates/target,
checkpoint ranks and structural prefixes. A column-keyed/u128 oracle
checks the normalized kernel, every actual projected row, target and known
operation counts. It requires exact graph/source/descent/endpoint/projection
identity and explains every primitive phase change as removed original
affine solver/conversion plus added quotient solver/conversion and kernel
projection events. All-phase conservation:
#dv(s.reuse_proof.phase_conservation.all_phase_counts_conserved).
Full affine rank #dv(s.reuse_proof.independent_full_affine_rank), quotient
rank #dv(s.reuse_proof.independent_quotient_rank); rows used
#dv(s.reuse_proof.baseline_rows_used)→#dv(s.reuse_proof.candidate_rows_used).
[pcc:unverified]

Native container/PRNG/RREF/Euclidean/fixed-factor and other bookkeeping
remains incomplete. Totals, pricing bands and ratios stay unknown and every
arm unrankable. This fresh execution is not reconstructed by subtracting38
from37; it admits no fastest-IC, wall-speed or rho-parity conclusion.
[pcc:unverified]

Limits60s fixture/300s worker/1000s whole/1800s runner/8GiB. Dedicated4vCPU/
8GiB CPU, actual pinned CPU #dv(e.pinned_cpu). Builds finish before timing;
wall is practicality only. Exact source/binary/input/argv/flow/status/raw/
qualifier/hash custody: #raw(e.run_path). Source/spec/manifests precede the
sole Runpod dispatch; collection/hash checks precede owned-pod deletion.
Native export claims no unavailable canonical-schema or independent signed
execution verdict. [pcc:unverified]
