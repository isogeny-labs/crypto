# Primo has no prime-field or bit-vector route for index calculus

- **Status:** `open`
- **Kind:** implementation gap and unmeasured hypothesis
- **Affected work:** [`integrations/primo/README.md`](../integrations/primo/README.md),
  [`research/prime_field_smt_20261005/RESULT.md`](../research/prime_field_smt_20261005/RESULT.md),
  and the `primo-smt` `SystemSolver`
- **Next action owner:** unassigned
- **Last updated:** 2026-11-13

## Problem

Primo revision `fcc274c23ff5b146cc43d80b1b0317faf91c7f08` can solve the
Boolean `QF_UF` rows produced by binary-field Weil descent, but it cannot yet
receive the repository's prime-field index-calculus systems because it
implements neither bit-vectors, finite-field theory, nor nonlinear arithmetic.

The original acceptance criterion remains an exact, correctness-checked
prime-field solver route followed by a matched solver-stage comparison on the
same inputs and host. Registering the Boolean plug-in does not meet that
criterion and does not establish a prime-field or end-to-end performance advantage.

## Evidence and uncertainty

- **Observed:** Primo's pinned `docs/capabilities.md` lists `QF_UF`, linear
  real/integer arithmetic, and their UF combinations; it explicitly lists
  arrays, bit-vectors, and nonlinear arithmetic as unimplemented. The existing
  [`research/prime_field_smt_20261005/RESULT.md`](../research/prime_field_smt_20261005/RESULT.md)
  records exact SAT/UNSAT controls for `QF_BV` and `QF_FF`, while all frozen
  20-bit cvc5 queries returned `unknown` at 120 seconds.
- **Derived:** no impossibility result is established. A checked
  bit-blast or finite-field frontend could make the formulas semantically
  consumable by a Boolean solver.
- **Suspected explanation:** unknown. The existing SAT/SMT survey reports that
  general Boolean backends enumerate the measured descended systems, but that
  evidence does not measure a future prime-field Primo frontend.
- **Missing capability:** a native Rust translation from the prime-field
  coordinate or Semaev encoding to a Primo-supported theory, or upstream Primo
  support for `QF_BV`/`QF_FF`, plus matched performance evidence.
- **What this evidence does not establish:** the cvc5 timeouts do not establish
  that Primo, a different encoding, or a future theory implementation must
  time out; the binary-field adapter says nothing about prime fields.

## What would unblock it

- [ ] Implement a native Rust frontend with exact semantics for one existing
  prime-field encoding (`QF_BV` coordinates or `QF_FF` Semaev), targeting a
  theory Primo actually supports, or qualify a pinned Primo revision with
  native support for that theory.
- [ ] Pass planted SAT and exhaustive UNSAT controls, independently validate
  every model in the original field/group semantics, and fail closed on
  `unknown`, timeout, parse error, or invalid model.
- [ ] Run a frozen, same-input, same-host solver-stage comparison against the
  fastest applicable exact reference, retaining timeouts and leaving
  end-to-end fields unset unless the full IC pipeline is separately measured.

These conditions are sufficient to establish a usable prime-field Primo
solver stage on the measured workload. They are not sufficient by themselves
to establish an end-to-end index-calculus performance change.

## Next discriminating step

- **Action or proof obligation:** implement a bounded native bit-blast of the
  existing prime-field coordinate encoding and feed the resulting Boolean
  formula to Primo.
- **Inputs and controls:** start with the committed toy SAT/UNSAT controls,
  then the frozen 20-bit input from `research/prime_field_smt_20261005`; retain
  the native exact oracle and the existing cvc5 encoding as controls.
- **Execution limits and stop condition:** freeze a protocol before timing;
  use the repository's 120-second historical cap for comparability unless the
  new protocol justifies a different fixed cap. Stop a cell on model mismatch,
  unsupported syntax, timeout, or the preregistered resource limit.
- **Outcomes and next actions:** exact terminal answers permit a matched stage
  benchmark; correct `unknown`/timeout is a bounded negative result; any model
  mismatch blocks performance interpretation and returns to the translator.
- **Recheck trigger:** a Primo release adds `QF_BV` or finite-field support, or
  the native translator and its controls land.

## Resolution and history

Unresolved. The 2026-11-13 binary-field `primo-smt` registration deliberately
keeps this prime-field obligation separate.
