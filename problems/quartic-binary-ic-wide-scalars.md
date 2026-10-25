# Quartic binary IC lacks wide coefficients, point keys and subgroup scalars

- **Status:** `open`
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
  subgroup scalar/order arithmetic, complete integration and replay.
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

- **Action:** execute the registered m=6/8/10/12 smoke and supported m=14/15
  full panels while building the wider path as a separate method version.
- **Inputs and controls:** frozen `specs/quartic-*.json`, two-target smoke,
  eight-target full rungs, same-target negation rho and repeated rho controls.
- **Execution limits:** native runner caps and the owner's USD 100 total
  additional cloud-compute ceiling; launcher access and current price are
  required before paid provisioning.
- **Outcomes:** verified solves price supported cells only; timeouts remain
  missing costs; a wide-path disagreement blocks the larger rungs.
- **Recheck trigger:** wider arithmetic implementation or completed receipts.

## Resolution and history

2026-10-25: source width audit and typed refusals recorded. Unresolved.
