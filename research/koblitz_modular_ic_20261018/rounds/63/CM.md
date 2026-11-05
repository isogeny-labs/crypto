# R63 degree-112 source and actual half-space descent

`generator112-01` and the independent actual-source `check112-01` pass the
strict native gate. The marked-23 source has degree 112 over F2(x), with all
112 standard monomials `lambda^i y^j` for `0<=i<16`, `0<=j<7` checked.
The generator construction produces 112 square pairs and verifies all 124
whole-source functions: the basis, three controls and nine named source
functions. Both source generators have checked derivative inverses; the
producer records 268 rational coefficient parity checks. The replay
independently rebinds both saved generator bundles, all 124 inputs and
roots, both inverse products, and changed-inverse/root negatives, without
repeating the producer's inverse computation. These are source-field
identities; the selected order-991 factor action is a separate test.

`half-descent63-02` passes on the actual R61 degree-32 source. It restricts
the saved involution to the complete R62 square-root spaces. Each inclusion
has a checked left inverse, satisfies `K A = S K` and `A²=I`, and rejects
an altered action. The complete F2 ranks are:

| Space | Root dimension | Fixed dimension | Trace rank | Trace one |
| --- | ---: | ---: | ---: | --- |
| Mixed | 10 | 6 | 4 | actual source function |
| Cubic | 128 | 64 | 64 | actual source function |

The source check binds all 138 retained roots to the actual rational
functions, checks `g²=f` for each, then checks the involution of each root
and its square: 20 mixed and 256 cubic identities. It also verifies all
70 fixed roots and both trace-one functions in the source quotient. The
10 roots lie inside the larger 128-dimensional space, so the dimensions
are not additive. The witnesses prove trace-one existence within these
two finite spaces; neither space has yet been identified with the complete
proper Riemann--Roch space. Their actual pole divisors and source model
normality remain to be constructed.

Pinned Lean 4.34.1/mathlib d13f23b audited five authored implications:
characteristic-two squaring injectivity in a field, fixedness of a unique
square root, integer pole-cap halving, transfer of an involution along a
left inverse, and exclusion of a trace preimage by an annihilating
functional. The audit reports zero failing and zero trusted native
decisions. These conditional theorems do not assert the curve-specific
source equations or valuations; the native certificates provide the
finite-source inputs separately.

The first half-space attempt is rejected despite its late PASS marker.
The R61 saved cubic bundle omitted the ambient constant vector used by
that script, causing Singular question-mark errors. Version5 reconstructs
that vector as the saved ambient inclusion times the saved mixed constant
and checks its nonzero, invariant property before the complete fresh run.
`results.json` preserves both terminal outcomes and 29 historical source
byte pins. The accepted half-space attempt never reads the failed partial
output.

The next construction lead is the trace-dual descending Frobenius lattice
in [R64 NORMALIZATION-LEAD.md](../64/NORMALIZATION-LEAD.md), using the
Singh--Swanson integral-closure theorem. For the rank-32 order the retained
discriminant predicts an F2 quotient length of 160. A complete step is a
binary kernel on that finite quotient; equality is an exact affine
normalization stopping certificate after the source-field bridge is
checked. The compact PID variable is `z=1/u` relative to the actual R61
field, and its old quadratic equation agrees under `V=z²v`. The degree-16
equation and all basis functions still need explicit source reduction.

Next checks: normalize both affine charts, compute the exceptional branch
valuations of `b,c,lambda,nu` and all registered denominators, then use
complete prescribed Riemann--Roch spaces for level-77 maps and subgroup
action. The node187 coupled divisor, full quotient saturation, arbitrary
reverse evaluator, and selected-991 action retain their own acceptance
tests. All original n=3 and same-field n=5 controls remain available.
