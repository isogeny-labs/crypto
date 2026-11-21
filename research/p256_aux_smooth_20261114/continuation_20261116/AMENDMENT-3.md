# Bounded batch output and finite-target control

Before request004, retain the complete solution/replay gates while disabling
Groebner verbose tracing only in public multi-arm unit batches. Rewrite the
single SetVerbose("Groebner",1) directive to level0 to stay below the
advertised20480-byte output cap. Preserve original input bytes and the
effective request. These units do not measure F4 matrices or regularity;
no degree or timing claim can follow from suppressed diagnostics.

After all depth1/m2 controls pass, allow one additional finite-target
qualification at depth2/m2, four raw arms in one batch. Its complete native
reference has one ordered relation and four signed domain points; paired
has four auxiliary-sign assignments for that relation. This tests the
doubling/finite-target path omitted by the infinity-target depth1 case.
Use the same response/replay/set controls. The total limit remains nine
requests and25 seconds per request; no depth8 performance cell is replaced.
This amendment precedes the additional CAS data [pcc:unverified].
