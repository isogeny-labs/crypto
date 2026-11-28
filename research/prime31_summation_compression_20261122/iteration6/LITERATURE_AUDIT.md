# Prime31 iteration 5: current indexing literature and applicability constraints

Date: 2026-11-23. Scope: read-only literature and mathematical audit. No cryptographic experiment or remote mutation was performed for this note.

**Status:** no DLP-free elliptic-curve relation-solving speedup established. The current literature does contain genuinely subquadratic-preprocessing **integer** 3SUM indexing, so the blanket statement that every known 3SUM index needs quadratic preprocessing must be retired. The gap is a charged, lossless reduction from encoded elliptic points to the required arithmetic or incidence representation.

All mathematical derivations below are **[pcc:unverified]**: they have written proofs but no machine-checked certificate. They are not claims of novelty. Contemporary preprints are reported as their authors' results; their full proofs and linked formal developments were not independently verified in this audit.

## 1. Model and notation

Use the recorded Prime31 curve

\[
p=2^{31}-1,\qquad E:y^2=x^3+847514846x+2087444724,
\]

with recorded prime group order \(n=2147446831\). Primality and the point count are inputs from the existing experiment state, not newly checked here. For asymptotics let \(G=E(\mathbb F_p)\) have prime order \(n\), with ordinary canonical point encodings. Let \(A=-A\subset G\) have \(L=2B\) signed factor-base points. Fix arity \(m\); the ordered sum map has domain size \(N=L^m\):

\[
f_m:[L]^m\longrightarrow G,\qquad
f_m(i_1,\ldots,i_m)=A_{i_1}+\cdots+A_{i_m}.
\]

Point addition and equality are available; scalar coordinates relative to a generator are not. Setup, target generation, memory, output verification, repetitions, relation independence, and final linear algebra must all be charged. A fixed 31-bit field alone does not define an asymptotic family.

## 2. Primary-source updates

### 2.1 Integer indexing now has subquadratic preprocessing

Chan, Itakura, Ma, and Yu, *3SUM Made Simpler…*, arXiv:2610.42012v2, Corollary 2.4 and its following paragraph, give integer 3SUM indexing with \(O(L^{1.965})\) preprocessing/space and \(O(L^{0.965})\) decision queries, with Monte Carlo error. Section 3 extends the approach to odd integer kSUM. The PDF is dated November 17, 2026; the arXiv version stamp says November 14.

Their reduction uses scalar affine hashes modulo a random prime, such as

\[
f(a)=(\xi a+\alpha)\bmod q,\quad
g(b)=(\xi b+\beta)\bmod q,\quad
h(c)=(\xi c+\alpha+\beta)\bmod q,
\]

digit representations, and scalar squared residuals to reject false positives. A bound for integer decision queries does not itself give complete elliptic decomposition reporting. The exact relation between these three hashes is essential; arbitrary hashes of point encodings do not supply it.

Primary: <https://arxiv.org/pdf/2610.42012>, especially §§2.2, 3.2 and Corollary 2.4.

### 2.2 A genuine selected-entry matrix primitive

Alman and Vassilevska Williams, arXiv:2610.06783, Theorem 3/Corollary 26, preprocess explicit integer matrices \(X\in\mathbb Z^{H\times D}\), \(Y\in\mathbb Z^{D\times H}\), with polynomial-size entries and \(H\ge D^{18}\), in \(O(H^2/D^{0.063})\) time, then answer an entry query in \(O(D^{0.437})\) time. This is a relevant new primitive with subquadratic setup. The missing elliptic reduction must construct its matrices within budget. Section 1.2 explicitly explains that their 3SUM speedup does not automatically solve three-collinear-points: the known reduction runs in the opposite direction.

Primary: <https://arxiv.org/html/2610.06783v1>.

### 2.3 Explicit finite-group coordinates are a substantive assumption

Majdoddin, arXiv:2610.17420v1, §19 “Model,” represents a finite abelian group as a product of known residue rings, and each element by its residue vector. Proposition 19.10 reduces boundedly many cyclic factors to integer 3SUM by lifting coordinates and enumerating carries; Theorem 19.11 handles all primes in that representation. An abstract isomorphism \(E(\mathbb F_p)\simeq\mathbb Z/n\mathbb Z\) does not furnish those input coordinates. For a known elliptic generator, computing this coordinate is the DLP.

Primary: <https://arxiv.org/html/2610.17420v1>, §§19.0–19.4.3.

### 2.4 Reporting now has a general-purpose result

