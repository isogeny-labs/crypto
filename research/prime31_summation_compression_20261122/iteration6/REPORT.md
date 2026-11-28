# Prime31: quadratic chains, exact pencils and available fibres

**Research checkpoint | 22 November 2026, America/Los_Angeles | PRs #4928 and #4962 merged; draft PR #4977**

The candidate now has a precise higher-arity representation: an oriented S3
chain joined to compact Dickson membership ladders. Every equation is at most
quadratic. On the stated rational affine domain, the full Dickson fibre gives
exactly the same signed witnesses as its liftable subset. A freely ordered
left chain is complete for unordered signed multisets of up to n points in a
prime-order group. [M1, M2]

The next task is to solve this compact system, including empty targets, at a
lower complete cost. Equation degree and input size alone do not establish
that bound. The new pencil analysis identifies the exact zero-component
query that a faster contraction must answer. The smooth-modulus refinement
supports every fixed factor-base exponent below 5/12 on a chosen field
family, including the optimal range of the stated conditional m=4 model.
Neither refinement supplies the missing sublinear query algorithm. [P1, F4]
[pcc:unverified]

**Three dedicated original-lock builds have stopped on inherited syntax
defects before native tests ran.** The latest run passed the prior parser
locations and failed in `binary_hyperelliptic_index_calculus.rs`; all five
independent command groups were attempted, but none executed the new native
programs. Mathematical deductions and written reviews remain
**[pcc:unverified]**. [E1, E4, M1, M2]

## 1. Inputs and the exact domain

| Item | Specified input or condition |
| --- | --- |
| Field | p = 2147483647 = 2^31 - 1 |
| Curve | y^2 = x^3 + 847514846 x + 2087444724 |
| Recorded group order | n = 2147446831, prime and odd |
| Root | A specified rational point R = (r,s), different from O |
| Membership | F(X) = D_N(X) - c, N = 2^k; F split and squarefree over Fp |
| Tree | Every auxiliary abscissa is finite and belongs to Fp |

The curve/order inputs are inherited from the lane. This iteration supplies
no new point-count certificate. Write W for the liftable fibre coordinates,
B = |W|, and H(X) = product over w in W of (X-w). The signed point base A
contains both points above each w, so |A| = L = 2B. The point at infinity O is
excluded. Odd group order makes every finite rational ordinate nonzero. [M1]

The result distinguishes three objects: a connected S3 tree whose auxiliaries
are actually in Fp; a polynomial ideal with coefficients in Fp but unrestricted
roots; and a projected higher summation polynomial. Only the first object
inherits rational-tree purity without further field restrictions. [M1, R1]

## 2. All-node orientation keeps the equations quadratic

For each internal vertex v, let its children have abscissae x_l and x_r.
Introduce a slope lambda_v; introduce y_v only for nonroot internal vertices.
The root coordinates are the constants (r,s). Impose

\[
\lambda_v^2=x_l+x_r+x_v,\tag{Q1}
\]

\[
2y_v\lambda_v=(x_l-x_v)(x_r-x_v)-(3x_v^2+a).\tag{Q2}
\]

For each nonroot internal child w of v, propagate the ordinate through

\[
y_w=\lambda_v(x_w-x_v)-y_v.\tag{Q3}
\]

The same expression recovers a leaf's ordinate after solving. Separate cubic
curve equations for every vertex are unnecessary: the factorization on the
next page proves curve membership inductively from the actual root. [M1, M2] [pcc:unverified]

At each leaf set z_(i,0) = x_i and apply k steps of z -> z^2 - 2, ending at c.
For k >= 1 this uses k equations and k-1 new variables per leaf. It enforces
D_(2^k)(x_i) = c without expanding a degree-N polynomial. [M1]

| Component | Variables | Equations |
| --- | ---: | ---: |
| Nonroot abscissae | 2m-2 | 0 |
| Nonroot internal ordinates | m-2 | m-2 |
| Slopes and their two constraints | m-1 | 2m-2 |
| Dickson intermediates | m(k-1) | mk |
| **Total** | **mk+3m-5** | **mk+3m-4** |

These counts hold for m >= 2 and k >= 1, with solutions sought over Fp. The
input construction uses O(mk) field-sized data given the valid curve and
fibre. Selecting or validating those inputs remains setup. The m=1 case is
direct target membership; N=1 is a separate linear membership case. [M1, M2]

## 3. Why full-fibre witnesses become rational

Let R=(r,s), put u=x-r and v=z-r, and suppose S3(x,z,r)=0 over Fp. Define

\[
\lambda=\frac{uv-(3r^2+a)}{2s},\qquad
P=(x,\lambda u-s),\quad Q=(z,\lambda v-s).
\]

The centered S3 identity yields lambda^2 = x+z+r. With f(r)=s^2,

\[
f(r+T)-(\lambda T-s)^2=T(T-u)(T-v).
\]

Thus the line meets the curve at -R, P and Q, with multiplicities, and P+Q=R.
The formula divides only by 2s. It covers x=z and coordinates equal to r,
including tangent cases. The parent point and two child abscissae determine
the oriented children uniquely. [M1, M2, E2]

