# N83 mixed-orbit relation yield and total-runtime selection

- **Status:** `investigating`
- **Kind:** empirical bottleneck and implementation gap
- **Affected work:** [N83 factor-base sweep](../research/koblitz_n83_factor_base_sweep_20261008/), [external XOR handoff](../research/koblitz_n83_factor_base_sweep_20261008/verification/external-xor-20261025/PROTOCOL.md), [full-D18 native continuation](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/PROTOCOL.md)
- **Next action owner:** N83 factor-base research task
- **Last updated:** 2026-10-26

## Problem

The primary Koblitz GF(2^83) factor-base sweep cannot yet select a base by
fully charged index-calculus runtime because retained explicit-orbit bases
have not produced a measured natural relation yield, closure/rank, linear
algebra, target descent, and same-target rho comparison under one frozen
end-to-end protocol. The current 5+1 and 4+2 mixed-orbit searches expose
bounded, first-identity-pattern SAT instances; a capped search leaves the
relation-yield question open.

There is no verified complete cold IC baseline on this primary `a=0` 81-bit
subgroup. The verified K1107/root11 cold comparison is on the separate
diagnostic `a=1` 53-bit subgroup and cannot supply this arm's baseline.

The affected curve is `icv1-f2m83-tm6151469093347-debefd74`, with the
primary-arm K=600 base and cross-stream Gray/hash residual pools. The
requested selection criterion is minimum total index-calculus runtime,
including setup and online phases, over the finite declared design grid.

## Evidence and uncertainty

- **Observed:** The [independently replayed hosted 5+1 split panel](../research/koblitz_n83_factor_base_sweep_20261008/verification/hosted-split-panel-20261025/attempt-1/replay-receipt.json) reached 20,000 native conflicts and zero models in each Gray/hash arm. Worker walls were 1,470,249 and 1,223,792 ms respectively on that Linux host; both statuses are `UNKNOWN_search_cap`. The prior 4+2 panel is likewise capped. The [local external-XOR receipt](../research/koblitz_n83_factor_base_sweep_20261008/verification/external-xor-20261025/receipt.json) records four first-pattern 5+1/4+2 Gray/hash arms, each ending `INDETERMINATE` at 2,000,001–2,000,002 conflicts with zero models after 119.2–192.5 seconds of L0 process wall. Its [replay](../research/koblitz_n83_factor_base_sweep_20261008/verification/external-xor-20261025/replay.json) rebuilds all four formulas from exact archive bytes and checks the raw-output hashes and counters.
- **Observed capacity and access gates:** The [full-D18 native preflight](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/PREFLIGHT.md) checked the exact 5,418,738-row S3 object, distinct producer and custody manifests, full-object provider checksums, clean source and release binaries. The local one-minute load remained 55.95–74.19 against the frozen maximum of 24 while cumulative swapouts increased, so the cadence-32 worker was not launched. Configured AWS spot and RunPod SSH endpoints were unavailable at bounded checks; the spot controller reported a ready instance without a ready SSH endpoint. The hosted AWS principal's exact-object read grant remains unapplied, and PR #2649 CI is blocked before runner assignment by the GitHub account billing gate. These are resource/access conditions, not evidence about solver yield.
- **Derived or proved:** None of the capped searches proves relation absence or a cost ordering.
- **Suspected explanation:** The native CDCL backend may be a material solver-stage bottleneck; an XOR-aware backend is a discriminating test, not yet an end-to-end improvement claim.
- **Missing capability:** A verified natural partial producer across enough independent source equations, graph closure, useful rank and log solving for the full-width subgroup, followed by target descent and matched one-target rho with full cost accounting.
- **Access or resource limit:** The frozen full-D18 positive/natural pair needs a host that passes its load, swap and scratch preflight and can read the two exact S3 objects. The existing checkouts and source archives are not substitutes for that host gate. The proposed exact-object IAM change still requires the separately requested authorization.
- **What this evidence does not establish:** A selected best base, relation rate, a complete index-calculus solve, or an end-to-end speedup.

## What would unblock it

- [ ] Independently replay natural partials against the exact S3 base/pool bytes, public target stream, original CNF/XOR/field equations, and original curve group.
- [ ] Collect enough checked rows to measure useful rank and solve full-width column logs; record failed attempts and charged setup cost.
- [ ] Complete and verify a previously unseen public target with all five online phases and run a same-target rho reference under matching resource envelopes.
- [ ] Apply the frozen finite-grid coverage and unsupported-combination dispositions, and compare total cost with uncertainty before filling `selected_best_total_runtime`.

