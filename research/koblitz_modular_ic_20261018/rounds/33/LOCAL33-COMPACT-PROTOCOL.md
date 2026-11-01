# Compact specialization of the exact LOCAL33 family (prospective)

The accepted LOCAL33 generator already verified the cleared polynomial
identities, including all51 standard-smooth products. Its expanded source
certificate is6.2MB and product record60MB. The original degree83 point
checker is live; do not infer terminal state or restart it because it has
not printed a result. Preserve its eventual accepted/failed/cap receipt.

This amendment changes representation of the SAME exact witness family.
Retain the native rational F2(u)[n,l,v] generators, Tate gcd/Bezout witnesses,
all syzygies and their Jacobian. Retain the product D(u) of every coefficient
denominator and verify D clears every coefficient. Serialize with short=0.
Native generic roundtrip must reproduce every object and all identities.

A text-only assembler lexically renames u,n,l,v to function arguments and
wraps these exact native expressions into a specialization procedure. The
procedure evaluates D FIRST and rejects D=0 before any rational expression.
A SECOND native roundtrip at the formal arguments u,n,l,v must reproduce
the original rational family, so syntactic assembly does not stand in for
a mathematical check. Point consumers use only this roundtripped procedure.

At points the raw J,S,Jacobian and H are evaluated in the residue algebra.
All exact point/source/plane checks, D/H unit conditions and full51product
unit-ideal witness requirements remain. Cleared and raw families differ by
units: J'=D*J, S'=D*S, so each cleared h*det product is D^4 times its raw
counterpart. No proper-model or branch claim is inferred without the same
successful local presentation certificate. Old expanded bytes are unchanged.

Native4GiB/1200seconds each. Use per-attempt isolated snapshot runner to avoid
shared-directory archive races. If the old83/5 point runs complete acceptably,
reuse those results; do not repeat merely for counts. If they reach a terminal
failure/cap, a fresh compact point-check invocation may use this equivalent
certified representation under the same bounds. No coefficient box, changed
unit tests, field expansion or cap increase. Record every original failure.

Before the first function-roundtrip/point data, the text assembler replaces
literal u^k tokens by lookups in a native precomputed field-power list. The
allocation bound is the largest exponent TOKEN in the retained native source;
Python does only lexical metadata/renaming. Every field multiplication occurs
in Singular, and the formal-argument native roundtrip checks the complete
transformed result. This changes expression evaluation, not any witness or
condition. No timing comparison/IC performance claim is made.

Original local33-check83-01 reached the1200second native cap(exit124)
without any completed point-presentation output. This is a censored
specialization computation, not a failure of the local presentation itself.
The sequential old field5 invocation was not reached. Preserve the complete
old input/log/exit and use fresh compact83/5 checks only after the compact
function's native formal-argument roundtrip passes. Bounds stay unchanged.
