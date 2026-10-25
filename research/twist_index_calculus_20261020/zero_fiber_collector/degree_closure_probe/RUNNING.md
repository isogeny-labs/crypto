# Managed execution receipt

One accepted launch, exit 0, at **2026-10-25T04:08:23Z**:
`twist-degree-closure-probe-panel-v1.service`, MainPID **3643550**,
observed `active/running` immediately after launch. It uses `Restart=no`,
`RemainAfterExit=yes`, the existing shared reservation and the original
per-producer 8 GiB / 1,200-second outer / 1,180-second cooperative limits.
This is an owner receipt, not a completed comparison verdict.

The complete twelve-configuration source/input freeze was committed first:
`c30a3dc6068b9907681e46634df14c52a8a40f1e`.
Runner SHA-256:
`d6e4f2f6d562f0a67bd5735123d63a6d744c9e685939f4c73d9c427ad0c7a92b`.
Workload SHA-256:
`254581dfebfcb992e8ce0c6b7896cbd7cb402259b0a5b72e34fd8c711cc0c88a`.
The native five-test gate, input preparation and actual v2 64/65 launcher
controls pass before launch. V1's queued infrastructure control was
quiesced before any phase was admitted; its records remain preserved.

Receipts: `../results/degree-closure-probe-v1/launch/`. Raw execution:
`../runs/degree-closure-probe-panel-v1/`. The owner records actual phase
launch times after waiting for the shared reservation. Queue time is not
an execution deadline extension, and an observation timeout is not a
restart authorization. Inspect this unit and its terminal/phase receipts
before any follow-up action; do not launch a second owner or repeat a slot.

Required outcome: all 24 producers, 12 independent first audits and 12
parities, with complete coverage or explicit missing/censored positions.
The terminal receipt is `metadata/terminal-exit`, and the native reduction
is `analysis/summary.json`. The source/binary/input seals must still pass.
No such complete verdict, relation-rank gain or full cost is claimed by
this launch record. The inspected control PDF is not a production report.

The original full recovery and certificate panels retain their existing
owners. Historical 189+21 execution, Diem generation, the quiesced legacy
freeze followup and EKS submissions are not repeated. All original broader
comparison and complete-price requirements remain open where not met.
