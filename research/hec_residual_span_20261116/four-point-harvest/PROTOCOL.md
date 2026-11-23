# Four-point cubic harvesting qualification

Prospective fixed-input structural gate. Same five first-pre-solve promoted
points and five core-orbit points as promotion-interpolation. Sixth no-trigger
stream is retained and not replaced. For each eligible workload choose all
10 triples of distinct core points with the promoted point fixed; try all
eight signs of the core points, fixing promoted sign positive. Exactly
400 trials. No selection by observed splitting/rank/target completion.

Four known points give unique cubic v, U of degree four, and H=(f-v²)/U
of degree at most two on genus-two degree-five models. Normalize H to monic
before root finding; retain degree drops (cubic leading coefficient can
vanish). Reject zero H, which would require f square. Check H*U=f-v² on
every trial and the original Jacobian sum on every split row, including
multiplicities. Insert the signed core/promoted affine coefficients and
residual points. All prefixes must agree for affine DSU, Sparse and native
Fast; any scalar returned must reconstruct the unchanged target. Consume no
additional old producer rows after the original first-promotion trigger.

Compare structural outcomes only with retained three-point outputs. There
are twice as many sign trials per point subset; neither trial counts nor
split counts establish a per-work or full-solve saving. No novelty claim:
classical interpolation/harvesting and polynomial factorization are the
mechanism. Literature screening: shared KB query on interpolation/sign
harvesting returned existing index-calculus sources; original source
https://www-fourier.univ-grenoble-alpes.fr/~viva/research/articles/SievingB.pdf
is a relevant prior-art lead, not yet a checked priority comparison.

Potential shared arithmetic, NOT implemented in this gate: with scaled
Lagrange parts L_i and U=product(x-x_i), L_i L_j/U equals
 gamma_i gamma_j product_{k!=i,j}(x-x_k). Thus
 H_sign=H_plus+4 sum_{i<j,sign_i!=sign_j} L_i L_j/U.
Only six quadratic cross terms and one base square/division are required
to generate all eight residuals. A cost gate must use a prepared reference
sharing interpolation bases and charge monic normalization, factor attempts,
bootstrap, rank and final verification appropriately. Genus3/4 not qualified.

Freeze source, imports, immutable ROOTS/PROMOTE, native module and binary
hashes, push Git before one invocation. No overwrite/retry; retain stdout,
stderr, exit and checkpoint. No S3/cloud. No timing or tenfold claim here.
