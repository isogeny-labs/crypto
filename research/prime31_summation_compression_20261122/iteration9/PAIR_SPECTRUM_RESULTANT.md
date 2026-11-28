# Iteration 9: squarefree signed pair-spectrum resultant, including misses

Date: 2026-11-22 America/Los_Angeles / 2026-11-23 UTC.
Status: author derivation, **[pcc:unverified]**. This is an exact algebraic elimination *baseline*, not a new sublinear online algorithm, not native-tested Rust, not a signed proof and not a complete ECDLP speedup. Its group-semiring meaning agrees with merged #4941; the distinct contribution is a fixed-degree S3 **resultant/gcd/Bezout formulation**, rational root isolation without generic factorization, and a tight B² *unoriented* pair-chart bound. No priority claim is made for resultants, x-coordinate symmetrization, or meet-in-the-middle.

## Setup and theorem

Let p>3, let E/Fp be a nonsingular short-Weierstrass curve, and let G be an odd-order subgroup of E(Fp). Choose B distinct rational abscissae with rational lifts P_1,...,P_B in G and P_i not equal to ±P_j for i≠j. Assume none is O. Define A={±P_i}, with L=2B; use the **actual point signs**, not an assumed root sign. Let R∈G\{O} and r=x(R).

Let U be the distinct nonidentity two-sums A+A, and let X be the set of their x-coordinates. Put D=|X| and the monic, squarefree, *split* polynomial

    H_2(V)= product_(v in X) (V-v) ∈ Fp[V].

Precompute for each v∈X a representative T_v∈U and **all** unordered signed-pair preimages of ±T_v. The oriented versions and orderings must be reproducible, including every collision.

For S3 the standard Semaev polynomial, define

    K_r(V)=Res_U(H_2(U), S3(U,V,r)) mod H_2(V)
          =product_(u in X) S3(u,V,r) mod H_2(V).
    J_r(V)=gcd(H_2(V), K_r(V)), monic.

**Theorem (Fp-point support, no multiplicities).** For nonidentity R:
(1) 0≤D≤B². The complete signed four-leaf relation set is empty if and only if BOTH H_2(r)≠0 and J_r=1.
(2) For v∈X, J_r(v)=0 iff there exist signed two-sums S,T∈U with x(T)=v and S+T=R. Its roots are all in Fp and can be isolated with the precomputed factor tree of H_2, without factoring an unknown closure polynomial.
(3) After restoring pair labels and signs, each root yields precisely the signed four-tuples when the exact group sum equals R. The H_2(r)=0 branch contributes every tuple with one zero pair sum. Repetitions, cancelling pairs, zero intermediate sums, and arbitrary leaf orderings are retained. For prime subgroup order n≥5, iteration6's free-order comb lemma converts every signed four-multiset for nonzero R to some valid rational affine Q1-Q3 chain. This is **not** fixed-balanced-tree completeness.

These statements concern the *Fp-rational support of the projected system*; neither scheme-theoretic multiplicity nor a Groebner solving degree is claimed. A polynomial factor or closure root by itself is never a signed certificate. [pcc:unverified]

## Proof of the B² pair-chart bound

There are L(L+1)/2 unordered signed leaf pairs, of which exactly B are of the form {P_i,-P_i} and have sum O. The remaining number is L²/2=2B². Simultaneous negation {a,b}→{-a,-b} acts **freely** on these nonzero-sum pairs: a fixed pair would require either a=-a and b=-b (impossible for nonzero odd-order points), or a=-b (zero sum). Thus there are exactly B² orbits of unordered nonzero signed pairs under simultaneous negation. Their sums map to abscissae, perhaps with further collisions, proving D≤B².

A constructive chart makes the count explicit. For i<j calculate exactly two points P_i+P_j and P_i-P_j; for i=j calculate 2P_i. These are B(B-1)+B=B² nonzero point additions/doublings; their x-values generate X. Store one expression per chart entry even when x-values collide; keep zero-pair entries separately for P_i,-P_i. This uses the same source E, not a torus DLP oracle. It is a factor-four **group-addition count reduction relative to naïvely enumerating all L² ordered signed pairs**, not a measured runtime claim and not a reduced asymptotic exponent.

## Exactness and the S3-to-rational guard

For actual nonzero rational curve points S,T,R, the Semaev S3 identity at (x(S),x(T),x(R)) vanishes exactly when one choice of signs of those points sums to O over the algebraic closure. Since each abscissa here already has an Fp point lift and both point signs lie in U (or R may be flipped), such a choice is rational. Global negation moves a solution whose total is -R to one whose total is R. Therefore for u,v∈X:

    S3(u,v,r)=0  iff  R∈{±S±T}, for any S,T∈U with x(S)=u,x(T)=v.

