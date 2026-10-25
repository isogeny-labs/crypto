# Wide quartic core: arithmetic and identity controls

2026-10-25. C2 implementation work, separate from the registered m<=15
backend and from any completed DLP or performance claim. The initial port
uses u32 coefficients/sentinels and u128 packed elements, Artin--Schreier
bit vectors, full group orders and subgroup scalars. Retain the original
backend and existing method versions for their frozen replays.

Before a larger-rung research run, verify:

- Exhaustive base-field/Artin--Schreier controls and seeded parity with the
  old backend at supported m, including all relevant top coefficient bits.
- Packed-coordinate round trips above 64 bits and explicit rejection of
  out-of-range coefficients/encodings. No mask-based truncation.
- Field multiplication and point/scalar arithmetic against an independent
  polynomial-basis binary-field implementation, using an exact tower-basis
  conversion. Include scalar and coordinate bits beyond u64.
- Old-rung field/curve/generator/order parity. For larger groups, exact
  order certification: a nonzero prime-order G, [r]G=O, and the unique
  multiple of r in the Hasse interval. Preserve the exact prime-check bound.
- Decimal/hex string serialization of wide orders, scalars and field masks.

The core port includes the existing decomposition algebra, without claiming
that the legacy u64 CountedGroup/stage pipeline accepts wide instances.
Full factor-base, oracle, relation/linear-algebra and ecbench integration,
matched rho and complete independently replayed DLPs remain C2 obligations.
Do not remove old width refusals or launch the frozen unsupported m16--18
specs as though the complete wide method already existed.

Any new constructor/large-field audit must freeze its source, exact seeds,
inputs and limits before output, preserve failures, and leave native data
plus an inspected visual report. Arithmetic tests and source parity do not
establish speed, a crossover or security of another field family.
