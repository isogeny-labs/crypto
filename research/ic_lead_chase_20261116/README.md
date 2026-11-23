# IC lead chase, 2026-11-16: experiment definitions

Experiment definitions from the parallel chase of the five open auxiliary-curve,
isogeny and transfer index-calculus leads (session log `docs/session-logs/2026-11-16.md`).
**Nothing here has been run as a measurement.** No result, benchmark record or leaderboard row is part of
this directory. Every number quoted from earlier studies is a re-derivation from
committed records and stays `[pcc:unverified]`.

## Files

| File | What it is |
|:--|:--|
| [`protocol.json`](protocol.json) | `experiment.protocol/v1` for the lazy orbit-member column sweep `c ∈ {1,2,3}` at `n ∈ {17,29,31}` against matched strong rho. **Frozen** before any data: canonical SHA-256 `19116bb1970ff8cb24ce14c7b028bb7f052e38cff954788df3bcf79c05dfbdb0` (freeze block excluded; computed with the queue's own Go encoding and cross-checked independently), bound source `3658d3aa2635ba602740cc0ebf62a678b74236c3`. Amendments A1–A5 record every change from the draft. 0 errors against `tools/aps-export/schemas/experiment-protocol.v1.json`. |
| [`make_specs.sh`](make_specs.sh), [`specs/`](specs/) | The frozen `ecbench.spec/v1` workloads, one session per curve: 9 arms (rho-frob, rho-strong, eager/lazy IC at c=1,2,3 with oracle `m=2`, A/A rho), 64 public targets, 5 rounds + 1 warmup, L2. All three pass `ecbench plan`. |
| [`analyze.sh`](analyze.sh) | The frozen analysis (sha256 `4f3073f7b66c99c92d9f992689bc12ca63d5f644e09bdb3c54ecbbebaff9370d`): orchestrates `ecbench verify --replay-all`, `compare` at w ∈ {0,0.1,1} and `band` (with the calibrated `w_cal` block when given). It computes nothing itself. |
| [`preregistrations/`](preregistrations/) | The five preregistration notes written during the chase, kept verbatim as the source record. |
| `../../experiments/queue/EXP-LAZYCOL-N{17,29,31}-001.json` | Queue registrations, one per curve, filed `blocked` (reasons below). |

## Readiness and checks (2026-11-17)

- The ecbench binary did not compile at `main` `3b0840b6f2` (`src/bin/ecbench/retained_chain_study.rs`: two `PrimeInstance` initializers missing the `representation` field added by PR #3744). The bound source adds `representation: None` at both, matching every other initializer; output is unchanged. Local release build at the bound source: sha256 `d1f4fc50c3f1987fd8e494e21f72fff8bf3bdb39b59978689cd56c6d9c8cb442` (rustc 1.97.0, this container). That hash is a development reference only; the admitted binary is the one built from the bound commit in the isolab image.
- `analyze.sh` was exercised end to end, with and without `w_cal`, on a 3-target, 1-round, L0 local smoke session of the n=17 spec in scratch storage. The audit passed (54 records, 27 replays reproduced, 0 problems). That session is a script test, not a measurement, and its numbers are not reported anywhere.
- The queue registrations decode under the queue's strict decoder, and its own `discover()` classifies all three as `blocked` with the stated reason.

## What still blocks the run

1. **Budget:** the latest shared AWS budget readback is over the monthly cap, and this runtime has no Iso Lab account identity to read live headroom. Unknown headroom admits no paid job (`taskq/deploy/eks-karpenter/COST_CONTROLS.md`).
2. **Approval:** an approved `experiments/specs/EXP-LAZYCOL-N*-001.yaml` with a committed decision.
3. **Image:** `source_commit` merged into `main`, plus a digest-pinned `isolab/tasks` image built from it with `image.ecbench_sha256`.
4. **w_cal:** a perf-ipc measurement on an isolated PMU host for the calibrated band.

## Why one formal protocol

The chase's adversarially verified verdicts (session log, 2026-11-16) ranked the leads:

| Lead | Verdict | Staging |
|:--|:--|:--|
| `lazy_orbit` | Eager→lazy is a real IC-baseline improvement; the n=31 distance to strong rho is parity, order-unresolved across the §5a band | **Frozen protocol + queue registrations** |
| `fb_shape` | Equal-size yield edge is real, but per-relation F4 cost is 1.55–1.66× higher; solving degree unchanged | Preregistration only; low priority |
| `transfer_o4` | Factor base Θ(p) vs rho Θ(√p); gap grows ~√p | Preregistration only; confirmatory closing run |
| `ghs_n7` | Every concrete route collapses to classical GHS; remaining question is mathematical | Preregistration only; no run |
| `volcano_floor` | One few-percent effect on an arm 17–170× from rho; mechanism falsified | Preregistration only; run only if capacity is idle |

Any of the other four can be formalized the same way if it is chosen for a run.

The preregistered prior, written before any data, is that no column count satisfies the strong-rho criterion at n=31, because the IC arm carries about twice rho's uncharged native work. The run's value is a measured `w_cal` and a CI tight enough to decide parity.
