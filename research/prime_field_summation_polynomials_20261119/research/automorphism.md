# Automorphism-orbit assessment for specialized Semaev roots

Date: 2026-11-19 UTC

## Requested track and status

| Requirement | Status | Evidence |
| --- | --- | --- |
| Audit `j=0` and `j=1728` rational automorphisms | Recorded local self-test pass for the explicit maps and field conditions below `[pcc:unverified]` | 84 automorphism self-tests plus positive and negative CLI fixtures |
| Determine when a fixed specialization can be quotiented | Recorded local self-test pass `[pcc:unverified]` | Signed-permutation test on the input point multiset; all orbit-derived roots compared with MITM |
| Preserve a negative control | Recorded local self-test pass `[pcc:unverified]` | Covariance holds but folding is rejected for a non-invariant `p=43` input; 0/2,600 random controls were invariant |
| Measure orbit and implementation costs | Provisional measurements retained | 2,600 matched cases, 20 repetitions; final integration should rerun serially because parallel tracks shared the host |
| Establish an end-to-end relation-search or ECDLP gain | Not established | Factor-base invariance, relation-label action, subgroup eigenvalues, canonicalization, and downstream costs were not benchmarked |

## Exact maps and field gates

All statements use a nonsingular short-Weierstrass model in characteristic
different from 2 and 3.

For `j=0`,

```text
E: y^2 = x^3 + B,  B != 0
phi(x,y) = (beta*x, y),  beta^3 = 1, beta != 1.
```

The nontrivial map is rational over `F_p` only when a nontrivial cube root of
unity lies in `F_p`; for prime fields this requires `p = 1 mod 3`. Its point order
and x-coordinate order are both 3.

For `j=1728`,

```text
E: y^2 = x^3 + A*x,  A != 0
psi(x,y) = (-x, i*y),  i^2 = -1.
```

The map is rational over `F_p` when `p = 1 mod 4`. It has point order 4, while
its x-coordinate action has order 2 because `psi^2=[-1]` and negation is invisible
on x-coordinates.

The implementation verifies the map on every transformed point `[pcc:unverified]`. These formulas
establish rational curve automorphisms; they do not, by themselves, identify a
full endomorphism order, a subgroup eigenvalue, or a GLV speedup.

## Covariance versus fixed-specialization folding

Let `R(x_1,...,x_n)` be the finite root multiset of the specialized summation
polynomial, represented by x-coordinates of signed point sums. For the x-action
`alpha` (`beta` for `j=0`, `-1` for `j=1728`), the verified covariance is `[pcc:unverified]`

```text
R(alpha*x_1, ..., alpha*x_n) = alpha * R(x_1, ..., x_n).
```

This maps one input specialization to another. It does **not** imply that the
root set of one arbitrary fixed specialization is closed under `alpha`.

The audit certifies within-specialization folding from the inputs when applying
the point automorphism induces a signed permutation of the original point
multiset. Changing lift signs and permuting inputs do not change a Semaev root
multiset, so this condition is sufficient. It is not logically necessary:
accidental root-set invariance can occur, but detecting that by first enumerating
all roots provides no root-extraction shortcut.

When the condition holds and the finite monic polynomial has degree `e`,

```text
F(X) = alpha^e * F(alpha^-1 * X).
```

Consequently nonzero coefficient exponents obey

```text
k = e mod ord(alpha).
```

Thus an invariant `j=0` specialization has the form `X^r G(X^3)`, and an
invariant `j=1728` specialization has the form `X^r G(X^2)`. The audit verifies
this support condition `[pcc:unverified]`. This is a special invariant-input sparsity result, not a
contradiction of the broader corpus finding that generic specializations are not
sparse.

## Sign-class quotient

The ordinary specialized polynomial has `d=2^(n-1)` sign classes after quotienting
global negation. When inputs are unions of full automorphism orbits:

- `j=0`, with `b` three-point input blocks:

  ```text
  d = 2^(3b-1)
  sign_orbits = (2^(3b-1) + 2^b) / 3
  ```

  The measured class reduction was 2.0, 2.666667, and 2.909091 for one, two,
  and three blocks; it approaches 3.

- `j=1728`, with two-point x-orbit blocks, every tested sign class was in a
  two-cycle. The class reduction was exactly 2.

The new `orbit` root method evaluates one signed sum per sign orbit and expands
its x-orbit, preserving multiplicity and roots at infinity. It is exact, but its
current representative evaluator recomputes each sum with affine additions.

## Positive and failure controls

Positive synthetic fixture:

```bash
./build/semaev_lab automorphism-audit --prime 43 --a 0 --b 7 \
  --x 2 --x 12 --x 29
```

