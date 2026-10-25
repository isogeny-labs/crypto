# N11 compressed sign-safe Kummer sidecar

The differential pair admits a one-label representation **if the step count
is available**. It has an exact signed-lift theorem in the abstract odd-prime
group model. A concrete all-state bound for theta xADD exceptions is still
missing, so this packet does not pass the cohort gate and makes no collector
speed claim.

## State invariant and proof

Start at `D=T`, `a=0`, `n=0`, with the pair `K(D),K(D+G)`. At each step the
projective branch `c` is 0 or 1 and `D' = 2D+cG`. Set `a'=2a+c (mod r)` and
`n'=n+1`. Substitution gives `D'=(2a+c)G+2^(n+1)T`. The two differential
formulas in [PROTOCOL.md](PROTOCOL.md) therefore preserve the invariant.
This induction never uses a target scalar or a known-answer target.

The pair determines the sign. Suppose
`K(X)=K(Y)` and `K(X+G)=K(Y+G)`. The first equality says `Y=X` or `Y=-X`.
In the latter case the second equality says either `-X+G=X+G`, giving
`2X=0` and hence `Y=X`, or `-X+G=-(X+G)`, giving `2G=0`. The latter is
impossible for nonzero `G` of odd prime order. Thus the map
`X -> (K(X),K(X+G))` is injective. This includes `X=0`; the pair has **zero
mathematical sign exceptions** in the frozen groups. At an endpoint, try the
two lifts and check the second coordinate, then verify
`X=aG+(2^n mod r)T` in the signed Jacobian.

For a fixed length `n`, the branch transcript encodes every integer from
`0` through `2^n-1` as `a=Σ c_i 2^(n-1-i) mod r`. There are exactly
`min(2^n,r)` possible labels. Any exact label state that cannot recover the
branch transcript therefore needs at least
`ceil(log2(min(2^n,r)))` bits in the worst case. The one-scalar `a` meets
that asymptotic bound. This only rules out deriving the label from the step
count alone; recovering it from the Kummer pair by solving a discrete log is
outside this local state model.

## Measured and unmeasured quantities

The [raw CSV](results/bounds.csv) and [JSON](results/bounds.json) have one row
for each of 336 frozen model/jump combinations. They give 72 bytes with an
external step count or 80 bytes with a stored counter, versus the current
80-byte pair and two labels. The existing regular source path computes three
xADD candidates at 24 M and 36 A each, so this compression does not reduce
its 72 M and 108 A per step. The [graph](state-graph.svg) compares explicit
state with the N7 fixed-offset model, whose state is 32r bytes but implements
a different update. No timing or rho comparison follows from the graph.

The exact quotient theorem bounds pair collisions at zero. It does not prove
the concrete projective map is defined on every encountered state, that a
signed lift can be extracted within a stated cost, or that the theta xADD
repair succeeds on every state. The current source returns `None` when all
coordinate candidates fail; the available results are finite samples rather
than an all-state exception bound. The raw rows mark this bound `null` and
`cohort_eligible=false`. A public known-answer cohort would be premature.

The [PDF](REPORT.pdf) is a neutral review summary generated deterministically
in Rust using a standard PDF font. The native Rust validator
independently recomputes all 336 rows, checks the source digest and exhaustive
small-group instances of the pair and recurrence lemmas, and verifies
`SHA256SUMS`. JSON v2 stores the same typed rows as arrays in the declared CSV
column order to keep the raw packet below the repository's artifact threshold.
The previous Python-generated v1 packet in commit `9dc8d81484` is superseded;
it contained no timings or cohort run.

From the repository root, regenerate and validate with:

```sh
cargo run --offline --release --manifest-path research/kummer_differential_collector_20261022/n11-compressed-sidecar/Cargo.toml --bin generate
cargo run --offline --release --manifest-path research/kummer_differential_collector_20261022/n11-compressed-sidecar/Cargo.toml --bin validate
```
