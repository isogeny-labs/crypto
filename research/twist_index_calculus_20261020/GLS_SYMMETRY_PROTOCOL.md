# Twist-induced Frobenius symmetry for index calculus: frozen protocol

Written before the GLS symmetry producer or its result file. The earlier
mixed-component experiment is a separate negative control; this study asks
whether a twist-derived endomorphism changes relation processing and the
one-target index-calculus pipeline on a group that actually carries it.

## Hypothesis and boundary

Let `E0/F_p: y²=x³+ax+b` and let `E_c/F_(p²)` be its quadratic twist
`y²=x³+a c² x+b c³`, where `c` is nonsquare in `F_(p²)`. Set
`alpha=c^((1-p)/2)` and
`psi(x,y)=(alpha² x^p,alpha³ y^p)`. On `E_c(F_(p²))`, the
twist-transported Frobenius satisfies `psi²=[-1]`. On a cyclic prime-order
subgroup stable under `psi`, determine the actual scalar `lambda` from
`psi(G)=[lambda]G`; check `lambda²=-1 (mod r)` and both possible roots.

The physical factor base contains `P_j` and `psi(P_j)` for each orbit key.
An ordinary signed relation matrix has one column for each physical point.
The symmetry arm replaces the second column by `lambda` times the first.
The exact raw relation set and pair-table oracle must stay identical. The
predeclared prediction is fewer columns, fewer rows required for full rank,
and possibly less cold setup work, but **no increase in raw relation yield**.
If this is a useful index-calculus improvement, the complete one-target
counted cost must fall after charging `lambda` construction, orbit building,
all relation attempts, row work, descent, and verification. A column-count
ratio or a shorter setup alone is a stage diagnostic.

## Frozen panel

Reuse these exact `E0/F_p` models from `MIXED_PROTOCOL.md`:

| p | a | b | #E0(Fp) | trace t | #E_c(Fp²) | selected prime r | cofactor |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 101 | 3 | 17 | 107 | -5 | 10,025 | 401 | 25 |
| 211 | 1 | 1 | 223 | -11 | 44,221 | 44,221 | 1 |
| 307 | 1 | 6 | 331 | -23 | 94,165 | 509 | 185 |

Use `F_(p²)=F_p[u]/(u²-d)` with the least positive nonsquare `d` in
`F_p`. Select `c` as the first nonsquare in lexicographic `(a,b)` order,
`a=0..p-1`, `b=1..p-1`, checking its quadratic character in `F_(p²)`.
Recount the full `E_c(F_(p²))` order by exact square enumeration; do not
accept the formula `(p-1)²+t²` without this check. Confirm primality of `r`,
point order, curve membership, `psi²=[-1]`, and the eigenvalue on every
selected base point.

For each curve use physical base sizes `B=12,24,36`. Enumerate points in
lexicographic `x=(a,b), y=(a,b)` order, project by the cofactor, discard
identity, and choose the first `B/2` distinct `psi` orbits. A base is the
ordered pairs `P_j,psi(P_j)`. Signed factors allow either sign of those
physical points. Every relation has exactly three distinct physical base
indices. The same signed pair-table and query sequence serve both arms.
All targets are `Q=[k]G` for `k` in `(37,113,271)`; the solver receives
only `Q`, never `k`. These are three separate one-target workloads per cell.

## Arms and stopping rules

1. `signed-flat`: `B` columns; signed width-three exact group oracle;
   collect target-independent `[a]G` queries, `a=1,2,...`, until full rank
   or `r-1` queries.
2. `psi-folded`: `B/2` columns using the verified `lambda`; same physical
   base, same oracle, and same query order; collect until full rank or
   `r-1` queries. Charge eigenvalue identification and orbit validation.
3. `rho-negation`: fresh one-target negation-quotient rho on exactly the
   same `Q`, with no shared collision table.

Then descend by querying `Q+[b]G`, `b=0,1,...`, until an exact width-three
decomposition appears or `r` queries have failed. Recover the scalar and
independently verify `[k]G=Q`. Retain every exhaustion and failed scalar;
never substitute a target or a different stopping rule silently. The two
IC arms may share an *enumerated input base* for equality checks but each
one-target cost starts cold and rebuilds its own pair table and matrix.

## Evidence and accounting

- Exact all-target relation census by enumerating every signed physical
  triple once, keyed by its group sum. Record raw witnesses, distinct
  coefficient rows, coverage, rank in each arm, orbit sizes, and duplicate
  rows. An independent audit must check each triple against direct group
  addition and the folded coefficient identity.
- For each of 27 one-target workloads per arm, record exclusive setup,
  target-query, descent, recovery-check, and total intervals; field and
  group operations, Frobenius/endomorphism evaluations, pair-table probes,
  modular row operations, memory, failure status, recovered scalar and
  independent verification. Record `lambda` search and all costs on the
  folded side. Report cold cost first; any reusable setup separately.
- Preserve a negative control replacing one `psi(P_j)` with an unrelated
  subgroup point: the asserted `lambda` relation must be rejected by the
  factor-orbit validator before timing. Preserve a repeated A/A run to
  expose count nondeterminism. Report L0 wall times as descriptive only.
- The base-field `E0(F_p)` mixed-point projection theorem and the
  `p`-Frobenius order-two action on `E0(F_(p²))` are structural controls,
  not same-target performance baselines. This experiment makes no claim
  that an `E0(F_p)` target transports to `E_c(F_(p²))`.

Use native Rust for the producer and independent replay. Keep command,
source hash, raw output, verifier receipt, report, editable graph, PDF and
failure records. The report must price every unpriced operation at 0, 0.1,
and 1 group-addition equivalent on **both** IC and rho sides where a
cross-method diagnostic is shown. An admissible speed claim still requires
the repository's isolated, sealed `ecbench` same-target gate and an
external fastest-implementation comparison. Until then use exact-count
and L0 diagnostic language only. Check the canonical scoreboard and state
why it changes or remains unchanged.
