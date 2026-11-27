# Operational pair-image degree gate: closed

Study `EXP-C25519-PAIRIMAGE-GATE-001` tested the remaining escape hatch after
the monolithic pair-trace F4 failure: whether an operational pair-image split
has instance-specific collision or repeated-root compression.

It does not. The exact native-Rust pair-image polynomials have degrees
`49`, `81`, and `361` at `d=7`, `9`, and `19`, exactly `d^2`; each is
squarefree. The canonical pair trace merely regroups those roots into one
degree-`d` diagonal block and `(d-1)/2` degree-`2d` blocks. Every factor,
trace-block equality, and coefficient digest is retained in the certificate.

Explicit pair-image construction and lookup are therefore closed as an
exponent-moving lead on these instances. They reproduce the repository's
known degree-`B^2` pair-root index and generic pair-sum/MITM structure. A
successor must prove an `o(d^2)` target-conditioned common-root algorithm that
does not materialize the pair image and still recovers a witness.

This `[pcc:unverified]` exact finite algebraic gate is not an asymptotic lower
bound or a complete ECDLP measurement. No JavaScript is used.
