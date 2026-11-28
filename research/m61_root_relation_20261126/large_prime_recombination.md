# One-large-prime recombination at the cube-root budget

This is a cost check of a standard large-prime index-calculus tactic on the
selected M61 root base. It is **not** a novel large-prime algorithm. Let
`B=2447` be the number of factor-base columns and
`r=329406144237565159` the prime subgroup order. Choose a signed
six-column tuple, project its sum by `[7]`, and call its negative `L`.
This produces a valid *partial* row cheaply: `L` is the unrestricted
large-prime point. Two rows with the same `L` can be subtracted to eliminate
that point, leaving a verified relation with at most 12 signed base terms.
With opposite `L` values, their rows can be added instead. Rows sharing
columns may cancel and resulting matrix rank must be checked separately.

## Collision ledger

Suppose distinct partial sums behave like independent uniform elements of
the order-`r` subgroup. For `M` partial rows, the expected number of
same-`L` pair matches is `C(M,2)/r`; if opposite values are also folded
and forced global-negation duplicates are excluded, it is approximately
`M(M−1)/r`. At the entire proposed `B²=5,987,809` budget, these are only
`5.44×10⁻⁵` and `1.09×10⁻⁴` matches respectively. One optimistic
sign-folded match needs about `√r≈574 million` rows. Obtaining roughly
`B` matches needs about `√(Br)≈28.4 billion` rows, before rank losses,
target descent, and the arithmetic and hashing needed to produce or store
each row. A direct independent six-sum uses five additions, while shared
partial sums can reduce that constant; the row-output count remains large.

For a growing family with `B=Θ(p^(1/6))` and `r=Θ(p)`, the optimistic
`B`-match cost is `Θ(√(Br))=Θ(p^(7/12))`, exceeding both the desired
`p^(1/3)` and generic square-root scale. The sampled rows need memory
of the same order if stored for hash matching. This is an expectation in
the uniform model, not a lower bound against structured coordinate methods.

Restricting `L` to a public pool of `K≤r` group elements does not repair
this random-model ledger. A uniform six-sum enters the pool with
probability `K/r`. After `S` generated partial sums, roughly `SK/r`
are accepted and their expected same-label matches are on the order of
`S²K/r²`. Requiring `Θ(B)` matches gives
`S=Ω(r√(B/K))`, minimized at `K=r`; a smaller pool spends more time on
rejected rows. A pool with algebraic structure could invalidate this
uniform model, but then its cheaper acceptance and recombination must be
exhibited and charged.

For target descent, a stored collection of `M` large-prime labels is hit
by a fresh target-adjusted six-sum with probability about `M/r`. Even
with `M=B²`, the expected search is `r/B²≈55 billion` fresh sums.
Keeping each large prime as a separate matrix column without recombination
does not remove this target-search obligation.

## Bounded native check

The [Rust triple-intersection probe](triple_intersection.md) enumerates
all `2^6·C(8,6)=1792` signed six-tuples on three toy fields and counts
exact equal-sum tuple pairs after cofactor projection. At `p=524287`,
the selected first root window has 36 matches, all 79 root windows average
91.18, and 128 equal-size random-x controls average 100.47; the
independent-uniform estimate is 97.72. At `p=131071`, the controls average
442.98 against a uniform estimate of 453.19. These toy counts support the
scale of the birthday model for controls; they do not justify applying
independence to the M61 root base or prove any asymptotic impossibility.

The only possible gain here is a **structured** large-prime map or pool
that creates many *independent* recombinations or target matches with
`Õ(B²)` total generation, filtering, storage, witness and solve cost.
No such map is supplied by this construction.
