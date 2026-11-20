# R107: lambda coefficient valuations over the two x=1 base completions

Bind the exact R51 triangular donor, R105 parser, and R106 order-32
base series by SHA-256. Substitute `x=1+t(s)` and `y=z(s)` or `1+z(s)`
into all 29 G51 source terms, evaluating x-power denominators as units
in F2[[s]] modulo s^32. Check `G(s,l)` has lambda exponents only
16,8,4,1,0. Then translate `l=alpha+u` over F4, where
`alpha^2+alpha+1=0`, and report each u-coefficient's exact s-order if
below 32 or a lower bound of 32 otherwise, with its leading F4 scalar.
The output must reconstruct the x=1 `u^16+u^8` fiber.

Do not infer a full normalized lambda branch count from truncated
coefficients. A Newton edge is certified only if every omitted or
zero-through-precision coefficient has a lower bound above the proposed
edge. Fresh replay must be byte-identical; a changed source hash must
fail. This is deterministic exact arithmetic, no statistical comparison
or cloud resources. Commit protocol/source before data and retain errors.

Sources: R51 donor, R105 mixed jets, R106 base uniformizers,
[Newton polygon theorem](https://www.math.columbia.edu/~gyujinoh/ANT_revised.pdf),
and [Stacks Hensel lemma](https://stacks.math.columbia.edu/tag/04GE).
