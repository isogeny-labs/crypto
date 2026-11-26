# Selector–tag compatibility: finite fibers, occupancy, and correlated escapes

Date: 2026-11-25. Scope: mathematical analysis and deterministic arithmetic checks for the local M31 study. No new randomized experiment, publication, or paid execution was performed for this note.

## 1. Exact problem and terminology

Use the study's ordinary M31 instance, with

\[
p=2^{31}-1=2147483647,\qquad r=2147446831,
\]

where the source manifest supplies the curve and a generator \(P\) of the prime-order subgroup \(G\) of order \(r\). Let \(Q=xP\) be the public DLP target. A **public tag** \(\tau(R)=(a,b)\) certifies \(R=aP+bQ\); it does not reveal \(x\).

A signed zero-sum relation gives

\[
\sum_i\epsilon_iR_i=O,\quad
\alpha=\sum_i\epsilon_i a_i,\quad
\beta=\sum_i\epsilon_i b_i\pmod r.
\]

It solves the DLP only if \(\beta\ne0\), in which case \(x=-\alpha\beta^{-1}\pmod r\). If \(\beta=0\), point verification forces \(\alpha=0\) as well. Such a relation supplies no DLP information. The useful-rank criterion for a conventional collector remains separate from this APS one-relation criterion. [pcc:unverified]

For an x-only selector, work on the nonidentity negation orbits of \(G\). Since \(r\) is odd, their number is exactly

\[
M=(r-1)/2=1073723415.
\]

Each orbit has one x-coordinate, and each permitted x-coordinate represents exactly one orbit. The frozen sampling scale considered below is \(N=32768\) distinct tagged orbits. Do not count both signs as independent samples.

Let \(f\) be a fixed nonconstant rational or polynomial selector, with its poles treated explicitly. On the actual eligible x-coordinate set, its nonempty fibers have sizes \(b_1,\ldots,b_J\), with \(\sum_j b_j=M\). These are **actual subgroup-point capacities**, not merely polynomial degrees. A polynomial of degree \(d\) gives \(b_j\le d\), but equality need not hold. A rational map of degree \(d\) has at most \(d\) distinct geometric points in a fiber; charts and poles must not silently disappear from its finite-field implementation.

The occupancy results assume the selector was fixed independently of a uniformly sampled set of \(N\) distinct negation orbits. Independent uniform coefficient pairs \((a,b)\) give uniform group points; rejecting the identity and repeated negation orbits gives the stated without-replacement model. A frozen hash does not itself prove this sampling hypothesis. Correlated point generation is deliberately treated separately below. [pcc:unverified]

## 2. Exact occupancy law, including adaptive output selection

Let \(N_j\) be the number of sampled orbits in fiber \(j\). The joint law is multivariate hypergeometric:

\[
\Pr[(N_j)=(n_j)]
=\frac{\prod_j\binom{b_j}{n_j}}{\binom{M}{N}},
\qquad \sum_jn_j=N.
\]

For every integer \(1\le k\le M\),

\[
\mathbb E\!\left[\sum_j\binom{N_j}{k}\right]
=\binom Nk\frac{\sum_j\binom{b_j}{k}}{\binom Mk}.
\tag{1}
\]

Proof: for each k-subset of a particular fiber, the probability that the entire subset appears in the uniform sample is \(\binom Nk/\binom Mk\). Sum over those subsets and fibers.

In particular,

\[
\Pr[\max_jN_j\ge k]
\le\min\left(1,\binom Nk\frac{\sum_j\binom{b_j}{k}}{\binom Mk}\right).
\tag{2}
\]

This already permits choosing the best terminal value after observing every tag. No additional assumption of equal fiber sizes is used.

If only \(b_j\le d\) is known, write \(M=ud+v\), \(0\le v<d\). Discrete convexity of \(\binom bk\) gives the sharp capacity-only envelope

\[
\sum_j\binom{b_j}{k}
\le u\binom dk+\binom vk.
\tag{3}
\]