Start at the specified rational root. The local formula gives rational
children. Odd rational group order keeps their ordinates nonzero, allowing
the same step at every internal child. All leaves are rational, and all
signs agree across edges. Therefore, as sets of Fp-valued tree assignments,

\[
V_{\mathbb F_p}(S_3\text{ tree},F(x_i))
=V_{\mathbb F_p}(S_3\text{ tree},H(x_i)).
\]

Q1-Q3 produce a bijective augmentation of these assignments. This is the
reason full Dickson membership can replace the expanded liftable polynomial
throughout the tree. [M1, M2]

### A projected S4 can still have a ghost

Take a nonliftable fibre coordinate w and a rational fibre point R with
abscissa r. Geometric points P,-P,R have total R, so S4(w,r,w,r)=0. But
q(U)=S3(w,r,U) has nonsquare discriminant 16 f(w) f(r): its two auxiliary
roots lie in Fp^2, and neither lies in Fp. Eliminating q against itself gives
zero; the finite-Fp chain rejects this tuple. Coefficients in Fp do not
replace a rational-root condition. [M1, E1]

## 4. Completeness belongs to the freely ordered left chain

For m >= 3 use auxiliaries q_j for prefix sums, with

\[
S_3(x_1,x_2,q_2)=0,\quad
S_3(q_{j-1},x_j,q_j)=0,\quad
S_3(q_{m-1},x_m,r)=0.
\]

The middle equation has 3 <= j <= m-1. For m=2 there is only
S3(x_1,x_2,r)=0. A particular ordered signed tuple appears in this affine
chart when its internal prefix sums are nonzero. [M1]

**Ordering lemma.** In a prime-order group of order n, any multiset of
1 <= m <= n nonzero points with nonzero total admits an order with every
prefix nonzero. Work backward: with t >= 2 remaining points and total S,
remove a point different from S. If no such choice exists, all t points
equal S, forcing (t-1)S=O. This contradicts 1 <= t-1 < n. The remaining
total stays nonzero; reverse the removals to obtain the order. [M1, M2]

Repeated points and cancelling pairs are covered. The result is complete for
unordered signed multisets of exactly m points, with free leaf slots. A
fixed sorted order can lose them. Enumerating every ordered tuple needs
permutation expansion or explicit identity-intermediate branches, with
their output cost charged. [M1, M2]

![A left chain accepts P,P,P,-P while every balanced pair tree has a zero child.](figures/rational-chain-topology.svg)

For prime n >= 5, P,P,P,-P totals 2P. The left-chain prefix sums P,2P,3P,2P
are nonzero. Every balanced two-pair partition puts -P with a P and therefore
contains an identity child. The theorem does not extend to an arbitrary fixed
balanced topology. The m <= n uniform guarantee is sharp: n+1 copies of P
have a zero prefix of length n in every ordering. [M1, M2]

## 5. Field guards and the complete-cost gate

For closure-based elimination, constrain the m-2 internal abscissae to Fp.
Leaf abscissae already lie there because F splits; the oriented equations
then force slopes and internal ordinates into Fp. [M1, M2]

At p=2^31-1, introduce h_1,...,h_30 for each internal x and impose

\[
h_1=x^2,\quad h_j=h_{j-1}^2\ (2\le j\le30),\quad h_{30}^2=h_1.
\]

The eliminated equation is x^(2^31)-x^2 = x(x^p-x), whose zero set is exactly
Fp. This adds 30 variables and 31 quadratic equations per internal x:

\[
\text{variables}=mk+33m-65,\qquad
\text{equations}=mk+34m-66.
\]

Zero has multiplicity two. Use zero-set and witness semantics, not scheme
length or raw multiplicities. A squarefree eliminated x^p-x guard uses a
split square/product circuit with 59 variables and 60 equations per internal
x. A native Fp solver needs neither added guard. [M1, M2]

### Assigned-point extraction is explicit; root solving remains charged

Per internal node, direct oriented extraction costs at most

\[
I_p+4M_p+S_p+6A_p+2C_p,
\]

where I, M, S, A and C denote inversion, general multiplication, squaring,
addition/subtraction, and constant scaling. A whole tree has m-1 internal nodes.
Membership evaluation adds mk squarings and mk subtractions. Independent
signed curve replay, row normalization, deduplication and output remain
additional. Writing z rows requires at least Omega(mz) point labels. [M1, M2]

The complete cold cost retains parameter/fibre setup, system construction,
solving, rational root isolation, extraction and output. The unresolved term
is the solver, especially an exact empty-target certificate. [M1]

An exact shared-state alternative counts valid prefixes by
f_j(S)=1_(S!=O) sum_(P in A) f_(j-1)(S-P). A sparse explicit build extends
each supported state by L choices, costing Theta(L M_(j-1)) group/hash
operations per level. Storing every prefix witness still takes Theta(L^j)
entries for fixed j; aggregate states can be smaller, and their memory and
query-edge regeneration must be priced. The quadratic representation avoids
constructing that graph as input. A faster exact contraction remains to be shown. [M1, M2] [pcc:unverified]

