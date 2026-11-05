#set page(paper: "a4", margin: 19mm)
#set text(size: 10.5pt)
= Fresh native verified recovery
Prime-field source and repaired decoder · 2026-11-03

The public-point recovery API now generates fresh signs with the shared native source, evaluates the repaired character program and verifies the candidate by a separate affine multiplication before returning it.

#image("pipeline.svg", width: 100%)

== Checked outcome
Three selected Rust methods pass once. All eight saved public targets recover exactly: checker labels 0, 1, 2, 25, 173, 292, 440 and 478. Each recovery uses 16 fresh source signs; saved signs and scalar labels never enter the online API. The four repair-fiber targets are included.

Deleting the final repaired term makes the k=173 query fail mandatory point verification. Invalid point and malformed packet cases reject. The verifier independently reproduces saved points, zero, base, double, negation and subgroup order. Twenty-five earlier tests were filtered; the compiler warning is retained.

== Binding and cost scope
The generic constructor binds the same subgroup modulus to source and program. It checks packet format, not its identity. The convenience constructor embeds the exact certified 477-term packet for F509/r479. Existing source domain and word limits apply; other curves require their own suitable packet.

Successful-query records total 128 sign queries, nine direct phases, 109 seed recurrences, ten anchor hits and 25/36 verifier addition/doubling calls. These are logical calls, include identity handling and exclude setup and the negative control. Complete field, allocation and integer costs are not yet instrumented. No timing comparison or full4563-case cohort ran.

== Next step and provenance
Instrument complete setup, source, decoder and verification costs, then finish the native full-cohort runner. Exact source/data locks, saved-target provenance, commands, compiler, raw logs and projected recovery records accompany this report. Source and sign-mask derivations in adjacent components supply the construction; this integration adds no new coefficient-identity claim.
