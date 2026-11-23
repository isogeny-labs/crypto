#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:10pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
= Weak-cover IC: recovery validation
16 November 2026 · Public synthetic weak elliptic family over Fp6

Two fresh workers complete under source #raw(s.source_revision.slice(0,9)),
exit0in40.445seconds. One attempt, zero retries. Both actual scalars match the
held-out answer after native and outer elliptic replay. [pcc:unverified]

#table(columns:(1.3fr,1fr),inset:6pt,
 [Endpoint protocol],[p1009 / seed251; IC then graph],
 [Planned / started / completed],[2 / 2 / 2],
 [Returned scalar],[72041284332044320],
 [Prime subgroup order],[263807420025969577],
 [Native verified relations],[487 each, m9],
 [Descent residuals],[704 each],
 [Graph input / output rows],[488 / 366],
 [Replayed output rows],[366],
 [Graph pivots / merges],[122 / 1094],
 [Refusals / fill rejects / fallback solves],[0 / 0 / 0],
 [Full recovery endpoint criterion],[Pass],
 [Comparative eligibility],[False],
)

The graph target remains protected and every projected row is recomputed
from original-input witnesses before solving. Both public-input APIs receive
zero in the legacy answer field; their known-answer flags are expected false.
Actual recovery and full-group replay determine acceptance. [pcc:unverified]

This is a once-only endpoint validation with no A/A controls or rho arm.
Recorded process walls are spent-work diagnostics; no ratio or interval is
estimated. Native rho-related report fields use analytic/placeholder inputs,
not a measured reference. Complete graph costs are charged in wall; legacy
primitive totals omit graph/wrapper work. [pcc:unverified]

The existing capacity model selects p1009 from the frozen candidate list;
it does not guarantee independent rows or runtime. Both methods explicitly
use the existing incidence/shared-table options. The earlier p127pilot's
900-second censor and failed gate remain unchanged. Success here permits
a new separately frozen comparison, not a retrospective scale-gate pass.
[pcc:unverified]

Evidence: byte-exact raw and original Rustsummary, progress logs, runtime plan,
pre/post hashes and local-evidence.json. Cgroup CPU snapshots are inclusive
environment diagnostics. Original binaries, source checkpoint and host facts
remain in #raw(e.run_path). No independent signed verdict, standard lab,
canonical-schema or admitted ranking assertion. [pcc:unverified]
