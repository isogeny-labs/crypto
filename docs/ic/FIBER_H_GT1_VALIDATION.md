# Fiber oracle default when `h > 1` (planner validation)

`ic-planner/5` selects `mitm-frobenius-fiber:fiber=closed,m=2` when the
Koblitz cofactor `h > 1`. That choice is **wired and correctness-checked**,
but the planner still marks the oracle stage
`Support::Extrapolated`: the counted Frobenius recipe panel that priced
`mitm-frobenius-counted:m=2` was prime-order (`h = 1`) or fiber-blind.

This note records what is already measured, what is not, and the smallest
ecbench session that can re-price the default. No S-column claim is made
here.

## Already measured (not a planner re-price)

| Evidence | What it shows | What it does not show |
|:--|:--|:--|
| `fiber::the_fiber_closed_table_finds_every_lift_the_plain_table_finds` on `K0-13` (`h = 4`) | Closed fiber table hits ≥ plain-over-lifts; every closed hit is a lift | End-to-end `S`, setup amortization, or counted vs fiber recipe cost |
| `planner::koblitz_oracle_uses_fiber_when_cofactor_exceeds_one` | Planner emits fiber-closed at `h = 4`, counted at `h = 1`, support Extrapolated | Yield or ns/probe |

## Smallest matched re-price (land when run)

Panel curve: **`K0-13`** (`icv1-f2m13-t181-515ee569`, cofactor 4) from
[`STANDARD_PANEL.md`](STANDARD_PANEL.md). Keep factor base, targets,
linalg, and instrument fixed; swap only the oracle.

```text
ecbench run \
  --curve K0-13 \
  --factor-base compact-orbit-scan:columns=<same>,raw_x_cap=100000000 \
  --oracle mitm-frobenius-counted:m=2 \
  --linalg incremental-gauss \
  ...

ecbench run \
  --curve K0-13 \
  --factor-base compact-orbit-scan:columns=<same>,raw_x_cap=100000000 \
  --oracle mitm-frobenius-fiber:fiber=closed,m=2 \
  --linalg incremental-gauss \
  ...
```

Promote the planner oracle stage to `Support::Measured` only after a
committed capsule compares the two arms on the same FB / target / linalg
contract (hit rate and full `S`, not wall time alone). Until then leave
`Extrapolated` and keep lifts / combined selectable by name.

## Related

- Fiber modes: `src/cryptanalysis/ic_framework/fiber.rs`
- Planner rule: `src/cryptanalysis/ic_framework/planner.rs` (`ic-planner/5`)
- Protocol part A: `research/isogeny_conductor_gap_ic_20261010/PROTOCOL.md`
