# Independent graded-joint review — TASK-N83-GRADED

The four assigned mathematical joints hold under the stated assumptions: a field of characteristic greater than 27, 27 selectors in each block, nonzero selector weights, and exactly the displayed homogeneous chain. I checked the primitive decomposition, proved injectivity on the relevant sector representatives, accounted for mixed square/quartic relations, and derived an explicit degree-17 boundary relation. The degree-16 argument has a positive one-exponent margin in its most restrictive sector. These are joint-level findings; this report does not cast a whole-claim vote.

Reviewer: `TASK-N83-GRADED`, parent archival task `T-995`, 2026-10-30. Producer revision supplied for this review: `c64f02c176787dbf40d455f95b75bd2dd5bfe5bf`. Preregistered plan revision supplied: `c92ca4f106a0`. I used only the assigned immutable graded snapshot, the permitted checker plan, and the two primary papers listed below. I did not read sibling reports or filtered/blind inputs, edit producer files, execute experiments, replay Lean, launch jobs, commit changes, or delegate work.

| Exact joint owned | Verdict | Mathematical basis |
| --- | --- | --- |
| block SLP and primitive decomposition | holds | The cited maximal-rank theorem applies, and the primitive complements give the claimed graded module decomposition. |
| tensor sector degree and no truncation | holds | The quotient map is injective on the rectangular monomial support containing every product in degrees at most 16. |
| free chain regularity/Koszul generation | holds | Pairwise coprime leading monomials give regularity, followed by homogeneous first-syzygy induction. |
| selector-square lifting and finite-characteristic FFD conventions | holds | Explicit degree-preserving lift removes quartic/quartic, then mixed, then square/square coordinates; the finite-field quotient is unchanged in the relevant degrees. |

## Grading and source template

Throughout this review every polynomial variable has ordinary degree one. A homogeneous relation of degree `d` on a generator `g_i` of degree `e_i` has coordinate degree `d-e_i`; zero coordinates are allowed. Thus square coordinates have degree `d-2`, and quartic coordinates have degree `d-4`. This is the shifted grading on the free syzygy module. The auxiliary weights used to select leading monomials are not the grading used in `d<=16`: the nonterminal quartics are not homogeneous for those auxiliary weights. Any downstream use of the plan's phrase “weighted total degree” should explicitly mean these generator shifts, not the monomial-order weights.

Write

\[
R=k[b_{i,a},U_j],\quad J=(b_{i,a}^2),\quad A=R/J,
\quad L_i=\sum_{a=0}^{26}b_{i,a}.
\]

If the original linear forms use weights `w_{i,a}`, the coordinate change `b_{i,a}=w_{i,a}x_{i,a}` is invertible exactly when every weight is nonzero. It changes the original highest squares into nonzero scalar multiples of `b_{i,a}^2`; the corresponding diagonal change of free-module basis preserves Koszul generation and degrees.

The chain consists of `m-1` quartics, for `3<=m<=9`. Set

\[
Q(x,y,z)=x^2y^2+x^2z^2+y^2z^2
 -2x^2yz-2xy^2z-2xyz^2.
\]

Then `F_0=Q(L_0,L_1,U_0)`, `F_j=Q(U_{j-1},L_{j+1},U_j)` for `1<=j<=m-3`, and `F_{m-2}=U_{m-3}^2L_{m-1}^2`. Empty index intervals are empty; at `m=3` there is just one nonterminal quartic and one terminal quartic. Direct expansion of `s3` in `prime_linear_source_donor.rs:15-16` yields this `Q`; making its third input the constant target leaves highest form `x^2y^2`. Lines 75–90 place these forms and the selector squares in exactly the stated chain pattern. This is static source inspection, not replay of the source validator on the 42 records.

## 1. Block SLP and primitive decomposition

I directly checked Lundqvist–Nicklasson, Theorem 2.7 and Section 3: for a monomial complete intersection with socle degree `t`, the theorem's characteristic condition gives maximal rank of multiplication by the specified power of the sum. Setting all exponents to two gives `t=27`; `p>27` handles powers through 27, and higher powers have zero targets. The statement applies over finite fields without an infinitude hypothesis. [Primary source](https://arxiv.org/html/1604.06820).

Here is the additional algebra, rather than an appeal to “SLP implies strings.” For general width `ell`, put `h=floor(ell/2)` and, for `0<=a<=h`,

\[
P_a=\ker(L^{\ell-2a+1}:A_a\longrightarrow A_{\ell-a+1}).
\]

Use `A_{-1}=0` and `binom(ell,-1)=0` at `a=0`. The map defining `P_a` is surjective by maximal rank and symmetry; its target dimension is `binom(ell,a-1)`. Hence

