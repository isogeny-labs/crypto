#set page(paper: "a4", margin: 14mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
#set par(leading: 0.42em)
#set heading(numbering: none)
#let data = json("../validation/summary.json")

= Independent APS transcript replay
Classical bounded correctness · 2026-10-29

*Observed result.* Fourteen actual native producer transcripts replay
successfully across twelve supported arms and three fixed public curves.
The same nine replay/corruption tests pass in both the standalone package
and repository-example build. This is verification support, not a speedup
or a new relation-finding method.

== Verification contract
The replayer uses separate ordinary-residue affine arithmetic. It links
serde, serde_json, clap and sha2, with no producer curve, solver, recovery
or Montgomery implementation. It binds the fixture/model/ICV1 identity,
counts the tiny curve's points, checks each labelled point, and replays
the supplied coordinate/sign witnesses. Recovery requires the recomputed
signed coefficients, nonzero B, the scalar congruence, and dG = Q.
Orbit pruning additionally requires independently checked label propagation.

#align(center)[#image("replay.svg", height: 91mm)]

*Boundary.* A checked witness does not prove that the solver supplied every
root. Sampling, draw history, deadlines, symbolic work and operation costs
remain uncertified. A no-witness receipt reports zero recoveries; producer
status and completeness flags do not become an absence proof.

== Sources and provenance
#link("https://iris.unitn.it/retrieve/e3835194-60e8-72ef-e053-3705fe0ad821/Amadori%2C%20Pintore%2C%20Sala%20-%20On%20DLP%20for%20prime-field%20EC%20%28rev%29.pdf")[APS revised manuscript, sections 3–3.1]:
known labels, signed zero sum, partition constraints and scalar recovery.
#link("https://www.jmilne.org/math/Books/EC2.pdf")[Milne, Elliptic Curves, I section 3, pp.25–28]:
group law, inverses and infinity. Native interfaces and limitations are
recorded in the earlier APS follow-up and complete-cost problem.

Tested producer base: #text(size: 8pt)[117c9c4f3ee85d4dc32f275893f6e50b591e4a47].
Per-input SHA-256 and native replay receipts are in validation/replays/;
source hashes and actual check outcomes are in validation/summary.json.
Third-party papers and broad build/infrastructure logs are not redistributed.

#pagebreak()
= Corpus, controls and remaining work

#table(
  columns: (2.3fr, 1fr, 0.7fr, 0.9fr, 0.8fr),
  inset: 4pt,
  stroke: 0.35pt + luma(180),
  table.header([Case], [Fixture], [Attempts], [Sets], [Witnesses]),
  ..data.corpus.cases.map(r => (
    text(size: 8pt, r.name), text(size: 8pt, str(r.fixture)),
    str(r.attempts_checked), str(r.coordinate_sets_checked),
    str(r.recoveries_verified)
  )).flatten()
)

Counts are per recorded input. These are curated fixed correctness controls,
not an unbiased yield population or a timing panel. Failed search attempts
inside the emitted transcripts are retained and checked.

== Exact curve identities
#for id in (
  "ICV1:fp-101:-5:107:36:unk:unk:r:9ced75c2406a",
  "ICV1:fp-211:-15:227:153:unk:unk:r:9004a75cb03c",
  "ICV1:fp-103:7:97:0:unk:unk:r:394a8c88b017",
) [
  #text(font: "DejaVu Sans Mono", size: 7.7pt, id) \
]

== Negative controls and retained failures
Tests reject changed points, labels, scalar representatives, curve/model
identities, targets, recovery coefficients and signs, coordinate membership,
duplicate keys/sets and contradictory outcome flags. A special orbit control
changes valid labels while preserving the points and selected witness;
the independent propagation check refuses it.

Initial generation completed nine cases, then the default guaranteed-label
orbit case hit the producer's short/overlapping-orbit guard before JSON
emission. Four later cases were not attempted. The three final orbit cases
use the existing producer unit test's random-label control with the same
seed. The first nine files were preserved; five remaining cases completed.
The refusal and initial lint correction remain in validation/failures.json.
Pre-transcript refusals still require an outer experiment export.

== Scope after this change
The bounded independent-witness replay gap is addressed. Full accounting,
ecbench/export integration, broader parameter support, a frozen protocol,
matched references, checkpoint authorization and controlled performance
validation remain open. No performance run or historical Python research
path was executed, and no full library-suite pass is claimed.

All nineteen historical archive hashes still match. Canonical IC leaderboard,
scoreboard, progress timeline and performance-gains values are unchanged:
this work supplies no admitted measurement or new curve route.
