#set page(paper:"a4",margin:(x:15mm,y:14mm),numbering:"1")
#set text(font:"DejaVu Sans",size:9pt)
#set par(leading:.5em)
#let d=json("analysis.json")
#let v=json("validation-summary.json")
#let label(s)=s.replace("-membership","").replace("-branches","")
#align(center)[#text(size:18pt,weight:"bold")[Three factor-base oracle leads]\
Asymmetric charts, selectors and checked torsion quotients\
2026-10-19 · native bounded run and results report]

= Actual point decompositions, with limits retained
The frozen experiment contains six cases on two exact models. It records
#d.trial_rows bounded trial/arm rows. Separate native field/group arithmetic
checks #d.original_point_witnesses returned original-curve witnesses and
#d.algebraic_roots algebraic roots. Every emitted witness sums to its original
target. Missing completions remain missing; no total-cost winner is admitted.

Exact models: `icv1-f2m7-tm13-ac10a42c` (r=71, h=2) and
`icv1-f2m13-t181-515ee569` (r=2003, h=4). EC1, full ICV1, field polynomial,
generator and source FB1 identities are retained in the copied input files.
The two quotient controls use C={±G, ±[2]G}; they are mechanism controls,
not selected large-field factor bases or evidence of unknown-scalar recovery.

