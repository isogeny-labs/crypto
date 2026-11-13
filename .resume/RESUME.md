# Resume: crypto @ aburan28/intelligent-ride-v0k039

## Goal
User (2026-11-13): "Chase all leads" for non-generic sub-sqrt / subexponential prime-field ECDLP.
Five open leads from research/notes/index-calculus/PRIME_FIELD_IC_CLOSURE_20261112.md and
RESEARCH_PRIME_FIELD_BREAKTHROUGH_PROGRAM.md: L1 PKM tower m>=8; L2 small roots beyond Coppersmith
(S4 at delta>=1/4, S5 at ~1/5); L3 digit-presentation solving degree; L4 lifting with growing rank
(Silverman-Suzuki); L5 compact full-summation message for two-isogeny fibre bases (KN-OPEN-08ba7e).

## Lead / hypothesis
Each lab writes PROTOCOL.md (frozen sha256, predictions, kill/survive rule) before any run.

## Spec
Toy local runs only (4 vCPU / 15 GB shared; per-lab cap 1 core, 3 GB). Evidence under
scratchpad chase/L*/ (path in session); to be copied into research/prime_leads_chase_20261113/.

## State
2026-11-13: labs dispatched as parallel subagents. msolve build denied (external code); in-repo F4 used.
No AWS CLI, no checkpoint upload token: S3 checkpoint upload impossible in this container.

## Runs in flight
L1-L5 subagents.

## Stop and success conditions
Per-lab frozen rule. Success = verified measured movement of an exponent or a closing argument with checker.
