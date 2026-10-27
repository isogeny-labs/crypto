# Implement a versioned quartic m=16 through m=18 path: continuation protocol

Canonical requirements remain in research/ic_algebraic_crossover_20261023/PROTOCOL.md.

Can the larger quartic workloads be represented and verified without coefficient, key, or scalar truncation?

Implementation blocker: widen field coefficients and Artin-Schreier loops/sentinels, point packing, tower basis, orders/scalars, and IC relation arithmetic. Preserve all narrow counters and committed replays. A size-guard change alone is invalid; no runtime estimate or success verifier exists yet.

Preserve every failure and cap. Success requires exact computation or complete independently verified scalar recovery, never extrapolation. Use run-remote.sh and verify-remote.sh where implemented. This blocked card is unlaunched; checkpoint/resume and merged code must be resolved before ready. Total additional cloud compute is capped at the owner-authorized USD 100 across all cards.