One proof is to move available mass toward the largest occupied bins until every bin but possibly one is full. Equivalently, the vector \((d,\ldots,d,v,0,\ldots)\) majorizes every feasible fiber-size vector. If a complete geometric capacity histogram \((h_j)\) is known, fill the largest capacities first until total mass M is exhausted; the same argument gives a potentially stronger envelope without assuming that half the roots lift to curve points.

### Choosing t high-occupancy fibers

Let \(H_t\) be the sum of the t largest occupancies. For \(L=at+v\), \(0\le v<t\), define

\[
J_{t,k}(L)=(t-v)\binom ak+v\binom{a+1}{k}.
\]

Whenever \(J_{t,k}(L)>0\), discrete convexity and Markov's inequality yield

\[
\Pr[H_t\ge L]
\le\min\left(1,
\frac{\binom Nk\sum_j\binom{b_j}{k}}
{\binom Mk\,J_{t,k}(L)}\right).
\tag{4}
\]

This is an explicit order-statistic bound for adaptively choosing the best t outputs. A simpler consequence of (2) is that \(\max N_j<k\) forces \(H_t\le t(k-1)\).

The capacity-only envelope (2)–(3) is available for a later exact native
evaluation at any chosen d and k. No numerical occupancy table for that
extension is included in the frozen Rust evidence of this cycle. The bound
concerns the stated frozen-chain sampling model; it is not an observed occupancy
measurement.

## 3. Complete fibers are much scarcer than partially occupied fibers

A terminal selector permits every eligible point in its fiber. A tag dictionary containing only some of those points is not an exact tagged factor base for that selector. The implementation must either cover the full allowed fiber, add a membership mask, or reject untagged extractor outputs and charge their cost.

Call fiber j **complete** if all \(b_j\) eligible orbits were sampled. Its exact probability is

\[
\Pr[j\text{ complete}]=\frac{(N)_{b_j}}{(M)_{b_j}},
\tag{5}
\]

with falling factorials and probability zero when \(b_j>N\).

Let \(W_{\ge k}\) count sampled points belonging to complete fibers of size at least k. Since the ratios in (5) decrease with the fiber size,

\[
\mathbb E W_{\ge k}
=\sum_{b_j\ge k}b_j\frac{(N)_{b_j}}{(M)_{b_j}}
\le M\frac{(N)_k}{(M)_k}.
\tag{6}
\]

There are at most \(M/k\) fibers of size at least k, so

\[
\Pr[\exists\text{ complete fiber of size}\ge k]
\le\frac Mk\frac{(N)_k}{(M)_k}.
\tag{7}
\]

These bounds allow arbitrary nonuniform rational-point densities and arbitrary adaptive selection of a subset of the complete fibers.

For \(N\le M/2\), define the terminal-count saving of all complete fibers by

\[
S=\sum_{j:\,j\text{ complete}}(b_j-1).
\]

Then

\[
\mathbb ES\le\frac{N(N-1)}{2(M-1)}.
\tag{8}
\]

To verify (8), for \(b\ge2\) divide \((N)_b/(M)_b\) by \((N)_2/(M)_2\). The remaining product is at most \(2^{-(b-2)}\). The inequality \((b-1)2^{-(b-2)}\le b/2\), followed by \(\sum b_j=M\), proves the claim. [pcc:unverified]

### Actual M31 numerical bounds

| Quantity | Exact bound | Decimal display |
|---|---|---|
| Expected sampled points in complete fibers of size at least 2 | \(1073709056/1073723414\) | 0.9999866278412 |
| Expected terminal-count saving S | \(1073709056/2147446828\) | 0.4999933139206 |
| Probability of any complete fiber of size at least 3 | \(N(N-1)(N-2)/(3(M-1)(M-2))\) | 0.00001017194355366 |

All three certificates can be checked with integer arithmetic within u128 at these parameter values. Decimal conversions are explanatory only. The probability formula can be displayed as a factorized rational if a later parameter range would overflow a chosen integer type.

**Implication and limit.** Sampling approximately \(\sqrt M\) independent tagged orbits is unlikely to produce any complete fiber containing three or more eligible points. Selecting the best outputs does not change that conclusion. It does not rule out deliberately correlated tags, a selector fitted to the sample, or a solver that profitably handles incomplete fibers with fully charged filtering. Those are different hypotheses.

