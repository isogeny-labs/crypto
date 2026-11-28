# Scheme-aware base-divisor cancellation amendment

V2d0f2e006a943 failed once, exit101, retained unchanged. Its480norm controls
pass; first actual split triple includes residual basepoint101,43, where all
cubic numerators vanish. V3 keeps ALL same inputs and fixes section evaluation,
not the nonzero-vector gate. No speed, general-lift or10xclaim.

For each same squarefree split degree3divisor u,y=v(x), require F(x,v)=0 mod u
and invertible F_y modulo u². Other charts remain unsupported. Derive common
factor g=gcd(u,N0(x,v),...,N7(x,v)) (degree<=2), rather than hardcode a root.
Lift v once by Newton to vtilde mod u² and require F(x,vtilde)=0 mod u².
For each numerator, compute N_i(x,vtilde) mod u², divide exactly by g, then
reduce mod u. Since the fixed residual divisor is reduced and g divides u,
this gives section values with the residual base divisor removed. At remaining
points division by g is a harmless common projective scale per point.
The residual discriminant is60 mod127, nonzero; repeated residual bases need
higher thickening and remain outside this gate. No repeated-u map claim.

Independent split-point oracle: when g(x)!=0 evaluate N_i/g. At a root of g,
evaluate (N_x+N_y*(-F_x/F_y))/g'(x), all nonzero denominators required. Compare
all120normalized trace tensors against six direct permutations of these
pointwise values, all4same point triples. Require480identities, four nonzero
8coordinate vectors, and retain computed g per triple. V2's480arbitrary-algebra
norm controls repeat unchanged, with explicit non-curve labels. V1basis controls
remain. Common denominator cancellation changes neither section basis nor
expected coordinate vectors (none were supplied).

Proof: Hensel error is squared, so u² precision gives correct first derivatives
along C at each simple point of u. A remainder modulo u² changes division by
g only by a multiple of u because g divides u. Exact polynomial division then
computes N_i/g on the reduced divisor, including its derivative limits at g=0.
Pole/diagonal charts, rational theta centering, source alignment and actual
Kummer inversion remain open. Full10x/genus2/4 obligations unchanged.

Freeze source/protocol/fingerprints/commit/checkpoint before one run to
RUN-V3.txt/stderr and EXIT-V3.txt. Retain any failure, no rerun without amendment.