#table(columns:(1.1fr,.8fr,.8fr,.8fr,.8fr,1fr),inset:3pt,
[*Case*],[*A points*],[*B points*],[*Union*],[*Projected cols*],[*Support*],
..d.cases.map(c=>([#{c.id}],[#{c.left_points}],[#{c.right_points}],[#{c.signed_points}],[#{c.columns}],[#{c.support}])).flatten())

#image("positive-controls.svg",width:100%)
Positive controls are conditioned supported targets, not yield samples.
All random arms share 12 training targets and 12 disjoint holdout targets.
Controls and holdout cannot increase training rank.

#pagebreak()
= Successful solver-component work
#image("f4-component-work.svg",width:100%)
The bars count F2 row-reduction multiplications from the unmodified native
F4 engine. A bar is shown only for a completely successful positive panel;
an incomplete panel is explicitly omitted, not assigned zero work. Extra
selector variables and every attempted asymmetric branch are retained.

This counter excludes parts of symbolic preprocessing, finite-root search,
allocation, hashing, setup, verification and rank. F4 solving includes a
substitution/enumeration procedure; its productive degree is not certified
degree of regularity. The positive panels are tiny and conditioned, and no
statistical significance or large-field extrapolation is claimed.

Every call uses grevlex, max degree4, a 750ms cooperative budget, a 2000ms
hard allowance per permitted branch, one Rayon thread, one million observed
matrix cells and a 1024MiB Linux address-space limit. Hard-stopped counters
are incomplete and stay in raw events. No budget or assertion was relaxed.

== Positive panels, including the A/A repeat
#text(size:6.7pt)[
#table(columns:(1fr,1.6fr,.5fr,1fr,1fr,.6fr,.5fr),inset:2.5pt,
[*Case*],[*Arm*],[*Hits*],[*F4 mults*],[*Encoding products*],[*Limits*],[*Degree*],
..d.cases.map(c=>c.aggregates.pairs().filter(kv=>kv.at(0).starts-with("positive-control/")).map(kv=>{
let a=kv.at(1)
([#{c.id}],[#{label(kv.at(0).split("/").last())}],[#{a.verified}/#{a.trials}],[#{if a.partial_counters { "partial" } else { str(a.field_reduction_multiplications_completed_calls) }}],[#{if a.partial_counters { "partial" } else { str(a.encoding_term_products_completed_calls) }}],[#{a.resource_limited}],[#{if a.verified == 0 { "—" } else { str(a.max_successful_observed_degree) }}])
})).flatten().flatten())]

#pagebreak()
= Fresh random probes and projected training rank
All arms receive the same original point targets within each case. Training
and holdout are disjoint, but bases were selected using earlier studies.
There is no claim of an untouched discovery population. Rank uses distinct
nonzero signed cofactor projections: kernel translates do not create fake
independent unknowns. No point logarithms are solved.

#text(size:7.5pt)[
#table(columns:(1fr,1.6fr,.7fr,.65fr,.65fr,.75fr,.65fr,.65fr),inset:2.8pt,
[*Case*],[*Arm*],[*Phase*],[*Hits*],[*Absent*],[*Unresolved*],[*Limits*],[*Rank gain*],
..d.cases.map(c=>c.aggregates.pairs().filter(kv=>not kv.at(0).starts-with("positive-control/")).map(kv=>{
let a=kv.at(1)
([#{c.id}],[#{label(kv.at(0).split("/").last())}],[#{kv.at(0).split("/").first()}],[#{a.verified}/#{a.trials}],[#{a.absent}],[#{a.unresolved}],[#{a.resource_limited}],[#{a.rank_gain}])
})).flatten().flatten())]

Absence is accepted only when it agrees with exhaustive original-inventory
point truth. An exhausted F4 call is not a refutation. A hard resource stop
is not a completed solver call. Full systems, roots, target scalars,
witnesses, partial events and independent audit fields remain available.

#pagebreak()
= What the three paths actually do
#image("route.svg",width:100%)

*Asymmetric:* solve A+A, A+B, B+B in fixed size-product order, stopping only
after a checked witness. Commutativity covers B+A; no same-base branch is
silently omitted. Compare against the same union's product-membership arm.

*Selector:* use an independent Boolean selector for each summand and guard
both component membership equations. The accepted domain is exactly the
union, not its unrestricted linear span. Both encodings still need actual
point witnesses after S3 roots.

*Quotient:* evaluate dual Frobenius as doubling followed by inverse
Frobenius. Solve on C+C at the mapped target, lift through complete
preimages, correct the small-torsion discrepancy and verify P+Q=R. The
cofactor4 case does not assume membership in 2E implies membership in the
chosen prime subgroup. Setup, transport and lifting are not free.

== Native CLI
```sh
factor-base-sweep compare-leads \
  --plan research/factor_base_oracle_leads_20261019/plan.json \
  --out runs/oracle-leads
factor-base-sweep verify --run runs/oracle-leads
```
The executable runs its own native workers. No Cargo launcher is used at
runtime. Comparison outputs are write-once; use a new directory, never append
to this frozen run. Old executables remain necessary for byte-bound resumes.

#pagebreak()
= Validation, accounting and unresolved claims
Executed source/protocol/input lock:
`aaf9e246a3af4be371d7089dc86fc32c61c2f5ac`.
The director compiled a hash-bound card; the mechanical tier ran its exact
commands; the director assessed the verbatim relay. Canonical check-relay
returned exit1 because reloaded output keys sort differently; separate
read-only verification confirms exact runner-order bytes and all bindings.
The failed canonical result remains retained. Tool elapsed times are
resource/receipt observations, not accepted benchmark timings.

One complete serial release-library command: #v.library.passed passed,
#v.library.failed failed, #v.library.ignored existing ignores, zero filtered.
The unchanged library binary was reused, not freshly compiled. Its tracked
dependencies and exact embedded traits fixture are hash-bound in the raw
binding receipt. The final CLI has 12 passing unit tests and seven real
subprocess integration tests; independent audit has five passing tests.
Strict CLI/audit Clippy, all-target compilation and nine CI-scope tests pass.

*Independent replay:* #d.members sealed members,
#d.unique_case_unordered_pair_checks case-wise unordered pair checks and
#d.lift_preimage_checks quotient preimage checks. Separate arithmetic on
this same host is not an independent-host certificate or a second F4 engine.

#table(columns:(1fr,1.5fr,1.5fr),inset:4pt,
[*Native-operation weight*],[*Complete total cost*],[*Total-cost ordering*],
[0],[Unknown],[Unknown],
[0.1],[Unknown],[Unknown],
[1],[Unknown],[Unknown])
This symmetric band applies to references too. Missing emissions and
calibration prevent ranking every arm. Magma F4/F5, msolve/FGb and applicable
Singular engines were not found on PATH; the matched external comparison is
unrun. No fastest implementation, accepted speedup, full-DLP S, rho win,
certified regularity or83/130-bit transfer is claimed.

Conductor returned127 and its pinned workflow404. Lean/Lake returned127.
These are unavailable checks, not passes. Authoring compilation/formatting
failures and all research resource stops are preserved. No historical
evidence, draft card, root dependency graph, solver limit or old assertion
was rewritten. No S3 or new paid host.

== Evidence and prior work
The source-linked README, PROTOCOL.md, PRICING.md, VALIDATION.md,
FORMALIZATION.md, copied inputs, card/receipt/assessment and raw archive
define the run. Analysis is generated natively from the extracted records;
Python renders recorded fields only. Existing scoreboard/leaderboard/browser
data are unchanged: there is no admitted IC/rho ratio, newly priced phase,
cross-method session, registry addition or complete pipeline result.

Prior quotient and selector derivation:
`research/sat_factor_base_review_20260908/continuation-01/EXACT_FACTOR_BASE_RESULTS.md`.
Chart input provenance:
`research/factor_base_chart_complements_20261019/README.md`.
Primary torsion-symmetry context and access limits are documented in
LITERATURE.md. All source/evidence hashes and reproduction commands remain
linked from this study; existing draft follow-ups stay draft and unrun.