### A quantitative target for the next solver

In the conventional uniform-target/full-independent-row model, write setup as
B^sigma, an empty-inclusive target query as B^tau, and sparse linear algebra
as B^2. Set alpha=max(sigma,2). The conditional complete exponent is
alpha/(alpha+m-1-tau), when m-1>tau and the balancing range is admissible.
Beating one half within this model requires alpha+tau<m-1. Thus for m=4 and
sigma<=2, the concrete missing query bound is **tau<1**. This is a
model-specific necessary budget, not an established algorithm or an ECDLP
lower bound. [R2]

## 6. Evidence, prior art, and the next discriminating action

| Evidence | Current standing |
| --- | --- |
| Mathematical note | Written derivation, independently reviewed; [pcc:unverified] |
| Frozen correctness source | Four new test functions and imported original tests prepared; none of the three builds reached tests |
| Original-root-lock CI | Three retained inherited-source compile failures; latest source 14a9f021b630fec0a7dd38cb724d71d68c99626b failed in binary_hyperelliptic_index_calculus.rs |
| Centered S3/discriminant records | Existing native evaluated-tier certificates, proof:null |
| New cubic-residual identity | First two builds skipped issuer/replay; third issuer compilation failed and replay was not reached; no new certificate claimed |
| Performance | No new timing panel or asymptotic solving bound |

