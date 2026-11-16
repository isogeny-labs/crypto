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

## Registered execution
Source8b074fa80042/checkpoint164952 preceded one release run, exit0/stderr empty.
Both maps recomputed;664split lines from16257,rank127/nullity1,all residuals,
corruptrow and8principalclass controls pass. Generator descents0,target13,
logs47376/619220 yieldk183049. Source/destination scalar andwrongscalar checks
pass; nativeHEC exactsamepair agrees(k183049,407trials,72relations).
Decision: correctness path now joined; full enumeration/dense LA is a reference,
not optimized asymptotic IC. Next efficient collector and complete coldcost.

## Errors
A disposable manifest-draft sed command failed; final explicit Cargo.toml and
release build succeeded before source freeze. No scientific execution failure.

## Reports and validation boundary
OnepageREPORT.pdf compiled with STAGES.svg, including proportional bars for the
two recorded map-scan counts, rendered and visually inspected. ENVIRONMENT.txt
records local host/compiler; no host-isolation or timing claim. No performance
record is created. NativeCI does not execute this standalone scientific gate.
