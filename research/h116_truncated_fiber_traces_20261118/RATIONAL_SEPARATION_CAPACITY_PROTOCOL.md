# H116 rational-map separation capacity protocol

Protocol `H116-RATIONAL-SEPARATION-CAPACITY-001` is frozen before either new
implementation is written or executed.  It measures the finite sample budget
available to the new 121-theorem Lean separation interface on the unchanged
eight-cell extension-field panel. [pcc:unverified]

## Frozen question and sample

For each curve over `GF(p^d)`, the sample `S` contains every distinct affine
`x` for which `y^2=x^3+a*x+b` has a solution.  Infinity is excluded because
the proved certificate is stated for polynomial numerator/denominator pairs
evaluated at finite field elements.  The raw strict cross-product degree
capacity is `|S|-1`; a future supplied map must also remove every sampled pole.
[pcc:unverified]

The eight cells, deterministic irreducible-polynomial rule, parent group
orders, and previous fixed-locus data are byte-bound in the JSON protocol.
The base-field distinct-x count is independently enumerated on the degree-one
curve; it is not inferred by filtering extension points.

## Frozen checks and controls

Every affine fiber must be a singleton at `y=0` or a pair `{y,-y}`.  Thus the
producer and independent verifier must reproduce

```text
affine points = singleton fibers + 2 * paired fibers
distinct x    = singleton fibers + paired fibers.
```

Both implementations construct the complete polynomial
`M_S(T)=prod_{x in S}(T-x)`, retain its canonical coefficient encoding, and
check that it is monic, nonzero, degree `|S|`, and zero at every sample.  This
is the empirical strict-bound control: a degree equal to the sample count can
remain invisible.  Adding one to its constant coefficient must be rejected at
every sample.  Every row is retained whether or not it passes.

## Requirement-to-evidence boundary

| Requirement | Evidence after this experiment |
|---|---|
| Strict finite-evaluation implication | Lean theorem |
| Strictness of the degree inequality | Lean `ZMod 3` control plus `M_S` in every cell |
| Raw distinct-x capacity | measured twice |
| Actual H116 numerator/denominator construction | open |
| Pole-free sample count | open until denominators are supplied |
| Cross-product degree below capacity | open until a degree certificate is supplied |

Timings and operation counts are diagnostic single runs.  This protocol does
not construct a rational or scheme morphism, solve ECDLP, or support a runtime
or asymptotic claim. [pcc:unverified]
