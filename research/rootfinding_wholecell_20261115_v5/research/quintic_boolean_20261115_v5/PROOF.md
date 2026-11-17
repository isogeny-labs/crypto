# Shared Boolean quintic support refutation: implemented bounded contract

The new route uses the exact function ring K[t0,...,t72]/(ti²-ti), with the
SAME73 coordinates of the first h=0x4208336d0d441df79c400,k=0 relaxed norm
family. The prior fixed h/k affine norm proof and prefix product-span proof
are imported unchanged; actual modulus/source/target and rank93/nullity73 are
reverified, and the particular/kernel literal norm columns replayed. This is
one norm chart, not all original roots, ghost/auxiliary coverage or outer search.

Each coefficient ANF is a dictionary monomial-bitmask -> exact K value.
Addition XORs matching values, multiplication unions bitmasks and XORs products,
and Frobenius squares field coefficients without changing monomial masks.
These are identities in the Boolean function ring, so specialization commutes
with every operation. Sparse supports may grow exponentially; no generic
linearity, tractability or novelty theorem is claimed.

All moduli H are monic quintics. Powers X0..X8 moduloH are built by multiplying
byX and replacing X5 with the lower five H coefficients. Monicity permits
this over a ring with zero divisors without division by parameter expressions.
Then R²modH is sum_i (Ri²)(X^(2i)modH). Iterating the18 exact subspace
recurrences Rnext=R²+lambdaR gives L_VmodH. Three further squarings give R8.
Every specialization is supported inV with all multiplicities iff R8=0:
L_V is squarefree split onV, and exponent8 covers all degree5 multiplicities.
NO squarefree shortcut, independent place quotient, or independent root selector
is introduced. Five-distinct roots may have rank3 OR4; no rank4 solution is
claimed without full root recovery/rank computation.

Expand R8's five field coefficients into415 binary-coordinate ANFs. Gaussian
insertion on monomial-incidence row vectors carries original-coordinate XOR
provenance. If constant1 reduces to0, the selected residual coordinates XOR
to1. Every supported specialization would instead give0, so this certifies
rejection of the WHOLE prefix cell. Every rejected cell retains full residual
ANFs plus selected coordinates, and the independent verifier XORs actual terms
to exactly{constant:1}. Missing certificates mean UNKNOWN. Residual rank is
reported only when that exact residual was constructed; a cap is not rank0.

Each prefix fixes canonical kernel coordinates0..depth-1 to the recorded low
prefix bits. Both children replace an unresolved parent atomically. Rejected
and unresolved final leaves are a prefix-free antichain with exact mass sum
2^73. Before a new node, global node/product/time limits preserve the entire
unvisited frontier as UNKNOWN. Node exceptions keep the current cell and queue;
any unexpected setup exception preserves an unverified UNKNOWN root prefix and
never claims the norm parameterization has been certified. No 2^73 state scan
is performed. Whole fixed-cell rejection requires zero unresolved mass; it
never implies original-system UNSAT or completeness.

Budgets:511nodes/depth8,4096monomials percoefficient,65536active accounted
monomials,100000 coefficient-products pernode and1000000 globally. Pair-product
upperbounds are reserved BEFORE multiplication; a new monomial is refused
BEFORE insertion exceeds the coefficient cap. Stored H/powers/remainders are
counted for active storage. Conservative refusals preserve UNKNOWN even when
later cancellation could shrink a product. Certificate bytes are capped2MiB;
raw evidence4MiB, compact stdout32KiB, worker90s/4GiB/onecore. Work refuses at78s,
actual traversal stops by76s to preserve/verify the full frontier. At84s the
serialized report demotes completion;88s preprint and external90s include final
startup/output. Raw observations remain provisional if the final report is late.

Independent controls use direct finite-field polynomial remainder arithmetic,
complete small source-product tables, and exhaustive<=3bit specializations.
They include repeated roots, a constant five-distinct GF16 rank4-capable source
fixture, corrupted certificate and corrupted prefix cover rejection, nonlinear
GF4 residuals and explicit product-cap refusal. Constant positive controls do
not establish actual hard-instance roots. Importing field arithmetic is shared;
independence is in symbolic multiplication versus specialized generic polynomial
reduction and independently formed source products. Author did ASTparse only.

Prior local jets/Newton faces/coupled-line and odd one-parameter gcd reductions
already exist. This exact Boolean quotient residual/span experiment differs
from truncated jets but classical ANF and linear-span refutations are not a
novelty claim. If capped or without actual exclusion, record the first growth
stage and revise the invariant/partition before any further experiment; do not
retry unchanged execution or market the evaluator as an outer search algorithm.

Pre-freeze review fixes: maxdegree is updated incrementally at inserted monomials, not rescanned on every pair. Multiplication checks the deadline every256 pairs. A conservative65536 allocation ledger charges UPPERBOUNDS for every dictionary construction, retaining charges even after temporaries are freed; it resets pernode while retained certificate term counts stay charged. This is stronger than a concurrent-live term bound and may cause earlier UNKNOWN. Initial actualH storage is preflighted before restriction; dictionary pair allocations are reserved before products and new keys before insertion. Scalar ANF scaling also reserves its coefficient products. The external4GiB limit bounds full Python/runtime memory; term counts are abstract symbolic storage, not heap-byte measurements. A new GF4 original literal norm four-bit family is checked against the symbolic R8 and independent generic remainder, in addition to constructed quintic controls. Final late/error/rawcap summaries clear exact rejected/unresolved/cover masses and wholecell flags.
