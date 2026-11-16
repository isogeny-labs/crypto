#set page(paper: "a4", margin: 19mm)
#set text(size: 10pt)
= PR #1749 review correction
2026-10-19 · implementation and evidence correction

#image("review-20261019.svg", width: 100%)

== Historical evidence boundary
The runner at a4daad2d441d157a32c36bd144fdf4478e7fa690 drew a sign
before checking whether the abscissa lifted. PROTOCOL.md requires the sign
after a successful lift. The source is corrected, with a regression fixing
the third F13 target to (5,7). No replacement experiment occurred.
Frozen raw-v4.tsv, summaries, figures, PDFs and receipts are preserved;
they do not establish compliance with the corrected target stream.

== Catalogue repair
A normal main merge combines identities and regenerates derived catalogue,
cover, trait, transfer, leaderboard, browser and dashboard files.
The complete N73 EC1 record in
research/ic_single_target_20260930/curve_ids.json was omitted by a
filename-restricted harvester. The generator now reads complete records
from curve_ids.json and curve_records.json and rejects a declared UID
that disagrees with its field and curve. Frozen inputs are unchanged.
Regression tests cover retention and corrupted-UID rejection.

== Unresolved merge gates
The denominator-cleared map equation now has a native IDC1 emitter and
checked Lean 4.34.1 ring proofs: degreeThreeCleared and
degreeThreeCurveEquationCleared in CubicAutomorphism.lean. Both sides
expand to $x^9 - 15 b x^6 + 48 b^2 x^3 + 64 b^3$.
Their axioms are propext, Classical.choice and Quot.sound. The IDC1 field
is $2^61-1$; the Lean statements apply to any commutative ring in the
stated interface. Neither certifies the separate finite-group decomposition.
Certificate: IDC1h95b8d6aaf113c1db; 32 certificate points and 32 independent
checker-stream points. The proof-source hash and axiom receipt are linked
in REVIEW-20261019.md.

The paired comparison needs a new versioned native ecbench protocol,
sealed session, verified answers, controls and audit. Its paired
isogeny/scalar decomposition also requires the IDC1 identity certificate
and faithful proof bridge. Other torsion-study certificates do not certify
this identity. Applicable exact-head CI must pass. Recording these gaps
does not make the PR ready to merge.

== Sources and graph impact
REVIEW-20261019.md links the sources, commands and existing obstruction
record, problems/nonrational-isogeny-fiber-dlp-cost-gap.md.
The corrected source is examples/isogeny_fiber_pair_20261012.rs.
Canonical indexes are regenerated without changing measured values,
incumbents or reference costs. Historical paired figures and benchmark
mirrors are preserved and cannot establish a newly admitted comparison.
This diagram records control flow; this PDF reports no new measurements.
