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
