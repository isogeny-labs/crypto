# Certified quotient prototype: development checks

This is development evidence, not a timed experiment. The complete reduced
grevlex bases agree exactly with the existing reference on the fixed
three-/four-variable seed-7 cases over both requested primes. The producer
also checks original-input membership, every input normal form, every final
S-pair, monicity and reducedness. The legacy checker agrees on these cases.
No witness or reference basis is passed to the constructor `[pcc:unverified]`.

The prototype constructs a certified vertex polynomial H(t), propagates
coordinate rules via linear subresultants only when their leading coefficients
are units modulo H, and derives a basis by exact quotient linear dependence.
Original inputs must vanish under the representation before change of order.
Nonunits, unsupported/disconnected systems, missing rules and exhausted caps
are explicit failures. Tests cover a nonunit, an exact inverse identity,
independent/dependent vectors, missing rules and rejection of a forged rule's
result by the final certificate checker. These tests are not comprehensive
formal verification or a signed independent execution verdict.

The normal release suite passes 35 tests, and the counter-feature suite 36;
strict all-target clippy passes in both configurations. `first-tests.log`
retains the initial test pass with a test-only unused-import warning;
`first-clippy.log` retains the range-loop lint finding. Both were corrected
without disabling a check. Final logs and source hashes are separate from
the old quadratic and original closeout receipts `[pcc:unverified]`.

Harness review also found that two completed children with unequal bases could
still schedule an A/A pair before stopping. Control dispatch now requires the
already-computed pair eligibility result. A fake-worker integration regression
checks that the first mismatched pair stops with no control children. No old
raw sample or admission result was rewritten.

`quotient::solve` returns original-input certificate rows and explicit metadata.
The worker's `quotient_info` records quotient dimension, coordinate count,
stored coordinate/basis certificate product degrees and final basis degree.
These are not a measured solving degree, top-form regularity or maximum paid
discovery degree. All quotient construction is inside the worker's solve/check
interval. It has no hidden Buchberger fallback. Its 3–8 variable guard is not
a claim that every accepted shape or any six-variable case was validated.

The counter build is separate and never supplies wall samples. Modular
products and nontrivial inversions are instrumented; the existing coefficient
update count covers sparse axpy, not dense quotient/linear-algebra writes.
No full-operation or GAE inference is supported.

QUOTIENT_PROTOCOL.md was written before implementation. The separate
QUOTIENT_SPEED_PROTOCOL.md fixes a stronger-baseline future comparison and
conditional holdouts before any timing. New scientific runs remain blocked
by the required S3 checkpoint upload, whose transport is unavailable.
No old raw evidence, accepted performance standing or IC/rho row is changed.

Prior-work receipt: [existing native FGLM study](../../jv_cover_sparse_fglm_20261018/README.md)
and [Sage/Singular polynomial-ideal documentation](https://doc.sagemath.org/html/en/reference/polynomial_rings/sage/rings/polynomial/multi_polynomial_ideal.html).
The prototype first establishes its finite quotient by certified coordinate
rules and original-input vanishing; it does not assume a free input GB.
Producer record PCC-20261115-17c5e1 pins the immutable SOURCE_SNAPSHOT.json
envelope and remains unverified. Its source/receipt/protocol hashes are
separate from the mutable status notes; no signed execution proof is claimed.
