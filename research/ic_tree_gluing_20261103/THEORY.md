# Two cross-field connections for prime-field index calculus

2026-11-03 · companion to [`README.md`](README.md) (the measurement).
**Class: theory note and proposal. No algorithm is priced, and no `S` value or
leaderboard row is produced.** The proofs here are sketches unless they are
marked "proved". Statements marked "heuristic" are not established.

The lab's prime-field IC work has tried every factor-base symmetry group
available to a single instance:

| Symmetry | Lab result |
|---|---|
| S_m on the summands | symmetric Semaev, symmetric PKM, Graeffe and remainder towers |
| Negation | used throughout |
| 2-torsion translations | FGHR D_m, Montgomery T₀ |
| μ₃ / μ₄ automorphisms | GLV/j=0 orbit bases |
| Frobenius | binary and extension fields only |
| Isogeny kernels | generic-group null theorem |

The tower presentations tried are the Kummer (μ_{2^t}), Dickson/torus and
isogeny (Lattès) towers. This note asks two questions:

1. Why does no factor-base structure lower the elliptic degree?
2. Which symmetry is left that keeps every input degree bounded as the arity m grows?

The connections come from complex dynamics and incidence combinatorics
(question 1) and from phylogenetic algebraic statistics (question 2). A grep of
both repositories for Elekes, Ritt, "toric fiber product" and phylogenetics
finds no hits.

## 1. The Ritt dichotomy: the three PKM tower families are Ritt's three families

**Ritt's theorem (1923) on permutable maps.** Let f and g be polynomials (or
rational maps) of degree ≥ 2 that commute and share no common iterate. Then,
up to a common conjugacy, each falls in one of three families:

- power maps z ↦ z^n;
- Chebyshev/Dickson maps T_n;
- Lattès maps, the maps induced on P¹ = E/±1 by an endomorphism of an elliptic curve E.

**Observation (proved; elementary).** The lab's three tower families are
exactly these three families, each attached to a one-dimensional algebraic group:

| lab tower | self-map | algebraic group it is an endomorphism of |
|---|---|---|
| Kummer, V = g·μ_{2^t} | w ↦ w² | G_m (the smooth locus of the **nodal** cubic) |
| Dickson / norm-one torus | x ↦ x² − 2 | G_m/±1, i.e. the Kummer line of the nodal cubic |
| isogeny / Lattès tower | x ↦ x([2]P) or x(φ(P)) | E/±1 itself |

Only the third family is compatible with the elliptic group law. Each Kummer or
Dickson tower is a doubling tower for a different group: the multiplicative group
of the nodal degeneration y² = x²(x+c) of E. The summation polynomials of that
degenerate group are linear in log coordinates and interact with the tower. The
summation polynomials of E do not.

**Lemma A (proved; rigidity).** Let Φ : E → E be a morphism of varieties such
that Φ(P)+Φ(Q)+Φ(R) = O whenever P+Q+R = O. Then Φ = φ + T with φ ∈ End(E)
and 3T = O.

Proof: every morphism E → E is a translate of a homomorphism. Then
3T = Φ(O)+Φ(O)+Φ(O) = O. ∎

So a tower that preserves the summation variety level by level is a Lattès
tower. Its factor base F = Φ_t^{-1}(S) is a union of cosets of ker Φ_t. A
relation over F is then a relation over E/ker Φ_t on the image of S, which is
the lab's generic-group null theorem (RESEARCH_ISOGENY_FIBERS_ECDLP.md
Thm 15.1). A tower that does not preserve it is, by Ritt, a power or Chebyshev
tower, which belongs to the nodal group.

**Dichotomy (heuristic as stated; Lemma A is its rigorous half).** A
factor-base tower is either:

- compatible with the group law, in which case the base degenerates to IC on an isogenous curve; or
- incompatible, in which case it is a tower for the nodal degeneration and gives no elliptic cancellation in S_{m+1}.

This predicts what the lab measured:

- the solving degree D of every Kummer/Dickson tower rises with the depth t (staircase 4, 5, 5, 6, 6 at m = 3);
- relation counts on structured bases equal those of random bases (lever-2 null, 5,760 instances).

## 2. Elekes–Szabó: the quantitative version

**Elekes–Szabó (2012), and the finite-field variants of Tao and of
Bays–Breuillard.** Let F(x,y,z) be an irreducible surface. Either
|Z(F) ∩ A×B×C| ≤ O(n^{2−η}) for all n-sets A, B, C, or F is locally the graph
of addition in a one-dimensional algebraic group: G_a, G_m or an elliptic
curve. In the special case, the large intersections come from approximate
subgroups (progressions) of that group.

