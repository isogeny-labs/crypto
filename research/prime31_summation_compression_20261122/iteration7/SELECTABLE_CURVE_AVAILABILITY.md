# Fixed-fibre point availability on a selectable curve

**Standing:** written mathematical deduction, [pcc:unverified]. No arithmetic experiment, formal certificate, or remote mutation. This note applies to selecting a curve after fixing a fibre; it does not change a prescribed ECDLP input curve.

## 1. Exact moments for every fixed coordinate set

Let p>3 be prime, let F be any fixed set of N distinct elements of Fp, and choose the coefficient pair (a,b) uniformly from Fp². This distribution is over short Weierstrass models, not uniformly over isomorphism classes. Put

\[
 f_{a,b}(X)=X^3+aX+b,\qquad
 B(a,b)=\#\{x\in F:f_{a,b}(x)\text{ is a square, including zero}\}.
\]

For distinct x,z, the affine map
\[
 (a,b)\longmapsto(f_{a,b}(x),f_{a,b}(z))
\]
has linear determinant x-z≠0. Thus these two values are independent uniform field elements. Each value is uniform individually. This is pairwise independence; no higher independence is claimed.

There are (p+1)/2 squares including zero, so the Bernoulli liftability indicators give exactly
\[
 \mathbb E B=\frac{N(p+1)}{2p},\qquad
 \operatorname{Var}B=\frac{N(p^2-1)}{4p^2}.
\]
Consequently Chebyshev gives
\[
 \Pr[B<N/4]\le
 \frac{4(p^2-1)}{N(p+2)^2}<\frac4N. \tag{1}
\]
These moments include singular models. They must not be reused as conditional moments after a curve restriction.

For reference, the singular pairs are exactly
\[
 (a,b)=(-3t^2,2t^3),\quad t\in\mathbb F_p,
\]
because \(f=(X-t)^2(X+2t)\) on the discriminant-zero locus. This parametrization is injective: for a≠0, \(t=-3b/(2a)\), and a=0 forces b=t=0. There are p singular pairs. Conditioning merely on nonsingularity can therefore multiply the upper bound in (1) by at most \(1/(1-1/p)\); independence after conditioning is unnecessary.

## 2. Exact odd-order subfamily

Let I be the coefficient pairs for which \(X^3+aX+b\) is irreducible over Fp. Then
\[
 |I|=(p^2-1)/3,\qquad
 \Pr[I]=c_p=\frac{p^2-1}{3p^2}. \tag{2}
\]

**Proof of the count.** In Fp³ the trace map is surjective because Tr(1)=3≠0. Its kernel has p² elements. Its intersection with Fp consists only of zero, again because Tr(u)=3u. Every other trace-zero element has degree exactly three, since 3 is prime. Frobenius groups these elements into orbits of size three, each the roots of exactly one monic irreducible cubic with zero quadratic coefficient. This gives (2).

This is a classical prescribed-trace count, not a novelty claim; see Ruskey–Miers–Sawada, Theorem 1.1 and the zero-trace subtraction immediately following it, cited below.

An irreducible cubic is separable and has no rational root. Hence the corresponding E is nonsingular and has no rational point of order two: on this model the nonidentity two-torsion points have y=0. Cauchy's theorem then implies that #E(Fp) is odd. Conversely, every nonsingular short model of odd rational order belongs to I.

Combining (1) and (2) gives, without any independence assumption between I and B,
\[
 \Pr[I\ \text{and}\ B\ge N/4]\ge
 c_p-\frac{4(p^2-1)}{N(p+2)^2},
\]
and
\[
 \Pr[B<N/4\mid I]\le
 \min\!\left(1,\frac{12p^2}{N(p+2)^2}\right).
\]
In particular, a growing fixed fibre admits odd-order curves with B=Θ(N).

## 3. Balanced liftability, with an explicit positive density

Let χ be the quadratic character extended by χ(0)=0 and define
\[
 S(a,b)=\sum_{x\in F}\chi(f_{a,b}(x)).
\]
The same pairwise independence gives
\[
 \mathbb E S=0,\qquad \mathbb E S^2=N(p-1)/p.
\]
Therefore
\[
 \Pr[|S|>N/2]\le\frac{4(p-1)}{pN}.
\]
On I none of the values vanishes, so \(B=(N+S)/2\). Define the good event
\[
 \mathcal G= I\cap\{|S|\le N/2\}.
\]
Then every good model has odd rational order and
\[
 N/4\le B\le3N/4,\qquad
 \Pr[\mathcal G]\ge
 \delta_{p,N}:=\frac{p^2-1}{3p^2}-\frac{4(p-1)}{pN}. \tag{3}
\]
For N≥16,
\[
 \delta_{p,N}\ge\frac{(p-1)(p+4)}{12p^2}>\frac1{12}.
\]
Thus this is a uniform constant-density statement for coefficient pairs, not merely the existence of one exceptional model.