[CI run 45096187034](https://github.com/isogeny-labs/crypto/actions/runs/45096187034),
job 137436389923, failed while compiling the original root crate. The
inherited `ecbench/spec.rs` test-fixture section around lines 894-1042 has
malformed calls and unclosed delimiters. This stopped execution before the
new example tests and before native cubic-residual identity issuance. The
failure is a build dependency, not a mathematical counterexample. A minimal
history-based restoration is committed at
[a4ddc035c0c7519d6bab7a438d8a168f74ae40db](https://github.com/isogeny-labs/crypto/commit/a4ddc035c0c7519d6bab7a438d8a168f74ae40db): 289 inserted lines preserve both parent policy implementations and all 15
distinct parent tests. The
[original-lock rerun 45097885135](https://github.com/isogeny-labs/crypto/actions/runs/45097885135)
failed in job 137441679059 before any test ran. Its compiler error is an
unclosed delimiter in inherited `src/cryptanalysis/ic_framework/mod.rs`,
reported at line 4528. Candidate and identity steps were skipped. The prior
ecbench restoration was retained; this second failure does not establish a
failed candidate test. The candidate arithmetic examples, original manifest
and lock were unchanged in these builds. [E1]

Artifact 14784865238 was downloaded and its ZIP SHA-256
`915598d695abeb8f35368527093cd1be101db1c0f457a9aeda2c44707aaf1693`
was verified. The complete source-bound
[second failure receipt](evidence/ci-restored-spec-failure/receipt.json),
source identities, compiler output and compressed job log are retained
alongside the first failure. The current-main follow-up published an exact
four-line restoration of the remaining framework guard, with zero deletions.
Its baseline already contains the Cargo table-header and eleven-line boundary
repairs, plus the newer spec additions; those files are preserved. The same
original Cargo.lock and candidate arithmetic remain in use. [E3] [pcc:unverified]

The separate general-PR run
[45100829948](https://github.com/isogeny-labs/crypto/actions/runs/45100829948)
failed at manifest parsing. Its actual checkout was the synthetic merge
`4a40be161b9b3e6bc4db8a8dc16864e82d754d7b`, whereas the run head was
`fc184b03d47e13ec922c9b489f3b3f8a976a8bd9`. Its duplicate-key error is
explained by three missing Cargo array-table headers, already repaired by
[69fad1271054225a95d5682f5fdcd11fd2a692d7](https://github.com/isogeny-labs/crypto/commit/69fad1271054225a95d5682f5fdcd11fd2a692d7).
This general-PR failure is outside the three dedicated rational-chain attempts
and supplies no new candidate verification. The manifest audit checks exact
source and TOML parsing, not a successful Cargo build. [E3] [pcc:unverified]

The third dedicated run used the frozen source at
[14a9f021b630fec0a7dd38cb724d71d68c99626b](https://github.com/isogeny-labs/crypto/commit/14a9f021b630fec0a7dd38cb724d71d68c99626b).
[Run 45113871392](https://github.com/isogeny-labs/crypto/actions/runs/45113871392),
job 137492117014, completed with failure at 04:56:20 UTC. The inherited
`src/cryptanalysis/binary_hyperelliptic_index_calculus.rs` produced mismatched
and unclosed delimiters at lines 640 and 1233; the compiler also reported 198
further unclosed delimiters. The failure occurred before test binaries or the
new native programs could execute. [E4] [pcc:unverified]

All five independent command groups were attempted after source capture;
each stopped at compilation. The identity issuer therefore did not execute,
and its subsequent replay command was not reached. This is distinct from the
skipped candidate and identity steps in the first two builds. Artifact 14790654045
retains the source hashes, compiler outputs and empty program-output files;
its ZIP SHA-256 is
`b1b6d310eab134128f525529eded863ac4f6147b08c1f66037d18ab3c4f7c104`.
**Zero library or new candidate tests executed in these three dedicated
builds.** A failing compilation is not an observed failed native verifier test.
[E1, E4] [pcc:unverified]

The frozen correctness panel uses the existing depth-six N=64/B=31 fibre,
the explicit Fp^2 ghost, repeated and corrupted certificates, the fixed-order
identity-prefix failure and its repair, the balanced-tree counterexample,
and all 64 ordered triples plus 256 ordered quadruples on a fixed four-point
signed base. These are specified controls, not yet reported observations.
The source and commands are retained in SOURCE_FREEZE.json. [E1]

Semaev 2015 already gives ground-field S3 chains and a related minimality
lemma. Existing native code already has Dickson ladders and chord/tangent
orientation of a final pair. This continuation adds the scoped all-node
representation, exact field restrictions and free-permutation completeness
accounting; literature-wide originality is unestablished. [L1, R1]

**Next action:** preserve the third source-bound failure and restore the
separate whole-crate prerequisite without weakening any check. For the
mathematical gate, construct an exact zero-component index for the actual
Dickson pair spectrum, with complete misses, charged setup, rational recovery
and signed output. The precise requested query bound appears in Section 8.
A fixed prime is a benchmark; an exponent claim needs the growing-family and
source-group assumptions stated in Section9. [M1, P1, F4]

### Sources and provenance

- **M1:** [MATHEMATICAL_NOTE.md](https://github.com/isogeny-labs/crypto/blob/a4ddc035c0c7519d6bab7a438d8a168f74ae40db/research/prime31_summation_compression_20261122/iteration6/MATHEMATICAL_NOTE.md), unchanged proof note;
  SHA-256 `948d36ca3d72661aa4cf16786ebed30fcb2ba146afb0df4420767a91e8f35ddc`.
- **M2:** [INDEPENDENT_REVIEW.md](https://github.com/isogeny-labs/crypto/blob/a4ddc035c0c7519d6bab7a438d8a168f74ae40db/research/prime31_summation_compression_20261122/iteration6/INDEPENDENT_REVIEW.md), independent written
  mathematical audit. This is not a formal proof or signed PCC certificate.
- **E1:** [SCOPE.md](https://github.com/isogeny-labs/crypto/blob/a4ddc035c0c7519d6bab7a438d8a168f74ae40db/research/prime31_summation_compression_20261122/iteration6/SCOPE.md), [SOURCE_FREEZE.json](https://github.com/isogeny-labs/crypto/blob/a4ddc035c0c7519d6bab7a438d8a168f74ae40db/research/prime31_summation_compression_20261122/iteration6/SOURCE_FREEZE.json), and
  [report-validation.json](report-validation.json). Source checkpoint:
  [bf56de345c02ef6f1c55bfa047087b0e880cb09e](https://github.com/isogeny-labs/crypto/commit/bf56de345c02ef6f1c55bfa047087b0e880cb09e).
- **E3:** [Follow-up source freeze](evidence/followup-main-freeze/SOURCE_FREEZE.json),
  the [current-main framework review](evidence/main-f0-framework-review/receipt.json),
  [boundary/spec review](evidence/main-f0-boundary-spec-review/receipt.json), and
  [manifest audit](evidence/main-f0-manifest-review/receipt.json). Baseline
  [f0ca664eca0081a30adfcc79dcf62a32de40d2af](https://github.com/isogeny-labs/crypto/commit/f0ca664eca0081a30adfcc79dcf62a32de40d2af);
  tracking commit
  [1650b344b9bd72237d9bafd26b3401da1385eafa](https://github.com/isogeny-labs/crypto/commit/1650b344b9bd72237d9bafd26b3401da1385eafa).
- **E4:** Third dedicated original-lock run 45113871392 / job 137492117014 at
  source 14a9f021b630fec0a7dd38cb724d71d68c99626b;
  [third failure receipt](evidence/ci-framework-restored-failure/receipt.json)
  and source-bound artifacts.
- **E2:** Existing [centered S3](../identity-certificates/centered-s3.json)
  record `IDC1hf62e2402e4871a82` and
  [discriminant](../identity-certificates/s3-discriminant.json) record
  `IDC1h869923a46a34c70c`. These certify evaluated identities at their
  stated scope; they do not certify the tree/completeness theorems.
- **R1:** The prior native example
  `research/prime31_ic_20261115/native/examples/prime31_m8_oriented_f4.rs`,
  blob `e65aff0b1ee1166aefdfd79775f1c4bc2f695a16`; bounded sibling source
  audit and original lane notes, as identified in M1.
- **L1:** Igor Semaev, *New algorithm for the discrete logarithm problem on
  elliptic curves* (2015), [arXiv:1504.01175](https://arxiv.org/pdf/1504.01175),
  Section 3 equation (4), Section 4.1 Lemma 2, Section 4.2.
- **R2:** [ROOT_REVIEW.md](https://github.com/isogeny-labs/crypto/blob/a4ddc035c0c7519d6bab7a438d8a168f74ae40db/research/prime31_summation_compression_20261122/iteration6/ROOT_REVIEW.md), the scope/provenance integration
  review and conventional full-rank exponent budget.

The research work in [PR #4928](https://github.com/isogeny-labs/crypto/pull/4928)
and the source-restoration follow-up
[PR #4962](https://github.com/isogeny-labs/crypto/pull/4962) have merged.
The latter merge was observed at 04:57:27 UTC, commit
`b0a8d8b0a7ec3bf3469589dda7010d5612c9f70a`. Its merge does not establish a
passing original-root build; the third dedicated run failed one minute earlier.
[E4] [pcc:unverified]
The [merged research directory on main](https://github.com/isogeny-labs/crypto/tree/main/research/prime31_summation_compression_20261122)
and the previously confirmed
[research Space](https://chatgpt.com/space/page_729b0b80bdb4819193f457c80ecabe84)
retain the research history. The PDF is a nine-page account of the quadratic
representation, exact pencil and growing-family refinements; previous pinned
reports and evidence remain unchanged.

**PCC records:** representation and completeness `PCC-20261123-ed96d7`;
scoped bucket/rank and conditional exponent budget `PCC-20261123-36ec87`;
literature/source audit `PCC-20261123-184d4e`; second dedicated failure
`PCC-20261123-774115`; current-main source/parser reconciliation and freeze
`PCC-20261123-a41174`. All remain unverified. These
records are claim tracking, not signed mathematical or execution verdicts.


## 7. Growing fields and available rational fibres

The family and density refinements supply inputs for the quadratic strategy.
They leave the target-solving term in Section 5 unresolved. All results in
this section are written deductions with **[pcc:unverified]** standing.

### A growing smooth p+1 family

Let Q=2^k and choose the least prime p congruent to -1 modulo 4Q. Linnik's
theorem gives p≤C(4Q)^L for absolute constants, hence Q=Ω(p^(1/L)). This
constructs a growing family without assuming infinitely many Mersenne primes.
Take L≥2 and let q be the largest power of two at most min(Q,√p/16). Then

\[
4q\mid p+1,\qquad q=\Omega(p^{1/L}),\qquad q\le\sqrt p/16.
\]

For a norm-one torus element z and a torus element ζ of order q, put
c=z^q+z^(-q). If c≠±2, the q trace coordinates

\[
x_j=z\zeta^j+(z\zeta^j)^{-1},\qquad 0\le j<q,
\]

are exactly the distinct rational roots of D_q(X)-c. Enumerating them takes
Θ(q) extension-field operations and output slots. A full torus generator is
unnecessary; its required power-of-two part can be selected and certified
separately. Prime finding and input certification remain setup costs. [F1, F2]

### Fix the fibre and select the curve

For any fixed N-element set F⊂Fp, uniform coefficient pairs (a,b) give pairwise
independent values of x^3+ax+b at distinct x. If B counts liftable coordinates,
including zeros at this unrestricted stage,

\[
\mathbb E B=\frac{N(p+1)}{2p},\qquad
\operatorname{Var}B=\frac{N(p^2-1)}{4p^2}.
\]

Exactly (p^2−1)/3 monic depressed cubics are irreducible. Combining this
count with a quadratic-character second-moment bound, without assuming
independence after conditioning, gives a density greater than 1/12 of
coefficient pairs whose cubic is irreducible and N/4≤B≤3N/4 when N≥16.
These curves are nonsingular, of odd order, and ordinary. [F3]

For the symmetric full Dickson fibre and p≡3 mod4, the involution
(a,b)↦(a,−b) preserves that balance and exchanges opposite Frobenius traces.
At least one of each pair is non-anomalous. A constructive check uses any
nonzero rational lift P: for p≥7, Hasse's bound implies [p]P=O exactly when
#E(Fp)=p. Switch to the twist when needed. Screening takes expected
O(N log p) field operations, with an O(log p) group-operation anomaly check.
These are written cost bounds, not measurements. [F3]

For fixed m, remove permitted leaves of order at most m−1. This discards only
O(m^3) points and preserves a Θ(N) base as N grows. The backward-peeling proof
then applies, even if the whole curve group is composite. The small-order
exclusion needs additional membership guards; it is **not included in the
frozen iteration-six equation counts**. This selection changes the curve
and does not supply a DLP-preserving transfer from a prescribed input. [F3]

### Fix an odd-order curve and select the fibre

For any already specified nonsingular curve E/Fp of odd rational order,
let c range uniformly over the regular full q-fibres of the norm-one torus
trace map. Write B(c) for the number of its rationally liftable coordinates.
There are M=(p+1)/(2q)−1 such fibres. An explicit character-sum calculation gives

\[
\left|\mathbb E_c\frac{B(c)}q-\frac12\right|
\le\frac{3\sqrt p/2+q/2+1}{(p+1)/2-q}.
\]

For p≥4096 and 4≤q≤√p/16, the bound is at most 1/8, so at least **1/6 of
regular fibres have B(c)≥q/4**. Uniform norm-one seeds, conditioned on
c≠±2, sample these fibres uniformly because each has exactly 2q seeds.
The expected number of seed trials is less than seven, including exceptional
seeds and rejected regular fibres. Enumerating and screening therefore finds
a suitable rational base in expected O(q log p) field operations plus charged
inversions, after the field and curve are supplied. Bit cost includes the
chosen field-arithmetic cost. [F2]

This second route keeps the original curve fixed. If that curve already has
prime order, all its rational points belong to the required group. The
result does not establish a growing prime-order curve family or a comparable
density inside a prescribed proper subgroup. Cofactor clearing can change
abscissae and destroy Dickson membership. Neither density result proves
independent relation rank, an efficient empty-target oracle, or an asymptotic
relation-solving speedup. [F2, F3] [pcc:unverified]

### Family and availability sources

- **F1:** [SMOOTH_TORUS_FAMILY.md](../iteration7/SMOOTH_TORUS_FAMILY.md),
  the retained growing-family argument. Its initial book-title attribution
  is corrected in F2: the source is Koukoulopoulos, *The Distribution of
  Prime Numbers*, Chapter 27, Theorem 27.1,
  [author's preliminary edition](https://dms.umontreal.ca/~koukoulo/documents/publications/primes.pdf).
- **F2:** [INDEPENDENT_REVIEW.md](../iteration7/INDEPENDENT_REVIEW.md),
  the independent family audit, and
  [FIXED_CURVE_FIBRE_DENSITY.md](../iteration7/FIXED_CURVE_FIBRE_DENSITY.md),
  the fixed-input-curve density refinement with the degree-five Weil
  character-sum argument; SHA-256
  `c1725df2f00f61a972edbb5cfee803fe97bd2c27d8f761fbb08b73385c70b7a1`.
- **F3:** [SELECTABLE_CURVE_AVAILABILITY.md](../iteration7/SELECTABLE_CURVE_AVAILABILITY.md),
  the exact moment, irreducible-cubic, twist-selection and small-order-leaf
  arguments; SHA-256 `245693a861b7f40f687045ffee79637ce3b6251d29c05595f46718894d9c8da6`.
  Ordinary prime-field classification: Waterhouse, *Abelian varieties over
  finite fields* (1969), Theorem 4.1,
  [primary archive](https://www.numdam.org/item/ASENS_1969_4_2_4_521_0.pdf).

These notes contain the full proofs and the costs excluded from their scope.
The fixed-source density result is tracked by `PCC-20261123-2d6d3e`; the
selectable-curve result by `PCC-20261123-0d1e17`. The sixteen claims covered by this report
at core `d6e1d03aff64ff168bd1d1d565a08c8f5032ef62` remain unverified. No new performance experiment or signed formal
proof accompanies this appendix.

The Space and progress Page were previously confirmed. The latest update
attempt has an unconfirmed outcome after RPC hangs; a subsequent browser
check was aborted without completing access. This report does not assert
that the newest content was saved there.

## 8. The exact reusable S3 pencil

This section uses a **new pair-spectrum polynomial H**, distinct from the
liftable-leaf polynomial in Sections 1–5. Let A be the sign-closed leaf
point base, U=(A+A)\{O}, X=x(U), D=|X|≥1, and
H(V)=∏_{u∈X}(V−u). Then |U|=2D, D≤B², H is split and squarefree, and
f(u) is a nonzero square for every u∈X. The inputs lie on the stated
nonsingular curve over Fp with p>3 and in an odd-order rational subgroup.
Let R≠O be the rational target and r=x(R). [P1]

### A rank-three matrix with a degree-D column product

For w(t)=(1,t,t²)^t, the usual short-Weierstrass S3 identity is

\[
S_3(u,v,r)=w(u)^t C_r w(v),\qquad
C_r=\begin{pmatrix}
a^2-4br&-2ar-4b&r^2\\
-2ar-4b&-2(r^2+a)&-2r\\
r^2&-2r&1
\end{pmatrix}.
\]

Thus M_r=(S3(u,v,r))_{u,v∈X}=W_X C_r W_X^t has ordinary rank at most
three. Define the homogeneous product

\[
\Phi_X(Z_0,Z_1,Z_2)=\prod_{u\in X}(Z_0+uZ_1+u^2Z_2)
=\operatorname{Res}_U(H(U),Z_0+Z_1U+Z_2U^2).
\]

The exact quotient-algebra contraction is

\[
K(T,V)=\prod_{u\in X}S_3(u,V,T)\bmod H(V)
=\Phi_X(C_Tw(V))\bmod H(V).
\]

For R≠O, a four-leaf relation is absent exactly when **r∉X and every
component K(r,v), v∈X, is nonzero**. The branch r∈X represents a relation
with a zero pair subtotal and must not be discarded; R=O is separate.
Low ordinary rank supplies the displayed representation, but taking each
column product is a degree-D operation. It does not follow that all its
zeros can be found in time bounded by the matrix rank. [P1]

### Exact degree, with the format limitation kept explicit

In the split algebra R_X=Fp[V]/(H),

\[
\deg_T K=2D-1,\qquad
[T^{2D-1}]K=-4f(V)H'(V)^2\pmod H.
\]

At a CRT component V=v, the diagonal factor is
S3(v,v,T)=−4f(v)T+(v²−a)²−8bv, of degree one; each of the other D−1
factors has degree two and leading coefficient (u−v)². This gives the
formula above. Its leading coefficient is a unit because f(v)≠0 and
H'(v)≠0 for every component. The statement is about **formal degree**;
reduction modulo T^p−T can change a representation of the finite-field
function. [P1] [pcc:unverified]

A fully materialized rectangular monomial table therefore allocates
**2D² field-element slots**. This is a cost of that dense format, not a
proof that every slot is nonzero, independent, or intrinsically necessary.
The O(D)-factor expression for Φ_X already gives a compact description.
The scalar product Δ(T)=Res_V(H(V),K(T,V)) has formal degree D(2D−1),
so a full dense scalar table would allocate 2D²−D+1 slots. Neither count
rules out a specialized recurrence or a data structure depending on X.
[P1]

There is a separate, precisely scoped fixed-basis observation. If p≥D+2,
the forms Φ_X for **all** D-subsets X⊂Fp span the homogeneous ternary
forms of degree D, a space of dimension binomial(D+2,2). To see this,
choose a set T of D+2 field elements and evaluate the form for
T\{a,b} at (ab,−a−b,1). Its factors are (u−a)(u−b); among those forms,
only the one omitting {a,b} is nonzero. This diagonal interpolation proves
the span assertion without a factorial or characteristic assumption.
It restricts a universal X-independent **linear** basis covering arbitrary
subsets. It is not a lower bound for the actual Dickson pair-spectrum
subclass, an X-specific nonlinear circuit, or the quotient K with its
X-dependent modulus. [P1] [pcc:unverified]

### Rational discriminants do not remove the remaining search

For each fixed v and r,

\[
\operatorname{disc}_U S_3(U,v,r)=16f(v)f(r).
\]

Both factors are nonzero squares on this source domain. When v≠r the
quadratic has two distinct rational roots; when v=r it becomes linear
with nonzero coefficient −4f(r). A nonsquare-discriminant filter therefore
rejects none of these source components. The group law computes the roots
as x(R±T_v), but checking their membership in X remains necessary. [P1]

For r∉X the exact translated-divisor identity is

\[
\prod_{u\in X}S_3(u,V,r)
=H(r)^2\prod_{T\in U}\bigl(V-x(R-T)\bigr).
\]

This includes root multiplicities. It excludes r∈X, where a translated
point reaches O and the corresponding factor is linear: substituting
H(r)=0 into this affine product is invalid. A projective chart or separate
exception branch is needed there. [P1]

### The algorithmic target is now explicit

The next candidate lemma must build a target-independent index for the
**actual** X, with setup Õ(D^s), s≤1, and exact empty-inclusive query
Õ(D^θ+h), counting any requested output h. Existing factored polynomial
contraction and direct rational-complement scans have θ=1 up to logarithms.
The rank-three representation has not improved that exponent. [P1]
[pcc:unverified]

In the D=Θ(B²) regime, setup after the pair chart gives σ=max(2,2s),
query exponent τ=2θ, and α=max(σ,2). Under the stated conventional
uniform-target and sufficient-independent-row model, beating the
square-root exponent would require

\[
\alpha\beta<\tfrac12,\qquad
1-\beta(3-2\theta)<\tfrac12.
\]

For β>0 the second inequality first forces θ<3/2. Only then can it be
rearranged as 1/[2(3−2θ)]<β<1/(2α). With s≤1, hence α=2,
a nonempty interval requires **θ<1/2**, equivalently τ<1. This is a
conditional target, not an achieved relation-solving bound. A full dense
inverse or Bézout certificate would itself have Ω(D) output cost under
that output contract; a Boolean answer or succinct certificate is not
excluded by that observation. Sign recovery, witness multiplicities,
memory and signed-row output must still be charged. [P1] [pcc:unverified]

**P1:** [S3_PENCIL_CONTRACTION.md](https://github.com/isogeny-labs/crypto/blob/d6e1d03aff64ff168bd1d1d565a08c8f5032ef62/research/prime31_summation_compression_20261122/iteration10/S3_PENCIL_CONTRACTION.md),
SHA-256 `9e9e64e76208202d05957794e7bd9371f9e427b2a5dded9f88aceb3ca40588ed`,
and [S3_PENCIL_INDEPENDENT_REVIEW.md](https://github.com/isogeny-labs/crypto/blob/d6e1d03aff64ff168bd1d1d565a08c8f5032ef62/research/prime31_summation_compression_20261122/iteration10/S3_PENCIL_INDEPENDENT_REVIEW.md).
The independent review accepts the displayed identities and the corrected
positive-denominator condition within their stated scope. The complete
notes identify the prior-art polynomial-arithmetic results and the earlier
S3 discriminant record. Claim `PCC-20261123-f0209c` remains unverified.

## 9. A stronger field-family range from prior art

Chang's *Short character sums for composite moduli*, Corollary 11, gives
a prime p<Q^(12/5+o(1)) in each coprime residue class whenever the largest
prime factor ℓ of Q satisfies log ℓ=o(log Q). The primary proof explicitly
treats a possible exceptional real zero. This is an unconditional prior-art
input, not a new prime-distribution theorem. [F4]

Put Q_k=2^(k+2), and choose p_k≡−1 mod Q_k. Then

\[
Q_k-1\le p_k<Q_k^{12/5+\varepsilon_k},\qquad
\varepsilon_k\longrightarrow0.
\]

The lower bound gives p_k→∞, with a distinct-prime subsequence if needed.
Fix **0<β<5/12** and put N_k=2^floor(β log₂ p_k). Choose η>0 with
ρ=β(12/5+η)<1. Eventually

\[
\log_2N_k\le\beta\log_2p_k<\rho(k+2)<k.
\]

Consequently N_k divides 2^k and 4N_k divides p_k+1. Also
p_k^β/2<N_k≤p_k^β, N_k≤√p_k/16, N_k≥4 and p_k≥4096 eventually.
These satisfy every size and divisibility hypothesis of the fixed-source
fibre-density lemma. **The endpoint β=5/12 is not asserted.** [F4]

For an already supplied nonsingular odd-order source curve, that lemma
selects a full regular fibre with N_k/4≤B_k≤N_k. Thus B_k=Θ(p_k^β),
with expected O(N_k log p_k) basic field operations, charged inversions,
and O(N_k) storage. The curve remains an input hypothesis; the argument
does not construct a growing prime-order curve family or prove density
inside a prescribed proper subgroup. [F2, F4] [pcc:unverified]

### The conditional m=4 optimum lies inside the available interval

For setup exponent σ≤2, query exponent 0≤τ<1 and the earlier conventional
yield/rank assumptions, the budget is

\[
\widetilde O(B^\sigma+B^2+pB^{\tau-3}).
\]

Balancing its last two terms gives

\[
\beta_* =\frac1{5-\tau},\qquad
\gamma_* =\frac2{5-\tau}<\tfrac12.
\]

The entire optimal range 1/5≤β*<1/4 lies below 5/12. The extra restriction
τ<1/2 obtained earlier by fixing the guaranteed exponent at β=1/5 is
therefore unnecessary for this power-of-two construction. The requirement
τ<1 remains, and no such exact empty-inclusive query algorithm is supplied
by the correction. Relation yield, independent row rank and complete solver
costs remain separate assumptions. [F4] [pcc:unverified]

This describes a chosen prime-field family, not an arbitrary input prime
or a DLP-preserving change of the source curve. Prime existence does not
make prime finding free: enumerating tQ_k−1 up to the supplied bound may
inspect Q_k^(7/5+o(1)) candidates. This is an available search bound, not a
necessary lower bound. Primality tests, certificates, source-curve
construction and group-order certification belong in cold setup whenever
those objects are not supplied. [F4]

**F4:** [FAMILY_SIZE_CORRECTION.md](https://github.com/isogeny-labs/crypto/blob/d6e1d03aff64ff168bd1d1d565a08c8f5032ef62/research/prime31_summation_compression_20261122/iteration10/FAMILY_SIZE_CORRECTION.md),
SHA-256 `841704b12548a1ab4c2e35062cff9b48a8a9229dd83d2e7b3426a629501c996d`,
and [FAMILY_SIZE_INDEPENDENT_REVIEW.md](https://github.com/isogeny-labs/crypto/blob/d6e1d03aff64ff168bd1d1d565a08c8f5032ef62/research/prime31_summation_compression_20261122/iteration10/FAMILY_SIZE_INDEPENDENT_REVIEW.md).
Primary source: Mei-Chu Chang, *Short character sums for composite moduli*,
Corollary 11, printed pages 4 and 39,
[author's PDF](https://math.ucr.edu/~mcc/paper/143%20Char060114.pdf) and
[author-submitted version](https://arxiv.org/abs/1201.0299v2).
Claim `PCC-20261123-391a91` remains unverified. The general Linnik exponent
remains valid; the stronger applicable smooth-modulus hypothesis removes
an unnecessary parameter restriction.

## Current continuation and retained standing

[Draft PR #4977](https://github.com/isogeny-labs/crypto/pull/4977), branch
`lane/prime31-s3-pencil-family`, was opened from
`816f9c3a942efed569049618c056d12a0c56d693`; its tracking commit is
`c2c25f9799c3f5492ade4d638c7aa6a3900acbde`. It carries the final third-failure
evidence and the iteration-ten notes. The native build remains bound to
source `14a9f021b630fec0a7dd38cb724d71d68c99626b`, run `45113871392`,
job `137492117014`; the documentation continuation is not another native
execution. [E4]

The third failure is tracked by `PCC-20261123-5ce73e`. Together with the
new pencil and family claims, **the sixteen claims covered by this core report remain unverified**
(core `d6e1d03aff64ff168bd1d1d565a08c8f5032ef62`).
No new native test result, identity certificate, timing panel, novel prime
distribution theorem, or asymptotic relation-solving speedup is asserted.
The exact research obligation is now an efficient structured zero-component
index, with every miss and required output accounted for. [P1, F4]
[pcc:unverified]
