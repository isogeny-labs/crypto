# Review outcome: force a chosen graph divisor before searching for a relation

The H2 cost loss is recorded in the sibling template-cost packet. This review
compares cheaper next steps and corrects the rho applicability assumptions.
All producer evidence remains unsigned: PCC-20261124-d02f72. [pcc:unverified]

## Actual reference conditions

The public model's j is neither0 nor1728. Its j and coefficients are not
fixed by p, p² or p³; all are fixed by p⁶. The repository's prime-field j0
sixfold fold and a bare proper-subfield Frobenius fold therefore cannot be
applied by assumption. This is not an exhaustive endomorphism/isogeny search;
other constructed maps and optimized negation/plain rho implementations
still need consideration. No rho walk ran here. [pcc:unverified]

The native invariant calculation checks c4³-c6²=1728*discriminant, known
j0/j1728 fixtures, singular refusal and the original public points. Sources
are DGM1999 §§3–4, Sage's official invariant formulas and the actual E6
constructor. [pcc:unverified]

## Competing cost opportunities

**H5, reduced field equations.** After a true quotient basis G is available,
adding NF_G(x_i^p)-x_i preserves every original Fp point. It may remove
non-rational components before expensive linear algebra. However, the current
root stage already uses gcd(f,x^p-x) after Berlekamp–Massey. It must pay for
new normal forms and any ideal update. On the exposed rejection pair, making
*all*398,639 root products free leaves1,029,748 of1,428,387 partial products:
an optimistic factor ceiling1.3871. This is not a full attack bound or a
prediction that the filter attains it. [pcc:unverified]

One coordinate's field equation does not generally make all coordinates
rational. Nor does dim ker(Frobenius-I) count rational points: F9 over F3 has
a one-dimensional fixed space, while x²+1 has no F3 root. The controls retain
this counterexample. The Joux–Vitse small-degree summation-polynomial paper's
§2.4 explicitly warns about high-degree field equations. Sparse FGLM §§3.1–3.3
already exploits multiplication matrices; this is not a new mechanism.
[pcc:unverified]

**H3, larger homogeneous kernel.** There are509 factor-base columns. The
recorded collection first admits rank508 at B=155648 with518 projected rows.
Earlier reported rank0 values are unadmitted lower bounds. A complete kernel
of dimension k must be computed and retained; one vector cannot represent
the missing directions. [pcc:unverified]

For k<=8, rank must be at least501, so at least501 projected rows are
necessary. The first recorded complete-batch checkpoint with that many rows
is B=153600 (503 rows). Within this fixed recorded schedule, at most2048
fewer B values could be examined before the observed stop, even under an
optimistic rank assumption. This is a necessary row-count bound, *not* a
bound on CPU cost or a certified earlier kernel. Extra descents may cost
more than that opportunity saves; k>8 and other workloads remain open.
[pcc:unverified]

## H6: construct relations containing a chosen large-prime divisor

Use graph state to choose an existing valid quadratic divisor Q=(U,V), then
force the principal function to contain Q before solving the remaining
conditions. This changes the earlier anchor-lookup question: the previous
zero-anchor descent result does not measure a constructor that imposes the
divisor from the start. No novelty or measured gain is asserted.

Let H be monic of degree7, U monic of degree2 over Fp, deg(V)<2 over Fp²,
and U divide H-V². Define

```
W = (H-V²)/U,                 C = x+c0,
T = t0+t1*x+t2*x²,            A = U*T-C*V,
S = U*T²-2*T*C*V-C²*W.
```

Then A²-C²H=U*S. This is the existing norm-quotient identity, specialized
to a degree-two graph divisor and a linear C; the generic identity was
already qualified in hypotheses-20261123. S has degree7 and leading
coefficient-1. Requiring S to lie in Fp[x] gives seven quadratic coefficient
conditions on the eight Fp coordinates of c0,t0,t1,t2. Expected dimension1
is a heuristic: dependence, singular components, rational-point scarcity
and lack of a cheap parametrization may defeat the construction.
[pcc:unverified]

If S has five distinct admissible small roots and a distinct irreducible
quadratic factor R, the existing principal checker can validate a row with
Q, R and five small points. Verify C is invertible on all support, full norm
factorization, signs, squarefreeness and original Jacobian relation. Feed the
checked row to the existing graph; a topological cycle alone is not a target-
useful independent relation. No graph/rank success follows from the identity.

The first falsification is small: choose the first retained two-quadratic
source deterministically, recover its planted (c0,t0,t1,t2), compile the seven
equations, check the full7x8 Jacobian and replay the original function.
Then freeze a bounded fiber experiment with all preparation/trace/F4/root/
factorization/graph checks charged. Reject a cost or yield claim unless
forced-divisor construction, failures and graph projections repay existing
collection at the same supplied-state boundary. Do not extrapolate from
dimension counting or a single planted point.

## Evidence and scope

[PLAN.md](PLAN.md), [Rust audit](review.rs),
[safe public output](results/public-review.json),
[complete raw-data archive](results/raw.tar.xz),
[source/input pins](source-worker-inputs.sha256), [primary hashes](literature.sha256).
The sole metadata call had status0/native/launcher/posthash, empty stderr,
and no new F4, root finder, rho or DLP execution. This packet is a review and
proposal, not a performance benchmark or full endomorphism classification.
[pcc:unverified]

Sources checked: Joux–Vitse CoverDecomp §§3.2–3.3 (structured/sieved families
and double large primes), its separate small-degree paper §2.4, Sparse FGLM,
DGM1999 primary author version, Sage invariant source, current rank tracker,
codimension-one quotient and principal/quadratic relation checks; earlier
H3/H4 and anchored-descent notes. KB has no loaded server/executable; ecdb
has no available read token/AWS access. Neither gap establishes novelty.

H6 needs an exact Rust constructor and positive/exceptional controls before
any new relation-finding experiment. The next packet tracks that work on
the same [draft lane PR #5094](https://github.com/isogeny-labs/crypto/pull/5094).
