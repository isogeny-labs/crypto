# Closed-target transpose and source-side composition

Frozen before new data, 2026-11-17. Reuse the actual rational transpose fiber
equations and fixed source/quartic/global data from TRANSPOSE_MAP_PROTOCOL.md.
Primary source review and unsigned standing remain as in COMPLETE_MAPS_PROTOCOL.
Native development qualifier only;600seconds/1GiB, no paid campaign or timings.

Input is a rational closed target divisor (u(Z),x(Z),y(Z)), including
multiplicities. Check the full quartic incidence. Factor u over Fp by
squarefree recursion, distinct-degree Frobenius gcds and deterministic
equal-degree splitting (shifts0..1023; failure retained). Replay every factor,
degree and irreducibility, retaining repeats.

For each irreducible factor of degree e, install its actual finite field.
Evaluate target coordinates there; run the same exact transpose
elimination/saturation/recovery over that field, reduce with source Cantor
arithmetic, and sum its e Frobenius conjugates. Require the trace coefficients
to lie in Fp before restoring the original degree8 field. Count every repeated
factor. This is closed-point trace, not selecting a scalar or discarding
nonrational target support.

Check all six pinned generic-bridge source fixtures and their actual target
divisors. Subtract d times the sum of the three formal reference fibers and
require psi(phi(D))=[3]D with exact source Mumford equality. Retain every factor,
extension degree, raw fiber, reduced value, trace and final source class.
Include zero explicitly. Reject a changed target-y and changed source-side
composition. Failure or timeout is retained with all partial output.

This extends affine closed-target input; arbitrary degree drops or target
projective boundary contributions are not assumed complete. Each raw fiber
must record its finite degree; generic fibers have degree9 and the three
known formal-reference fibers have executed degree8 controls. A new degree
drop remains a boundary event to investigate, not an automatic success.
