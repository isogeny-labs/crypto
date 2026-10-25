# Continued factorization of DiSSECT timeout entries

Authorized continuation, 2026-10-21. This protocol concerns previously seen
inputs and timeout records. It does not preregister unseen curve features or
claim a statistical explanation of the historical skips.

## Scope and exact inputs

Preserve the prior external DiSSECT source/run bundle as an immutable input.
Use the six curves of `backlog/XP-20261021-nist-skipped-curves/curves.json`
at crypto commit `975ead8681008d4e1bbe88f910ea39030af21f2c`.
The upstream source is DiSSECT commit
`8edc3030fb91411a55aa6c07d050507d840c5e55`.

Ten earlier result rows contain timeout strings, representing eleven
unavailable output fields. Reconstruct each requested integer from the
exact hexadecimal input parameters, never from a floating-point JSON value:

- `twist_order`, degree 2: `(p-1)^2 + t^2`, for the P-224 stream's block
  5736, the P-256 stream's block 876, and the two published controls.
- `conductor`, degree 4: `abs(t * (t^2 - 2p))`, for the P-224 stream's
  block 1485. Its algebraic split into `abs(t)` and `abs(t^2-2p)` is
  available to the search and is charged as setup.
- `kn_factorization`: `4n-1` for the P-256 stream's block 876, and `6n+1`
  and `7n-1` for published P-256.
- `square_4p1`: **the actual code factors `4p-1` and `4n-1`**, despite its
  name and mixed-sign description. Both P-256 rows share `4p-1`; the
  skipped row's `4n-1` duplicates the `k=4` target above.

Thus the exact denominator is nine distinct integers. Stream coordinates
and unchanged imported fixture keys identify inputs here; no new catalogue
curve names are introduced.

## Search

Use native PARI/GP with proof enabled and native GMP-ECM. Thin shell
orchestration may query public FactorDB over HTTPS for factor hints.
Treat every remote factorization, status and primality label as an
unverified hint. Preserve its raw response. Never submit results to that
service or use a service status as a primality certificate.

Start with public hints and exact algebraic splits, then bounded PARI
factorization with a 600 second limit per remaining target. Limit execution
to two simultaneous factor workers, 4 GB each. For residual composites,
run a first ECM stage of up to 100 curves at B1=100000, limited to 600
seconds per target; retain curve parameters and outcomes. Escalation to a
larger ECM bound must be recorded as another attempt with its own limit.
Stop a target once a complete independently verified factorization exists.
No paid remote machines are launched by this protocol.

This is a completion search, not a performance comparison. Process limits
are execution controls; do not publish algorithm-speed claims from these
unisolated runs. Preserve timeouts, errors, failed searches, partial factors,
residual cofactors, exact commands, versions, inputs and exit statuses.

## Verification and outputs

For every complete result, independently verify that each factor is prime,
each exponent is a positive integer and the factor product equals the
original integer exactly. Use PARI `isprime` and retain reproducible PARI
primality certificates where supported. Verify public hints by exact
division before using them. A probable prime or an unsplit composite is
not completion. For the square-part trait compute
`product(q^floor(e/2))` from the verified prime factorization, not merely
from trial division. Preserve each original timeout as historical evidence;
new factorization records supplement it.

Report all nine targets and their status. Reconcile the nine results with
the ten affected upstream rows and eleven unavailable fields. Keep factoring
an integer, repairing a tool result and explaining the original selection
as separate conclusions. No historical skip reason follows from completing
these factorizations alone.

Land the native scripts, exact target list, verified factors, certificates,
source/input hashes and raw attempts in this study, with a source-linked
report, SVG and inspected PDF. Pack new raw attempts as a deterministic
archive under AGENTS.md section 13a. Existing upstream output remains
unchanged. Keep unresolved targets explicit; bounded failure is not a
nonexistence or security claim.
