# An unexecuted degree-eight route for the unshifted quartic stratum

This is a derivation and bounded next plan, not executed code or a novelty
claim. It does not change any frozen source, inputs or raw evidence. The
supported source multiset has binary rank at most one, so this route may
overlap the repository's already completed rank-one original-root component.
That overlap must be checked before describing this as new coverage or
choosing it over the existing strong specialized method.

## Exact coefficient identities on the actual a=0,b=1 binary curve

Let K=GF(2^83) with the identified original modulus. The curve is
Y^2+XY=X^3+1. Fix the actual target point T=(z,w), with z nonzero, using the
same sign convention as the norm importer. Set

    F=X^3+r X^2+s X+t,    G=h X+k,
    N=F^2+XFG+(X^3+1)G^2.

Require the UNTRANSLATED stratum N=(X+z)(X^5+cX). Equating coefficients gives

    h^2+h=z,
    k=r^2+h r,
    h s=r k+k^2,
    t=k,
    c=k^2/z,
    s^2+s k+k h+h^2+c=0.

These are literal polynomial coefficient identities. The equation t=k follows
from t^2+k^2=0 by injectivity of squaring in the field, with no discarded
square-root branch. Since z is nonzero, a solution to h^2+h=z has h nonzero,
so division by h and z in the following elimination is legitimate. Also h
cannot be one. The h equation has zero or two solutions in K, determined by
the absolute trace of z; do not replace an unsolved equation by two samples.

Define k(r)=r^2+h r and A(r)=r k(r)+k(r)^2. Substitution s=A/h into the last
identity and multiplication by the NONZERO h^2 z gives

    Q_h(r)=z A(r)^2+z h A(r)k(r)+z h^3 k(r)+z h^4+h^2 k(r)^2=0.

The leading coefficient is z and the degree is exactly eight: A has degree
four and all other terms have degree at most six. Thus there are at most
eight distinct field values r for each admitted h, at most sixteen overall.
This is a coefficient-candidate bound for this stratum, not an original-root
count or a global complexity claim.

Recover k,s,t,c from every r and replay every coefficient of the literal norm
N=(X+z)(X^5+cX). Retain the target-ordinate condition F(z)+G(z)w=0 under the
admitted importer convention. The norm alone can vanish at T or its conjugate.
When F(z)=G(z)=0 both conditions hold; this exceptional fiber must be kept,
not rejected by division by G(z). No inversion of G(z) is required.

## Source support and outstanding recovery interfaces

For each retained c, let a=c^(2^81). The quintic is X(X+a)^4 and has supported
sources iff a belongs to V, equivalently c lies in
span_F2(1,alpha^4,...,alpha^68). Source tuples are all distinct permutations
of (0,a,a,a,a), with the c=0 all-zero tuple counted only once. As a useful
cross-check, k=0 forces r=0 or h; then s=t=c=0 and the X^2 residual h^2 cannot
vanish for this nonzero-z target. Hence the c=0 norm stratum is empty here.
This last statement depends on the declared monic norm chart and z nonzero;
it is not a general zero-source exclusion.

Function solutions, source tuples and original polynomial roots are different
outputs. Several r/h pairs may map to one c; keep every function branch and
deduplicate only under the declared output key. Zero abscissas can be ramified
or torsion. If G(a)=0, the norm may contain nonrational/conjugate point branches
even though the abscissa is in K. Original auxiliary-coordinate, ordinate,
ordering, ghost and exceptional recovery must use a separately admitted
complete interface. This note supplies neither that interface nor a global
GF(2^83) all-roots result. A translated stratum (X+b)(X+a)^4 is a different
family and is not silently covered by these equations.

## Concrete bounded next experiment

First compare symbolic coefficient expansion and the elimination identities on
all small binary fields/parameters under a fixed finite cap. Independently
enumerate original F/G coefficient tuples in the tiny fields and compare the
complete coefficient/function sets, including target-ordinate and zero-G
branches. Check z=0 separately as unsupported by this elimination; never divide
there. Check the c=0 contradiction and repeated sources explicitly.

Only after reviewing the current rank-one component for duplication, admit the
actual original target, field and norm normalization. Solve h^2+h=z exactly.
For every h, compute gcd(Q_h,X^(2^83)-X) by modular Frobenius powering, then
recover ALL its linear factors with a separately source-identified exact field
root routine. A failed splitter or cap leaves UNKNOWN, not an empty set. Keep
all at most sixteen coefficient candidates and replay literal norm, point and
support checks. Freeze commands, target/root order, source/input hashes and
full budgets first. Proposed bounds:90seconds,4GiB per process,one core,
at most200000 scalar predicates,degree<=8 polynomial storage and bounded
deterministic splitting. No enumeration of2^73 states,2^18 source elements,
or the full d0 field is authorized by this plan.

The next decision is either scoped closure, a complete finite coefficient
branch, a failure of normalization admission, or a bounded inconclusive root
extraction. None certifies the genuine F4/F5/F6 release gate, novelty, or8x
global superiority.