Observed: the input was invariant, covariance and coefficient support were
verified `[pcc:unverified]`, four sign classes formed two orbits, and the orbit-derived roots matched
MITM. One sign class summed to infinity; the finite polynomial was `X^3+1`.

Failure control:

```bash
./build/semaev_lab automorphism-audit --prime 43 --a 0 --b 7 \
  --x 2 --x 12
```

Observed: covariance to the transformed-input specialization was verified `[pcc:unverified]`, but
the input and fixed root set were not invariant, so orbit folding was rejected.

The self-test also rejects `j=0` rationality for `p=1019` (`p=2 mod 3`) and
`j=1728` rationality for `p=1000003` (`p=3 mod 4`).

## Provisional matched timings

Each row used 200 preconstructed invariant inputs. Both methods returned sorted,
distinct finite roots; all root multisets and infinity counts were checked before
timing. Each timed batch ran 20 times. Input generation and validation were
excluded. Because other optimization agents shared the five-CPU host, these
timings are provisional and must be rerun serially after integration.

Selected rows:

| Curve/field | Degree | Sign classes/orbits | MITM | Orbit method | Observation |
| --- | ---: | ---: | ---: | ---: | --- |
| `j=0`, `p=1009` | 4 | 4/2 | 0.808 us | 0.596 us | Orbit method 1.36x faster in this row `[pcc:unverified]` |
| `j=0`, `p=1009` | 256 | 256/88 | 17.249 us | 75.366 us | Orbit method 4.37x slower |
| `j=0`, `p=1000003` | 256 | 256/88 | 21.111 us | 107.636 us | Orbit method 5.10x slower |
| `j=0`, `p=2147483647` | 256 | 256/88 | 27.467 us | 184.881 us | Orbit method 6.73x slower |
| `j=1728`, `p=1009` | 128 | 128/64 | 8.701 us | 50.075 us | Orbit method 5.75x slower |

The arithmetic orbit count decreases as predicted, but the prototype loses to
batched MITM beyond the smallest `j=0` case because it performs many serial
affine inversions. Orbit compression alone is therefore not a root-extraction
wall-time improvement `[pcc:unverified]`. A credible next experiment is to combine sign-orbit
representatives with the projective or per-level batch machinery developed in
the separate all-roots track.

## Relation-search applicability gates

An orbit quotient can avoid redundant work across different specializations if:

1. the factor base is closed under the rational automorphism;
2. the input scheduler canonicalizes complete input orbits;
3. transformed relation labels are retained correctly;
4. any named cyclic subgroup is preserved and its action is verified `[pcc:unverified]`, including
   `phi(P)=[lambda]P` when scalar labels are folded; and
5. representative selection and downstream linear algebra are costed.

For a compatible walk with mostly full size-`m` state orbits, `sqrt(m)` is only
the ideal birthday-iteration factor. No such walk or end-to-end relation pipeline
was implemented here. Likewise, reducing three or two algebraic classes does not
establish the same wall-time ratio.

## Reproduction and evidence

```bash
make test
./build/semaev_lab automorphism-benchmark --prime 1009 --a 0 --b 7 \
  --seed 6000285944033488934 --max-arity 10 \
  --bench-cases 200 --bench-repeats 20 \
  --output results/optimization/automorphism_j0_p1009.csv
```

Complete CSVs cover all three campaign fields for `j=0` and the rational
`j=1728` campaign field. Logs are under `results/raw/automorphism_*.log`.

Primary references:

- Gallant, Lambert, and Vanstone, “Faster Point Multiplication on Elliptic Curves
  with Efficient Endomorphisms,” CRYPTO 2001 `[pcc:unverified]`:
  https://doi.org/10.1007/3-540-44647-8_11
- Semaev, “Summation polynomials and the discrete logarithm problem on elliptic
  curves,” Cryptology ePrint 2004/031:
  https://eprint.iacr.org/2004/031
- Bos, Costello, and Miele, “Elliptic and Hyperelliptic Curves: A Practical
  Security Analysis,” PKC 2014, sections 2–3:
  https://www.iacr.org/archive/pkc2014/83830154/83830154.pdf

Short checks:

- Does covariance imply a fixed polynomial is invariant? No; the input multiset
  must be invariant up to signed permutation.
- Does a threefold class quotient prove a threefold wall-time gain `[pcc:unverified]`? No; the
  representative evaluator and downstream costs remain.
- Does a rational map prove a useful subgroup eigenvalue `[pcc:unverified]`? No; the named subgroup
  action must be verified separately `[pcc:unverified]`.
