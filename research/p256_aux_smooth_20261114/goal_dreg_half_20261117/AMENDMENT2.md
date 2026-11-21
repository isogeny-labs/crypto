# Public service forbids Alarm

Request001 returned HTTP200/transport0, but success=false/exit1:
`Runtime error in 'Alarm': Illegal operation`. SetQuitOnError stopped before
the known-CI probe or either complete relation calculation. No result is
admitted. Preserve all request/response bytes and original exporter.

The public wrapper supplies its own advertised300-second execution limit
and forbids the program from setting Alarm. The20-second internal CPU cap
in COMPLETE_QUALIFICATION.md cannot be enforced on this service. Record
this resource-limit change explicitly: the one corrected request is bounded
by the service's300-second limit and unchanged25-second client deadline,
not a20-second CAS bound. If the client times out, do not retry, and do not
assume that remote execution has stopped.

Use the **single remaining request** from the original total ceiling of2
for the depth2 qualification with exactly the `Alarm(20);` line removed.
Retain SetQuitOnError and every mathematical input byte. This is one
source-corrected development submission, no automatic transport retry.
No access rejection or budget stop occurred. No third request or depth4
submission in this qualification; depth4 remains unrun even if this passes.
Preserve the original m=2–7/depth8 target and all acceptance gates.
