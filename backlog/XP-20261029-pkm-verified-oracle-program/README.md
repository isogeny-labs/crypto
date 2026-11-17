# Symmetric PKM: complete verified oracle program

This is a blocked planning card, not a runnable experiment. The goals, prior-work
audit, proposed 20% cost and 2x memory targets, controls, sequencing and stop rules
are in [PKM_GOALS.md](../../research/prime_ecdlp_three_leads_20261029/PKM_GOALS.md).

G1 checks actual supported point decompositions; G2 prices construction through
verification; G3 separates chain removal, symmetry and solver effects; G4 tests
one measured bottleneck improvement; G5 checks depth/resource growth; G6 connects
useful relations, sparse LA and target recovery to cold IC cost. Each later
stage depends on the earlier verification and measurement gates.

The older XP-20261015-pkm-m3-n21-m4-n20 card concerns raw degree reach and remains
historical. Its m=4,N=20 cell already reached productive degree 8 rounds. This
new card asks about full verified cost and improvement, including the symmetric
presentation's construction/extraction costs. It does not repeat that old cell
as an untested novelty claim.

Prerequisites: merge/review the producer, implement the named native full-cost
panel and independent checker, qualify equal-system external solving, freeze
the experiment protocol/inputs/caps, provide checkpoint/artifact transport, and
qualify the supported execution/resume route. The future runner named in
card.json does not exist yet. The source commit is an input reference, not a
claim that this card can execute at that commit. The verifier list and resume
fields remain empty because neither capability is implemented for the panel.

The 1 CPU, 16 GB RAM, 16 GB disk and 1 hour fields are proposed small-pilot ceilings.
Runtime estimates are null: there is no qualified cost pilot from which to
derive them. They do not extrapolate the old solver-only timings or assume that
N=20 fits. Old plain-arm 256–512GB projections exceed the default lab pool's
128GiB cap. Funding is 0; no task, lease, objective, schedule, capacity change or
reward is posted. Reviewer approval of a future ready card sets any priority.

Done for this planning card: native validation succeeds and all missing pieces
are explicit. Done for execution: the goals' frozen acceptance rules, verified
outputs, full cost/reference comparisons and evidence land in a follow-on PR.
Merging the plan does not close the research workstream.
