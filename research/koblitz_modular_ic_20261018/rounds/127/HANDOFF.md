# R127 handoff

R127 source/protocol were pushed at `63ebb056b` before computation. At the degree-7 discriminant point, the actual base fiber over F128 factors as `(Y+123)(Y+90)^2 Q4(Y)` with irreducible quartic `Q4=Y^4+66Y^3+83Y^2+38Y+25`. The double root 90 is the only non-simple base-order factor. Result SHA-256: `87b87127fab784f9bf48239081194a9ebcf67456cb4204c3e1d53956e9221e30`.

Independent GF128 arithmetic and source parse, quartic irreducibility, Lean finite product, byte replay and corrected changed-source control passed. One negative-control fixture setup error is preserved in `FAILURE-01.md`. Next expand around `(x,y)=(a,90)` and lift its Newton branch before the degree-16 lambda extension.