The S₃ surface is special: it *is* the elliptic group law. So a factor base A
with many more relations than random must be close to a coset progression of
E(F_p). Writing down such a set needs discrete logarithms. A Kummer base is a
progression in G_m, the wrong group. For a set that is multiplicatively but not
elliptically structured, the theorem gives an upper bound on the relation count
over the random expectation; it does not give an enhancement. That is the
lever-2 null. The finite-field statements hold only for |A| below a power of p,
so this is the heuristic reading at IC sizes, not a proof.

**What it says about the degree (heuristic).** In any tree-shaped PDP system,
the separators are the partial-sum abscissas X = x(P_i + P_j). Their elimination
ideal is the pair-sum set Σ₂(V) = {x(P+P') : P, P' ∈ F}. By Elekes–Szabó this
set is "generic": it has size about |V|² and no low-complexity presentation. F4
has to reach this degree, or the analogous degree for larger partial sums.
**Conjecture B:** the t-growth of D comes from the separators, not from the
membership towers. Every lab measurement agrees with it, but none tests it
directly. The direct test is to plant a separator constraint and compare D.

## 3. Phylogenetic gluing: the symmetry that is left

**Algebraic statistics.** In a group-based phylogenetic model on a tree, each
internal vertex carries the group sum of its children's states. Its ideal is
glued from the star ideals of the tree's vertices along the internal edges.
These are Sturmfels–Sullivant (2005) and Sullivant's toric fiber products
(2007). For codimension-zero toric fiber products, a Gröbner basis of the glued
ideal is assembled from those of the pieces, in degree at most the pieces'
maximum. The model's automorphism group is the automorphism group of the tree;
for a balanced rooted binary tree with m = 2^k leaves it is the iterated wreath
product S₂ ≀ … ≀ S₂, which is the Sylow 2-subgroup of S_m.

**The ECDLP relation is a group-based tree model.** A decomposition
R = P₁ + … + P_m is a rooted tree: the leaves are factor-base points, the
internal vertices are partial sums, and each vertex carries the elliptic group
law of its children, which is S₃. The lab's two families are two extremes:

- **plain chain (caterpillar).** The tree is a path, its automorphism group is
  trivial, and its inputs have degree ≤ 4 for every m. Each relation is seen m!
  times.
- **full symmetric (S_{m+1}(e)).** It quotients all of S_m but has one vertex
  of degree 2^{m−1}, so its input degree, and with it D, is exponential in m.
  The lab measured D = 8 at m = 4.

**The tree-glued cherry-symmetric system (this proposal).** Pair the summands
into cherries of a balanced tree. Encode each cherry (P_{2c−1}, P_{2c}) by the
symmetric pair (s_c, q_c) = (x+x', xx'). Its membership is the degree-2
remainder tower T^{2^t} ≡ 1 mod (T² − s_cT + q_c). Its vertex is
S₃(s_c, q_c, X_c), the symmetrized S₃ of degree 4. Internal vertices glue the
X's by S₃.

- This quotients (S₂)^{m/2} of the wreath symmetry, i.e. 2^{m/2} copies.
- It keeps **every input at total degree ≤ 4 for every m**.
- It has the same variable count as the caterpillar.

As far as the lab's records show, it is the only one of the three encodings
that both removes a growing part of the symmetry and has bounded input degree.

**Why bounded input degree is the condition that matters for a
subexponential cost.** Take |F| ≈ p^{1/m}, N ≈ m·t + O(m) unknowns and
t = log₂ p / m. One PDP costs about binom(N+D, D)^ω, there are about m! targets
per relation, and the relations number about |F|. The total is
L_p(1/2)-subexponential only if log binom(N+D, D) = O(√(log p · log log p)) at
m ≈ √(log p / log log p). Since N ≈ log p, that needs:

1. **D = O(m) up to log factors, which rules out any encoding with a degree-2^{m−1} input**; and
2. **D nearly independent of the depth t**, because t ≫ m at the optimum.

Condition 1 rules out the full symmetric arm asymptotically, whatever it gains
at small m. Condition 2 is what the measurement tests: the *t-slope* of D at
fixed m. The full gluing theorem would give D bounded by the cherry D. It is
not expected to hold, because the S₃ gluing is not toric and Conjecture B
predicts separator growth. The measurement asks whether the tree arm's t-slope
is below the caterpillar's.

**Kill criterion (pre-registered in `.resume/RESUME.md` before the solver
runs).** The tree's D is at least the caterpillar's in every comparable cell.
