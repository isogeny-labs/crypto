# Protocol: higher-degree image-closed fiber calculus for the n=83 cover

Frozen before the new closed-point audit was implemented or run.  This is a
deterministic structural assessment, not a timing experiment.

## Question

For the fixed binary cubic cover

```text
E : y^2 + x y = x^3 + 1,
C : v^2 + u^2 v = u^7 + u,
phi(u,v) = (u^3, u v + 1),
```

can the degree-two closed point in a generic rational fiber supply projected
factor-base collisions or independent relations that justify implementing an
image-compatible genus-three relation collector?

The pinned target subgroup has prime order

```text
r = 2417851639230796216685689.
```

## Frozen fixtures

- Odd extension degrees `n = 9, 13, 19, 23, 83`.
- At every degree, the public generic point `P=(1,0)`.
- At `n=83`, also the pinned order-`r` generator
  `(0x477f77103dfad59850800, 0x2fa5e737d542c4e4fd5c3)`.
- Positive parity control: `n=2`, where the nontrivial cube roots of unity are
  rational and the same fiber splits into three degree-one points.
- Exceptional control: the ramified point `(0,1)`.

The field moduli are the repository's deterministic sparse irreducibles.  The
receipt records every modulus.

## Exact checks

1. Factor the reduced Mumford support of `phi^*([P]-[O])` over the base
   field and reconstruct it from the reported irreducible components.
2. Push every component separately by the residue-field norm already used by
   the general Jacobian pushforward.
3. Check that a component of degree `d` pushes to `[d]P`, that the component
   images sum to `[3]P`, and that the general pushforward returns the same sum.
4. Require factor degrees `[1,2]` on every odd-degree generic fixture and
   `[1,1,1]` on the even-degree positive control.
5. On the pinned order-`r` generator, check that the degree-two label is
   `[2]P`, is nonzero, and remains distinct under the prime-order subgroup
   assumptions.
6. Replay the group identity `[3]P=P+[2]P` in Lean for an arbitrary additive
   commutative group.

Any failed reconstruction, orbit closure, component image, parity control, or
sum check stops the audit.  No failed fixture may be dropped.

## Relation and cost gates

The candidate is admitted to implementation only if both gates pass:

1. **Information gate.**  The higher-degree fiber must create a distinct
   projected column collision or a known-source-log relation not implied by
   scalar multiplication on `E`.
2. **Complete-cost gate.**  A named relation search, linear algebra, blind
   individual-log descent, memory model, and verification path must have total
   exponent below `r^(1/2)` or predict a finite crossover against the strong
   source rho implementation.

Raw closed points with the same projected elliptic label are merged before
rank or matrix size is counted.  A projected identity that is only
`[3]P=P+[2]P` fails the information gate: when `P=[a]G` is source-labelled,
both right-hand labels already have known scalars; when `P` is the unknown
target, the identity reveals no scalar.

If the fiber candidate fails, arbitrary degree-two cover primes are considered
only with an explicit target-local router.  Projecting ordinary ambient
genus-three relations retains the prior approximately `q^(4/3)` collection
model and is not a new arm.

## Controls and claim boundary

- The even-degree split control detects a factorization implementation that
  always reports `[1,2]`.
- The ramified point detects confusion between reduced support and
  scheme-theoretic multiplicity.
- The existing exhaustive 486-class `GF(8)` norm table remains the independent
  arbitrary-class pushforward control.
- This audit may establish a scoped rejection of the degree-two rational-fiber
  design.  It does not prove that every higher-degree or adaptive relation
  calculus is impossible.
- No wall-clock, throughput, attack, security, or speedup claim is in scope.

## Literature gate

- P. Gaudry, *Index calculus for abelian varieties of small dimension and the
  elliptic curve discrete logarithm problem*, JSC 44 (2009),
  <https://doi.org/10.1016/j.jsc.2008.08.005>.
- P. Gaudry, E. Thomé, N. Thériault, C. Diem, *A double large prime variation
  for small genus hyperelliptic index calculus*,
  <https://eprint.iacr.org/2004/153>.
- C. Diem, *Index calculus with double large prime variation for curves of
  small genus with cyclic class group*, <https://arxiv.org/abs/math/0606607>.
- Repository implementations and measurements:
  [`../ic_improve_20261014/04-genus3-2lp/`](../ic_improve_20261014/04-genus3-2lp/)
  and
  [`../ic_improve_20261014/04b-genus3-2lp-rebalance/`](../ic_improve_20261014/04b-genus3-2lp-rebalance/).
- Independent bounded-branch control:
  `crypto-autoresearcher/ideas/artifacts/ECDLP-IDEA-002/p1546_projected_smoothness_counting_gate.md`.

The classical sources justify the rational-place/full-Jacobian baselines and
their `q^(4/3)` genus-three scale.  None supplies a source-labelled
higher-degree target-local router.  P1546 gives a scoped `Omega(r)` relation
evaluation bound for fixed bounded-degree reduction branches and leaves only
compact growing-degree/adaptive routers unclassified.
