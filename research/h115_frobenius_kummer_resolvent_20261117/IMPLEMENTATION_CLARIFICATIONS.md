# H115 pre-measurement implementation clarifications

Freeze date: 2026-11-17

Status: frozen before producer implementation and before any candidate score.
These conventions resolve representation choices not fixed by `PROTOCOL.md`.
They do not change the candidate family, candidate cohort, labels, acceptance
rule or cost boundary.

## Extension-polynomial order

Write a monic degree-`d` polynomial as

```text
c0 + c1*z + ... + c_(d-1)*z^(d-1) + z^d.
```

Enumerate coefficient tuples `(c0,c1,...,c_(d-1))` in ordinary
lexicographic order over canonical integers `0,...,p-1`. The first irreducible
tuple is the frozen extension modulus. Extension elements use the same
low-degree-first coefficient order and are serialized as canonical base-`p`
integers `sum_i c_i p^i`.

Irreducibility is checked natively by verifying `z^(p^d)=z` modulo the
candidate and `gcd(z^(p^(d/l))-z,f)=1` for every prime divisor `l` of `d`.

## Point and sum serialization

- The point at infinity is serialized as `null`.
- An affine point is serialized as `[x_integer,y_integer]` using the extension
  integer convention above.
- True candidate fibers for nonzero targets must not contain infinity.
- Field sums are accumulated in the extension and accepted as base-field
  values only if every nonconstant coefficient is zero.

## Rational-line null control

The rational-line coset equals the whole rational subgroup and necessarily
contains infinity, where affine coordinates are undefined. For this null
control only, omit infinity and sum the frozen coordinate monomials over the
`r-1` affine points. This produces a constant control vector as intended.

This convention is ineligible for candidate selection and is not applied to
true fibers or random-line controls. A producer or verifier that inserts a
coordinate sentinel for infinity is invalid.

## Random-line control

Represent `E[r]` in the checked ordered basis `(G,T)`, where `G` is the row
generator and `T` is the producer's first deterministic transverse torsion
point. Draw nonzero pairs `(a,b)` by the protocol seed stream and retain a line
only when `b!=0` and its Frobenius image is not a scalar multiple of the line.
Deduplicate projective lines by normalizing to `(a/b,1)`. Continue the stream
until eight lines are retained.

For a line coset moment, first sum in the extension, then apply

```text
d^(-1) * sum_{i=0}^{d-1} Frobenius^i
```

coordinatewise. The output must be fixed by Frobenius and lie in `F_p`.

## Deterministic torsion construction

Let `N_d=#E(F_(p^d))` from the trace recurrence. Enumerate extension `x`
values by canonical integer and choose the canonical square root with smaller
serialized integer. Project each resulting point by `[N_d/r]`; retain the
first point `T` for which `T` is nonzero and `Frobenius(T)-T` is nonzero.

Require all `r^2` pairs `[i]G+[j]T` to be distinct and killed by `r`. The
transverse kernel generator is `Frobenius(T)-T`; its line must have exactly
`r` distinct elements and equal the checked kernel of `V-1` on the enumerated
torsion table.

## Output and digest boundary

The producer writes newline-delimited canonical JSON records followed by one
canonical summary record. Object keys are emitted in source-fixed order;
integers are decimal; no floats occur. The raw digest is SHA-256 over every
data-record byte including its trailing newline and excludes the summary
record. The independent verifier must recompute every mathematical field,
point, moment, channel, prediction, score and decoder value from `CURVES.json`;
it may not trust producer summaries.

## Cost counters

Counters increment at the public wrapper boundary for base/extension field
addition, multiplication, inversion, Frobenius, point addition, scalar
multiplication, enumerated extension states, enumerated kernel/fiber terms,
moment terms, predicate evaluations and decoder queries. Setup, candidate,
each control class and decoder phases are reported separately.
