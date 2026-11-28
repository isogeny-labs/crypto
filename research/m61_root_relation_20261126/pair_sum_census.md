# Exact pair-sum support on the selected M61 base

The [native Rust census](native/examples/pair_sum_census.rs) enumerates the
`B²=5,987,809` x-classes of signed two-point sums on the selected
`B=2447` root x-base. It includes one `x(2P_i)` for each column and
`x(P_i+P_j)`, `x(P_i−P_j)` for every `i<j`. Global negation gives the
same x-coordinate, so these are precisely the factors, with counted
multiplicity, of the pair-sum divisor `D₂` in the
[quotient-algebra note](pair_sum_quotient.md).

The program stores each `(sum_x, source)` record, sorts all 5,987,809
records by `sum_x`, checks every adjacent equality, and also tests whether
any sum x-coordinate returns to the original root base. The selected
instance gives:

```text
counted_pair_x=5987809 distinct_pair_x=5987809 collided_x_classes=0 duplicate_entries=0 root_x_returns=0
```

Thus **`D₂` is squarefree and has degree exactly `B²` on this instance**.
The same executable projects every base point by `[7]`, forms all projected
pair sums by the homomorphism property, and sorts those x-coordinates too.
It again finds **5,987,809 distinct values**, with zero duplicates and
zero projected infinity entries. Thus cofactor projection does not
compress the selected pair-sum support either. Independently of this
scan, a seven-element kernel could shrink x-support by at most a factor
of seven, never from `Θ(B²)` to `Θ(B)`.
The ratio-quartic locator's planted pair target has a unique pair x-class
among this support. This exact finite result closes the possibility that
large pair-sum collisions or pair-to-base returns compress the selected
`D₂` algebra. It does not rule out compression in a growing family, a
higher-arity relation structure, or a different factor base.

For scale, independent uniform x-values would have about
`C(B²,2)/p=7.77×10⁻⁶` accidental pair collisions, and `B³/p=6.35×10⁻⁹`
pair-to-base returns. The observed zeros are therefore unsurprising under
that model. This comparison is an expectation, not evidence of
independence.

Reproduce from the repository root:

```sh
cargo run --release --offline --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --example pair_sum_census
```

One local release run with both full and projected scans took 8.59 seconds
total, including 8.33 seconds to generate the elliptic sums; timings are
illustrative. The executable uses
`Θ(B²)` records and `Θ(B² log B)` comparison-sort work, so the census is
a diagnostic within the selected budget, not an improved relation
algorithm.