The two-sum chart includes both signs of each representative by construction. No nonrational auxiliary is ever introduced, and no S4 projected ghost can enter this fixed-point construction.

The root-product identity for the resultant holds because H_2 is monic and split with *distinct rational roots*. At a root v of H_2, K_r(v)=0 iff some S3(u,v,r)=0. Since H_2 is squarefree, the monic gcd has exactly those v as simple roots. For R≠O, all signed four-tuples split after two leaves into (S,T), possibly with S=O or T=O. If neither is O, their x are in X and the gcd detects them. If S=O, then T=R; equivalently x(R)∈X because U is sign-closed. The symmetric T=O case is identical. This proves the exact emptiness criterion.

A full ordered output enumerator takes the retained pair preimages at each (u,v), chooses actual signs, verifies S+T=R, and expands leaf orderings. The special zero-sum pair table has exactly 2B ordered entries. It is combined with the two-sum preimages of R in both split orders (zero-left and zero-right, disjoint since R≠O). Group replay catches all sign/repetition cases. Any affine Q1-Q3 reconstruction requires nonzero internal prefixes and the corresponding oriented slopes; it is independent of this target-set decision. Retain P,P,P,-P as a fixed-balanced-tree negative control.

## Reproducible exact algorithm, including a certified miss

Preprocessing (target independent):
1. Verify curve/order domain, all B rational lifts and ± pairs. Build the B² expression chart, the zero-pair list and deduplicated X. Keep collision buckets and canonical group encodings.
2. Build H_2 and a balanced *subproduct tree of its known linear factors*. Every leaf root is visibly in Fp. Compute and freeze hashes of point/chart/poly input arrays; record the exact curve and x-fibre parameters.
3. All exact polynomial arithmetic is over Fp. Never infer an Fp root from coefficients alone.

Query(R):
1. If R=O, use the exact group-semiring pair complement F2(S)F2(-S), including O; the x(r) formula is not defined at O. For R≠O continue.
2. Evaluate H_2(r). Build the degree-at-most-2 polynomials f_u(V)=S3(u,V,r) for every u∈X; compute their product modulo H_2, the monic gcd J_r, and, on an empty candidate, an *extended-gcd witness* A_r(V),B_r(V) satisfying A_r H_2+B_r K_r=1.
3. If H_2(r)≠0 and J_r=1, return EMPTY with input hashes, the nonzero H_2(r), the Bezout pair and the independently recomputable equality. This is a finite-field algebraic certificate for **this supplied signed base**, not for unspecified relations.
4. Otherwise isolate each root of J_r against the known linear-factor tree. For each v, the polynomial S3(U,v,r) has degree at most two in U. Isolate its rational roots and intersect with X (or equivalently compute the two point candidates R±T_v and look up their x). Every hit must still undergo exact signed group replay. Handle zero-pair branches separately. Emit all ordered four-tuples, a deduplicated signed row view, and true completeness / failure status. If any operation times out or cannot produce a Bezout identity or complete witnesses, return INDETERMINATE, never EMPTY.

There is **no generic Fp² root isolation**: H_2 already splits, known x-roots are stored, and the other isolated factor has degree≤2. Quadratic formulas / polynomial gcds must check degenerate linear and constant cases and reject nonsquare discriminants. A purported pair sum involving O must use the separate zero branch. Returning only one witness per x is insufficient for a complete-output contract.

## Actual dimensions, cost and memory recurrence

Let M(d) be the field-operation cost of degree-d polynomial multiplication, C_add a genuine elliptic addition/doubling, C_lookup a canonical point/abscissa lookup, C_sqrt(p) an exact rational square-root check, and C_verify the charged signed row replay cost. Let Z_R be the number of emitted **ordered signed** quadruples, including zero-pair branches.

Source setup costs B lifts/sqrt checks plus O(B² C_add) for the chart, O(B² C_lookup) for collision buckets and O(M(D)log D) field operations for the H_2 product tree. The tree uses O(D log D) Fp-element slots; the chart uses O(B²) points/labels; a lower-memory tree build can trade off storage for recomputation. Rejected source-fibre trials and any subgroup/order validation remain additional setup.