## 4. Fixed chain versus adaptive chain

Equations (1)–(8) permit adaptive terminal/output selection for one fixed selector. They do not permit fitting the selector coefficients to the sampled points for free.

For K selectors fixed independently of the sample, a union bound multiplies the relevant event bound by at most K; actual search and evaluation costs also remain chargeable. For unrestricted coefficient fitting, use a capacity bound for the entire permitted selector family, such as the separate adaptive quadratic-selector certificate in this study. A sequentially fitted chain must be analyzed as an adaptive family, even if its final coefficients are frozen afterward.

Likewise, deliberately generating \(R\) together with a correlated partner is not sampling independent orbits. A real correlated construction can escape these bounds. Its tag transport, finite support, independence, and extraction costs must be proved separately. [pcc:unverified]

## 5. M31 multiplicative and Dickson fibers

### Ordinary multiplicative powers

Since \(v_2(p-1)=1\), for every \(s\ge1\),

\[
\gcd(2^s,p-1)=2.
\]

Thus \(x\mapsto x^{2^s}\) has one rational fiber of size 1 at zero and \((p-1)/2\) nonzero rational fibers of size 2. Its formal degree \(2^s\) does not give \(2^s\) rational branches. This statement concerns actual fibers on \(\mathbb F_p\), before curve-point eligibility can reduce them further.

### Dickson powers from the norm-one torus

Let \(d=2^s\), \(1\le s\le30\). The Dickson polynomial satisfies

\[
D_d(u+u^{-1},1)=u^d+u^{-d}.
\]

The two relevant cyclic groups are the split group \(\mathbb F_p^*\), of order \(p-1\), and the norm-one torus in \(\mathbb F_{p^2}^*\), of order \(p+1=2^{31}\). The quotient \(u\mapsto u+u^{-1}\) identifies inverses. The exact histogram of nonempty rational fibers of \(D_d(X,1)\) on \(\mathbb F_p\) is:

| Source/category | Number of output values | Distinct rational x-roots per output |
|---|---:|---:|
| Generic nonsplit torus images | \((p+1)/(2d)-1\) | d |
| Generic split images | \((p-3)/4\) | 2 |
| Output +2 | 1 | \(d/2+1\) |
| Output -2 | 1 | \(d/2\) |

Rows with equal capacities are added; they do not describe disjoint *capacity labels*. They do describe disjoint output categories.

Proof: on the norm-one torus, the d-th-power map has kernel size d and image of order \((p+1)/d\). Each nonendpoint inverse pair in the image gives d x-values. Output +2 has the d-th roots of 1, including both fixed inverse points \(\pm1\), hence \(d/2+1\) x-values. Output -2 has d roots of -1 and no fixed inverse points, hence \(d/2\). On the split group, the kernel has size 2 and the image has odd order \((p-1)/2\); its generic inverse pairs contribute two x-values. The endpoint +2 is already included in the torus category. There is no split contribution to -2. Split and nonsplit generic outputs are disjoint.

The total capacity is exactly p. At s=1, the first, second, and +2 rows all have capacity 2 and merge to \((p-1)/2\) fibers of size 2; the -2 row has size 1. This agrees with \(D_2(X,1)=X^2-2\). Do not count the overlapping capacity-2 rows as different roots of a single fiber.

A deterministic native check can use \(i^2=-1\) and

\[
z=(2+i)/(2-i)=429496730+1288490189i.
\]

Its norm is 1. To see its exact order without assuming an unrecorded power
calculation, put \(u=2+i\), so \(z=u/u^p\) and \(u^{p+1}=5\). Then
\(z^{(p+1)/2}=5^{-(p-1)/2}=-1\): quadratic reciprocity gives
\((5/p)=(p/5)=-1\) because \(p\equiv2\pmod5\). As \(p+1=2^{31}\),
z has exact order p+1. This gives a seed-free torus fixture for a future native
check; it does not certify eligibility of any x-root on the research curve.

