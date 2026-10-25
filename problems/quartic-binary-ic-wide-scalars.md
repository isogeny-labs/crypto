# Quartic binary IC lacks wide coefficients, point keys and subgroup scalars

- **Status:** `investigating`
- **Kind:** implementation gap; larger-rung measurements outstanding
- **Affected work:** [quartic ladder C1/C2](../research/ic_algebraic_crossover_20261023/PROTOCOL.md)
- **Next action owner:** current IC campaign
- **Last updated:** 2026-10-25

## Problem

The existing characteristic-two quartic path supports base-field degree
m<=15 and cannot execute the requested m=16,17,18 rungs without widening
coefficients, loop bounds/sentinels, packed coordinates, order and scalars.

This is a representation limit of the native `gaudry_quartic_bin` and
`f2m4_torsion` path. Its field is GF((2^m)^4); the proper intermediate field
is essential. The limitation is not a mathematical obstruction and says
nothing about prime-degree binary fields or prime fields.

## Evidence and uncertainty

- **Observed:** native plan refusals for m=16,17,18 are preserved in
  `research/ic_algebraic_crossover_20261023/runs/quartic-width-audit-20261023-01`
  in the campaign's evidence archive. `Gf2q::new` explicitly requires m<=15.
- **Derived from source:** coefficients and Artin--Schreier sentinels are
  u16; `q as u16` loops wrap at m=16; packed quartic point coordinates,
  group order, scalars and tower conversion use u64. Removing the guard
  cannot supply a valid wider implementation.
- **Observed historical evidence:** the original m=14 single-target gap
  near 4.3 times measured rho is distinct from the extrapolated crossover
  near subgroup size 2^68. Neither is a new campaign measurement.
- **Suspected explanation:** none needed for the source width limit.
- **Missing capability:** exact u32 coefficients and >=128-bit packing,
  subgroup scalar/order arithmetic were absent in the original path. The
  new standalone `gaudry_quartic_bin_wide` core supplies these, with
  exhaustive/independent controls, high-bit encodings and exact order
  certificates at m=16/17/18. All three independent audit replays are
  byte-identical. The registered ecbench/stage/CountedGroup adapters remain
  u64 and refuse wider specs; full method integration is still missing.
- **New finite evidence:** [wide-core audit](../research/ic_algebraic_crossover_20261023/reports/run-wide-core-audit-20261025/report.pdf),
  [protocol](../research/ic_algebraic_crossover_20261023/WIDE_GROUP_AUDIT_PROTOCOL.md),
  [raw/binary archive reproduction](../research/ic_algebraic_crossover_20261023/verify-wide-core.sh).
  Field/point/scalar/native-count parity passes old supported control rungs.
  Full old-method generator/order parity and complete IC remain separate.
- **Constructor qualification:** an excluded reducible family preserves
  the returned field; the one before/after instruction profile is attribution,
  not a native speedup. Six Lean algebraic dependencies are checked; their
  finite-field/irreducibility bridge remains unformalized. See the
  [proof map](../research/ic_algebraic_crossover_20261023/lean/README.md).
- **What this evidence does not establish:** a measured crossover, cost at
  unsupported rungs, or advantage on fields without that intermediate field.

## What would unblock it

- [ ] Implement the wider path without truncation, with exact field,
  curve, subgroup-order, scalar and serialization tests at m=16--18.
- [ ] On every old supported rung used as a control, match field/point
  operations and verified scalar outputs against the original path.
- [ ] Run the frozen eight-target cold ladder, full pipeline and strongest
  matched rho, with all failures charged, independent replay and native
  cost bands. Record unsupported or resource-limited cells explicitly.

Wider arithmetic only enables measurements; it does not predict their cost.

## Next discriminating step

- **Action:** retain the completed 24-record smoke/replay, let the actual
  additional capped L0 m14 run (started 17:54:16 UTC) finish and its m15
  successor execute,
  and integrate the verified wide core into a separately versioned full
  factor-base/oracle/relation/linear-algebra path. Original L2 full panels
  and eight-target wide IC/rho requirements remain open.
- **Inputs and controls:** frozen `specs/quartic-*.json`, two-target smoke,
  eight-target full rungs, same-target negation rho and repeated rho controls.
- **Execution limits:** native runner caps and the owner's USD 100 total
  additional cloud-compute ceiling; launcher access and current price are
  required before paid provisioning.
- **Outcomes:** verified solves price supported cells only; timeouts remain
  missing costs; a wide-path disagreement blocks the larger rungs.
- **Recheck trigger:** wider arithmetic implementation or completed receipts.

## Resolution and history

2026-10-25: source width audit and typed refusals recorded. Standalone wider
core, exact order/high-scalar controls, three independent replays and the
4,634-pass full library gate completed. Full integration and ladder unresolved.

2026-10-25 constructor follow-up: the old narrow6 profile records 8,774,220
simulated whole-process Ir, and seven even/odd native field/product controls
pass before editing. Commit c754e7111 ports the already proved exclusion to
the narrow constructor; its new library gate and candidate profile/parity
checks are queued behind the real m14 resource lock. This source remains
unverified until those receipts exist. The active quartic binary is unchanged.
