# Historical IC leaderboard tolerance rejection

- **Status:** open
- **Affected work:** full unmodified leaderboard regeneration; observed while publishing PR5070
- **Next owner:** historical evidence / validation-gate maintainer
- **Updated:** 2026-11-23

After exact Git/source hydration and original artifact-reader recovery, the unmodified leaderboard generator refuses E27. Direct calls to the current validation gate reject47 E27 headline relative tolerances (first0.0015290519892966362, maximum1e-6),174 E30 and162 E32 headline tolerances. [Receipt](../research/trace_zero_halftrace_elimination_20261123/checks/leaderboard-existing-gate-failure.json), [generator failure](../research/trace_zero_halftrace_elimination_20261123/checks/leaderboard-regenerate-22.txt), unsigned statement PCC-20261123-848c58. [pcc:unverified]

This remains after restoring original sparse-excluded manifests, certificates, reports and raw files. The normal artifact reader uses verified Git/cache recovery; these local reads establish no remote S3 custody. No old evidence, threshold, gate or leaderboard output was edited. [pcc:unverified]

Next: the owning validator must reconcile each exact historical headline with the current1e-6 contract and independently sign any replacement evidence. Then rerun the unchanged full generator and its check. A blanket tolerance increase, deletion of historical rows or producer-written pass flag does not discharge the gate. PR5070's measured half-trace panel, full root integration, artifact-store migration and independent admission remain distinct.
