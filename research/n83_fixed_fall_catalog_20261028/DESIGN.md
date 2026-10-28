# Fixed-base FFD/LFD certificates and structural catalog

This stage reuses every one of the 48 hash-identified n83, ell=27 PDP systems
from `../n83_fixed_degree_sweep_20261028`. All three original curves, m=2..9,
planted and natural targets remain in scope. No new factor base is selected.
The three fully enumerated bases remain identical across m on their curve.

## Literature and definitions

Caminata and Gorla, [Solving degree, last fall degree, and related
invariants](https://arxiv.org/html/2112.05579), define V_F,d by bounded-degree
multiplication closure, and distinguish last fall, solving degree and regularity.
Kosters and Yeo, [Notes on summation polynomials](https://arxiv.org/html/1503.08001),
warn against inferring regularity from the binary first fall. Both primary texts
were checked before implementing this stage. Existing exact first-fall code,
`../n83_ic_degree_sweep_20261027/src/bin/fall.rs`, is reused unchanged.
Existing `examples/ic_algebraic_followup/hky.rs` provided closure conventions;
its small-field representation is not substituted for n83. The current lead
atlas executable is absent in the adjacent research checkout; the frontier map
and existing factor-base references were consulted. No novelty claim is made.

## Computations and admissible conclusions

1. Recompute explicit first-fall witnesses for binary/Koblitz and degree-four
   coefficient-minor exclusions for prime, using the exact saved presentations.
   Replay all 48 certificates in fresh processes. Prime bounds do not give LFD.
2. Index all 48 source systems with SHA-256, byte length, line/byte offsets for
   every equation, variable blocks, and expanded Boolean term/degree/support
   metadata. Preserve prime symbolic expressions without fabricating expanded
   counts. The original source is retained in the prior content-addressed archive.
3. Compute Boolean degree-two closure for all 32 binary/Koblitz systems. Store
   every row operation, including zero reductions, and immutable inserted rows.
   Replay the entire queue and coefficient data in a fresh process.

All Boolean field equations must be present; all other input terms are already
multilinear. Initial nonzero equations of degree at most two enter the queue.
Rows have a degree-compatible column ordering. Every inserted row of degree
zero or one contributes all variable multiples; no multiplication of a quadratic
row is admitted merely because Boolean reduction happens to lower its degree.
Thus each operation lifts to V_F,2 in the ordinary polynomial ring with the
original field equations. Distinct leading columns give an echelon basis; the
rows of degree at most one span the intersection with that degree subspace.
Queue exhaustion therefore certifies the degree-two closure, not global LFD.

### Restricted global upper certificate

A nonzero degree-zero/one consequence and no input generators of degree below
two establish LFD >= 2. LFD <= 2 is admitted only when either:

* 1 is in the computed closure; or
* the derived linear constraints leave at most 20 free Boolean variables, and
  **every** assignment in that affine space satisfies **every** original equation.

For the second case, let J be the ideal of the derived linear constraints and
all field equations. J is contained in I(F), because all these constraints are
consequences of F. Exhaustive checking gives the reverse inclusion: over the
Boolean ring, an equation vanishing on the affine space belongs to J. After
linear triangular substitution, linear constraints plus Boolean equations for
free variables form a degree-compatible Groebner basis, with each generator
in V_F,2. Degree-compatible division expresses any f in I(F) using products of
these generators of degree at most max(deg(f),2). This gives the claimed upper
bound. A plateau or an unchecked affine space gives no upper bound. Certificates
are producer-replayed; independent scientific/formal review remains required.

Controls: quadratic contradiction (unit), quadratic pair defining an affine
line (two assignments), nonlinear xy plateau (must not certify LFD), and
coefficient/operation tampering (must fail replay). Existing first-fall controls
are also required. None of these small controls replaces a target input.

## Frozen execution envelope

Sequential native Rust, one process per operation, 8 GiB virtual-memory cap.
First-fall batch and structural catalog each have 600 seconds. Each first-fall
replay and each closure/replay has 60 seconds plus five seconds kill grace.
Outer budget 5400 seconds. No retries, no deletion, no discretionary exclusions.
The existing host fallback follows the recorded EKS shared-bucket permission
failure; the refreshed EKS overview shows no active duplicate of this batch.
Times are diagnostics only. This is not a timing comparison or recovery test.
Every completed case, resource limit, error, partial output and instrument is
uploaded to S3, with periodic 30-second uploads and a final hash manifest.
The previous fixed regularity data remain separately identified and unchanged.

## Ongoing goal and next selection phase

The objective remains datasets that test FFD/LFD behavior and identify factor
bases lowering PDP regularity. This batch supplies a fixed-base baseline and
search diagnostics; it cannot identify an elite base because no candidate is
changed. A later frozen selection phase must preserve all three curves/m2..9,
match factor-base size, separate selection targets from held-out targets, retain
basis encodings and point manifests, and measure regularity with the same
instrument/resource limits. Sparsity, low FFD, or low LFD alone will not be a
regularity-improvement verdict. Any universal FFD/LFD claim needs a precise
quantified statement and proof or a valid counterexample, beyond finite samples.
All raw data, unsuccessful attempts, source and file schemas remain in S3/Git.
