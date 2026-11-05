# Complete coordinate-defect accounting on the proper models

DEFECT50 terminates with exit0, no error lines and its exact marker. The
finite-chart quotient has F2 dimension898. On the infinity chart the
dimensions after imposing z^k, for k=1,2,4,8,16,32, are respectively
16,32,64,86,94,94. Hence the full z-primary contribution has dimension94.
The sum898+94=992 accounts for the entire plane-to-integral-v defect on
the proper two-chart model. This includes the previously isolated cyclic
128-dimensional block at u=1, and leaves770 finite-chart dimensions outside
that block (derived subtraction, not a separate support decomposition).

Why the local truncation is complete: put M=F2[T,z]/(Ainf,Binf).
Equal dimensions of M/z^k M and M/z^(2k) M imply z^k M=z^(2k) M.
It follows by iteration that the powers have stabilized; the quotient is
exactly the finite z-primary part. At nonzero z, the class of z is a unit
and the truncation removes that part. Multiplicities at z=0 are retained.

There is an independent global consistency equation. The accepted overlap
is diagonal on the plane power basis t^i, i<32, with weights4i, and on the
relative basis t^i v^j, i<16,j<2, with weights4i+2j. Thus their finite
pushforwards to P1 split respectively as sums of O(-4i) and O(-4i-2j).
The sums of weights are1984 and992. Since chi(O_P1(-w))=1-w, the
arithmetic genera are1953 and961. The difference992 equals the complete
native chart-length sum. This derivation uses the actual transition maps,
not a guessed degree/genus formula for a plane closure.

Primary sources checked: [Stacks30.8.1](https://stacks.math.columbia.edu/tag/01XS)
for cohomology of O(d) on projective space and
[Stacks36.5.1](https://stacks.math.columbia.edu/tag/0AVV) for affine
pushforward and cohomology. The bundle decomposition is the present
derivation from the explicitly checked monomial transition functions.

Under the retained smooth genus61 identification, the relative proper
model still has total normalization defect961-61=900. This is a useful
exact consistency target for the remaining normalized charts. The failed
finite producer's reported818 is candidate data pending independent
verification, and is not used in the accepted992 calculation. Proper
normalization and the ordinary divisor/factor-map constructions continue
from those chart checks.
