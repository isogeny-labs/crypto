# Isolated (isolab-eks, L2) replay of the priced DP/equivariant residual study: frozen before the run

Date: 2026-11-01. Replays `research/ic_residual_dp_eq_20261101` (frozen spec `specs/E-p36.json`, run locally at L0) on Iso Lab / EKS Spot with native L2 admission.

## Design
`spec-p36.json` is the study's L2 spec for p36 (curve, 11 arms: rho.negation reference; baseline res1/res2/res4; candidates res{1,2,4}dp and res{1,2,4}eq; A/A copy res2aa; 16 targets; target seed 20261601;
measurement seed 20261602; alternate_measured) with one change: `measurement.rounds` 3 -> 5, because the lab's admission requires at least five measured rounds, a warmup, a reference and a matching A/A control.
The code is the main-branch commit named in the dispatch (it contains `residual-fused-dp` and `residual-fused-eq`, merged in PR #3119); the image is built from that exact commit by the `Iso Lab experiment` workflow.

## Claims
- R1 (replay): counted GAE of every record is a deterministic function of the algorithm seed and instance, so records of rounds 1-3 reproduce the local L0 session's counts exactly (same target seed, measurement seed and
  arm parameters); rounds 4-5 are new draws. If rounds 1-3 differ, the study's counts are not host- or isolation-independent and that is reported.
- R2 (isolation): the lab's native L2 gate, replay audit and A/A admission pass for the session. Wall-clock is reported only as the lab records it; no wall-clock claim is made about the algorithms here.
- R3: the paired candidate/baseline S ratios at w = 0, 0.1, 1 and the distance to rho are recomputed on the 5-round data with the harness's `compare` and `band` and set beside the L0 numbers; they are not pooled.
Class: infrastructure/replay evidence; not an algorithm speedup claim.

## Decision rule
Report the lab's result.json state verbatim. A failed admission (L2, replay or A/A) is reported as failed with its receipt; nothing is retried or edited to pass. One dispatch only (a duplicate or rerun is a new experiment).
