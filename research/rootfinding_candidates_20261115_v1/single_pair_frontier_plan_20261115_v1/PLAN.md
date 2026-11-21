# Single-pair normal chart: a possible affine-rank-three frontier

UNEXECUTED next derivation/experiment. Current paired_factor measured source is
unchanged. The existing 2+2+1 pattern has centroid b and affine source rank at
most two, so it overlaps old low-rank original-source coverage. This note
instead considers H=(X+a)^2 Q3, with a in V and a monic cubic Q3. Four distinct
source abscissas can have affine rank three. No novelty, original-root coverage
or speed gain is asserted; the new chart must pass independent admission first.

Let E:y²+xy=x³+Ax²+B, B!=0, negative target T=(z,yT), z outside V, and fixed
G=hX+k, F=X³+rX²+sX+t with
t=z³+rz²+sz+(hz+k)yT. The literal norm N=F²+XGF+(X³+AX²+B)G² equals (X+z)H.
Since a belongs to V and z does not, a+z is nonzero. Thus H has a double root
at a iff N(a)=N'(a)=0. No inversion by a or G(a) is admitted without guards.

## Normal branch a!=0 and G(a)!=0

Every norm root has a rational ordinate y=F(a)/G(a). Enumerate ALL field roots
of y²+a*y=a³+A*a²+B, using a complete quadratic solver; there are at most two.
The derivative is

    N'=kF+XG(X²+s)+X²G².

At each ordinate y, the double-root derivative condition is equivalent to

    s=(k/a)*y+a²+a*G(a).

The imposed target root and F(a)=G(a)y then uniquely give

    r=[G(a)y+a³+z³+s(a+z)+G(z)yT]/(a+z)².

Therefore this normal chart has at most two norm-parameter pairs per fixed
(h,k,a). This follows from complete ordinate enumeration and two exact linear
recoveries, not from a generic rank assumption. Recompute t, the full literal
norm, exact division by X+z, and exact division of H by (X+a)^2. Require a monic
cubic quotient and verify every coefficient identity. A zero residual checks a
candidate, while the guarded derivation proves completeness within this chart.

The remaining cubic has all roots in V iff L_V(X)^4 vanishes in K[X]/Q3;
nilpotency exponent four retains multiplicities up to three. A deterministic
source-coset splitter or separately admitted complete finite-field root routine
must then recover all three roots with multiplicities. Append the two copies
of a and emit every distinct source ordering. This is a norm-source result;
original ordinate/sign/auxiliary/exceptional S3 recovery remains separate.

## Exceptional branches must remain present

If a=0, division by a is invalid. For k!=0, norm vanishing forces
F(0)=k*sqrt(B), while N'(0)=k²*sqrt(B)!=0, so a double zero is impossible under
these premises. This proof needs its own tiny oracle before operational use.
For a=k=0, retain the common-factor branch instead of discarding it.

If G(a)=0, norm vanishing forces F(a)=0. Then F and G share X+a and
N=(X+a)^2 times the norm of their quotients. Complete recovery of this reduced
function and its cubic source support is required. It may contain conjugate
nonrational ordinates and ramified/torsion cases; do not replace it with the
normal branch's at-most-two count. Until separately implemented, return UNKNOWN
for that chart, not an empty source set or a global completion flag.

## Bounded next experiment

Use fresh source/review/input IDs and <=90 seconds, 4GiB per process, one core.
First exhaust tiny ordinary binary curves and fixed source spaces. For every
admitted normal (h,k,a), compare ALL returned parameter pairs against every raw
r,s assignment, literal norm double-root tests, exact cubic factors and an
independent source-product oracle. Exercise odd and even extensions, repeated
and zero cubic roots, both ordinate branches, a=0 and G(a)=0 exceptions.

Only then freeze actual GF(2^83) target/cells and a source-derived finite list
of a values BEFORE computation. Retain every normal and exceptional chart.
Charge field setup, all quadratic solves/inversions, norm/factor/support checks,
all source orderings, original lift work if implemented, serialization and
memory. A capped exceptional recovery makes the complete comparison unknown.

The actual a domain still has 2^18 values, and global h,k search remains large.
This note supplies neither an enumeration-free global cover nor a matched
F4/F5/F6 baseline gain. Any further affine-prefix batching needs its own shared
parameter proof; independent favorable a or ordinate choices cannot certify a
whole cell. Review old rank-three work for overlap before claiming new coverage.
