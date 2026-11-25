# Prior-art and mechanism audit: 2+1 pair-trace split

The repository research-KB wrapper was queried before this protocol was
written. It returned status 127 because no catalog executable is installed.
The fallback review therefore searched the Isogeny Labs `crypto` tree and its
retained reports. No novelty claim is made.

## The broad 2+1 proposal is prior work here

The split

```text
S3(x1, x2, u) = 0,
S3(u, x3, xT) = 0
```

is the repository's standard affine chained-S3 encoding, not a new lead.
It is specified in `docs/ml-dataset/PRIME-F4-CHAIN-CONTRACT.md`, implemented
for prime fields in `examples/fb_designs_slope.rs` and the ML-dataset solver,
and linked in `docs/prime-signed-mitm.md` to the split-S3 construction of
Amadori, Pintore and Sala.

The especially close H2 arm in `research/factor_base_designs_20261015`
combines this S3 chain with a root-polynomial membership circuit. Its retained
prime-field `m=3` cells reached solving degree 6 at `t=2,3,4`, used about
`2^14.05`, `2^26.10`, and `2^32.36` counted F4 operations, and timed out at
`t=5`. Its fitted cost slope was `9.158` against the break-even value `2`.
That evidence rules out presenting a generic S3 split as a new mechanism or
repeating H2 unchanged.

## Narrow surviving mechanism

For odd `d`, an unordered pair `{w1,w2}` in `mu_d` has a canonical
parameterization

```text
z^2 = w1*w2 in mu_d,
q = w1/z,
c = q + q^-1,
w1 + w2 = z*c,
w1*w2 = z^2.
```

Squaring is a bijection on `mu_d`, so `z` is unique; replacing `q` by `q^-1`
does not change `c`. Thus `(z,c)` is in bijection with unordered pairs,
including repeated pairs. The allowed trace values are the roots of

```text
(c - 2) * (U_n(c) + U_(n-1)(c)),  n=(d-1)/2,
U_0=1, U_1=c, U_(k+1)=c*U_k-U_(k-1).
```

They can be evaluated by a deterministic fast-doubling circuit with only
quadratic equations. The two S3 equations can likewise be quadratized by
retaining their sum/product coefficients and square terms. This exact
combination—swap-quotiented pair trace, chained S3, and explicit
quadratization—was not found in the repository.

The tradeoff is explicit: full three-root symmetry is reduced only by the
swap inside the chosen pair. A distinct unordered triple has three possible
2+1 partitions, and the trace circuit adds state. The experiment below asks
whether lower input and solving degree can repay those costs; it does not
assume that they can.
