# Exact section basis gate passed

Sourcea423521ebe50/checkpoint20261115T195733Z preceded one exit0execution,
empty stderr. First registered anchor(18,52) accepted, tangent y=107x+31,
residual[97,48,121]. RRdimension8, multiplication rank18, diagonal coefficient
rank112/nullity8. All exact identities hold; corruption rejected. All8section
vectors emitted inRUN.txt. This is a local producer algebra gate, not independent
replay, general divisor lift, isogeny alignment or performance measurement.

Next V2 correctness gate: polarized norm evaluates symmetric tensor numerators
using traces in degree3quotient algebras. Repeated support tests validate algebra
only; raw numerators can vanish on diagonals and do not supply a complete map
chart there. No10xclaim; original frozen gate/source/output remain unchanged.

## V2 trace gate failed and retained

Sourced0f2e006a943/checkpoint20261115T200442Z preceded one execution, exit101.
All480norm/determinant controls passed, including irreducible cubic t^3+t+4
and nonsemisimple controls. First actual point triple produced zero raw section
vector and assertion stopped. No second execution. Originalstdout/stderr retained.

Diagnosis: anchor18,52 has residual polynomial[97,48,121], whose root101
corresponds to point(101,43) on tangent y=107x+31. Every cubic numerator N
vanishes there, although section N/l is regular after cancellation. Raw N
evaluation therefore has a known common base divisor; the allzero-vector
rejection is correct. This is not failure of the coefficient basis or norm
identity, and not a valid Kummer image.

Next separate V3 must cancel this same base divisor on the thickened closed
divisor, retaining ALL same test triples rather than filtering offending inputs.
Need Hensel v modulo u², exact common-factor division, independent derivative
checks at residual points. Repeated-support Kummer charts remain unsupported.

## V3 cancellation gate passed

Frozen source228cd63f6cca/checkpoint20261115T201024Z preceded one execution,
exit0, emptystderr. No input was dropped or changed. Same480norm/determinant
identities and480split tensor identities pass. Three triples detectedg=[26,1]
(x-101), fourthg=[1]; Hensel lift/exactdivision matches independent pointwise
derivative limits and all4coordinate vectors are nonzero:

| Omitted point index | Corrected projective vector |
|---|---|
|0|57,73,54,87,74,42,88,34|
|1|121,73,118,120,116,95,49,71|
|2|35,71,74,62,83,10,36,46|
|3|87,55,96,38,92,57,50,88|

Basis ranks18/112,8sections andcorruption control also pass. These are local
producer correctness observations; no independent replay or performance claim.
Irreducible/nonsemisimple tests validate algebraic norm identities only, not
non-split curve divisor transport. The actual curve triples are split, squarefree
and F_y-unit. Repeated support, projection poles, rational theta centering,
source-to-target projective alignment and general input lifting remain open.
V2failure remains immutable; V3is a separately committed/protocolled correction.
The next necessary step is alignment with source quotient coordinates followed
by actual public-divisor inverse solving and group checks, with no supplied
input discrete logarithm. Full cold10x/genus2/4 requirements remain unchanged.