\[
\dim P_a={\ell\choose a}-{\ell\choose a-1}.
\]

For `a>=1`, if `Lw` belongs to `P_a` then

\[
L^{\ell-2a+2}w=0.
\]

This map from `A_{a-1}` to `A_{ell-a+1}` is an isomorphism: the dimensions agree and SLP gives maximal rank. Thus `w=0`. Multiplication by `L` is injective from `A_{a-1}` to `A_a`, so dimension counting proves

\[
A_a=LA_{a-1}\oplus P_a.
\]

The map `L^{ell-2a}:A_a -> A_{ell-a}` is also an isomorphism. Therefore a nonzero `v in P_a` survives through `L^{ell-2a}v`, and its next power is zero. Earlier powers cannot vanish: multiplying such a vanishing equality to the last allowed power would contradict injectivity. The same argument establishes independence for every chosen basis of `P_a`.

For degrees `d<=h`, induction now gives

\[
A_d=\bigoplus_{a=0}^d L^{d-a}P_a.
\]

For `d>h`, transport the already established decomposition of `A_{ell-d}` through the isomorphism `L^{2d-ell}:A_{ell-d}->A_d`. The resulting formula is

\[
A_d=\bigoplus_{a=0}^{\min(d,\ell-d)}L^{d-a}P_a.
\]

This proves a direct sum of graded cyclic `k[L]`-modules, with annihilator exactly `(L^{ell-2a+1})` on each primitive basis vector. It does not require these summands to be subalgebras. At the maximal primitive degree `a=13`, the two nonzero terms are `v,Lv`, and `L^2v=0`; at `a=12` the string has four terms through `L^3v`.

The characteristic hypothesis is substantive. In characteristic any prime `p<=27`, `L^p=sum b_a^p=0`, while `A_p` is nonzero. Thus `A_0 -> A_p` has rank zero and fails maximal rank. In particular the proposed length-28 string beginning at `1` does not exist. This directly diagnoses the failure of this joint when that hypothesis is removed; it does not assert the same low-degree behavior for every such prime.

## 2. Tensor sectors and the injectivity needed for relations

Choose homogeneous primitive bases in each block. Tensoring their direct sums over the field, and then adjoining the polynomial variables `U`, preserves directness. For a tuple of primitive basis vectors of degrees `a_i`, let `v` be their tensor, `K=sum a_i`, and `c_i=27-2a_i`. Its sector has the explicit graded module isomorphism

\[
\phi_v:\left(S/(L_i^{c_i+1}:0\leq i<m)\right)(-K)
 \longrightarrow Sv,\qquad q\longmapsto qv,
\]

where `S=k[L_0,...,L_{m-1},U_0,...,U_{m-3}]` is an abstract free polynomial ring. The tensor products of the independent block strings, together with unrestricted `U` monomials, prove both injectivity and surjectivity of this map. Each `F_j` is a polynomial in the displayed `L,U`, so it preserves each summand and commutes with the sector projection. Cross-sector cancellation is consequently impossible.

A homogeneous degree-`d` relation on the quartics has a sector coordinate `q_j` of degree

\[
r=d-4-K.
\]

If `r<0` the sector is absent. Otherwise use the unique representatives with each `L_i` exponent at most `c_i`. Every monomial of `q_jF_j` has `L_i` exponent at most `r+2`, since all variables have ordinary degree one and `F_j` has exponent at most two in each `L_i`. Crucially, this is a bound on the products before quotient reduction.

For every `i`,

\[
c_i-(r+2)=27-d+2+K-2a_i
 \geq 29-d-K
 \geq 33-2d.
\]

The first inequality uses `a_i<=K`; the second uses `K<=d-4`. At `d<=16` the final quantity is at least one. All products therefore belong to the vector space spanned by monomials with `L_i` exponent at most `c_i`. The quotient map by the monomial ideal `(L_i^{c_i+1})` is injective on this vector space: its standard monomials are a basis, and no such monomial lies in that ideal. Hence a zero sum of the products in the sector is a literal polynomial identity in `S`. This proves the injection missing from an arithmetic-only treatment; no cancellation of discarded terms is possible because there are no discarded terms.

The following boundary checks include the maximal allowed primitive degree at every listed degree. The same formula treats every integer degree between them.

| Relation degree `d` | Largest contributing `K=d-4` | Smallest cap, concentrated in one block | Product exponent bound at that `K` | Margin |
| --- | --- | --- | --- | --- |
| 8 | 4 | 19 | 2 | 17 |
| 12 | 8 | 11 | 2 | 9 |
| 15 | 11 | 5 | 2 | 3 |
| 16 | 12 | 3 | 2 | 1 |
| 17 | 13 | 1 | 2 | -1 |

