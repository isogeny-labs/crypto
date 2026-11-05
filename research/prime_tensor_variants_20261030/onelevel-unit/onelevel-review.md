# Independent source review: one ternary contraction call

2026-10-30 UTC. Reviewer `/root/ternary_global_review`. Read-only review of
`PROTOCOL.md`, `onelevel.cpp`, the preserved strict-build failure and the
available run001--003 controls/summaries. The implementation agent performs the
full exact replay; I did not add a redundant execution or a timing experiment.

## Verdict

No blocking defect found in this fixed finite development unit. It implements
actual group-child sums, extension-ring interpolation and rational Hermite
recovery, rather than substituting a known answer for any of those operations.
The review does not certify the general recursive algorithm, asymptotic
parameter admission, novelty, practical speed or an ECDLP solver.

## Structural checks

- `mode_children` forms separate beta indices for target row AND source
  column. Each row chooses its local/constant term or one of its two crossing
  modes, so the27 patterns cover its complete nonzero expansion. Because each
  tile is a singleton with j=0, all b>0 source moments vanish exactly; no
  asymptotic pruning assumption is hidden here.
- For each pattern and each of the three nonempty groups, the code loops over
  all four assignments of its two retained edges. It evaluates the packet
  coefficient, multiplies source-bit powers and the two original unary pin
  tensors, and then sums. When two modes share a source, the two separate
  multiplications correctly implement the source power. Each original row
  and column bit appears in exactly one group. The two empty groups contribute1.
- Local row factors use l0(z)=1-z or l1(z)=z, and a selected crossing
  coefficient has the correct corresponding sign. The row/column matrix
  indices and the X/Y pin indices agree with source D.8.
- The separately written direct deformed sum genuinely enumerates all64
  X,Y assignments and retains their pin weights. The exact rational global
  polynomial is assembled directly from the kernels, without the mode or
  child routine. These controls share elementary kernel arithmetic but not
  the group factorization.

## Exact arithmetic and interpolation

- The F27 polynomial U^3-U-1 has no F3 root, which suffices for degree3
  irreducibility. Multiplication reduces U^3 to U+1 and U^4 to U^2+U correctly.
- Unit inversion begins with an actual exhaustive F27 inverse and uses
  y <- y*(2-a*y). Its error squares each iteration even when arithmetic is
  performed throughout at the target modulus. The final full-precision
  product check is present. All26 nonzero residue elements are controlled
  at each of the two child precisions.
- `qchart` computes f(t+9z) by exact Horner substitution. Rational conversion
  cancels common three-parts and rejects a remaining nonunit denominator;
  it never tries to invert C'(0)=-720 modulo a power of3. All kernel-chart
  coefficients are tested for integrality.
- The25 canonical extension points are distinct modulo3, so all their
  differences are units. The in-place divided-difference loop runs in
  descending index order and uses the denominator point[j]-point[j-order].
  Conversion from Newton coefficients uses the correct preceding-point
  products. All25 recovered coefficients, including all three extension
  coordinates, are compared with exact rational charts at each active node.

## Hermite recovery

The six active nodes are -2,-1,1,2,4,5. Each has multiplicity6, giving36
conditions for a polynomial of degree at most24. `HermiteWeights` computes

    E_t(0)=product_(q!=t)(-q/(t-q))^6,
    E_t(t+9z)=product_(q!=t)(1+9z/(t-q))^6

and its reciprocal truncated to degree5. For chart coefficient c, its stored
weight is

    E_t(0)*sum_(j=c..5) reciprocal[j-c]*(-t/9)^j,

which is exactly the linear D.9 functional. The code applies it both to exact
rational charts and separately to canonical coordinate lifts of interpolated
modular charts. It sums all rational contributions before cancelling and
reducing at root precision; all three recovered coordinates are checked.
The fixed precision is p+120 = p+4*6*3+8*3*(1+1), as frozen in the protocol.

## Evidence and controls

The available successful summaries report128 root cases,16 ones and112 zeros;
19200 mode/direct/exact-chart comparisons;518400 row-choice patterns;
1555200 actual child contractions;6220800 actual child assignments;
1228800 direct six-bit assignments;19200 full-chart coefficient comparisons;
and128 exact-plus-lifted Hermite recoveries. The loop structure supports these
counts. These are structural coverage counts, not timings.

The required unsplit-mode regression computes correct/direct9 and faulty0
modulo27 at the fixed witness. Nonunit inversion and non-three-integral
rationals are rejected. The wrong component direction has an explicit failed
locality witness. The initial strict compilation failed on an unused
`operator!=`; its log is retained. I found no undisclosed arithmetic failure
in the available records. Replay identity is an implementation-agent receipt,
not an independent execution by this reviewer.

## Essential limitations

The forced M=L=3, r=5 example does not meet the global load, child-shrinkage or
sufficiently-large admission conditions. Its three nonempty children have two
edges each; this is not the asymptotic M/8 bound. It exercises one active call
followed by direct leaves, not immutable creation-precision behavior across
multiple active recursion levels. Singleton tiles make higher geometric modes
zero, so this unit does not check nonzero b>0 moments. It also uses ell=1 and
has no nilpotent-V coefficient test. The separate written global proof must
carry those obligations, and a general implementation/formalization remains
open.

Reviewed source SHA256:
`60ae64eede3f9580a268ceba9832d6a47ea6235468bb13c82aa089e5360c5f6a`.
Reviewed protocol SHA256:
`ae3f862221e187f71fabd8545c911945e468b7fbf563cee4b296480f771fb900`.
