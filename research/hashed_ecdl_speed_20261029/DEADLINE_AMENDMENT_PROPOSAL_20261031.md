# Proposed amendment: deadline and per-case limit (for the Coordinator)

Status: **proposal, not adopted.** [PROTOCOL.md](PROTOCOL.md) is frozen and
unchanged. Its 1,800-second original deadline stays in force until the
Coordinator accepts, edits or rejects this proposal. Written 2026-10-31 on
branch `claude/hashed-ecdl-ab-driver-20261031`.

## Why

The protocol asks for one bounded job (at most one c8a.2xlarge worker, 4 GiB,
1,800-second original deadline, no automatic retries) that runs the warmup, at
least five A/A rounds and at least five interleaved A/B rounds over 84 inputs.
A short local sizing pass says a single execution takes minutes, so the
deadline covers well under 1% of that schedule. Under the frozen deadline the
driver would record nearly every execution as `not_run_deadline`. The rows
would be kept, but the run would answer nothing.

## Sizing evidence (L0 local diagnostic, not a result)

Everything below comes from a contended 4-vCPU sandbox with no isolation. Some
executions overlapped other processes. Use these numbers only to size limits.
They are **not** runtime or speedup measurements. Raw rows, binary hashes and
commands: [sizing-L0-20261031/](sizing-L0-20261031/) (`rows.jsonl`,
`analysis.json`, `IDENTITY.txt`, `p17-probe.txt`, `sizing.svg`).

| Curve (inputs) | Arm | Observed solve wall per execution | Samples | Outcome |
| --- | --- | --- | --- | --- |
| p=5 (28) | baseline | 244–249 s | 3 driver rows + 1 probe | validated |
| p=5 (28) | candidate | 67–68 s | 1 driver row + 2 probes | validated |
| p=7 (28) | baseline | 282–290 s | 4 driver rows + 1 probe | validated |
| p=7 (28) | candidate | 92–93 s | 2 driver rows + 1 probe | validated |
| p=17 (28) | baseline | 717 s, then exit 1 | 1 probe (contended) | `resource limit: parsimonious program evaluation` |
| p=17 (28) | candidate | 698 s, then exit 1 | 1 probe (contended) | same resource error |

Arm replay (`verify`) took 2–5 ms per receipt. Only seed 1 was sampled for
each curve, so the spread across seeds is unknown. Both p=17 probes ran at
the same time on a busy host, so their uncontended time is likely lower. On
the sampled p=17 input, both arms exhaust a resource before the 2,000,000,000-step
budget: under the frozen policy these become retained unsuccessful rows.

## Estimate for the frozen schedule (one worker, serial)

The estimate takes per-curve means over the 28 inputs of each curve. One
round runs each case once per pair member.

| Item | Estimate |
| --- | --- |
| One A/B round | about 59,000 s (16.4 h) |
| One A/A round (baseline twice) | about 69,900 s (19.4 h) |
| Warmup pair | 380–1,400 s |
| 5 A/A + 5 A/B rounds | **about 644,000 s (179 h, 7.5 days)** |
| Same, if p=17 runs twice as fast uncontended | about 445,000 s (124 h) |
| Frozen deadline / estimate | 1,800 s / 445,000–644,000 s, roughly 1/250 to 1/360 |

p=17 cases are about a third of the inputs. They take most of the
budget (57–67% per round), and in the one sample they produce no certificate in either arm.

## Proposed amendment

1. **Per-case limit: 1,800 s per execution** (`--case-limit-secs 1800`). This
   reads the original figure as a per-execution bound. It is about 2.5 times
   the longest observed execution (717 s, p=17, to resource error) and about
   6 times the slowest validated one (290 s). A case that reaches it is a
   retained `timeout` row, never re-run.
2. **Global deadline: split the schedule into one job per round** (10 jobs plus
   the warmup in the first job). Each job gets a 90,000 s (25 h) deadline,
   about 1.3 times the largest per-round estimate. They run sequentially on one
   c8a.2xlarge with the same seed, inputs and arm binaries, and no automatic
   retries. A preempted round is recorded as preempted, its rows are kept, and
   it is not silently replaced. One 7-day Spot job with no retries is likely
   to be preempted. Per-round jobs limit the loss to one round.
   *Fallback if the single-job form is kept:* a global deadline of 840,000 s
   (about 1.3 times 644,000 s).
3. **Before the measured run,** an uncontended sizing on the admitted
   c8a.2xlarge: each arm once on one input per curve, labelled L0, using
   `hashed-ecdl-panel run --allow-incomplete-panel`. If p=17 time-to-error
   differs from the above by more than 2 times, the Coordinator re-derives
   limits 1 and 2 from it.
4. Unchanged: 84 inputs, at least 5 A/A and 5 A/B rounds, 3-repetition
   minimum (met by 5 A/B rounds), 4 GiB, one thread, the step and size policy
   in each problem, retention of every unsuccessful row, and the 95% paired
   interval.

## What is not proposed

The proposal does not drop or reweight p=17 cases, change any budget, reduce
rounds, or move to counted-only units. Those would change the protocol's
question. Per-round jobs need a small driver change: a `--rounds` selector
that runs a contiguous range of rounds from the same seeded schedule. The
current driver always runs the full schedule. Coordinator approval of item 2
would authorise that change.

Caveat for every use: on 64-bit builds lambda <= 64, so root_u =
lambda/131072 = 0 and public inputs take the finite-size fallback. Nothing
here exercises the recursive subexponential kernel, and
`ic.tensor_contraction_v1` is an exponential toy (protocol H1). Neither
Certicom challenge is affected.