At degree 16 a block primitive of degree 13 cannot contribute at all: its coefficient degree would be negative. Splitting `K` among several blocks makes each cap less restrictive than concentrating all of it in one block. For example `(a_0,a_1)=(6,6)` has `K=12` and cap 15 in both blocks, whereas concentration at `a_0=12` has cap 3. This covers multi-block attempts to conceal a quotient relation.

At degree 17, choose the last-block primitive degree 13 and degree-zero primitives in other blocks. The terminal product has last-block factor `L_last^2`, above cap 1, and is lost. Section 5 below gives a nonzero primitive and a concrete relation at exactly this boundary.

## 3. Free-chain regularity and homogeneous Koszul generation

Give each `L_i` order weight `m` and `U_j` order weight `m-2-j`. These are positive, including weight one for `U_{m-3}`. Refine weighted comparison by any monomial order. If the three inputs to a nonterminal quartic have weights `a,b,c`, then `a>c` and `b>c`. Relative to weight `2a+2b` of `x^2y^2`, the other five monomials have strictly smaller weights, with positive differences respectively `2(b-c)`, `2(a-c)`, `b-c`, `a-c`, and `a+b-2c`. Thus the leading coefficient is one and the leading monomials are

\[
L_0^2L_1^2,\ U_0^2L_2^2,\ldots,\ U_{m-3}^2L_{m-1}^2.
\]

Their supports are pairwise disjoint. This includes the terminal endpoint and the `m=3` case. To spell out the product criterion used here, if monic polynomials are `f=M+f'`, `g=N+g'` with coprime leading monomials `M,N`, their S-polynomial satisfies

\[
Nf-Mg=f'g-g'f.
\]

Both products in this representation have leading monomial strictly below `MN`, which is their least common multiple. This is the required standard representation for the Buchberger criterion; it applies to every pair and every prefix.

Now let `I` be a prefix ideal and let `h` have a nonzero normal remainder `r` modulo that prefix. If `hF` belongs to `I` for the next quartic `F`, then `rF` belongs to `I`. Some prefix leading monomial `M` must divide `LM(r)LM(F)`. Coprimality of `M` and `LM(F)` forces `M | LM(r)`, contradicting normality. Thus `F` is a nonzerodivisor modulo `I`. The first polynomial is nonzero in a domain and the prefix ideals are proper, so this proves a regular sequence.

For completeness, let `sum_j q_jF_j=0` be homogeneous in `S` of ordinary total degree `t=d-K`. Modulo all but the last generator, regularity implies `q_s` belongs to their ideal. Because the generators have degree four, taking the homogeneous component of degree `t-4` in any expression gives

\[
q_s=\sum_{i<s} h_iF_i,\qquad \deg h_i=t-8=d-K-8.
\]

If this degree is negative, the expression is zero. Subtract `sum_i h_i(F_i e_s-F_s e_i)` from the relation and induct on the prefix. This proves generation by first Koszul syzygies, with precisely the claimed homogeneous degrees. No exactness theorem for a whole Koszul complex is needed for this step.

This also resolves the attack range 8–16. At `d=8,K=0`, the relations `F_j e_i-F_i e_j` have constant multipliers and are present. At `d=16,K=8`, a primitive times such a relation again has constant sector multiplier. For `K>d-8`, the free relation must be zero: there is no polynomial of degree `d-K-8` to multiply a Koszul generator. In particular the most truncation-sensitive sectors `K=9,...,12` at degree 16 have no quartic relation. A relation involving three or more quartics is handled by the same last-coordinate induction; it supplies no additional exception.

## 4. Transport, mixed relations, and finite-field convention

Transport a free-sector expression by `phi_v`. This map is `S`-linear, so `phi_v(h_iF_j)=F_j phi_v(h_i)`. The transported multiplier has degree `K+(d-K-8)=d-8`. It is immaterial whether that multiplier becomes zero in the sector. Summing across the direct sum yields quartic/quartic Koszul generation in `A` through degree 16.

The passage back to `R` can be performed explicitly. Write a full homogeneous relation as

\[
z=\sum_a s_a e_{b_a^2}+\sum_j q_j e_{F_j},
\qquad \deg s_a=d-2,\quad\deg q_j=d-4.
\]

Here `a` ranges over selectors in all blocks. Reduce modulo `J`, express the quartic part as a sum of quartic/quartic Koszul vectors, lift their homogeneous multipliers of degree `d-8` to `R`, and subtract the resulting actual Koszul vectors. The remaining quartic coefficients lie in `J`. The monomial ideal property gives homogeneous expressions

