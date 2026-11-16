# Two-term Frobenius representation of the regular double-source fiber

UNEXECUTED. New hypothesis after the free-k parameterization, not a claimed
speedup or novel general solver. Frozen workers, inputs and raw evidence are
unchanged. This plan concerns characteristic two only.

## Exact field-level coefficients

Use the regular guards, target, h,a,ya and notation of
research/regular_block_20261115_v2/PROOF.md. Put c=a+z !=0 and write
s=s0+s1*k, r=r0+r1*k, t=t0+t1*k. Directly:

    s0=(1+h)*a²
    s1=ya/a+a
    r0=(h*a*ya+a³+z³+s0*c+h*z*yT)/c²
    r1=(ya+s1*c+yT)/c²
    t0=z³+r0*z²+s0*z+h*z*yT
    t1=r1*z²+s1*z+yT.

All sums are field addition (XOR in a binary polynomial basis). Set
F0=X³+r0*X²+s0*X+t0, F1=r1*X²+s1*X+t1, G0=hX,
C=X³+A*X²+B and D=(X+z)(X+a)². Then

    N(k)=N0+k*N1+k²*N2
    N0=F0²+X*G0*F0+C*G0²
    N1=X*(F0+G0*F1)
    N2=F1²+X*F1+C.

Every Ni is exactly divisible by D. This is a polynomial identity in the
formal parameter k: the root/derivative and target identities hold over K[k],
so division is justified globally, not by finitely many evaluations. N0/D is
monic cubic, while N1/D and N2/D have degree at most1. Therefore

    Q(k)=X³+b*X²+T(k)*X+U(k), b=h²+h+z,
    T(k)=alphaT*k²+betaT*k+gammaT,
    U(k)=alphaU*k²+betaU*k+gammaU,

where comparison of the X4/X3 coefficients of N1,N2, or equivalently of
H=(X²+a²)Q, gives the explicit scalars

    alphaT=r1²
    betaT=1+h*r1
    alphaU=1+r1+z*r1²
    betaU=r0+h*s1+z*betaT
    gammaT=r0²+h*r0+h²*A+z*b+a²
    gammaU=h*s0+z*(gammaT+a²)+a²*b.

These are TWO-term linearized quadratics plus constants, not arbitrary
83-by83 binary linear maps. This field structure can survive when the exact
binary incidence graph has a single large connected component. Ordinary
formal Jacobians still cannot remove the squared terms.

## Eliminate the common k² without losing compatibility

Define Delta=alphaU*betaT+alphaT*betaU and let
T'=T+gammaT, U'=U+gammaU. The same k appears in both equations:

    alphaU*T'+alphaT*U'=Delta*k.

If Delta!=0, recover k=(alphaU*T'+alphaT*U')/Delta. Enforce ONE original
linearized equation with a nonzero square coefficient, then the other follows
from this identity. alphaT and alphaU cannot both vanish here: alphaT=0
implies r1=0 and alphaU=1. Thus choose T when alphaT!=0, otherwise U.
Together with T in W2, U in W3, this gives an exactly equivalent system on
35+52=87 support coordinates for the actual prefix domain. It is not yet a
complexity reduction: the remaining one field equation still expands to up
to83 binary constraints, with at least4 free dimensions when consistent.
The benefit must come from cheaper structured operator application or a
better elimination layout, and must be measured against the current exact
83-variable Gaussian representation including conversion costs.

If Delta=0, do not divide or discard the fiber. Impose the compatibility
alphaU*T'+alphaT*U'=0 and keep either original nonzero-square equation in k.
For fixed compatible T,U this is a scalar additive quadratic: a nonzero
linear term permits0or2 roots, a zero linear term has exactly one square root,
and zero coefficients require the explicit constant test. The total search
over compatible support coordinates may remain large. Retain the full
binary affine family or return UNKNOWN; no empty-branch inference is valid.
The forbidden regular value k=h*a remains in the separate common-factor
branch, even when it lies in a compatible image family.

## Falsifiable bounded next experiment

Create a NEW worker and fresh independent review; no prior source mutation.
First independently derive/replay all six coefficients and D divisions on
GF4/GF8/GF16, every k in the existing nonvacuous fixtures. Exhaust Delta=0 and
Delta!=0 branches using all target/source ordinate choices within a fixed
200000-state cap, including k=h*a and r1=0 guards. Compare complete affine
families by independent membership/rank, then exact support, multiplicities
and all source orderings against exhaustive multisets. New fixture selection
must be frozen before execution. A missing degenerate fixture is a measured
coverage gap, not a passed degenerate branch.

Then freeze the SAME actual h,a=1,ya=0 fixture as the current profile, without
choosing an easier cell after rank observations. Record the six scalar
coefficients and Delta, represent the support-coordinate equation as structured
multiplication/Frobenius operations, and compare its complete affine family to
the existing83-k family. Retain EVERY bounded candidate k, its cubic Q, L_V mod Q and the fourth-power residue, including rejected candidates. The current profile did not retain its sixteen rejected Q/residue pairs, so independent post-run rejection replay is a concrete missing evidence obligation. A separate reviewer must replay every recorded rejection and support certificate without rerunning candidate search. Measure construction, elimination, support/factor/
ordering, serialization and peakRSS separately AND total. Three same-process
component reps may calibrate costs, but only cold matched end-to-end baselines
can establish the release gate. Cap nullity12/4096 enumeration,90s,4GiB,1core.

Reject this optimization hypothesis if conversion plus elimination costs exceed
the dense representation on the fixed actual cell; retain the exact coefficient
identity and try matched structural SCC/BTF or a different admissible chart.
A large connected incidence component only rejects block-diagonal separation,
not a matched block-triangular decomposition or this two-term field structure.
Full h/a/ya search, exceptional fibers, original auxiliary roots, odd fields and
genuine F4/F5/F6 matching remain separate obligations.
