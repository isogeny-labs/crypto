# Independent original-G transition review

The candidate passed an independent clean-copy locked/offline rebuild and replay (exit 0, build 7.05 seconds, 120-second command cap). Replay stdout is byte-identical to the producer's retained output. All 31 final manifest entries and all 16 pre-execution manifest entries match their recorded hashes. The fixture bytes equal the prior 21-model fixture file. The review plan preceded source/results inspection; this review was not source-blind after that point. Producer files were unchanged.

## Checked result

Recomputed from raw JSON: each arm has 63 transitions over 21 distinct workload IDs and both have 126 matching projected corners. Labels were independently recomputed from (0,1) by (a,b)<-((2a+c) mod r,2b mod r) for every consecutive row. Candidate exact signed recovery is 63/63: 61 computed, two degree1_unique_signed_lift. Eager helper exact recovery is 61/63, with two general_conversion_inapplicable outcomes retained. Neither arm records successful repair roots or labelled fallback. Candidate states, labels, branch choices and selected slots equal eager values. The fixture primes range from 739 to 2972219.

## Source checks and mathematical correspondence

pruned.rs813+ and eager.rs768+ explicitly assert valid G and Q, rG=rQ=0, G+G nonzero, 2<=r<=2^63 and canonical labels before each step. DERIVATION.md gives the independent group identity and exact unsigned no-overflow bound. These checks repair the earlier missing subgroup hypothesis for this fixture. They establish annihilation, not an independent proof of exact prime order; annihilation suffices for the label congruence used here.

advance receives validate_transition=false. Its normal selected-pair path returns without constructing expected signed points. The fixture then calls signed::lift(g,nk,nkp,kg,...) directly on propagated output before constructing aG+bQ or projecting its expected neighbour. Source inspection found no planted target scalar read in the transition/recovery path. Signed reconstruction from public G,Q and labels exists in fallback, but fallback counters are zero in all retained rows. Thus this fixture closes the earlier supplied-expected-neighbour dependency for the observed normal original-G transitions.

The two-call path evaluates the selected double and mixed addition in the same retained order as eager. New evaluated and selected masks distinguish the skipped slot. Candidate degree1 lifting uses the propagated plus neighbour to disambiguate signs; the final exact signed comparison is a later check rather than a recovery input.

## Scope and remaining coverage

No blocking discrepancy was found in the reported bounded transition/recovery claims. The checked trajectory is the labelled Kummer state and its signed representative. This fixture does not invoke relation acceptance, graph insertion, elimination, or a full collector stopping rule; it must not be described as verifying relation-graph trajectory or target solves. The report correctly calls the two eager outcomes helper failures, not failed collector solves.

There are zero successful geometric repairs, zero Delta-zero helper outcomes and zero labelled fallbacks in this fixture. The included Delta helper is not qualified by these rows, and no torsion-grid coverage transfers here. Three successive states per model are fixed diagnostic examples, not independently sampled trials or a full cohort. Model setup/subgroup checks remain outside phase counter totals; exponentiation/nonresidue-search counting limits remain explicit. No timing or complete cost ratio follows.

The source establishes the ordering and checked equality using retained native Jacobian arithmetic as reference; this review did not independently implement the entire Jacobian law or prove general recovery totality. No additional scientific execution or cloud work occurred.

Sources read: producer PLAN.md, REPORT.md, src/check.rs, src/eager.rs, src/pruned.rs, prepare.py, exact retained outputs and manifests; earlier independent Kummer source review (subgroup reduction counterexample) and immutable recovery-neighbour scope addendum. General recovery paper claims are not used as a substitute for these implementation checks; producer references Chung–Costello–Smith sections 2,3,6.4. Official scientific promotion remains with root/coordinator.
