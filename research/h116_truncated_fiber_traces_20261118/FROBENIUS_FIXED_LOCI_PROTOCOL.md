# H116 Frobenius fixed-loci exhaustion protocol

Protocol `H116-FROBENIUS-FIXED-LOCI-001` is frozen before either new
implementation is written or executed. It instantiates the 119-theorem Lean
bridge on the unchanged eight-cell pointwise panel. [pcc:unverified]

## Question

For each point `P` over the frozen degree-`d` extension, find its least positive
Frobenius period `r`, so `pi^r(P)=P`. The new Lean result proves that, over an
arbitrary algebraic prime-field extension, every point belongs to some such
positive fixed locus. This experiment measures the finite orbit strata and
checks that their union is exact on the frozen panel. [pcc:unverified]

## Frozen cells and measurements

The curves, degrees, deterministic irreducible-polynomial rule and parent data
are byte-bound to `POINTWISE_CHARPOLY_PROTOCOL.json` and
`POINTWISE_CHARPOLY_RESULT.tsv`. For each cell the producer will emit the
least-period histogram, every `pi^n` fixed count for `1<=n<=d`, the union of
proper-period strata, the full-period stratum, and characteristic/coefficient
mutation counts separated by least period. [pcc:unverified]

Acceptance requires every point to have a least period dividing `d`, the
histogram to sum to the group order, each fixed count to equal the sum of the
strata whose periods divide its exponent, the first fixed locus to equal the
base-field count, and the degree-th locus to equal the full group. The correct
characteristic expression must vanish in every stratum. [pcc:unverified]

## Controls

Checking only `pi(P)=P` must omit points in every cell. More strongly, the
union of all strata with period properly dividing `d` must omit a nonempty
full-degree stratum. These are coverage controls for the formal theorem's
universal quantifier over positive periods. Wrong trace and wrong norm must
also reject at least one point per cell. No failed or empty stratum may be
replaced after execution. [pcc:unverified]

## Boundary

This is an exhaustive small-field orbit census, not the missing generic
Frobenius characteristic theorem. It does not identify the pointwise map with
a rational or scheme morphism, improve ECDLP runtime, or establish a
subexponential algorithm. Producer and verifier timings are diagnostic only.
[pcc:unverified]