For a query define the product degree at level j by d_0≤2 and d_(j+1)≤min(2D,2d_j); the sum of degrees across every balanced product-tree level is ≤2D. Thus construction of K_r costs O(M(D)log D) field operations, not constant time just because each S3 factor is quadratic. Extended gcd costs O(M(D)log D) with fast Euclidean arithmetic, including the non-hit certificate; ordinary schoolbook polynomial arithmetic instead gives a conservative O(D²) field bound. The nonzero H_2(r) evaluation can be done with a root product or polynomial Horner in O(D) field operations. **Every empty query still constructs the full product or a separately proven equivalent index.**

If J_r has h distinct roots, finding all of them in the known split factor tree costs at most O(M(D)log² D) field work with standard fast remainder/gcd tree operations (or O(Dh) with direct checks), and then at most 2h degree≤2 candidate abscissae for the nonzero-pair branch; exceptional polynomial degrees require explicit handling. Output sign/backpointer enumeration and verification cost O(Z_R C_verify) plus lookups. Misses have Z_R=0 but retain the product, gcd, Bezout and H_2(r) cost. A streaming output avoids O(Z_R) extra materialization memory; retaining all rows costs Ω(Z_R) labels.

The **actual intermediate dimensions** are: B input x-lifts; B² unoriented pair-chart records; D distinct x-roots (D≤B²); H_2 degree D; ≤D quadratic S3 query factors; K_r remainder degree <D; J_r degree h≤D; ≤2h candidate partner roots, before pair-label output multiplicity. These are dimensions of the specified elimination, not claims about Gröbner/Macaulay matrix dimensions or solving degree.

Under quasi-linear M, setup σ=2 (up to logs), an empty-inclusive query has τ=2 (up to logs) when D=Θ(B²), and complete output adds Z_R. The conventional full-independent-row m4 budget requires max(σ,2)+τ<3. This explicit polynomial contraction instead has 2+2=4, so it does **not** prove a sub-square-root cold solver. Non-generic lower bounds for different representations are not inferred. In a high-collision base with D≪B², measure D and setup honestly; no such bound is supplied by the fixed-curve Dickson density theorem.

At Prime31 B=31 the chart has exactly 961 expressions, versus L²=3844 ordered signed pair expressions; D≤961 and the full source pair-label count is still 3844 when ordered multiplicities are retained or regenerated. The exact independent uniform-target expectation and miss probability are those derived in iteration8; no targets have been executed in this iteration. [pcc:unverified]

## Literature, previous lanes and execution standing

- Semaev, 2015 rational-auxiliary S3 chains: https://eprint.iacr.org/2015/310 . The rational chain itself predates this note.
- Faugere–Huot–Joux–Renault–Vitse, EUROCRYPT 2014 symmetric Semaev equations, DOI 10.1007/978-3-642-55220-5_3 . Symmetrization itself is prior art.
- Earlier in-repo work: merged #4928 iteration6 (rational/ghost/comb), iteration7 (fibre), merged #4924 (pair radical/moment), merged #4941 iteration8 (group-semiring m4 contraction), current #4920/#4946 selector and chain lanes. This note deliberately avoids their source paths.
- Current indexing caveat: Chan–Itakura–Ma–Yu (arXiv:2610.42012, revised 2026-11-14), Vannimmen (arXiv:2611.09656), Dinur–Golovnev–Yan–Zhu (arXiv:2611.13294). Integer hashes require a DLP-free lossless elliptic reduction; generic reporting still charges L^m input construction. No novelty claim or blanket quadratic-preprocessing theorem is made.
- Existing source-bound CI: iteration6 original-root run45096187034 failed inherited spec.rs before tests; rerun45097885135 failed in inherited ic_framework/mod.rs; later run45100829948 failed at duplicate Cargo.toml key before tests. These do not certify this new derivation or an implementation. Historical iteration1/companion successful jobs remain matched only to their source commits.
- Research KB MCP absent here; direct repository records and primary public literature consulted. ChatGPT research Page was inaccessible: interactive authorization is needed in this noninteractive runtime. No Space mutation, native Rust execution, EKS job, full cold benchmark, signed PCC verdict, or performance claim occurred.

## Next discriminating gate

Implement this exact decision/output/certificate oracle as original-arithmetic Rust with source-frozen F101 complete-target census (including O, bad affine order, duplicate/collision and one-dimensional/degenerate S3 cases), then independent Prime31 uniform blind targets with stored seeds and explicit empty certificates. Compare the same cells with group-semiring MITM; the objective is **semantic and cost validation**, not to rebrand MITM as a breakthrough. Then pursue a genuinely different contraction with a proven τ<1 empty-inclusive bound and quantitative family/subgroup/rank obligations. Do not launch performance without repository checkpoint/EKS admission. Keep all PCC claims unverified until signed independent verification.