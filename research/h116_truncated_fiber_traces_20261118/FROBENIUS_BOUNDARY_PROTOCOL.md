# H116 Frobenius-boundary protocol

Protocol `H116-FROBENIUS-BOUNDARY-001` freezes the excluded p=31, r=37
fixture before execution. It compares two normalized rational maps at infinity:

1. the retained degree-37 `V-1` Vélu map;
2. the independently constructed degree-36 scalar map `[t-1]=[-6]`.

Both are composed with the same target coordinate scaling of differential
`a=t-1=25 mod 31`, so their target coefficients are `(a^4A,a^6B)` and their
leading x-ratio coefficient is one. [pcc:unverified]

For a formal group law `F`, the identity `F(X,0)=X` makes `F(X,Y)-X`
divisible by `Y`. With `V-1=[a]-pi` and the exact Frobenius parameter
`pi(z)=z^p`, the normalized local parameters therefore agree below degree p
and have first delta `-a^-1 z^p`. The x-coordinate double pole predicts that
the normalized `V-1` x-ratio first differs from the normalized scalar x-ratio
at `z^(p-1)=w^15`, with coefficient `2/a=10 mod 31`. [pcc:unverified]

The producer constructs `[6]` as `[2]([3])`; the independent verifier uses
division-polynomial recurrences for `psi5`, `psi6^2` and `psi7`. Both check the
cleared rational-map curve identity. Frozen negative controls omit the target
normalization, use the wrong target coefficients, and mutate reversed numerator
coefficient 15. No fixture, precision or prediction may be replaced after
execution. Exact field-operation diagnostics are retained without timing or
scaling interpretation. [pcc:unverified]

