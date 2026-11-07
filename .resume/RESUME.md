# Resume: crypto @ aburan28/wizardly-einstein-i9seky

## Goal
User request (2026-11-07): bring each ECDLP method's measured cost to what its
algorithm predicts (rho/BSGS/kangaroo constants vs sqrt(pi r/2A), index-calculus
exponent vs its family's theory) and keep iterating until they match.

## Lead / hypothesis
Rho's gap to sqrt(pi/2A) at r = 2^18..2^28 is set-up and r-adding excess of a
4-16 jump table built by joint double-and-add. rho.negation_v3 / rho.plain_v3
(commit 1d03f077eb) build the table from shared ladders (2 additions a jump)
with k ~ sqrt(F/2) jumps. Local diagnostic (examples/rho_v3_diag.rs, 2000-30000
runs/size): v3 S/floor 1.33 / 1.26 / 1.15 / 1.09 / 1.08 / 1.05 at 2^17.7..2^28
against legacy 1.63 / 1.53 / 1.35 / 1.20 / 1.14 / 1.11.
Second lead: ic.pipeline with fixed B = 32 has theoretical alpha = 1 (measured
1.005); B ~ r^(1/3) should give alpha = 2/3 (no size=auto yet).

## Spec
research/ecbench_rho_v3_ladder_20261107/spec.json (interleaved, A/A of v2).

## State
Code committed; legacy ids replay bit-for-bit (288/288 of the 2026-10-06
prime-toy session). Local L0 session next (no AWS identity in this container,
so the isolab-eks receipts could not be read back).

## Next steps
1. Run the spec locally, verify --replay-all, table, bound fit for v3.
2. Report + graph + PDF, benchmarks/ entry, benchrec record, PR.
3. IC: add factor-base size=auto (B = c r^(1/3)) and measure alpha.

## Runs in flight
None.

## Stop and success conditions
v3 search phase within ~2-3 % of sqrt(pi r/2A) and total S/floor -> 1 as r grows.

## Open questions
EKS/L2 confirmation needs an AWS identity (not in this container).
