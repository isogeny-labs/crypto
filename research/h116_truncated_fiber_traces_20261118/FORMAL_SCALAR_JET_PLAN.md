# H116 formal-scalar low-jet construction plan

Status: frozen exploratory control plan. This is an additive excluded-fixture
checkpoint and does not modify the gated H116 protocol or open target data.

## Requirement-to-evidence map

| requested requirement | frozen evidence |
| --- | --- |
| new non-generic construction | compute the local jet from `V-1=[t-1]-pi` in the elliptic formal parameter, not from a generic walk or relation collection |
| avoid dense degree-`r` map construction | candidate accepts only `(p,A,B,t)` and fixed precision; the retained full `N,D` record is loaded only by the oracle comparison |
| control | compare with the excluded `p=31,r=37` full Vélu map and the already checked Bostan recurrence |
| mechanism challenge | inject the first Frobenius term and require the first difference at the predicted `(p-1)/2` coefficient |
| normalization challenge | wrong differential or wrong scaled target coefficients must disagree with the oracle prefix |
| measurement | exact base-field additions, multiplications and inversions for candidate construction, recurrence control and full-map oracle reported separately |
| proof boundary | distinguish the formal-group identity and precision cutoff from the still-open Lean elliptic-isogeny/formal-group instance |

[pcc:unverified]

## Frozen candidate

For `E: y^2=x^3+A*x+B`, use `z=-x/y` and write

```text
w_curve(z) = -1/y = z^3 q(z),
q = 1 + A z^4 q^2 + B z^6 q^3,
x(z) = z^-2 q(z)^-1.
```

Let `a=t-1 mod p`. Compute the invariant differential
`omega=dx/(2y)` from `q`, integrate it to the formal logarithm `L`, and obtain
the scalar series

```text
[a](z) = L^-1(a L(z)).
```

The normalized target scaling `(x,y) -> (a^2 x,a^3 y)` has coefficients
`Atilde=a^4 A`, `Btilde=a^6 B` and target parameter
`z_target=[a](z)/a` before the Frobenius correction. Construct

```text
x_target(z_target) / x_source(z)
```

and re-expand it in `w=1/x_source`. Return coefficients below `w^11`.
[pcc:unverified]

## Mechanism and precision prediction

The formal parameter of Frobenius begins with `z^p`. In

```text
V-1 = [a] - pi,
```

the normalized parameter therefore differs from `[a](z)/a` first at degree
`z^p`. Since an x-coordinate has leading order `z^-2`, the normalized
x-coordinate ratio can first differ at `z^(p-1)`, i.e. coefficient
`w^((p-1)/2)`. Thus `p>19` predicts equality below `w^11`; the smallest
in-domain prime `p=23` places the first possible difference at coefficient 11.
[pcc:unverified]

## Frozen controls and acceptance criteria

1. On the excluded `p=31,r=37` fixture, the candidate's eleven coefficients
   must equal the quotient of the retained full reversed `N,D` map.
2. Its first nine coefficients must equal the independently implemented Bostan
   recurrence.
3. Adding the leading normalized Frobenius perturbation `-z^p/a` must leave
   coefficients zero through ten unchanged for `p=23` and change coefficient
   eleven; for the excluded-boundary `p=19`, it must first change coefficient
   nine.
4. Replacing `a` by `a+1` or omitting the target coefficient scaling must
   disagree with the full-map prefix.
5. Candidate operation counts exclude JSON parsing, full-map quotient work,
   controls and report serialization. Oracle/control counts are recorded
   separately.

Any failed equality is retained as a result; the precision, curve or fixture
must not be tuned after observing it. Passing this checkpoint would support a
fixed-size formal construction on this excluded fixture only. It would not by
itself supply the missing Lean proof of `V+pi=[t]`, establish kernel semantics
for arbitrary input, or prove an ECDLP speedup. [pcc:unverified]