These checks are jointly needed to select a factor base on the requested
criterion. A verified partial relation would resolve only the first gate.

## Next discriminating step

- **Action or proof obligation:** Run the [first-pattern external XOR protocol](../research/koblitz_n83_factor_base_sweep_20261008/verification/external-xor-20261025/PROTOCOL.md) on both retained pools and both 5+1/4+2 shapes, with identical backend settings, independent process budgets, and complete model replay when SAT.
- **Parallel full-D18 action:** On a resource-qualified host, run the [native cadence-32 protocol](../research/koblitz_n83_factor_base_sweep_20261008/verification/d18-full-orbit-20261025/native-cadence-replay-20261026/PROTOCOL.md) against the separately hash-checked complete S3 generation. First require the constructed control and independent native group/row replay; only then launch one public natural query under its frozen cap. Preserve internal cap classification, phase records and host charge.
- **Inputs and controls:** Exact S3 compressed-object hashes and pilot manifest, trial-zero public corpus, known-answer group-lift tests, and matched Gray/hash process limits.
- **Execution limits and stop condition:** Record finite time, conflict, and memory caps per arm; preserve `INDETERMINATE`, timeout, or OOM without converting them to UNSAT.
- **Outcomes and next actions:** Verified partials advance to graph/rank measurement. A SAT model rejected by the group check calls for model blocking and a different pattern. Two censored arms call for a backend/encoding bottleneck study. Infrastructure failures require a fresh replay before interpretation.
- **Recheck trigger:** A checked natural partial, a changed solver backend or encoding, a completed matched panel, or a qualified host that permits the frozen full-D18 pair to run.

## Resolution and history

Unresolved. The 2026-10-25 XOR bridge and local pilot are a solver-backend
probe. The 2026-10-26 native D18 input preflight passed while the capacity
gate failed before worker launch. Merging this record does not complete the
selection criterion.
probe. Merging this record does not complete the selection criterion.

### 2026-10-26 full-D18 solver-gate update

The [ordered full-D18 run](../research/koblitz_n83_factor_base_sweep_20261008/reports/run-20261025T2339Z-ordered-d18/REPORT.md) used the exact frozen 32,643-column, 5,418,738-point generation. Its constructed positive returned independently checked rank-one rows `[0,166,332,498]`; its single natural public target reached the internal 2,400-second wall cap after checked import/model construction and yielded no terminal solver result. The independent audit status is `PASS_CENSORED_NATURAL`. This is a resource-bounded observation, not evidence of a zero relation rate.

The separate [native XOR cadence-32 attempt](../research/koblitz_n83_factor_base_sweep_20261008/reports/run-20261026T0121Z-cadence32/REPORT.md) ended at its 300-second internal cap before `import_checked` on a saturated shared host. The natural arm was skipped. Its independent audit status is `CAPACITY_CENSORED_BEFORE_MODEL`; it does not compare solver cadences. The native kernel's 120-case exhaustive mixed CNF/XOR differential check covers cadences 1, 2, 4 and 32, while a full-D18 constructed control under cadence 32 is still open. The [combined result](../research/koblitz_n83_factor_base_sweep_20261008/reports/d18-solver-gates-20261026/RESULT.md) preserves these separate gates.

An inspected AWS execution principal lacked S3 read access to the exact full-D18 generation. The exact-object IAM proposal is unapplied; automatic approval review rejected that persistent grant pending explicit authorization. A separately approved source-archive upload does not authorize the grant or a model-artifact upload. This is an access condition, not a mathematical or solver result.

**Next discriminating step:** run the committed native D18 positive and natural pair on a host with recorded memory limit, CPU isolation and fresh provider-checked GET of the exact object/wrapper. Require a clean source and binary identity, positive original-field and original-CNF/XOR independent replay, then preserve the natural terminal result or explicit censoring with charged resource use. If the positive reaches a verified model, launch the natural arm; if it caps before import, change host capacity before solver interpretation; if a natural model fails group replay, block or repair the encoding and rerun. A checked natural partial advances only the first gate: independent multi-equation yield, closure, rank/logs, unseen-target descent, matched rho and finite-grid coverage remain required for total-runtime selection.
