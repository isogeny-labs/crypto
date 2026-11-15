# APS source-only checkpoint continuation

## Goal
Resume the user's four requested steps: measurement readiness, approved source
checkpoint, baseline measurement, and implementation/comparison of the four
APS leads. Source is SOURCE_COMMIT in isogeny-labs/crypto. No experiment is
launched by this checkpoint job.

## Lead / hypothesis
The APS source implements a two-variable line chart with required horizontal
coverage, checked F4 trace reuse, unequal partitions/arbitrary guessing and
retained-base full/frontier controls. Read `research/aps_cost_leads_20261106/README.md`, `PROOFS.md` and
`ACCOUNTING.md` in the same study directory. Performance improvement is unmeasured.

## Spec
research/aps_cost_leads_20261106/PROTOCOL.md is an unfrozen design continuation.
There is no admitted pilot or main-data protocol. Four held-out size classes,
matched references, controls, complete costs, replay/export and EKS admission
remain required. Development fixtures are not held-out data.

## State
Only the source checkpoint was explicitly approved on 2026-10-30. The standard
gate's actual receipt determines whether this capture uploaded successfully.
The reviewed source snapshot and selection are under aps-checkpoint-source/.
The private PCC ledger, shared full logs/index, raw build logs, detailed host
capsules and rejected histories/fixtures are excluded. Public source claim
files remain available at the pinned Git commit, with unverified standing.

## Next steps
Read research/aps_cost_leads_20261106/HANDOFF.md and validation/ receipts.
Development commands in the cloned pinned source:

```sh
cargo test --release --locked --test aps_cost_leads -- --test-threads=2
cargo test --release --locked --manifest-path tools/aps-export/Cargo.toml
tools/lean/proof-gate.sh --out /tmp/aps-frontier research/aps_cost_leads_20261106/lean/Frontier.lean
gh run list --repo isogeny-labs/crypto --workflow aps-source-checkpoint.yml --commit SOURCE_COMMIT --limit 1
```

These are correctness/admin checks, not authorization to launch a benchmark.
Collect the actual checkpoint receipt and independently read back stored objects when access
allows. Then finish accounting/replay/export and freeze the held-out four-size panel
and disjoint pilot protocols before any performance/EKS session. Complete
run admission and execute the originally requested panel; record any later
prospective amendment before its affected data.
Never infer a source upload or benchmark pass from this resume card.

## Runs in flight
The original authoring environment may still be executing the separate broad
correctness command:

```sh
cargo test --release --locked --config profile.release.package.crypto.codegen-units=256 --lib -- --test-threads=2
```

Its private command/log/exit receipts are in
`/workspace/aps-measure-setup/nov6-root-corrected/`; read
`/workspace/aps-measure-setup/APS-COST-LEADS-CURRENT.md` for current process
state before touching it. An absent exit is pending, never a pass. Those raw
logs and host details are not in this checkpoint. If that environment no
longer exists, rerun correctness checks from the pinned source.

This GitHub job runs only administrative custody tests and the standard
scripts/checkpoint.sh gate, bounded by its prospective native T10 lease
deadline and a 120-second transfer timeout. No APS performance, profiler or
EKS job is launched. Check the workflow's normalized receipt for its exit.

## Stop and success conditions
Stop before transfer on a changed source/selection, unpushed history, native
ledger difference, expired lease window, wrong token destination or source
closure failure. A successful standard gate means the approved source
checkpoint uploaded; it does not admit performance, attest execution, close
T9/research, or grant denied publications. Keep auto-merge off while the
native-ledger publication/reconciliation hold remains.
