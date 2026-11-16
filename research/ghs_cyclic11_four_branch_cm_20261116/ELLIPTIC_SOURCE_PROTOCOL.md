# Frozen explicit elliptic source search for two F3125 traces

16 November 2026. The opposite-pair quotient screen yielded ordinary
non-descended trace candidates `t=2` and `t=24` over
`K=F5[z]/(z^5+4z+1)`. Search short Weierstrass models
`E_{A,B}: y^2=x^3+A*x+B` over this fixed field.

In deterministic order `A=1..40`, `B=0..3124`, skip singular models
using `4A^3+27B^2 != 0`. Precompute all field squares and cube values;
count every K-point exactly. Stop once one model of each trace is found,
recording coefficients in the polynomial-basis integer encoding and the
full point count. If either trace is absent in the bounded population,
record the entire scanned population and censor that arm. Verify the
field arithmetic by checking `a^3124=1` for all nonzero `a`.

This produces exact source equations and orders, not a map to the genus-five
quotient. It does not benchmark any attack. Freeze protocol and native Rust
source before execution, retaining compiler, source hash, stdout and
stderr. Any correction requires an amendment.