To obtain a curve-independent bound for the actual subgroup capacities, cap each b_j by the corresponding row's geometric capacity and require \(\sum b_j=M\). Do not replace the b_j by half the geometric capacities without evidence.

## 6. A constructive correlated escape: translated reflection pairs

Take a publicly tagged \(A\in G\), \(A\ne O\), and let \(H=[2^{-1}\bmod r]A\). Define a degree-two map **on the elliptic curve** by

\[
h_A(R)=x(R-H).
\]

Its ordinary fiber is exactly \(\{R,A-R\}\). Both points have cheap public tags:

\[
\tau(A-R)=\tau(A)-\tau(R).
\]

Thus one can deliberately generate complete tagged pairs. This genuinely leaves the independent-sampling model of Section 2 and is a useful positive control.

Qualifications:

- h_A generally depends on both original coordinates. It is not a univariate rational function of original x. Implement it using complete point formulas or explicit denominator/infinity charts; a bare x-only S3 equation includes additional sign choices.
- This map partitions oriented curve points, not original negation orbits. If an implementation canonicalizes point signs, it must preserve the sign in the tag and in the h_A membership chart; it cannot silently reuse the x-only universe M from Section 1.
- The fixed point \(R=H\) is a singleton; infinity and exceptional charts need explicit handling.
- The built-in relation \(R+(A-R)-A=O\) has \((\alpha,\beta)=(0,0)\). Pair manufacture is not useful relation manufacture.
- Other relations among a correlated base are not thereby proved useless. Each must pass point verification and the \(\beta\ne0\) test. [pcc:unverified]

## 7. What two fixed anchors actually generate

Write \(I_A(R)=A-R\), \(I_B(R)=B-R\), and \(\Delta=A-B\). Then

\[
I_A I_B(R)=R+\Delta.
\tag{9}
\]

If \(A\ne B\), the nonzero \(\Delta\) generates the prime-order subgroup G. Any nonempty finite support closed under both involutions is therefore all of G: closure gives invariance under translation by \(\Delta\), and the translation orbit has length r. This falsifies the narrow proposal of a small exact support closed under two distinct tagged subgroup reflections. It is not a nonexistence result for truncated circuits, varying anchors, or other maps.

For words of exact length s in these two involutions, cancellation of adjacent equal involutions leaves an alternating word. There are at most s+1 distinct actions, not \(2^s\). More explicitly:

- At depth \(s=2h\), the states lie in \(\{R+k\Delta:-h\le k\le h\}\).
- At depth \(s=2h+1\), they lie in \(\{A-R+k\Delta:-(h+1)\le k\le h\}\).
- At all depths up to s, there are at most \(2s+1\) actions.

Actual point or negation-orbit collisions can reduce these counts further.

There is also a global selector-degree obstruction. The two reflections generate a dihedral group of \(2r\) automorphisms of E. A nonconstant rational map invariant under both has degree at least \(2r\), because these automorphisms fix its function field. If it has the form \(F(x(R))\), its degree on E is \(2\deg F\), so \(\deg F\ge r\). This concerns global invariance under both fixed reflections, not merely interpolation on selected sample points.

### Bare S3 correspondences have a different, still polynomial support

