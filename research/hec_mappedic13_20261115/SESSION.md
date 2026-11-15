# Mapped IC handoff

## What was done
Reviewed current shared log and publicmap13 handoff. KB query, native Tian
line-sieve/descent source and primary Tian §1/4.2 reviewed. Implemented full-base
line relations, exact nullspace, representation-aware rational descent, actual
public maps and source/destination scalar checks. Frozen correctness protocol.

## Decisions made
Unknown target is B=2*(3,21), with no relative scalar provided. Require rank127;
retain failures and cap cases. Existing native HEC IC is a same-pair algorithmic
control, without timing comparison. No speed/novelty or independent code claim.

## Open threads
Build, commit/lock/checkpoint, execute once. Then report and extendPR4274,
follow current-head CI, and prepare complete-cost protocol spanning all setup.

## Errors
None so far; taskT-6240 epoch1. Shared resume untouched.
