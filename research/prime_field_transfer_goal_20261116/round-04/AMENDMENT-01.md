# Version02: primitive polynomial remainders

The original v1 u5/u7 setup calls each hit the declared60-second limit in
the symbolic-kernel stage. Preserve both124 exits and raw outputs. No
kernel/model is obtained from either call. This does not reject the CM
construction or give a runtime lower bound.

Before new arithmetic, version02 replaces repeated fractional polynomial
Euclidean remainders with primitive fraction-free remainders over the
Eisenstein integers. Clearing coefficient denominators and dividing by
their common content changes a polynomial only by a nonzero field scalar.
The Euclidean norm a²−ab+b² and nearest integer-pair division compute
contents. Check strict norm decrease and exact integral division.
This is standard polynomial arithmetic, not a new transfer mechanism or
a claimed speed result. Source comparison: FLINT's primitive/subresultant
polynomial-GCD documentation and release3.3.1 gcd_subresultant.c; no claim
to implement FLINT or match its performance.

The field/kernel definition, source/target choices, fixed inputs, source
precision lists, LLL delta/step limit, exact-root certificate and60-second
per-call limits stay the same. Source files live under v2/; every v1 file
and qualifier remains unchanged. Use new model/receipt names.

First reproduce the u3 kernel and all18-case qualification with v2, because
the arithmetic implementation changed. Compare its exact kernel and
accepted root/image certificates to v1. Then attempt the original larger
small-fixture setup and public-input bins. If this still fails, a direct
division-polynomial constructor is a further versioned candidate; do not
extend deadlines or overwrite the failures silently.
