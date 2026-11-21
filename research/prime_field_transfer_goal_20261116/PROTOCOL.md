# Round-one development correctness protocol

Frozen before native calls. No timing or performance experiment is authorized
by this protocol. Local builds, unit tests and static checks are development
work under `isolab-eks`; paid benchmarks require its independent admission.

## Model constructor

For odd prime p, positive integer r and five source Weierstrass coefficients
a1,a2,a3,a4,a6, store the integral model as

    A1=1+2^(r+1)c1,  A2=2^(r+1)c2,  A3=2^(r+1)c3,
    A4=2^(r+1)c4,    A6=2^r c6.

Choose c1=(a1−1)2^(−r−1) mod p and ci=ai 2^(−r−1) mod p for
i=2,3,4. Choose c6=a6 2^(−r) mod p, adding p when needed to make it odd.
The source field projection is direct modular exponentiation. For bounded r,
expand coefficients and compute c4 and Delta independently to check v2(Delta)=r
and v2(c4)=0. The binary tangent cone is y(y+x), so the written Tate-algorithm
interpretation is split type I_r. This interpretation is not a new formal
Neron-model implementation.

Fixed checks: p=17,r=19, coefficients (0,0,0,2,2); p=257,r=83,
(0,0,0,23,11); p=37,r=7,(0,-1,1,-10,-20). A 256-bit source-model
check uses the public secp256k1 p and r, coefficients (0,0,0,0,7), comparing
the modular exponentiation to OpenSSL. This last check is only coefficient
construction, not a transfer, a subgroup solve or a scaling benchmark.

## Arbitrary-lift negative control

Use the fixed rational curve

    y^2+xy=x^3−64x+288,

with rational points S=(4,8), T=(2,12). Its component labels at 2 are
kappa(S)=2, kappa(T)=1 modulo 5, using the oriented tangent convention
of Cremona section 2; the model has c4 odd and v2(Delta)=5.

Scan only good primes 11≤p≤251. Select the first with #E(Fp)=5h,
with nonzero [h]S and [h]T lying in the same order-five subgroup, and
[h]T=[n][h]S for which h−n(2h) is nonzero modulo 5. Construct the
exact rational point U=[h](T−[n]S). Check red_p(U)=O and its component
label nonzero. This is a fixture-selection check for a specific false
shortcut, not evidence that every possible transfer fails.

Independent paths: exact rational group addition over Q and modular point
addition over Fp. No source scalar is used by the model constructor. The
small scalar enumeration in this negative control belongs solely to the
auditor and is explicitly unsuitable as a general transfer evaluator.

## Controls and outputs

- Perturb one compressed coefficient; source-fiber equality must fail.
- Change c6 parity; the expected discriminant valuation must fail.
- Reject inconsistent component labels on equal source group relations.
- Retain raw stdout/stderr, compiler version and build exit status under
  `receipts/round-01/`; do not replace failed outputs.
- Do not claim a practical reduction, speedup or novelty from these checks.

The next branch is decided by outputs. If arbitrary lifting fails the kernel
test, retain the component target and demand a compatible torsion/section
lift; do not repair labels by computing the source logarithm.
