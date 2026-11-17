#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#let s = json("summary.json")
#let fmt(x) = str(calc.round(x, digits: 3))
= Native S4/F4 basis-phase feasibility
15 November 2026 · Source `55b513d1c` · Local pinned CPU0 · Producer observations

The full planned cohort completed: 36 systems, 864 batches, 144 warmups and
720 measured batches. Each batch makes eight phase calls. The process took
15.128 seconds and exited 1 because 48 reference-invalid batches were retained.
There was one measurement attempt and no retry. [pcc:unverified]

Thirty-three of 36 systems pass all controls and have F4-basis/full-reference
wall ratios 0.135–0.173, with upper intervals below 0.5. All 12 p1039 systems
pass. F4 still needs quotient/action matrices, root recovery and group signs.
This basis-phase result justifies testing that extraction. [pcc:unverified]

#table(columns:(1fr,2fr),inset:5pt,
 [Cases], [Retained outcome],
 [p67 / seed61 / residual0], [Reference quotient closure returns None; F4 finishes untruncated, finite quotient dimension 60.],
 [p127 / seed59 / residual3], [Same reference closure failure; F4 finishes untruncated, finite quotient dimension 60.],
 [p127 / seed71 / residual1], [Local median A/A tolerance fails.],
)

All 48 batches on the two reference failures remain invalid for comparison;
the other rejected system remains inconclusive. No native F4 timeout is
observed. These exact inputs are extraction regressions. [pcc:unverified]

== Measurement boundary
Each phase includes Weil restriction and input conversion. Shared symbolic
S4 preparation (9 curves), public residual generation and qualification
(36 calls) are outside the phase and reported in raw fixtures. F4 basis
materialization is inside its phase; staircase enumeration and correctness
checks follow it. The candidate and reference return different kinds of
output. This is a lower-bound feasibility comparison, without a complete IC
pipeline or rho crossover claim. [pcc:unverified]

The graphs on the following pages show geometric means and deterministic
paired-round bootstrap 95% intervals over five measured rounds. These intervals
describe repeated timings on fixed inputs. Green cases pass the predeclared
local controls; gray cases retain failed native/reference or timing conditions.
[pcc:unverified]

Host: AMD EPYC 9V74, one pinned CPU (CPU0), 4-CPU cgroup quota, 8-GiB virtual
ceiling, 900-second whole-run limit. Public engine defaults are pinned and
SIMD/debug overrides cleared. No standard EKS/L2 admission. [pcc:unverified]

== Evidence and source review
`summary.json`, `runtime-plan.json`, `local-evidence.json` and pre/post-call
hashes record the native analysis and flow. Raw/checkpoint/copied binaries
remain under `/workspace/ic-local-runs/local-s4-f4-probe-20261115-08`.
Canonical export validation remains unavailable (encrypted VLT1 source).
Source review used the existing native `f4_fp`, `gaudry_cubic` and residual-walk
note 11.4–11.6. KB is unavailable; primary F4 author PDF retrieval returned 503.
These observations have no independent signed verdict. [pcc:unverified]

#for p in (67,127,1039) {
  pagebreak()
  heading(level:1,[p = #p: basis phase / full coordinate solve])
  [Source `55b513d1c` · Fixed public S4 inputs · Eight-call batches · Five measured rounds]
  image("ratios-p"+str(p)+".svg",width:100%)
  [Batch wall ratios with paired-round bootstrap 95% intervals. The F4 output
  is a basis; complete coordinate/root recovery remains to be integrated.
  Gray cases remain invalid or inconclusive. [pcc:unverified]]
  parbreak()
  table(columns:(1fr,1fr,1.4fr,1fr),inset:3pt,
    [Seed / residual], [Basis / reference], [95% interval], [All controls],
    ..s.cases.filter(r=>r.p==p).map(r=>(
      [#r.seed / #r.ordinal], [#fmt(r.basis_phase_over_complete_reference.geometric_mean)],
      [#fmt(r.basis_phase_over_complete_reference.bootstrap_95.at(0))–#fmt(r.basis_phase_over_complete_reference.bootstrap_95.at(1))],
      [#if r.eligible_local_controlled_comparison {[Pass]} else {[Rejected]}],
    )).flatten())
  parbreak()
  [See the first page for flow, failed cases, accounting boundaries, host,
  source review and local raw custody. Every planned batch remains retained.
  Full numbers and invalid reasons are in `summary.json`. [pcc:unverified]]
}
