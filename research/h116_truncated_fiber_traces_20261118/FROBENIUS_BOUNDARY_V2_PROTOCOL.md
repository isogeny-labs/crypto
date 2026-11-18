# H116 Frobenius-boundary v2 protocol

`H116-FROBENIUS-BOUNDARY-002` retains the v1 fixture, precision, independent
verifier, controls, first-difference prediction 15 and delta prediction 10.
Version one remains immutable and rejected by its scalar-map curve-identity
control. [pcc:unverified]

The only permitted producer correction is the sign documented in
`FROBENIUS_BOUNDARY_V1_ERRATA.md`: the rational duplication numerator ends in
`+A^2D^4`, not `-A^2D^4`. The v2 producer is a separate source file and writes a
separate result. Any other construction, prediction or fixture change
invalidates v2. [pcc:unverified]