Vannimmen, *From Function Inversion to Preimage Reporting*, arXiv:2611.09656 (November 6, 2026), Theorems 1 and 7/Corollary 18, gives all-preimage reporting for an efficiently evaluable function on \([N]\), with arbitrary short-bitstring codomain, tradeoff

\[
TS^2=N^2\Delta,\qquad \Delta=\max_y|f^{-1}(y)|,
\]

and \(\widetilde O(N)\) preprocessing. Completeness holds with high probability over preprocessing. The reporting bound uses the maximum fibre size, not merely the current output. Theorem 16's inverse dependence on current fibre size is a **one-witness** bound.

The direct specialization to canonical elliptic \(m\)-sums needs no logarithms: evaluating \(f_m\) uses point addition. However, substituting \(N=L^m\) leaves \(\widetilde O(L^m)\) cold setup; this route does not supply the missing implicit index. An emitted tuple is directly verifiable, but that verification does not certify that a reported empty fibre is complete.

Primary record: <https://arxiv.org/abs/2611.09656>. Full paper inspected through a PDF mirror because the primary PDF fetch failed: <https://borowiecki.dev/pdf/2611.09656>.

### 2.4b Later reporting result: actual output sensitivity

Dinur, Golovnev, Yan, and Zhu, *Function Inversion Reporting*, arXiv:2611.13294 (November 9, 2026), Theorems 3–4 and 4.1–4.2, materially improve the reporting guarantee. With \(r_y=|f^{-1}(y)|\) and collision probability \(q(f)\), they give

\[
\widetilde O(N^3/S^3+N^2r_y/S^2),
\qquad
\widetilde O(N^3q(f)/S^2+Nr_y/S)
\]

query bounds, respectively. Both retain \(\widetilde O(N)\) preprocessing and simultaneous correctness for all queries with probability at least \(1-1/N\). The extension covers polynomially large codomains, hence encoded elliptic sums when their encoding range is polynomial in \(N\). This provides genuine reporting dependence on the current output, improving the landscape beyond a global-\(\Delta\) bound. It still leaves \(N=L^m\) cold construction and does not certify zero-error emptiness. Their arithmetic-partition improvement for integer kSUM remains representation-specific.

Primary record: <https://arxiv.org/abs/2611.13294>; full paper inspected at <https://arxiv.cnoctave.top/pdf/2611.13294> after the primary PDF fetch failed. This is a bounded follow-up requested during the iteration-6 review.

### 2.5 Existing indexing and generalized-birthday results retain their scopes

Dinur–Golovnev's ICALP 2026 3SUM index remains a different tradeoff with quadratic preprocessing; its useful pair partitions rely on integer congruences. It is not a proof that no other index can preprocess subquadratically.

Primary: <https://drops.dagstuhl.de/storage/00lipics/lipics-vol374-icalp2026/html/LIPIcs.ICALP.2026.78/LIPIcs.ICALP.2026.78.html>.

Wagner's generalized birthday paper distinguishes XOR/integer arithmetic from general cyclic groups. Its Theorem 2, credited to Wei Dai, reduces discrete logarithms to general cyclic-group kSUM; Theorem 3 charges conversion via logarithms. The attacks on elliptic-curve blind signatures do not give a direct generalized-birthday algorithm for summing encoded elliptic points.

Primary: <https://people.eecs.berkeley.edu/~daw/papers/genbday.html>. PDF inspected: <https://www.enseignement.polytechnique.fr/informatique/profs/Francois.Morain/Master1/Crypto/projects/Wagner02.pdf>, §3.

## 3. Exact scalar-residue buckets are already a DLP oracle

**Proposition [pcc:unverified].** Let \(G=\langle P\rangle\) have prime order \(n\), and fix \(2\le M<n\). Suppose an oracle evaluates

\[
\beta(Q)=\bigl([\log_P Q]_{\{0,\ldots,n-1\}}\bigr)\bmod M
\]

for every group point. Then \(\lceil\log_M n\rceil\) oracle calls, with polynomial-in-\(\log n\) group overhead, recover the full logarithm.

**Proof.** Put \(Q_0=Q\). Recursively set

\[
a_i=\beta(Q_i),\qquad
Q_{i+1}=[M^{-1}\bmod n](Q_i-[a_i]P).
\]

If \(s_i\in[0,n-1]\) is the canonical logarithm of \(Q_i\), then \(a_i=s_i\bmod M\). The integer \((s_i-a_i)/M\) lies in \([0,n-1]\), so it is exactly the canonical logarithm of \(Q_{i+1}\). Thus the \(a_i\) are the base-\(M\) digits of the original logarithm. Each division step can use ordinary scalar multiplication by \(M^{-1}\bmod n\), costing \(O(\log n)\) group operations. No logarithm is assumed in the reduction itself.

