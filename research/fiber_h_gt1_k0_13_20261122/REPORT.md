# Report: K0-13 counted vs fiber-closed (planner re-price)

[pcc:unverified] Producer session observations below; CI has not signed a
pcc verdict for this capsule.

## Outcome

Matched panel on `K0-13` (`icv1-f2m13-t181-515ee569`, `h = 4`) completed
**128/128 verified**. Both IC oracles recovered every planted scalar under
identical `compact-orbit-scan:columns=1,raw_x_cap=100000000`, `stride-renew`,
and `ic.pipeline_full` accounting.

| Arm | Mean cold `S` | 95% interval | Verified |
|:--|---:|:--|---:|
| `rho-frob` | 2.083 | [1.774, 2.446] | 24/24 |
| `ic-counted` | 5.966 | [5.837, 6.098] | 24/24 |
| `ic-fiber-closed` | 14.483 | [14.353, 14.613] | 24/24 |
| `rho-frob-aa` | 2.083 | [1.774, 2.446] | 24/24 |

Paired ops ratio **fiber / counted = 2.427** [2.396, 2.459] (linearised_t,
8 workloads). Class: **accounting** for planner support (Extrapolated →
Measured). Not an advance: fiber is costlier in cold `S` on this contract.
No leaderboard S-row.

## Checks

- Session `ECBS1hbaa236332ada`, isolation L0 (operation counts; not wall claim).
- Audit OK: 12 sampled replays identical
  (`sessions/run1.audit.json`).
- Spec hash pinned in session `spec.json`; protocol frozen at commit
  `d17ec015016a` before data.
- Fiber arm still carries unpriced native counters
  (`fiber_*_uncharged`); counted arm reports none. Band reading unresolved
  inside §5a (ops ratio crosses 1 across weights).

## Decision

Promote planner `h > 1` oracle stage to `Support::Measured` citing this
capsule. Keep fiber-closed as the default (yield / lift-hit wiring); do not
claim it beats counted in `S`.

## Paths

- Protocol: `PROTOCOL.md`
- Spec: `specs/k0_13_counted_vs_fiber.json`
- Session: `sessions/run1/`
- Table: `sessions/run1.table.md`
- Compare: `sessions/run1.compare.txt`
- Figure: `figures/mean_s.svg`
