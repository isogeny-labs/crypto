# Proof map

| Statement | Lean declaration | Status before the build |
|---|---|---|
| The cubic evaluated at the peeled abscissa factors as three differences | `RepeatedFactor.cubic_identity` | Gate v2 checked; standard axioms |
| A power equal to zero implies zero in a commutative ring without zero divisors | `RepeatedFactor.power_zero` | Gate v2 checked; no axioms |
| A valid evaluated ideal certificate forces a repeated peeled abscissa | `RepeatedFactor.rejection_of_certificate` | Gate v2 checked; standard axioms |

The final theorem quantifies over every power and uses an explicit
no-zero-divisor hypothesis. The producer only searches powers 1,2,3 over
the seven pinned prime fields. The root values `f_i` and multiplier values
`h_i` are scalar evaluations at the candidate triple. The certificate
hypothesis is the evaluated polynomial identity; a native independent
sparse expansion checks the coefficient identity for every emitted
certificate.

This does not yet formalize the native parser, coefficient-expansion
checker, prime-field representation, S4-to-group bridge, distinct-factor
decoder or the operational cost. It is a conditional rejection lemma,
not a full Lean proof of the executable collector. No `sorry`, custom
research axiom, kernel bypass or unverified native evaluation is permitted.
Record the actual compiler result and transitive axioms after checking.

Gate v2: Lean 4.34.1, compiler commit
`5045d0056413266e57c625dcd7c365b10e377c52`, exit 0. Standard axioms here
are `propext`, `Classical.choice`, `Quot.sound`. Gate v1's failed
zero-exponent branch and its `sorryAx` diagnostics remain in the raw history;
they are not accepted proof evidence. A separate `lean4checker` executable
was not found at the checked local tool locations.
