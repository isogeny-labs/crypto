# Fiber oracle default when `h > 1` (planner validation)

`ic-planner/5` selects `mitm-frobenius-fiber:fiber=closed,m=2` when the
Koblitz cofactor `h > 1`. That choice is **wired, correctness-checked, and
now support-Measured** on the matched K0-13 capsule below. Measured means
the recipe was priced end-to-end on `h = 4`; it is **not** an S-column win
over counted (fiber was ~2.43× counted cold `S` on that panel).

No leaderboard S-row is promoted from this note alone.

## Already measured

| Evidence | What it shows | What it does not show |
|:--|:--|:--|
| `fiber::the_fiber_closed_table_finds_every_lift_the_plain_table_finds` on `K0-13` (`h = 4`) | Closed fiber table hits ≥ plain-over-lifts; every closed hit is a lift | End-to-end `S` alone (see capsule) |
| `planner::koblitz_oracle_uses_fiber_when_cofactor_exceeds_one` | Planner emits fiber-closed at `h = 4`, counted at `h = 1`, support **Measured** | That fiber beats counted in `S` |
| Capsule `research/fiber_h_gt1_k0_13_20261122/` session `ECBS1hbaa236332ada` | Matched FB/targets/linalg; both oracles 24/24 verified with full cold `S` | Amortized hugepage / Top-layout claims; sizes beyond n=13 |

## Matched re-price (committed)

Panel curve: **`K0-13`** (`icv1-f2m13-t181-515ee569`, cofactor 4) from
[`STANDARD_PANEL.md`](STANDARD_PANEL.md). Factor base, targets, and
instrument fixed; only the oracle swapped.

| Arm | Oracle | Verified | Mean cold `S` | 95% interval |
|:--|:--|---:|---:|:--|
| `ic-counted` | `mitm-frobenius-counted:m=2` | 24/24 | 5.966 | [5.837, 6.098] |
| `ic-fiber-closed` | `mitm-frobenius-fiber:fiber=closed,m=2` | 24/24 | 14.483 | [14.353, 14.613] |
| `rho-frob` | `rho.signed_frobenius_full` | 24/24 | 2.083 | [1.774, 2.446] |

Paired ops ratio fiber/counted: **2.427** [2.396, 2.459] (diagnostic;
§5a band unresolved). Audit: 12/12 sampled replays identical. Paths:
`research/fiber_h_gt1_k0_13_20261122/{PROTOCOL.md,REPORT.md,sessions/}`.

Planner oracle stage support for `h > 1` is `Support::Measured`. Lifts /
combined remain selectable by name. Do not cite an S-column improvement from
this panel.

## Related

- Fiber modes: `src/cryptanalysis/ic_framework/fiber.rs`
- Planner rule: `src/cryptanalysis/ic_framework/planner.rs` (`ic-planner/5`)
- Protocol part A: `research/isogeny_conductor_gap_ic_20261010/PROTOCOL.md`
