# Protocol: K0-13 matched counted vs fiber-closed (planner re-price)

Frozen before data. Implements the smallest re-price in
[`docs/ic/FIBER_H_GT1_VALIDATION.md`](../../docs/ic/FIBER_H_GT1_VALIDATION.md).

## Question

On Koblitz `K0-13` (`icv1-f2m13-t181-515ee569`, cofactor `h = 4`), with the
planner factor-base / target / linalg contract held fixed, how do
`mitm-frobenius-counted:m=2` and `mitm-frobenius-fiber:fiber=closed,m=2`
compare on hit rate (verified solves) and full cold `S`?

## Boundary / reference

- Floor: ecbench per-run generic floor in `S`.
- Reference arm: `rho.signed_frobenius_full` (fully priced).
- No security claim; no S-column leaderboard promotion from this capsule alone.
- Planner `Support::Measured` for the `h > 1` fiber default is allowed only if
  both IC arms complete with verified answers and committed full `S` on this
  matched contract.

## Frozen inputs

| Field | Value |
|:--|:--|
| Curve | `koblitz a=0 n=13` (`K0-13`) |
| Factor base | `compact-orbit-scan:columns=1,raw_x_cap=100000000` |
| Targets | `stride-renew` |
| Linalg | ecbench `ic.pipeline_full` default (incremental-gauss) |
| Counted oracle | `mitm-frobenius-counted:m=2` |
| Fiber oracle | `mitm-frobenius-fiber:fiber=closed,m=2` |
| Targets per curve | 8 |
| Target seed | `202611221001` |
| Rounds / warmup / order | 3 / 1 / alternate |
| Session seed | `202611221101` |
| Isolation | `L0` (operation counts; not a wall-time claim) |
| Spec | `specs/k0_13_counted_vs_fiber.json` |

## Success / stop

- **Success for Measured flip:** both IC arms verified on every completed
  workload cell; capsule committed with table of hit rate and mean `S`;
  fiber default remains the planner choice when `h > 1`.
- **Stop / keep Extrapolated:** any IC arm unverified majority, missing full
  `S`, or instrument failure. Do not invent an S claim from wall time.
- **Inadmissible after seeing data:** changing FB columns, targets mode,
  seeds, oracle params, or dropping failed cells.

## Class

Accounting / support re-label of an already-wired default. Not an advance
unless fiber `S` is lower than counted on this matched contract (report
either way; Measured does not require a win).
