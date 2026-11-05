# Candidate catalogue

Every entry is a hypothesis with a concrete experiment. A quadratic membership
encoding supplies an input degree, not a solving-degree bound. Compare bases
at the same post-filter signed point count B and orbit-column count K wherever
possible; otherwise publish the size mismatch and a size-matched control.
Use the same targets, summand count and complete branch policy within each pair.

## First execution tranche

Run families 1, 2, 5 and 7 first, with families 15 and 16 as separate pipeline
arms. This choice tests dimension, nonlinear structure and actual membership
cost. Families 3, 4 and 6 are inexpensive construction controls. Follow-on
tranches remain preregistered; they are not cancelled by this ordering.

| ID | Construction and yield hypothesis | Membership / degree experiment | Prior work and discriminating failure |
| --- | --- | --- | --- |
| 1 | **Shallow Frobenius forest.** Union L seed d-dimensional spaces and their 83 Frobenius translates. Compare `(L,d)=(1,10),(2,9),(4,8),(8,7),(16,6)` at fixed seed entropy; deduplicate and count actual admitted subgroup points. More components might retain support while reducing per-component dimension. | One global component/phase selector plus d bits per summand; also solve every fixed-tag branch. Count all L^m phase branches. Test whether lower branch degree survives the global selector and improves certified work per relation. | Orbit unions already exist. Novel test is a cardinality-matched global selector versus complete branching. Reject a claimed degree win if it comes only from omitted tags or a larger frontier. |
| 2 | **Trace-clean forest.** Build each seed space inside `ker Tr`, then close under Frobenius. Select points directly in the primary subgroup. Test against the unconstrained forest after matching K. | Linear trace equations plus the same selector as family 1. Measure full-system degree and natural relation yield. | Trace-zero bases are established; the old compact-orbit prefilter almost never rejected. No automatic factor-two gain applies after targets and points already lie in the odd-order subgroup. |
| 3 | **Low product-span forest.** Seed spaces `span(θ,θα,…,θα^(d−1))`, choosing α by the dimensions of V·V and V·V·V; hold d and K fixed against random spaces. | Direct S3/S4 and chain encodings; include the product-span basis in the certificate. Test whether reduced product dimension lowers a complete Macaulay degree or width. | Geometric progressions are old; small m=3 shape experiments were null. The new gate is a forest/global-selector comparison at larger tractable d, with negative cells retained. |
| 4 | **Shifted affine forest.** Union disjoint `a_j+V_j` with the same total admitted points as family 1. Freeze shifts using only lift density and intersection statistics. | Affine-linear output maps, exact tag selector, all shifts included. Compare to the corresponding unshifted spaces under identical variable order. | Affine spaces were already screened. Require a joint yield/degree difference beyond the recorded affine negative results; changing the base size does not qualify. |
| 5 | **Quadratic trace slices inside a forest.** For u in an ambient seed space, require `Tr(u^(2^e+1))=b_e`; close accepted x values under Frobenius. Sparse quadratic constraints may remove poor columns while keeping a compact representation. | Substitute u's linear coordinates into each quadratic form, then add exact component/phase selectors. Compare to a random equal-cardinality subset of the same ambient space. | G4 was proposed but unbuilt. Coefficients must be in F2 for individual trace equations to be Frobenius-stable; arbitrary coefficients require an orbit closure. Full-field trace slices alone have a size/admission problem; use the explicit ambient family. |
| 6 | **Artin–Schreier affine-image control.** `x=u²+u+c`, then Frobenius closure. Sweep source spaces and record kernel/intersections. | Squaring and `u²+u` are F2-linear; compare the exact image with a directly constructed affine space and deduplicate identical sets. | This is an affine-space control, not a new nonlinear family. Any claimed advantage must survive the equal-point-set control. Two-to-one behavior holds only when 1 lies in the source direction space. |
| 7 | **Bilinear tensor image.** `x=c+u+v+λuv`, with u and v each in a shallow space. Select λ independently of targets; record fibers and collisions. | Compare direct substitution with auxiliaries `w_ij=s_i t_j`; retain all membership solutions and exact lift checks. Test full-system degree, variable count, matrix width and operations. | Distinct from an additive parallel sumset. Reject if noninjectivity erases size or auxiliary selectors restore the degree/width cost. |
| 8 | **Reciprocal forest.** `x=1/(u+β)` for a frozen forest, excluding poles, followed by Frobenius closure. | Equation `x(u+β)=1` plus forest membership; compare direct elimination and the inverse-variable encoding on the same set. | General pole sweeps were negative, and β=0 has a special S3 identity at b=1. Include that exact-set/identity control; do not count it as evidence for an improvement. |
| 9 | **Rational 2-torsion coordinate frame.** `1/(x+1)` in a forest or selected affine component, using the existing symmetrised S3/S4 encoding. | Compare symmetrised variables with the ordinary x encoding for the identical point set; include denominator exceptions. Measure all selector constraints. | Existing u-frame gained at some small sizes and lost at degree 31. This is a forest/encoding retest, not a new symmetry. Cofactor clearing kills rational 2-torsion; saturation alone adds no new primary-subgroup points. |
| 10 | **Even-weight normal-basis necklaces.** Use weight-2 and hash-selected weight-4 cyclic necklaces, lift and retain subgroup points, then match K. | Exact-weight circuit and cyclic-phase selector versus a lookup of the identical set. Measure global degree, circuit width and complete cardinality. | Hamming balls already exist. The open experiment is an exact constant-weight membership encoding. Weight 3 has trace one and is inadmissible as a direct x-base for the primary a=0 odd-order subgroup; projection changes the representation and becomes a separate arm. |
| 11 | **Cyclotomic sparse-sum base.** Use sparse sums of powers of a root of order 167, with Frobenius/sign-equivalent recipes deduplicated and only admitted subgroup lifts retained. | Compare a sparse-position selector, a sequential weight counter and a balanced lookup for the identical point set. | Degree-23 yield was previously indistinguishable from random orbits. The untested question is membership solving degree and its cost at degree 83. Verify the cyclotomic field and orbit construction before using it. |
| 12 | **Phase-staggered typed slots.** Use domains `V,σ^s V,σ^(2s)V,…` with `s∈{1,d}` for different summands; sum support may increase because domains differ. | Freeze slot types in S3 chains. Compare against plain slots, a matched union base and complete permutation coverage. | G2 preserved degree on small cells. There is no automatic m! gain at equal total base: typed slots change the domain and may cover fewer tuples. Measure unique group sums. |
| 13 | **Collision-optimized orbit code.** Draw a public pool of orbits; select K using group pair/three-sum collision energy without using eventual targets. | Balanced quadratic lookup for column and phase. Compare random selection with identical circuit structure and report the full circuit cost. | New selection policy over existing orbit bases. Field-coordinate additive energy is only a screening feature; the objective must use group sums. Low pair energy need not imply high five-sum support. |
| 14 | **Jointly selected bipartite 3+2 domains.** Select orbit pools A and B for high unique support of `3A+2B`, with fixed combined K. | Typed 3+2 chain or exact multilevel join, recording both list widths and all compatibility branches. | Existing work paired sequential/hash/Gray streams without this joint objective. Train on collision statistics and validate on disjoint targets. Fixed K must include both pools. |
| 15 | **Small low-degree core plus residual envelope.** Use a low-dimensional core and one/two residual orbit columns from a larger fixed envelope. | Measure the residual membership systems, then exact partial-relation graph cycles, duplicate rate and rank gain. Replay each combined row. | Large-prime adapters exist. Higher partial yield only helps when cycles produce independent full rows; compare usable-row yield and full graph cost. |
| 16 | **Dimension/slack and arity co-design.** Shrink each core space while increasing relation arity only when measured useful yield compensates. Sweep d and summands independently before coupling them. | Whole-system degree at each `(d,m)` and exact coverage; compare direct symmetric polynomials and chains when both are available. | Shantz–Teske slack already lowers toy refutation degree. This is the positive tuning arm: the missing result is a jointly better yield-adjusted work point, with the entire selector/chain included. |

## Algebraic admission checks

- At prime degree 83, there is no proper intermediate field over F2.
  A subfield base is a positive control only on composite-degree diagnostics.
- `ord_83(2)=82`: small single Frobenius-invariant linear spaces cannot be
  assumed to exist. Forests and orbit unions require their tags/phases.
- For the primary a=0 model, odd-order subgroup points are doubles and obey
  `Tr(x)=0`. A normal basis has generator trace one, so odd-weight coordinate
  words cannot directly represent their x coordinates. Verify this subgroup
  admission point-by-point in the constructor.
- For F2 coefficients, `Tr(x^(2^e+1))` and the exponent `83−e` give the same
  quadratic trace function. The simple invariant family supplies 41 such
  paired exponents plus the linear trace. An independent-equation heuristic
  cannot supply the roughly 67 cuts needed to reduce a full 83-bit domain
  to roughly 16 bits. No degree-83 base size is inferred from that heuristic.
- Clearing by [4] commutes with Frobenius but changes x. A polynomial-space
  constraint on the source x does not become the same constraint on the
  projected x. Encode the projection or use directly admitted subgroup points.

These checks are admission obligations and known identities, not newly
measured yield results. A formal proof map and identity certificate are owed
before a new mathematical claim is promoted.
