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