**Scope.** This excludes treating exact low scalar bits as a free primitive. It does not cover noisy or partial-domain oracles unless their guarantees also cover these adaptively constructed points. If the factor-base points were generated with known scalar labels, that alone does not provide the labels of an arbitrary target containing the unknown DLP.

## 4. What x-only and small-quotient buckets cannot provide

### 4.1 Exact compositional compression

**Proposition [pcc:unverified].** If \(G\) has prime order and \(b:G\to C\) obeys

\[
b(P+Q)=\mu(b(P),b(Q))\quad\text{for all }P,Q\in G
\]

for a deterministic binary operation \(\mu\), then \(b\) is injective or constant.

**Proof.** If \(b(P)=b(Q)\) with \(P\ne Q\), translating by any \(R\) gives \(b(P+R)=b(Q+R)\). Thus \(b\) is invariant under a nonzero group translation. That translation generates the prime-order group, so \(b\) is constant.

Consequently an exact many-to-one bucket composition cannot replace point addition. In the usual homomorphism formulation, any map to a group of order strictly below \(n\) is trivial. This is not an obstruction to multivalued carries, approximate relations, or restrictions to a proper subset of pairs.

### 4.2 A fixed additive carry set for x-only buckets must be large

**Proposition [pcc:unverified].** Let \(G\) have odd order, and let \(b:G\to\mathbb Z/M\mathbb Z\) be sign-blind: \(b(P)=b(-P)\). Suppose one fixed set \(C\subseteq\mathbb Z/M\mathbb Z\) satisfies

\[
b(P+Q)\in b(P)+b(Q)+C\qquad\text{for all }P,Q\in G.
\]

Then

\[
|\operatorname{im}b|\le\gcd(2,M)|C|\le2|C|.
\]

**Proof.** Set \(Q=-P\). Then \(2b(P)\in b(O)-C\). Multiplication by two in \(\mathbb Z/M\mathbb Z\) has kernel size \(\gcd(2,M)\), proving the cardinality bound.

For a general abelian bucket group \(H\), the same assumptions imply

\[
\operatorname{im}b\subseteq b(O)+(C-C),\qquad
|\operatorname{im}b|\le |C-C|\le |C|^2.
\]

Indeed, the equations for \((P,-P)\) and \((P,P)\) imply \(b(2P)\in b(O)+(C-C)\); doubling is bijective on an odd-order group. For XOR bucket groups, \(2b(P)=0\), and directly \(b(2P)\in C\), so the stronger bound \(|\operatorname{im}b|\le|C|\) holds even without sign-blindness.

Thus a nontrivial, increasingly many-bucket map depending only on \(x(P)\) cannot have a globally fixed constant-size arithmetic carry set. Affine coordinate signs, input-dependent carry sets, restricted pairs, and probabilistic filters fall outside this proposition. In particular, x-coordinate addition is legitimately two-valued on the Kummer quotient; this proposition does not eliminate that algebraic structure.

**Identity-excluding version [pcc:unverified].** The weaker bound \(|\operatorname{im}b|\le|C-C|\) still holds when \(b\) is defined only on \(G\setminus\{O\}\), is sign-blind there, and the carry condition is imposed only for all triples \(P,Q,P+Q\ne O\). For any \(U,V\ne O\) with \(U\ne\pm V\), set \(P=(U+V)/2\) and \(Q=(U-V)/2\). The admissible sums \(P+Q=U\) and \(P-Q=V\) place \(b(U)\) and \(b(V)\) in the same translate \(b(P)+b(Q)+C\). Thus \(b(U)-b(V)\in C-C\). If \(U=\pm V\), the difference is zero. Fixing \(V\) proves the bound. Merely excluding cancelling pairs therefore does not create a globally balanced constant-carry hash; restricting to a proper factor-base pair domain is a different hypothesis.

### 4.3 Hashing an encoding is not arithmetic hashing

**Observation [pcc:unverified].** For a uniformly random hash into \(K\) buckets, and fixed distinct points \(P,Q,P+Q\), the hash of \(P+Q\) is independent of the hashes of \(P,Q\). A candidate set of \(c\) output buckets determined only from the first two labels retains the correct sum with probability \(c/K\). Repeating enough independent filters can recover completeness, but their cost and false negatives must be counted. This is a statement about that random-hash model, not every structured hash of elliptic coordinates.

## 5. A scoped obstruction to a global thin matrix encoding