The equation \(S_3(x_R,x_{R'},x_A)=0\) permits \(x_{R'}=x(R\pm A)\); it does not specify only the oriented reflection \(A-R\). With two fixed anchors and s such steps, reachable x-values lie in

\[
\{x(R+jA+kB):|j|+|k|\le s,\ j+k\equiv s\pmod2\}.
\]

There are \((s+1)^2\) integer index pairs before group or x-coordinate collisions. To count them, use \(u=j+k\), \(v=j-k\): each ranges from -s to s with the parity of s. Consequently a claim of exponential distinct support from two fixed S3 anchors must account for these repeated branches.

### Useful and useless collisions

Known edge identities and collisions between identical symbolic normal forms have zero tag coefficients. A collision between genuinely distinct normal forms can still be useful. For example,

\[
2R=A+k\Delta
\]

can give \(\beta\ne0\). Finding such k in a declared range is an interval-DLP/orbit-intersection task. Encoding it compactly does not prove it can be solved cheaply. Do not discard this possibility as impossible; require an actual extractor and charge its search. [pcc:unverified]

## 8. Positive next research task, with explicit novelty and ownership gates

**Unproved research hypothesis:** simultaneously co-design a public tagged source, richer exact selector fibers, and an extractor that rejects or solves whole index classes without enumerating their members. A success must go beyond concatenating the translated-pair identities of Section 6. [pcc:unverified]

The following known constructions are controls, not new algorithmic claims:

- Translated quotient pairs above: verified cheap correlated tag transport, with zero useful rank from their defining identities.
- Four independently offset progression arms \(R_{j,k}=S_j+[k]\Delta_j\), with tags \(\tau(S_j)+k\tau(\Delta_j)\). A bit-and-point-addition circuit gives compact membership and exact tags; it is a generic interval/knapsack encoding, not a new relation-finding speedup. Existing work #4311 already addresses fixed-slope/interval ideas, and #5091 owns enumerated recursive joins. Do not launch another pair-table study. [pcc:unverified]

One concrete candidate requiring investigation is **adaptive high-incidence field involutions** fitted to a frozen public tagged pool. A nondegenerate projective involution has paired abscissae satisfying

\[
cuv-a(u+v)-b=0,\qquad a^2+bc\ne0,
\]

or \(v=(au+b)/(cu-a)\). For c nonzero, \(u+v=(cu^2+b)/(cu-a)\) supplies a rational degree-two quotient; if c=0, use the invariant product uv instead, since the sum is constant. Poles and fixed points require explicit projective treatment. A field involution is not automatically a curve automorphism or a public tag transporter. Both endpoints must already have verified tags or be generated by a separately proved tag-transport mechanism. [pcc:unverified]

The falsifiable synthesis gate is to produce a depth-two-or-greater shared selector with more than two tagged eligible roots in its relevant terminal fibers, while certifying the entire finite rational support and every tag. Record whether the support is complete or masked. Charge all pair generation, fitting, inversions, scans, rejected candidates, and output matching. Adaptive fitting must pass the family-capacity analysis in the separate checker; Section 2 alone does not cover it. A four-point interpolation example or a list of known pair identities is only a construction control.

The subsequent extraction gate is stricter:

1. Freeze source revisions, the source-generation law, public input suite, selector family, output semantics, seeds, and resource caps before testing. Use M31 throughout. No new random run is authorized by this note alone; it belongs in the next prospectively frozen manifest.
2. Use the verified reflection-pair control and the independently replayed archived useful APS relation to test both rejection of \(\beta=0\) identities and acceptance of genuine \(\beta\ne0\) relations.
3. Give the candidate only public curve points, public tags, and its selector description. Do not pass planted indices or the verifier's DLP scalar.
4. Require a checkable record of skipped index classes: for example an exact algebraic inconsistency certificate for a branch, or a proved map that solves a whole class at once. Merely iterating classes and their members is enumeration.
5. For a first bounded implementation gate, propose caps of at most N=32768 input tagged orbits, at most 4N new group additions, at most 128N base-field multiplications, and 256 MiB working memory per instance. Count inversions, roots, comparisons, and allocations separately as well; these multiplication/addition caps are not a complete timing model. The actual next manifest may tighten them against the already owned baseline. These are feasibility caps, not a claimed winning budget.
6. Return at least one new verified public \(\beta\ne0\) relation in the predeclared suite and report every attempted instance, restart, and failure. Replaying the archived relation is a validation control, not new extraction success.
7. Only after this gate passes, compare useful output per fully charged work and complete cold-solving cost with the source-exact existing collector and optimized negation-map rho. Use the owner-provided #5091 comparison rather than duplicating that study. If input generation already consumes the available rho budget, record that failure even if extraction itself is fast. [pcc:unverified]

Passing synthesis without public extraction establishes a representation, not a faster solver. Passing extraction without a complete cost advantage establishes a working mechanism, not an ECDLP speedup. A negative result should identify which exact condition failed so that the next cycle can change one mathematical ingredient instead of repeating the same audit.
 [pcc:unverified]