# Original-model dimension/extension obstruction [pcc:unverified]

This is an author-derived application of standard abelian-variety theory, not a new attack or an independent formal proof. It screens a concrete successor to the rejected solver-only strategy: transfer to a smaller-dimensional abelian variety. The previous binomial-family isogeny screens are not substituted for these actual mixed-term models.

## Criterion and proof

Let J/Fp have dimension g, and let a certified nonzero point have prime order ell. Put U=(ceil(sqrt(p))+1)^(2(g-1)). If ell>U, J is Fp-simple.

Suppose otherwise. Poincare reducibility over the perfect field Fp gives an Fp-isogeny J~A*B with both positive dimensions less than g. Isogenous finite-field abelian varieties have the same Frobenius polynomial and rational point count. Therefore #J(Fp)=#A(Fp)*#B(Fp). The prime ell divides #J(Fp), so it divides at least one factor count. Hasse–Weil gives #A(Fp),#B(Fp)<=(sqrt(p)+1)^(2(g-1))<=U<ell, a contradiction. Simplicity over Fp does not imply absolute simplicity or irreducibility of every Frobenius polynomial; no such stronger claim is made.

In particular any Fp-homomorphism from J to an abelian variety of dimension<g is zero: a nonzero image would give a proper positive-dimensional quotient and hence a decomposition up to isogeny. This includes elliptic-factor transfer. Nonconstant maps to genus-zero curves remain possible and are not excluded. Same-dimensional isogenies to different models remain possible.

## Extension-degree/dimension product

Let a homomorphism defined over F_(p^d) send the rational ell-subgroup nontrivially to an h-dimensional abelian variety A/F_(p^d). The image has order ell. If h*d<g, then

#A(F_(p^d)) <= (p^(d/2)+1)^(2h) <= (sqrt(p)+1)^(2hd) <= (ceil(sqrt(p))+1)^(2(g-1)) <ell,

again impossible. The middle inequality follows by expanding (sqrt(p)+1)^d>=p^(d/2)+1. Hence preserving the subgroup requires h*d>=g. Weil restriction provides the same dimension accounting. This concerns abelian-group homomorphisms on the prime subgroup; arbitrary nonlinear encodings, pairings to multiplicative groups and nonhomomorphic reductions require their own analysis.

## Actual original-panel integer witnesses

|g|p|ell|integer upper for dimension g-1|
|--:|--:|--:|--:|
|2|101|4663|144|
|2|251|61667|289|
|2|503|65179|576|
|2|1009|241259|1089|
|2|2003|2001787|2116|
|2|4001|16050337|4225|
|3|61|124459|6561|
|3|211|4620611|65536|
|3|401|31525763|234256|
|4|31|239753|117649|
|4|61|16790591|531441|

All11 pass. dimension.rb also directly checks every positive h,d with h*d<g by integer ceil-sqrt(p^d) bounds, verifies primality by exact trial division, and matches c/ell against prior ADMISSION.json. Source nonzero/order membership is REUSED from native-kummer-pair-screen freeze dd1f0f2dba2: main.rs checks nonzero inst.d1 and [ell]inst.d1=0 on the exact model before emitting each retained MODEL line. It is not independently recomputed in this static derivation. That source/protocol/raw exit0 chain is included in INPUT.sha256.

## Implication for candidate choice

Lower-dimensional elliptic transfer must pay extension degree at least g; generic rho there has the usual p^(d/2) scale. Hyperelliptic h>=2 transfer using the standard double-large-prime heuristic has scale p^(d*(2-2/h)). For h<g and h*d>=g, the finite h,d possibilities at genera2/3/4 do not yield an exponent improvement over the original hyperelliptic p^(2-2/g) benchmark with these standard methods. This is a conditional algorithm comparison, not a lower bound on every algorithm on the image or on concrete constants. It does not exclude a specialized image algorithm, pairing method, or same-genus nonhyperelliptic model. No10x performance claim follows from the criterion.

Decision: do not prototype base-field elliptic splitting or low h*d extension transfers on this original panel. A new geometric candidate must respect same-dimension model transport, or explicitly supply another mechanism outside this homomorphic reduction. Known genus3 Smith transport remains an example of the permitted same-dimension route; its current implementation's measured negative cost case is unchanged. A generic genus4 Jacobian-model construction is still a substantive open obligation, not supplied here.

## Sources checked

Primary: J.S.Milne, Abelian Varieties v2.00, https://www.jmilne.org/math/CourseNotes/AV.pdf , I Proposition10.1 (printedp42,PDFp48), II Theorem1.1 (printedp75,PDFp81) and Tate-module/Frobenius interpretation immediately afterward. These sections were opened/read in this turn. They supply reducibility, rational cardinality and Weil bounds; the large-prime criterion and h*d deduction are displayed above. No quotation or novelty claim.

KB source:0550a958dcd2a68327a2ef3fb26ad575 (crypto-autoresearcher knowledge/findings/KN-FIND-61347e.md) discusses the complementary factor and ambient accounting of elliptic embeddings; it is contextual prior work, not a proof that these particular J split or are simple. Local original-panel source-order gates above supply model-specific premises. Independent mathematical review remains open.
