# Exact proof replay before another runtime claim

Implement/qualify a validation algorithm; no performance result is predicted.
All older frozen protocols/subjects/closed records remain intact. The user's
checkpoint waiver persists for bounded local L0 work, zero cloud spend.

## Sources and scope

KB unavailable: loaded MCP absent; wrapper has no executable. Fallback read
shared Row/normal_form/s_polynomial/compact verifier and installed Singular4.4.1
manual interred5.1.64, lift5.1.80, liftstd5.1.81, reduce5.1.131, division5.1.26.
The manual guarantees liftstd transformation identities. reduce presupposes a
standard basis; do not use that assumption circularly. Previous Isabelle2025-2
groebner.ML provenance review informs witness replay; no novelty claim.

Hypothesis: complete identity witnesses may be cheaper to validate than full
basis regeneration. This is an algorithm change to checking, not permission
to exclude checking or reinterpret old paid clocks [pcc:unverified].

Stage1 only: add a Rust wire producer and checker-script exporter, with no
new larger fixtures, runtime campaign or automatic follow-up selection.
Original-system equations are the sole constructor input. Use existing
compact/expanded constructors and Rust division to emit witnesses; never use
an external reference's rows in construction.

## Exact acceptance argument

The wire carries acyclic membership rows (definitions reference only originals
and earlier definitions; basis references only originals/definitions), one
identity expressing EVERY original in the final basis, and one standard
representation for EVERY unordered S-pair. The latter must have each multiplier
term times the referenced basis leading monomial strictly BELOW the pair's
LCM. An arbitrary S-pair identity alone is insufficient: its trivial defining
representation exists even for a non-GB set and must be rejected.

Rust validates canonical coefficient/exponent shapes, complete index coverage,
acyclic references, monicity/reducedness and monomial bounds. Singular checks
all polynomial identities and binds each encoded leading monomial to its own
ring ordering; it also checks standard-representation bounds in its arithmetic.
The generated checker uses no std/slimgb/lift/liftstd/reduce/division or new
basis construction. Both ideal containments plus Buchberger's criterion and
reducedness justify the complete reduced basis. This is producer code and
unsigned evidence, not an independent signature or execution-proof backend.

Names in the checker ring are generated x0.., never interpolated user strings.
Use dp/grevlex, p2147483647 for CAS qualification. Explicitly reject unsupported
large characteristics in the Singular exporter; wide Rust constructor support
does not transfer to that mature CAS route.

## Budgets and qualification

Keep constructor admission/caps unchanged, including default8/research16,
input<=32/degree4/terms32, quotient dimension<=8 and existing proof budgets.
Additional wire witnesses have explicit independent caps: <=16 definitions,
<=256 nonzero basis rows, <=5000 polynomial terms/degree64, <=50000 cofactor
terms per identity, <=100000 total wire cofactor terms, <=1000000 referenced
polynomial terms per identity and <=5000000 across the wire. No silent cap
raise after failure. Encoded checker <=16MiB; each child30s/2GiB/output16MiB,
CPU0/thread1, whole correctness qualification<=10minutes; no retries.

Freeze source, copy read-only executables, capture distinct build/pre/post.
Qualify on existing n3 seed7 p31 fixture only plus explicit tiny hand-written
three-variable failure systems. Required failures: changed membership
coefficient, circular/future node, missing input/S-pair coverage, malformed
exponents/indices, non-monic/non-reduced basis, a non-GB set with a fabricated
zero S-pair witness, and a non-GB set using its mathematically true but
nonstandard trivial S-pair representation. Retain all failure diagnostics.

Any later speed experiment needs a separate prospective protocol naming the
complete computation, fastest supported controls, all preparation/construction/
serialization/consumer checking costs, matched complete answers, A/A and paired
scope as appropriate. No old full-cost ratio changes when this module lands.
