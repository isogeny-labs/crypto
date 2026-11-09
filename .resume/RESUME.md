# Resume: crypto @ claude/happy-johnson-9edk2r

## Goal
Run the prepared trace-line reference panel (research/trace_c1_2lp_20261027/reference-d{2,3,4,5}.json)
on Iso Lab EKS Spot via the GitHub route (isolab-experiment.yml on main), as a measured stage
diagnostic of the Tr_{N/d} trace-selected factor-base line against the standard subfield line and matched rho.

## Lead / hypothesis
Trace line lifts every nonzero x (exact, research/trace_homomorphism_tests_20261109 T4), so the
four-summand MITM reference should need fewer trials than the standard line. This is the exact
MITM reference only, NOT the algebraic C1 experiment of PROTOCOL.md (still blocked on the C1 oracle).

## Spec
Specs frozen on main; source_revision = main HEAD at dispatch (recorded in the session log and
research/trace_c1_2lp_20261027/isolab-20261109/LAUNCH.md). c8a.2xlarge, seconds=1800, one dispatch per size.
Dollar ceiling: USD 10 for all four dispatches including failures.

## State
Dispatches pending/in flight: see LAUNCH.md for workflow run IDs.

## Next steps
1. Await each run; read step summary; download receipts only via the lab's verified route.
2. Report per-size outcome, L2/A/A admission, and the IC/rho gap; no claim beyond the MITM reference.

## Runs in flight
See research/trace_c1_2lp_20261027/isolab-20261109/LAUNCH.md.

## Stop and success conditions
No automatic resubmission. A failed or ambiguous run is reconciled against its original run ID first.

## Open questions
Whether the S3 receipts are readable from this session (no valid AWS identity here).
