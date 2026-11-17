# IC lead chase, 2026-11-16: experiment definitions

Experiment definitions from the parallel chase of the five open auxiliary-curve,
isogeny and transfer index-calculus leads (session log `docs/session-logs/2026-11-16.md`).
**Nothing here has been run.** No result, benchmark record or leaderboard row is part of
this directory. Every number quoted from earlier studies is a re-derivation from
committed records and stays `[pcc:unverified]`.

## Files

| File | What it is |
|:--|:--|
| [`protocol.json`](protocol.json) | `experiment.protocol/v1` for the one experiment worth staging: the lazy orbit-member index column sweep `c ∈ {1,2,3}` at `n ∈ {17,29,31}` against matched strong rho. Validates against `tools/aps-export/schemas/experiment-protocol.v1.json` (0 errors). `freeze.status` is `draft`. |
| [`preregistrations/`](preregistrations/) | The five preregistration notes written during the chase, kept verbatim as the source record. |

## Why one formal protocol

The chase's adversarially verified verdicts (session log, 2026-11-16) ranked the leads:

| Lead | Verdict | Staging |
|:--|:--|:--|
| `lazy_orbit` | Eager→lazy is a real IC-baseline improvement; the n=31 distance to strong rho is parity, order-unresolved across the §5a band | **Formal protocol here** |
| `fb_shape` | Equal-size yield edge is real, but per-relation F4 cost is 1.55–1.66× higher; solving degree unchanged | Preregistration only; low priority |
| `transfer_o4` | Factor base Θ(p) vs rho Θ(√p); gap grows ~√p | Preregistration only; confirmatory closing run |
| `ghs_n7` | Every concrete route collapses to classical GHS; remaining question is mathematical | Preregistration only; no run |
| `volcano_floor` | One few-percent effect on an arm 17–170× from rho; mechanism falsified | Preregistration only; run only if capacity is idle |

Any of the other four can be formalized the same way if it is chosen for a run.

## Before running `protocol.json`

1. Bind the ecbench commit and binary SHA-256 in every arm's `instrument`.
2. Add the analysis script (`analysis_plan.script` with path and sha256) by amendment,
   and set `frozen_before_data`.
3. Freeze through the experiment queue (`docs/experiment-queue/README.md`): canonical
   SHA-256 excluding the `freeze` block, commit, timestamp.
4. Run on isolab with native L2/replay/A/A admission, a perf-ipc host for `w_cal`, a
   resume checkpoint and a `bench.record/v1` record, via the experiment-director /
   executor-mechanical card path.
5. No row without the validation gate (replay certificate, validator report, matched
   rho with the §5a band).

The preregistered prior, written before any data, is that no column count satisfies
the strong-rho criterion at n=31, because the IC arm carries about twice rho's
uncharged native work. The run's value is a measured `w_cal` and a CI tight enough to
decide parity.
