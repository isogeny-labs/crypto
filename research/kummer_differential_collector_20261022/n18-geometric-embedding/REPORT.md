# N18: geometric Kummer projection certificate

The declared N16 charts are a projective linear image of the classical
genus-2 Kummer embedding on every frozen model. The [native producer](src/main.rs)
proves six universal coefficient identities over `Z[s,t]`, then checks the
published fast-Kummer transformation and a valid branch-point-pair divisor
on all 21 models. A [separate validator](src/validate.rs) checks the
degree-bounded residual on an exact integer interpolation grid and
recomputes the 21 [standardized CSV rows](results/certificate.csv).

## Geometric derivation and source map

For a degree-two divisor with points `(x1,y1),(x2,y2)`, set
`s=x1+x2=-a1`, `t=x1*x2=a0`, and `y_i=b1*x_i+b0`. The classical map
`kappa=(1,s,t,kappa4)` uses
`kappa4=(F0(x1,x2)-2*y1*y2)/(x1-x2)^2`; this is a
Kummer embedding, not a definition inferred from the source code.
[Müller, p. 5](https://www.math.uni-hamburg.de/home/js.mueller/general_kummer.pdf)
gives the underlying linear-system construction, and
[Arnth-Jensen–Flynn, Eq. (4.2)](https://jtnb.centre-mersenne.org/item/10.5802/jtnb.654.pdf)
prints the corrected symmetric `F0` polynomial.

For the frozen monic quintic, `f6=0` and `f5=1`. Write

```text
F0 = 2f0 + f1*s + 2f2*t + f3*s*t + 2f4*t² + f5*s*t².
F0 - f(x1) - f(x2)
   = -(x1-x2)² * [f2 + f3*s + f4*s² + f5*s*(s²-t)].
(y1-y2)² = b1²*(x1-x2)².
kappa4 = b1² - f2 - f3*s - f4*s² - f5*s*(s²-t).
```

The first equality is an exact symmetric-polynomial identity. The producer
checks it separately for each `f0,…,f5` coefficient basis over the integers;
the validator checks a grid that determines every residual of bidegree at
most `(5,3)`. The last line is exactly the general Kummer `xi4` expression
in [Chung–Costello–Smith (CCS), §6.3, Eqs. (5)–(6)](https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/recovery.pdf),
and exactly the expression inverted by N15's `general()`.

CCS §7.4 gives the fast-Kummer transformation `tau=(xi1,xi2,xi3,xi4) M D`.
N18 reconstructs `M D` directly from that printed formula and the selected
theta constants; it equals N16's `C1` in all 21 fields. The two determinants
are nonzero in each field. N17's exact basis identity then gives
`C2(1,-a1,-a0,q3)=C1(1,-a1,a0,q3+sigma*a0)` with
`sigma=1+lambda+mu+nu`. Thus the direct degree-two chart `C2`, the N15
inverse chart `C1`, and the published fast-theta projection refer to the
same geometric Kummer point. CCS §§7.1–7.2 print the same Rosenhain setup
and generic direct projection as the frozen source.

The degree-one formula is a **written algebraic limit**, not a finite test:
send the second point to the unique infinity of the monic odd-degree
quintic. With local parameter `z`, `x2=z^-2` and
`y2=±z^-5(1+O(z²))`; after scaling by `z²`, the classical coordinates tend
to `(0,1,x1,x1²)`. Sending `x1` to infinity gives the identity vector
`(0,0,0,1)`. Applying `C1` yields N16's declared degree-one vector and
the theta-null identity `(a:b:c:d)`. The polynomial continuation in N16
handles colliding points and the source `a0=0` zero-numerator locus;
`det(C1),det(C2) != 0` ensures those declared vectors never all vanish.
Together with the earlier exact action certificates, this establishes the
geometric interpretation of those charts for reduced classes on the 21
frozen models, using the cited classical embedding theorem.

Müller's displayed `F0` on p. 5 visibly prints `f5*s*t`, omitting one
factor of `t`. That term duplicates the `f3` monomial and does not have
bidegree five. Arnth-Jensen–Flynn Eq. (4.2) prints `f5*s*t²`; CCS's
general Kummer quartic is consistent with that correction. The certificate
uses the independently printed corrected term and preserves the source
discrepancy rather than silently changing the formula.

## Separate xADD and collector boundary

CCS §7.3 supplies the theta xADD identity on its rational domain. The
source multiplies each coordinate by the product of the other three fixed
difference coordinates, clearing all four denominators. Equality with
the geometric sum extends as a polynomial identity across special valid
triples **where the cleared vector is nonzero**. An all-zero vector is
still undefined. The N15 finite replay saw no such mixed-zero output in
414,285 calls; this is not an all-state zero-output bound.

A total signed-label escape is available for integration: N11's invariant
is `D=aG+2^n T` with public signed `G,T`. On a zero-output theta call,
recompute signed `D` by native scalar multiplications, form the required
successor and successor-plus-`G` in the Jacobian, and project both through
the now geometrically identified chart. This uses no target secret.
It has **not** been integrated or cold-priced, so its exceptional count,
complete operation cost and matched-rho ratio remain `null`. The new
1,260-target cohort remains unrun; its requirement stays open.

| Requirement | Status | Evidence |
| --- | --- | --- |
| Universal classical-to-Mumford formula | Exact, six coefficient identities | Native producer and independent grid validator |
| Published `tau=C1` and `C2` basis match | Exact, 21/21 models | Reconstructed matrix, determinants, CSV |
| Degree-one and identity limits | Written derivation | Local parameter argument above |
| Nonzero theta xADD result | Published rational law plus polynomial continuation | CCS §7.3; output must be nonzero |
| Zero-output signed fallback in collector | Not integrated | N11 label reconstruction path above |
| New 1,260-target collector/rho ratio | Not run / `null` | Full cold-cost protocol remains open |

The [PDF](REPORT.pdf), [graph](status-graph.svg), and CSV present counts
and nulls without a speed ranking. [Primary-source digests](SOURCES.md)
and [protocol](PROTOCOL.md) retain provenance. Reproduce from the repo root:

```sh
CARGO_TARGET_DIR=/dev/shm/n18-target TMPDIR=/dev/shm \
  cargo run --offline --locked --release \
  --manifest-path research/kummer_differential_collector_20261022/n18-geometric-embedding/Cargo.toml \
  --bin certify
CARGO_TARGET_DIR=/dev/shm/n18-target TMPDIR=/dev/shm \
  cargo run --offline --locked --release \
  --manifest-path research/kummer_differential_collector_20261022/n18-geometric-embedding/Cargo.toml \
  --bin validate
```
