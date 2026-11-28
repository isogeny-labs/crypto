# Pre-data amendment 1: balance measured arm order

Filed after the first `ecbench plan --doc` and before any source build or timed run. The original committed protocol and spec at `ef1648b167` remain frozen as version 1; `SPEC-K13-L0-v1.json` and `PLAN-v1.json` preserve their exact contents. The planner warned that alternating order with one measured round cannot balance arm order, because the warmup parity reverses the measured order.

Version 2 changes only `measurement.rounds` from 1 to 2. Eight arms, two fixed public targets and one warmup now give 8 × 2 × 3 = 48 planned executions, of which 32 are measured. Alternating measured order is balanced within each target. The per-execution cap remains 300 seconds and the outer watchdog remains 1,200 seconds. The new plan must verify registered K13, exact first two target IDs, arm order and seeds. All other protocol criteria and the L0 diagnostic boundary remain in force. No run data existed at amendment time. [pcc:unverified]