**Proposition [pcc:unverified].** Let \(G\) have prime order \(n\), and let \(\varnothing\ne A\subsetneq G\). The \(n\times n\) integer matrix

\[
K(r,s)=\mathbf 1_A(r-s)
\]

has rank \(n\) over \(\mathbb Q\). Therefore a single exact factorization \(K=XY\) over characteristic zero has inner dimension at least \(n\).

**Proof.** For the proof only, identify \(G\) abstractly with \(\mathbb Z/n\mathbb Z\), and form \(F_A(Z)=\sum_{a\in A}Z^a\), of degree at most \(n-1\). The circulant eigenvalues are \(F_A(\zeta^j)\), where \(\zeta\) is a primitive \(n\)-th root. At \(j=0\), the value is \(|A|>0\). If another value vanished, the rational polynomial \(F_A\) would be divisible by the minimal polynomial \(\Phi_n(Z)=1+Z+\cdots+Z^{n-1}\). The degree bound and 0/1 coefficients would force \(A\) to be empty or all of \(G\), a contradiction. Hence all eigenvalues are nonzero.

This rules out one particularly tempting use of the new thin-matrix primitive: a single small-inner-dimension factorization of the **entire** group membership matrix. It says nothing conclusive about selected factor-base/target submatrices, nonlinear decoding, a family of products, finite-characteristic rank, target-dependent construction, or genuinely special fibres. It is a standard Fourier/cyclotomic argument, not a claimed new lower bound for ECDLP.

## 6. Generic kSUM reduction, with costs and domain restrictions

**Reduction [pcc:unverified].** Given \(Q=[x]P\), generate \(m\) lists of \(L\) uniform random scalars \(a_{ij}\), storing points \([a_{ij}]P\) and their labels. A solver that finds a tuple summing to the arbitrary target \(Q\) immediately gives \(x=\sum_i a_{i,j_i}\bmod n\). The full cost is list generation \(O(mL\log n)\) group operations plus solver setup/query cost and output verification, with its success probability retained.

For a zero-target-only solver, instead use fresh random pairs \((a,b)\) and points \([a]P+[b]Q\). A returned zero-sum tuple gives \(A+xB=0\pmod n\). Conditional on the point inputs, the hidden \(b\) values remain uniform; with distinct list positions and an oracle seeing only point encodings, \(B\ne0\) with probability \(1-1/n\). Then \(x=-A/B\).

This explains why an arbitrary-list generic-group generalized-birthday improvement would already have DLP consequences. It does **not** apply automatically to a solver promised a special Dickson factor base, to uncharged offline advice, or to a representation-specific algorithm. Those are separate models and must be assessed constructively.

## 7. S3-chain prior art boundary

Semaev's 2015 paper, §3 equation (4), already imposes \(x_i\in V\subseteq\mathbb F_q\) and rational-field auxiliary coordinates in an S3 chain. Lemma 2 relates it to the higher summation polynomial under a minimal-arity hypothesis, treats non-rational lifts through rational 2-torsion, and excludes identity intermediate points in the proof. Section 4.2 explicitly leaves the large-prime-field solver unclear. Therefore connected-tree purity or identity-cut handling for full Dickson fibres can be a useful correctness refinement, but the rational-auxiliary S3-chain mechanism itself is established prior art.

Primary: <https://arxiv.org/pdf/1504.01175>, §§3–4.2, especially equation (4) and Lemma 2.

## 8. Concrete remaining admission test

The most relevant new route is to supply a DLP-free reduction to selected subset/vector convolution or selected matrix entries. A credible candidate must provide all of the following before an asymptotic improvement is admitted:

1. A computable signed-coordinate or restricted-family representation, with a proof of its relation-preserving property. Exact low scalar residues cannot be assumed.
2. A construction cost below the claimed improved total. Enumerating the full \(L^m\) domain to build a compact index does not count as an implicit construction.
3. The actual dimensions, number of products/trials, and target dependence of the matrix/convolution reduction. A small arithmetic circuit for an individual S equation is not a thin factorization of its zero indicator.
4. A reporting or emptiness guarantee with quantified failure probability and charged amplification. One returned witness, all witnesses, and certified emptiness are different requirements.
5. A growing prime-field family, a matching relation-yield model, and sufficient independent relations after removing cancellations and dependencies. A local relation-query exponent does not by itself establish a full DLP exponent.

The current positive deliverables are the corrected primary-source landscape and explicit applicability lemmas. The mathematical and implementation work on connected affine S3 trees can continue independently as a correctness improvement. This audit does not establish novelty, a subquadratic elliptic index, or an ECDLP asymptotic speedup.
