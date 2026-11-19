# GHS frontier workstreams 3–7

This study executes five follow-up workstreams on one source line:

1. isolated optimized-rho measurement and parallel distinguished-point
   prerequisites;
2. baseline/R17/R36/R64 composition;
3. matched plane-quartic IC versus optimized rho on the restricted GHS-weak
   `E_0/F_(q^3)` family;
4. a live ECBench implementation and solver census; and
5. independent validation plus fail-closed frontier publication.

Draft PR: #4814, stacked on the R00–R99 implementation PR #4757.

## Current status

The lanes are being prepared in parallel. No new timing sample, IC/rho ratio,
frontier row, or leaderboard promotion exists yet `[pcc:unverified]`.

The selected runtime currently has no AWS identity, and checkpoint status has
no fresh pushed checkpoint for this branch. Those are execution-admission
facts, not algorithm results. Correctness, source inventory, protocol,
validators, and launch artifacts can still be completed before admission.

## Standing boundary

Until a new admitted session passes the frozen gates, the established boundary
is carried without modification:

- generic prime, random-binary, and Koblitz curves are rho-led in the admitted
  cold one-target cells;
- the restricted GHS-weak `E_0/F_(q^3)` family is the separate IC-led exception;
- the validated prime P20 `N=45` factor base remains 7.95 / 8.24 / 9.62 times
  matched rho across the frozen pricing band;
- historical F5/F6 gains are solver-local and do not lower cold ECDLP cost; and
- optimized signed-Frobenius rho has a 1.246642-fold instruction reduction,
  while its isolated L2 wall effect remains unmeasured.

These statements are source-bound historical controls, not outcomes of this
study.

## Promotion dependency

```text
source + protocol + correctness
             |
             v
checkpoint + budget + pool + exact image
             |
             v
L2 reference/A-A/candidate measurements
             |
             v
replay + independent validation + pricing bands
             |
             v
generated frontier/scoreboard/leaderboard publication
```

Parallel preparation does not remove any edge in this chain. A downstream
validator must refuse missing or partial upstream evidence.

## Evidence map

- [`SOURCES.md`](SOURCES.md): literature and prior-evidence receipt.
- [`PROTOCOL.md`](PROTOCOL.md): common hypotheses, controls, gates, and status
  vocabulary.
- `ws3_parallel_rho/`: production parallel-rho prerequisites and isolated
  measurement package.
- `ws4_composition/`: candidate-composition package.
- `ws5_matched_ghs/`: frozen restricted-family grid.
- `ws6_solver_coverage/`: live implementation/solver census.
- `ws7_validation_publish/`: independent validation and publication gate.
- `eks_admission/`: read-only admission evidence and launch readiness.

