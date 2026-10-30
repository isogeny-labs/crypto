# A recent existing claimed sub-exponential prime-field algorithm

2026-10-30 UTC. This materially changes the prior-art assessment. The initial
focused audit did not include these October2026 manuscripts. They are now
explicitly included rather than treating the norm/divisor candidate as the
only mathematically supported route. No new original algorithm is claimed.

The public preprint _A Subexponential Time Algorithm for Elliptic Curve
Discrete Logarithm_, version0.2, claims a classical expected-time algorithm

    2^{O(b/(log b)^(1/4))}, b=ceil(log2 p),

for valid prime-field ECDLP with known odd prime whole order, and a separate
fixed-cofactor extension. Its Theorem4 is conditional on its Theorem2,
which invokes AppendixD.1 of the separately released _The Exponential Time
Hypothesis is False_. That appendix states and proves a general Boolean
tensor-contraction bound in the ordinary bit model.

The documents are [pinned and hashed in the source triage](source-triage.md).
Public repository/author labels establish provenance only; they are not
institutional endorsement, peer review or a correctness certificate.

## The actual mathematical mechanism

The contraction theorem claims, under its exact degree, description,
evaluation and precision hypotheses,

    (s+L_in)^O(1) * 2^{O(s/sqrt(log s))}

deterministic bit work and polynomial space on an s-edge bipartite Boolean
tensor network. It allows moderately large implicit vertex predicates with
explicit charged evaluation cost. It does not offer free real arithmetic,
unbounded precision or free truth tables.

The ECDLP paper's key encoding is multiplication with O(b) Boolean edges
and maximum vertex degree O(b^(3/4)), using two stages of exact Fourier
transforms over a finite ring. A straightforward bit circuit's extra log b
edge factor would destroy the required saving. The moderately large
transform vertices retain polynomial-time predicates; they do not compute
elliptic logarithms.

Independent affine hashes of injectively compressed curve points define m
separate public factor bases. Membership takes O(b) edges per point and
base listing costs are charged cold. Its second-moment coverage bound is
over the independent public hash seeds, not a planted-decomposition sample.
Complete point addition, membership and an m-point sum require O(mb) edges.
The external contraction theorem then gives query work
2^{O(mb/sqrt(log b))}.

Fresh mixed-target labels and a matrix-only dependency recover the target
without supplied factor-base logs or a full-rank heuristic. Including base
listing, failed queries, seed restarts, root extraction, prime selection and
linear algebra, the expected bound is

    2^{O(b/m+mb/sqrt(log b))}.

Taking m near(log b)^(1/4) gives the displayed sub-exponential exponent.
This is an existing publicly released derivation, not an invention in this session.

## Why the tensor theorem does not automatically settle our norm candidate

The [separate size check](norm-encoding-size.md) confirms that the literal
shared-remainder coefficient system uses Theta(mt) field-state words and
O(tm^2) dense scalar arithmetic constraints. Translating those constraints
with the compact O(b)-edge field gadgets gives O(tm^2 b)=O(mb^2) Boolean
edges when t=Theta(b/m). Even its explicitly exposed state words occupy
Theta(b^2) bit ports. D.1 then supplies only an upper bound
2^{O(mb^2/sqrt(log(mb^2)))}, which does not establish exp(o(b)). The hashed
construction's O(mb) compactness is a separate property, not inherited by
our coefficient circuit. This is a limitation of this translation and
bound; it is not a runtime lower bound or an impossibility result for a
better encoding or symbolic solver.

## What the primary-proof checks actually support

The [cost review](contraction-cost.md) read D.1 and the full D.4 proof.
It independently checked packet shrink, degree invariants, interpolation
counts, description/evaluation costs, precision absorption and the summed
recurrence. The [arithmetic review](contraction-precision.md) independently
re-derived coefficient-ring valuations, canonical creation-precision lifts,
Cauchy integral charts, tapered Hermite recovery, weighted mode truncation
and packet factorization. Neither review found a concrete gap in those steps.
They remain scoped proof readings, not formal certification of every result
in either manuscript or of the native implementation.

The parent additionally read the ECDLP paper's multiplication construction
and proof, modular-arithmetic and point-addition encodings, affine-hash
coverage, extraction and complete-cost analysis. The principal-root argument
over the possibly composite Fourier ring uses cancellation by a half-turn
and the invertibility of2, not an invalid field assumption. Canonical words,
no-wrap convolution, all exceptional addition cases and zero-test witnesses
are explicit. The hash variance/counting and hidden-label conditioning are
consistent with the stated reduction. No concrete transfer gap was found in
this reading; it is not a proof of implementation parity or novel authorship.

The [existing Rust crate validation](rust-validation/README.md) freshly passes34 release tests on unchanged source and lock, including forced recursive/direct controls and a tiny complete relation pipeline. An already-started separate public tiny CLI also exits0 and reports verified scalar4. The source pin, exact fixture, retrospective receipt and hashes are preserved. This does not test an asymptotic-sized recursive case or either published challenge. The root library suite was not run.

## What has not been established here

The separate [native GMP unit](lemma-unit/README.md) now passes72 fixed
polynomial cases,308 Cauchy charts,8832 node-error checks,1152 final-error
checks and four negative controls. It tests exact scalar-coordinate D.9
reconstruction and2^pt perturbations, with byte-identical replay of all
seven final tables. The first negative control's coverage mismatch and
its actual-injection correction are preserved. This is not a full
extension-ring multiplication or contraction test. [Integration notes](INTEGRATION.md)
explain raw storage and hash-based regeneration.

- General correctness and complete cost of the existing native recursive
  implementation remain independently auditable obligations. Its small
  controls are evidence for those cases, not a general theorem certificate.
- The literal source constants force a finite-size fallback on practical
  graphs. This does not refute an asymptotic theorem. Absent performance
  measurements also do not refute its proof.
- No actual P-256, secp256k1 or published challenge logarithm is recovered in
  this study, and no useful concrete runtime bound is claimed.
- The strict novelty requirement remains unmet: transcribing or renaming
  this existing algorithm cannot satisfy a request for an original one.

The best-supported currently identified claimed route to sub-exponential
prime-field ECDLP is therefore this tensor theorem chain. The norm/divisor
candidate remains a separate unproved research direction, not a demonstrated
alternative breakthrough. The appropriate next step for this existing route
is proof-to-implementation audit, with all precision and representation
invariants preserved; a new mechanism would be needed to meet novelty.
