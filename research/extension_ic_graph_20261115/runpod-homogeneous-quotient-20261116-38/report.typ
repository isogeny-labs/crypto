#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:9pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
#let dv(v)=if v==none {[unknown]} else if type(v)==bool {repr(v)} else {str(v)}
= Reuse the homogeneous row basis
16 November 2026 · Trial38 · Runpod CPU · Recorded-matrix component

Source #raw(s.source_revision.slice(0,9)), original source
#raw(s.original_source_revision.slice(0,9)). Six immutable graph matrices
from37, p=#dv(s.p), scalar modulus #dv(s.l). Both modes must determine the
same target #dv(s.actual_scalar). This creates no new DLP sample and does
not reprice37end-to-end. [pcc:unverified]

#table(columns:(80pt,115pt,115pt,100pt),inset:4pt,align:left,
 [Recorded arm],[Baseline modr products],[Reuse modr products],[New word events],
 ..s.outcomes.map(r=>([#dv(r.arm)],[#dv(r.prices.at(0).observed_primitives.modr_mul)],[#dv(r.prices.at(1).observed_primitives.modr_mul)],[#dv(r.prices.at(1).observed_primitives.word_op)])).flatten()
)
#image("component-subtotals.svg",width:100%)

Both modes pay for the same homogeneous checkpoint. The baseline discards
its basis before a full affine RREF. The candidate checks codimension1,
extracts the kernel h with a free coordinate fixed to1, substitutes x=h·t,
and projects affine equations into the two coordinates (t,target). New
kernel negations, dot products/additions and known state events are charged;
the original zero-normalization behavior is preserved. [pcc:unverified]

The copied Rust qualifier uses a column-keyed/u128 RREF and Fermat inverse,
without the driver's Shoup factors or support-list updates. It independently
checks every affine prefix state and semantic counter, the saved checkpoint
rank/arithmetic, unique normalized kernel, all projected rows, actual target,
new event conservation and primitive prices. [pcc:unverified]

Only observed algebra/state-event subtotals are shown. Other native
RREF/Euclidean/fixed-factor/container/control work remains incomplete;
complete costs, bands and ratios remain unknown. Diagnostic hashes and
validation instrumentation are outside algebra subtotals and inside wall.
No wall-speed, full-pipeline, cross-cohort or rho-parity admission.
[pcc:unverified]

One worker/six paired replays, zero warmups/retries;60s worker/180s whole/
900s runner/8GiB. Dedicated4vCPU/8GiB CPU, actual pinned CPU #dv(e.pinned_cpu).
Builds finish before timing; wall is practicality only. Exact original inputs,
raw/qualifier/source/binary/argv/flow/status/hash custody: #raw(e.run_path).
Source/spec/manifests precede the sole dispatch; collection/hash checks
precede owned-pod deletion. Native export claims no unavailable canonical
schema or signed independent execution verdict. [pcc:unverified]