\[
q'_j=\sum_a t_{aj}b_a^2,\qquad\deg t_{aj}=d-6.
\]

Subtract

\[
\sum_{a,j}t_{aj}(b_a^2e_{F_j}-F_je_{b_a^2}).
\]

This removes every quartic coordinate and leaves a relation purely on selector squares. These squares are a regular sequence in `R` (distinct variables, with all other variables adjoined freely), so the induction above, with generator degree two, expresses it as square/square Koszul vectors with multipliers of degree `d-4`. All stages preserve total relation degree. Negative multiplier degrees mean the relevant stage is zero. This includes the often missed mixed syzygies, whose basic degree is six; square/square syzygies have basic degree four, and quartic/quartic ones degree eight.

I directly checked Caminata–Gorla, Section 1, Definition 1.3 and its preceding definitions. Their finite-field convention uses `B=F_q[x]/(x_i^q)`, generator-shifted syzygy degree, and the module generated by Koszul vectors and `g_i^{q-1}e_i` as trivial syzygies. Their FFD tests the highest forms in that quotient. Here the specified field is prime, so `q=p`. [Primary source](https://arxiv.org/html/2112.05579).

For the stated integer `p=4835703278761981908090881`, all homogeneous components of ordinary degree at most 17 inject into `B`, since the ideal `(x_i^p)` contains no nonzero polynomial of degree below `p`. This applies both to every coefficient component and to the relation target. Thus the degree-`d` syzygy kernels over `R` and `B` agree for `d<=17`. Their Koszul images agree as well. A nonzero Frobenius trivial generator has shifted degree `p deg(g_i)`, hence at least `2p`; those generators cannot contribute in degree 17. Some may already be zero in `B`, which contributes nothing rather than a lower-degree vector. This proves precisely the claimed convention match. It depends on cardinality `q=p`, and should not be justified by calling `q` the characteristic in a general extension field.

This argument does not extend indiscriminately to degree `p`: for example `x^{p-2}e_{x^2}` becomes a syzygy modulo `x^p`, although its product is nonzero in the polynomial ring. The strict low-degree inequality is the reason the transfer here is valid.

## 5. Explicit boundary relation and small-characteristic attacks

In the last block put `a_j=b_{2j}`, `c_j=b_{2j+1}` for `0<=j<13`, `z=b_26`,

\[
H=\prod_{j=0}^{12}(a_j-c_j),\qquad
H_j=\prod_{r\ne j}(a_r-c_r),\qquad
T=\sum_{j=0}^{12}(a_j+c_j),\quad L=T+z.
\]

The `2^13=8192` monomials of `H` are distinct squarefree monomials with coefficients `+1` or `-1`, so its class is nonzero. For each pair,

\[
(a_j+c_j)H=(a_j^2-c_j^2)H_j.
\]

Consequently `TH=0` modulo squares, `LH=zH`, and `L^2H=0` modulo squares. Also `zH` is nonzero, so `H` lies exactly in a length-two primitive string in the stated characteristic. In the full polynomial ring the following identity supplies the missing explicit lift:

\[
L^2H=z^2H+(2z+T)\sum_{j=0}^{12}(a_j^2-c_j^2)H_j.
\]

Writing `U=U_{m-3}` and `F_T=U^2L^2`, the vector

\[
H e_{F_T}-U^2H e_{z^2}
-\sum_{j=0}^{12}U^2(2z+T)H_j
 (e_{a_j^2}-e_{c_j^2})
\]

is a homogeneous relation of degree 17. Its quartic coordinate degree is 13 and every square coordinate degree is 15. Project the coefficient ring to the last-block square-zero algebra by setting all other selectors and all `U` to zero. Every generator other than the terminal generator vanishes under this projection, while the displayed terminal coordinate becomes `H!=0`. The terminal coordinate of any Koszul vector belongs to the ideal of the other generators, so this relation cannot be Koszul. In the finite-field convention the high-degree Frobenius generators do not change that conclusion. This proves an actual relation at the cap failure, not merely a failed inequality. Identification with any stored per-case witness remains an artifact-binding step.

There are explicit counterexamples to extending the polynomial-ring conclusion into small characteristic. For prime `p<=27`, the class `L^{p-2}` in the last block is nonzero: its squarefree monomials have coefficient `(p-2)!`, nonzero modulo `p`. Yet

\[
L^{p-2}F_T=U^2L^p=U^2\sum_a b_a^p.
\]

Therefore the actual polynomial-ring syzygy

\[
L^{p-2}e_{F_T}-\sum_a U^2b_a^{p-2}e_{b_a^2}
\]

has degree `p+2`. The same last-block projection shows it is non-Koszul, because its terminal coordinate remains nonzero. For `p=2,3,5,7,11,13` these degrees are respectively `4,5,7,9,13,15`, explicitly breaking the through-16 conclusion if the characteristic assumption is dropped. At `p=2`, `L^2=sum b_a^2`, and the terminal coordinate is simply one. These are statements in the full polynomial ring and its quotient by selector squares; the finite-field FFD convention in characteristic two needs separate care because the highest selector squares themselves become zero in `B`. For `p=17,19,23`, the displayed counterexamples have degree above 17; they show why the full SLP construction fails but are not evidence of a below-17 relation for those primes.

## Machine verification and concrete remaining dependencies

I inspected `LefschetzBoundary.lean` rather than executing it. It contains eight arithmetic declarations over natural numbers, with the essential equality `r+4+K=d` present. None defines polynomial rings, primitives, monomial supports, regular sequences, syzygy modules, or the FFD convention. The source is consistent with the numerical inequalities used above. This report supplies a mathematical audit of the missing polynomial arguments; it does not upgrade the file into a machine-checked polynomial theorem. I did not independently replay its compiler output or axiom listing.

The remaining dependencies for using these joints on saved records are concrete:

1. Bind the mathematical template and the nonzero weight normalization to each exact input/source/hash. The supplied donor source references `prime_first_fall_donor.rs` and `prime_terminal_math.rs`, which are absent from this snapshot; neither the actual field implementation nor saved case inputs were available within this assignment. I therefore take primality and the record-to-template binding as explicit premises, and make no per-record validation claim.
2. If formal assurance is required, implement the polynomial objects and the module/support/Koszul arguments proved above in the proof assistant. The current eight arithmetic declarations are separate completed components.
3. Use the ordinary degree plus generator shifts stated at the beginning, and keep the monomial-order weights distinct in any revised proposition. For a finite-field FFD invocation, use `q=p` for this exact field and the strictly below-`p` comparison.

No broken step was found within the four assigned joints under these premises. The small-characteristic vectors above are explicit negative controls outside those premises, not observed failures of the characteristic-greater-than-27 construction. Filtered closure, coefficient-function separation, and rational-point claims are outside this reviewer's ownership.

## Sources actually read and snapshot identity

All local source files listed here were read in full; the literature sections actually consulted are specified. Directory listing and SHA-256 calculation were read-only. No sibling report was accessed.

| Actual local path | SHA-256 |
| --- | --- |
| `/home/ec2-user/n83-candidate-recovery/proof-bridge-review-inputs/graded/PRIME_LEFSCHETZ_ARGUMENT.md` | `31a7eb352373149428e47afbe5c03b9243cf9375aaf06ca34c5fbcf9d6fefdab` |
| `/home/ec2-user/n83-candidate-recovery/proof-bridge-review-inputs/graded/LEFSCHETZ_PROOF_MAP.md` | `91cbfac992caf4545d500b4fd130c97ee905527647d17f68c5ff1d2f64eeda01` |
| `/home/ec2-user/n83-candidate-recovery/proof-bridge-review-inputs/graded/LefschetzBoundary.lean` | `1278a3102c773081aafab2c7c193cc0fff49e9654f2dbd4884024f6cdc7d8f59` |
| `/home/ec2-user/n83-candidate-recovery/proof-bridge-review-inputs/graded/src/prime_linear_source_donor.rs` | `ce6042fcecc4e3d997acaa48b9af1426622a08e0a76c875673ccf1923d6a7b2f` |
| `/home/ec2-user/crypto-n83-degree-sweep-20261027/research/n83_candidate_pdp_20261028/reviews/proof-bridge-20261030/checker-plan.json` | `fd92fc470a0c19455c0f1df386638de07dcffe0c702a5c9984dcf0dc73aeeea8` |

- Samuel Lundqvist and Lisa Nicklasson, *On the structure of monomial complete intersections in positive characteristic*, arXiv v2, 2016-11-09: [https://arxiv.org/html/1604.06820](https://arxiv.org/html/1604.06820). Consulted Section 2 through Theorem 2.7, Remark 2.3, Section 3's characteristic discussion, and Theorem 3.8.
- Alessio Caminata and Elisa Gorla, *Solving degree, last fall degree, and related invariants*, arXiv v2, 2022-06-01: [https://arxiv.org/html/2112.05579](https://arxiv.org/html/2112.05579). Consulted Section 1, especially the finite quotient and syzygy definitions preceding Definition 1.3, and the displayed Section 4 examples returned with the definition search.