**Ordinarity is a deduction, not an additional random-curve assumption.** Over the prime field with p>3, a supersingular elliptic curve has Frobenius trace zero, hence rational order p+1. That order is even. Every model in I is therefore ordinary. The primary reference is Waterhouse, Theorem 4.1 with extension degree one, together with Corollary 4.4; see sources below. This implication is specific to the stated prime-field setting.

Every liftable coordinate on a good model yields two distinct signed rational points, so the signed base has \(2B\in[N/2,3N/2]\). The identity and rational two-torsion are absent.

## 4. Non-anomalous curves for the symmetric Dickson fibre

Now assume additionally \(p\equiv3\bmod4\) and \(F=-F\). These hypotheses hold for the full split fibre of \(D_{2^k}(X)-c\), k≥1, in the smooth-torus family: the doubling recurrence makes \(D_{2^k}\) even. In that family use k≥4 to obtain N≥16; discarding smaller finite prefixes changes no growing-family assertion.

The coefficient involution
\[
 \iota(a,b)=(a,-b)
\]
satisfies
\[
 f_{a,-b}(-x)=-f_{a,b}(x).
\]
It preserves irreducibility, and χ(-1)=-1 implies
\[
 S(a,-b)=-S(a,b),\qquad B(a,-b)=N-B(a,b)\quad\text{on }I.
\]
Thus it preserves \(\mathcal G\). Over the whole field the same identity gives
\[
 \#E_{a,b}(\mathbb F_p)+\#E_{a,-b}(\mathbb F_p)=2p+2;
\]
the two Frobenius traces are opposites. At most one member has anomalous order p. There are no fixed pairs in I because b=0 makes the cubic reducible.

It follows that at least \(\delta_{p,N}/2>1/24\) of all coefficient pairs are simultaneously nonsingular, ordinary, of odd non-anomalous rational order, and balanced on the fixed F.

**Theorem.** For every prime \(p\equiv3\bmod4\), every fixed symmetric F⊂Fp with \(16\le |F|=N\le p\) admits an ordinary, nonsingular curve of odd order different from p, with \(N/4\le B\le3N/4\). This applies to any already certified full split Dickson fibre satisfying these size and symmetry conditions, including the field \(p=2^{31}-1\). It does not prescribe the trace or the factorization of the group order.

### A charged construction without full point counting

Sample coefficient pairs until \(\mathcal G\) holds. Equation (3) bounds the expected number of independent trials by less than 12.

A cubic is irreducible precisely when \(\gcd(f,X^p-X)=1\), so degree-three modular exponentiation and a polynomial gcd test it in O(log p) field operations. Screen F using quadratic characters; this costs O(N log p) field operations by binary exponentiation. Since p≡3 mod4, a selected nonzero square has square root \(h^{(p+1)/4}\), with O(log p) field operations. These are elementary cost upper bounds, not measured implementations.

Choose any nonidentity rational lift P on a good model. For p≥7, Hasse's bound gives \(0<\#E(\mathbb F_p)<2p\). Consequently,
\[
 [p]P=O \iff \#E(\mathbb F_p)=p.
\]
Indeed, the forward implication forces the nonzero P to have order p, so p divides a group order strictly below 2p; the reverse follows from Lagrange's theorem. If the test is nonzero, retain E. If it is zero, switch to \(E_{a,-b}\), which remains good and has order p+2. The test costs O(log p) group operations. No full point count is needed merely to remove anomalous models.

Prime/fibre generation and certification, the Θ(N) coordinate enumeration, all screening and square roots, and output storage remain setup costs. Materializing the signed base uses Θ(N) output slots. This construction supplies no sublinear root-finding or empty-target procedure.

## 5. Connection to the exact-m affine-chain completeness condition

Odd group order alone is insufficient for the iteration-six free-permutation completeness lemma. Four copies of an order-three point have nonzero total, but every ordering has a zero length-three prefix.

The backward-removal proof does extend if **every permitted leaf has order greater than m−1**. At a stage with t≥2 remaining leaves and nonzero sum S, a removable leaf different from S exists unless all t leaves equal S. The exceptional case gives \((t-1)S=O\); since S is then itself a permitted leaf and \(t-1\le m-1\), it contradicts the leaf-order condition. Repeating the removal constructs the required nonzero prefixes. No bound on the order of arbitrary intermediate sums is needed.

