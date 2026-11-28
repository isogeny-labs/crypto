#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:10pt)
#let s=json("summary.json")
#let e=json("local-evidence.json")
= MST cover IC: candidate refused, recovery succeeds
16 November 2026 · Public synthetic weak Fp6 family · p1009, seed263

Both fresh public-input solves recover the held-out scalar after native and
outer elliptic replay. One call, exit0 in27.517seconds, source
#raw(s.source_revision.slice(0,9)). The MST arm uses the original-solve
fallback, so the frozen projection criterion fails. [pcc:unverified]

#table(columns:(1.6fr,1fr),inset:6pt,
 [Planned / started / completed workers],[2 / 2 / 2],
 [Actual verified scalar recoveries],[2],
 [Returned scalar],[66296859173437586],
 [MST input rows / nonzeros],[509 / 4577],
 [Candidate rows / nonzeros],[382 / 10904],
 [Eliminated columns / trees],[127 / 127],
 [Candidate edges / selected edges],[19297 / 1703],
 [Maximum column degree],[51 of cap64],
 [Resource refusals],[0],
 [Proxy-guard refusals / original-solve fallbacks],[1 / 1],
 [Admitted / replayed projected rows],[0 / 0],
 [Frozen nontrivial-projection criterion],[Failed],
 [Comparative eligibility],[False],
)

The proxy compares dimension times nonzeros: input #raw("509 × 4577"),
candidate #raw("382 × 10904"). Fill outweighs the smaller dimension under
this proxy. The guard rejects the candidate before witness replay or solving;
its rows are not admitted. All planning/refusal/fallback work remains inside
recorded cold wall. A work proxy does not establish a wall-speed result.
[pcc:unverified]

The scalar result comes from the original selected system, which is solved
and checked normally. The full validation flag is false because a nontrivial
accepted projection without fallback was required, despite both workers
recovering correctly. No ratio, interval or rho arm exists in this once-only
endpoint design. [pcc:unverified]

This result concerns forced75percent retention and this preselected challenge.
It does not test an adaptive stopping rule or a different retained-column set.
The original rule stays frozen; no unchanged retry or full timing follow-up.
Existing unit checks establish their tested algebraic scope, not a checked
projection for this refused matrix. [pcc:unverified]

Evidence: exact original Rustsummary/raw/progress and pre/post hashes, under
#raw(e.run_path). Legacy primitive totals omit graph work; no primary
counted-cost, independent signed, canonical-schema, standard lab or admitted
ranking assertion. [pcc:unverified]
