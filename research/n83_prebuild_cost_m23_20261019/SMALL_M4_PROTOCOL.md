# Frozen K2048/D12 four-summand native gate

Frozen 2026-10-19 before implementation or execution. This extends the open
m=4 obligation in `RESULT.md`. It is a bounded relation-feasibility and
solver-progress gate; full-corpus useful rank and complete IC cost remain
separate obligations.

## Inputs and policy

Use primary `icv1-f2m83-tm6151469093347-debefd74`, subgroup
`2417851639230796216685689`, cofactor 4, and exactly the K2048 and D12
archives, manifests and point sets pinned in `SMALL_BASE_PROTOCOL.md`.
Use target 0 of `public-targets.json`, SHA-256
`2895a78d28935a992d8d6155492eae928e165eafcc622e6c866080b8f91e70b5`.
The same file retains targets 1–7 for the subsequent corpus gate. Keep the
validation sidecar outside every worker container.

Both arms use the existing native `ChainedS3Encoding` with four summands,
identity mask zero, all signed-Frobenius representatives and free phases,
300,000 variables, 8,000,000 domain clauses, 100,000 conflicts, one model,
1,790 seconds from process start and a separate 1,800-second outer cap.
The x-only model must pass its original-field equations; then enumerate
both signs of all four decoded x coordinates, require stored point
membership, verify the exact target group sum and sparse coefficient row,
and record its rank increment. `Unsat` remains `UNVERIFIED_MODEL_UNSAT`;
it is not promoted to nonexistence of group relations.

Run two import preflights, then one positive control per base with the
first four representative points (indices 0,166,332,498). Pin their seven
x blocks, four selectors and phase zero. Require a returned model, checked
group lift, rank one, and changed-target/changed-row rejection. A separate
native verifier must reread the content-addressed archive, check selected
points and labels with general field/group arithmetic, recompute the
sparse row, and reject tampering. Both controls must pass before starting
public searches. Reuse that verifier for any returned natural relation,
binding it to the exact public corpus and target index.

Run public target 0 in K1,D1,D2,K2 order, fresh process/output per arm.
All four arms are required even if an earlier public search caps. Stop on
invalid inputs, instrumentation failure, failed controls or invalid model
or lift; preserve the failure. Spot interruption retains its partial arm
and stops the card. A retry requires a separately committed continuation.

## Host and receipts

Use the already authorized SSH Spot runner, CPU 31, hard 4 GiB, zero swap,
one CPU quota, network disabled, read-only inputs and root filesystem,
and the previously pinned Amazon Linux image. Record source commit and
source/binary hashes before controls. No concurrent compilation during
the card. Unrelated host activity makes these L0 diagnostics. Do not stop
or reconfigure other workloads. Use no new instance or On-Demand fallback.

Before import/model/root/search, append and fsync a phase receipt. Attach
the existing atomic solver observer before root propagation and fsync
samples every ten seconds: completed decisions, conflicts, restarts,
XOR passes/time/scans/row operations/repivots, phase, elapsed wall and
process memory. The samples are not simultaneous atomic snapshots.
Write a durable internal-cap receipt even if model or root work stalls.
Retain source and binary identities, object and label hashes, all progress,
container exit/OOM/configuration, UTC intervals and missing final receipts.

Separate import/index, corpus validation, model construction, positive
pins, root propagation, search, group verification and rank insertion.
All failed/capped work stays charged. A capped search supplies censored
work, not zero natural yield. Keep `candidate_id` and
`selected_best_total_runtime` null. No complete-cost, fastest-tool,
qualified-host or matched-rho claim follows from this gate. Follow a
checked natural relation with a frozen disjoint-corpus/rank comparison;
otherwise use observed solver work to choose one next implementation
change. Preserve the original full-cost and m=2/m=3 obligations.