For fixed m, discard rational base points satisfying \([d]P=O\) for any \(2\le d\le m-1\). The geometric d-torsion contains at most d² points, so the total number removed is at most
\[
 C_m=\sum_{d=2}^{m-1}(d^2-1)=O(m^3).
\]
Negation preserves order, so this removes signed pairs. In particular the retained coordinate count satisfies the conservative bound
\[
 B'\ge N/4-C_m=\Theta(N)
\]
for fixed m and growing N. Successive additions test the excluded small orders in O(m) group additions per lifted point. This is additional charged setup.

The exclusion is also an additional membership condition if the polynomial system is to use this restricted base. For a known excluded coordinate set T, it can be enforced by
\[
 u_i\prod_{t\in T}(x_i-t)=1
\]
at each leaf, with O(|T|) extra multiplication gates per leaf when converted to a quadratic circuit. The frozen iteration-six counts do not include these extra guards. Without the exclusion or an equivalent exact restriction, retain the original prime-order scope or handle identity-intermediate branches explicitly.

The resulting completeness statement remains for freely permutable signed multisets, nonzero target, and the affine left chain. It does not assert completeness of a fixed sorted ordering or arbitrary balanced tree.

## 6. What remains unproved

This proves rational point density on a selectable ordinary, non-anomalous curve and, for fixed m, preserves that density after removing small-order leaves. It does not prove:

- a prime group order or a prime divisor r≥p^ε of a required size;
- that Θ(N) fibre points lie in a selected prime-order subgroup;
- a suitable density after conditioning on a prescribed trace, prescribed group order, or a specified input curve;
- random-like relation counts, independent relation rank, or a fast root-solving step.

Cofactor clearing changes point abscissae and need not preserve the Dickson fibre. Replacing the chosen curve also supplies no DLP-preserving transfer from an original fixed curve. These are separate requirements.

A useful exact next criterion is: if a desired class of **prime-order** models has coefficient-pair density \(\eta_p\), then its intersection with balanced fibre models has density at least \(\eta_p-4(p-1)/(pN)\). A positive bound would follow from \(\eta_p>4(p-1)/(pN)\). No such admissible-class density theorem is established here.

## 7. Sources and independent check

- Existing lane input: remote-5fb/SMOOTH_TORUS_FAMILY.md, sections 1–4; its split-fibre construction is retained. The present argument replaces only the unknown *selectable-curve rational-lift density* portion of section 4, not the large-prime-subgroup qualification.
- Existing exact-m proof: higher_arity_math_v5.md, section 5. The permitted-leaf order refinement follows directly from its backward-removal argument.
- F. Ruskey, C. R. Miers and J. Sawada, *The Number of Irreducible Polynomials and Lyndon Words with Given Trace*, SIAM J. Discrete Math. 14(2), 240–245 (2001), Theorem 1.1 and the zero-trace identity on p.241. [Author publication page](https://webhome.cs.uvic.ca/~ruskey/Publications/Traceq.html); [primary article PDF mirror, read](https://static.aminer.org/pdf/PDF/001/061/772/the_number_of_irreducible_polynomials_and_lyndon_words_with_given.pdf). The article attributes prescribed-trace formulas to Carlitz (1952). The elementary degree-three proof above is included in full.
- W. C. Waterhouse, *Abelian varieties over finite fields*, Ann. Sci. École Norm. Sup. 4e série 2(4), 521–560 (1969), Theorem 4.1, p.536, and Corollary 4.4, p.540. [Primary archive PDF, read](https://www.numdam.org/article/ASENS_1969_4_2_4_521_0.pdf). These support the ordinary/supersingular classification and trace bounds used here.
- Andrew Sutherland, MIT 18.783, Lecture 14 (2017), Theorem 14.2 and Corollary 14.3, supplies a direct proof of the prime-field supersingular trace-zero implication. [Lecture PDF, checked as a corroborating exposition](https://math.mit.edu/classes/18.783/2017/LectureNotes14.pdf).
- Independent mathematical spot-check by the higher_arity_math agent accepted the count, moments, balanced intersection, twist symmetry and ordinary/non-anomalous implication, and supplied the [p]P anomaly-test refinement. This is a written independent review, not signed PCC.
- Sources checked (KB): no callable research-kb tools were exposed, and no wrapper was present in the bounded materialized checkout. Fallback used the existing lane notes and primary articles above. No literature-wide novelty claim is made.

No code, experiment, timing run, PR content, pinned report, or pinned mathematical note was modified by this derivation.
