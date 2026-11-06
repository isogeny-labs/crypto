# Terminal observation: v3 amendment 2 (protocol version 5), 2026-11-06

**Outcome: version 5 stopped at its qualification (stopping rule 1).** No
scientific session ran, and no scientific seed was consumed.

## What ran

| Field | Value |
|---|---|
| Job | `ic-line-v3a2-qual-20261106`, launched 2026-11-06T03:27:15Z |
| Launcher | CodeBuild `967de55c-630d-4da4-a9d6-0b45fb4659cd` |
| Pod | `bc4bf679-0b12-4e0c-a021-b18bf4ecf398` on c8a.2xlarge |
| Binary | `/usr/local/bin/ecbench` with sha256 `73362eed…`, equal to the pin |
| Spec | `l2-validation.json` (fp20, seed 59297, 2 targets): 108 executions |
| Launcher result | `state: failed`, exit 23 |
| Job exit | 42, the L2 isolation gate |

## What the evidence shows

- **The run itself is clean.** All 108 solves are verified. The audit reports OK:
  108 records, 108 verified, 90 replays reproduced, 0 problems.
- **The L2 gate refuses 102 of 108 executions:** grades L1 102, L2 6.
- **Every refusal has the same first blocker:** `L2: solve too short to admit
  wall time: solve … is under the 1.0 ms minimum`.
- **The node was quiet:** for example, seq 0 shows 0 steal, 0 foreign-busy and
  0 intruder threads on its pinned CPU.
- **Solve times:** the rho, BSGS and kangaroo arms solve in tens to hundreds of
  µs. The rho-neg and rho-plain arms solve in 52.7–228.5 µs. The `ic-mitm` arm
  was refused in 6 of its 12 executions (549–996 µs over 6 below the minimum).

## Consequence for the design

The 1.0 ms minimum belongs to the instrument, not to the host, and fp20 is too
small for it:

- The qualification spec (`l2-validation.json`, unchanged on `main`) cannot
  pass on this binary however quiet the node is.
- The tournament's own positive-control arms, `rho-neg` and `rho-plain` at
  fp20, solve in the same tens-to-hundreds of µs, so the scientific sessions
  would fail the same gate.
- This explains the v1 outcome: both sessions exited 42 at
  `l2_isolation_gate`.

Another rerun of this design cannot succeed. A redesign is needed, and the
choice belongs to the owner or coordinator. For example:

- a curve size at which every arm's solve exceeds 1 ms;
- a qualification spec that meets the same minimum;
- or controls whose wall time is not required at L2.

## Earlier attempt on this design

Version 4's qualification, `ic-line-v3a1-qual-20261106`, ended with
`launcher_failed`, exit 127, before any job existed. `isolab-eks-mcp` was not
on `PATH`; #3517 fixed that.

Both v4 and v5 ran from the pinned image, so the USD 5 ceiling was not
approached. Each attempt used one launcher build and, for v5, one pod for under
4 minutes.
