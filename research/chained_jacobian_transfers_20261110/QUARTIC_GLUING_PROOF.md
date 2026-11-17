# Actual (4,4) gluing and six geometrically distinct Jacobians

The complete changed native suite passes 31 tests with zero failures,
ignores or filters,including all old source guards,formatting and strict
Clippy. All 14 source and fixture digests remain unchanged. On each actual
T_i,the 256 elliptic-product 4-torsion inputs have kernel 16 and image 16;
both kernel projections are bijections,and all 256 kernel-pair tests have
inverse factor Weil pairings. Full raw36,799 bytes → standard XZ7,248 bytes,
digests and exact roundtrip are pinned by PCC-20261112-642759.
Scientific standing is unsigned. [pcc:unverified]

## Sources and hypotheses

Use the admitted smooth models C_i,T_i and actual F101 maps of
SUCCESSOR_GLOBAL_PROOF.md,QUARTIC_COVER_PROOF.md,T2_QUARTIC_PROOF.md and
GEOMETRIC_JACOBIAN_DISTINCTION.md/b05286. Write B_i=E_i x F_i,
P_i:B_i→Jac(C_i),R_i:Jac(C_i)→A_i=Jac(T_i),K_i=R_i P_i.
The old global arguments give deg P_i=deg R_i=4,
P_i^*lambda_C=2lambda_B and R_i^*lambda_A=2lambda_C,with actual factor
embeddings after R_i. Their geometric2-torsion intersection certificates
are essential to the embedding claim;norm4 alone would not suffice.
[pcc:unverified]

External CT41 atca083268c29def7b189f74faa31dfdfae33712bf and primary
Howe–Lauter,arXiv:1202.6308,Definition2.1,Lemma2.3,Proposition2.8 and
Lemmas2.9–10 supply the gluing-exponent/Frobenius–Verschiebung bounds,
credited and pinned in f07968. Milne,Abelian Varieties,I Theorem7.2/
Remark7.3 gives prime-to-p torsion;IV §2 Lemma2.2 and its proof gives the
isogeny Tate-cokernel degree formula. The latter source was read directly
from the same primary PDF SHA256f5ca4e63e5092a4b102daad1470e4cbed5fe8f82115e3a28c8881e3f67f6aaef.
Galbraith corrected Chapter26,Theorem26.2.3,Exercise26.2.4,formula26.1
and §26.3.1 gives the pairing/divisor conventions and Miller recurrence,
pinned in8b9b20. Its warning about inverse pairing conventions is retained.
These are established methods;no novelty claim is made. [pcc:unverified]

Sources checked (KB):the MCP server is absent and the repository wrapper
has no executable (ct38-kb-status-01.txt). Local accepted proofs,the exact
external note and the primary papers are the fallback. Search scope was
the project and generic pairing/identity modules;the BLS12 API fixes an
incompatible field and transfer_audit/pairing.rs is a cost model. The new
adapter shares the existing field,elliptic,polynomial and legacy Cantor
engines explicitly. No producer Balanced/Richelot/trace evaluator is called.

## Lemma 1:ramified nonzero elliptic2-torsion pullback

For an admitted cover (X,Y)=(N/Q²,yM/Q³),let P=(x_0,0) be nonzero
elliptic2-torsion. None is the rational signed infinity image ±q,whose
nonzero ordinate excludes2-torsion. In the native cases U=N-x_0 Q² has
degree4,g=gcd(F_source,U) has degree2 and U/g=(x-r)²,with g squarefree
and F_source(r) nonzero. Thus the complete fibre is W_1+W_2+Z_++Z_-,
where the W_j are Weierstrass points and Z_± are the conjugate pair over r.
These are simple points as divisors on the source curve;the squared
factor of U records the x-coordinate collapse at target2-torsion.
[pcc:unverified]

The divisor of x-r shows Z_++Z_- is linearly equivalent to
infinity_++infinity_-. The divisor of Q shows f^*(O) is linearly
equivalent to2infinity_++2infinity_-. Therefore

    f^*(P-O) ~ W_1+W_2-infinity_+-infinity_-,

which is the balanced input (g,0,0). If the chosen branch-chart point
W=(a,0) belongs to g,div(x-a) gives2W~infinity_++infinity_-.
The class becomes W_other-W. Writing g=(x-a)h,with h monic linear,
the remaining quintic coordinate satisfies w=-L/h(a);its Mumford pair
is (w+L/h(a),0). This avoids an invalid half-integral infinity correction.
If g(a) is nonzero,the unchanged old chart handles (g,0,0) directly.
[pcc:unverified]

For all six factors,the native checker retains all three nonzero2-points,
including exactly one chart-W corner each. It validates every pullback,
its2-annihilator,and all 256 factor addition-table entries on E[4].
The old Y≠0 pull and chart arithmetic remain byte-identical;only the
parent child-module registration changed. The Picard identities above
are the written bridge from polynomial inputs to actual pullbacks.
[pcc:unverified]

## Lemma 2:full elliptic4-torsion and its pairings

Set r=x³+ax+b,t=3x²+a. The cleared duplication ordinate is

    8y³*y([2]P)=-8r²+12xrt-t³
      =x^6+5ax^4+20bx³-5a²x²-4abx-8b²-a³.

For y≠0,its roots identify points whose double is nonzero2-torsion.
Together with O and the three2-points,the checker obtains exactly16
distinct on-curve,4-annihilated,Frobenius^6-fixed points per elliptic
factor. It checks every addition and a basis spanning all16 points.
Milne's torsion theorem makes this an exhaustive geometric E[4] population.
The polynomial identity has a faithful core Lean ring proof and a native
identity.certificate/v1 record from the unchanged repository checker;
the evaluated certificate alone is only a finite-field check.
[pcc:unverified]

Miller's recurrence for n=4 gives

    f_4,P(T)=line_P(T)²/(x_T-x_[2]P),
    e_4(P,Q)=f_4,Q(P)/f_4,P(Q)

for independent basis points,with vertical factors and normalized leading
coefficients retained. The direct ratio is independently cross-checked
using disjoint divisors D_P=(P)-(O),D_Q=(Q+S)-(S):

    e_4(P,Q)=f_4,Q(P-S)*f_4,P(S)
              /(f_4,Q(-S)*f_4,P(Q+S)).

All undefined or zero presentations are rejected;at least one valid S
must pass. Each basis value eta satisfies eta²=-1,eta^4=1 and Frobenius
fixes eta. Bilinearity gives e_4(aP+bQ,cP+dQ)=eta^(ad-bc).
All256 kernel-pair products are1 on every model. Inverting only the
second factor rejects exactly96 pairs:6 ordered F2 bases have16 lifts
each. Initial protocol128 was a pre-native counting error,retained in
immutable V1 with explicit V2 correction2c95c6. [pcc:unverified]

## Theorem 3:the actual polarized gluing is a maximal4-graph

Composition gives deg K_i=16 and K_i^*lambda_A=4lambda_B. Its actual
polarized adjoint H_i satisfies H_i K_i=[4];surjectivity of K_i then
also gives K_i H_i=[4]. Hence Ker K_i is contained in B_i[4]. Since
16 is prime to101,the kernel is etale of order16. Each factor restriction
K_(i,E),K_(i,F) is an embedding by the old intersection certificates.
Thus either projection of Ker K_i is injective:two kernel pairs with
the same E coordinate differ in the F factor kernel,and conversely.
Both target elliptic4-torsion groups have16 points,so the projections
are bijective. The kernel is the graph of an F101-defined isomorphism
E_i[4]→F_i[4]. It is maximal isotropic for the product Weil pairing
because the principal polarization descends along K_i,so the graph
is an anti-isometry. One can see isotropy directly on integral Tate
pairings using Milne I Lemma13.1/Proposition13.2. For kernel elements
represented by v,w in (1/4)T_2(B_i),their images under K_i lie in M.
The polarization identity gives
e_B(4v,4w)=4e_A(K_i v,K_i w),which vanishes modulo4. Reduction of the
Tate pairing is the4-Weil pairing,so its product on the two factors
is1. The native complete graph/pairing checks provide
explicit finite support for this global polarized argument.
[pcc:unverified]

Consequently these actual principal Jacobians are (4,4)-split,with
primitive factor embeddings and degree4 elliptic covers. A smaller
factor isogeny cannot account for the maps,because its dual would give
a nontrivial kernel in the corresponding actual pullback embedding.
This conclusion uses the global constructions,not only kernel counts.
[pcc:unverified]

## Theorem 4:CT41's two-adic bound is sharp on each actual T_i

Let M=T_2(A_i),and let W_E,W_F be its rational Frobenius isotypic
spaces. For an embedded elliptic subvariety B⊂A_i,the quotient A_i/B
is an abelian variety. On Tate modules the kernel of the quotient map
is T_2(B);its target is torsionfree. Thus T_2(B) is saturated in M.
Its rational span is the corresponding W,so T_2(B)=M intersect W.
Explicitly,any element in the intersection has a nonzero power of2
multiple in T_2(B),and saturation removes that denominator.
[pcc:unverified]

Therefore the image of T_2(K_i) is exactly
N=(M intersect W_E)+(M intersect W_F). Milne IV Lemma2.2(b) and deg K_i=16
give [M:N]=16. Also4M⊂N from K_i H_i=[4]. CT41's projection argument
injects M/N into (1/4)(M intersect W_E)/(M intersect W_E),a group
isomorphic to (Z/4)^2. Equal order makes this an isomorphism. Thus
the actual two-adic mixing quotient is (Z/4)^2,attaining the bound16,
with the principal polarization supplied by the actual Jac(T_i).
This strengthens the external abstract-lattice realization at its
stated open geometric step;it does not establish a general realization
theorem for arbitrary lattices. [pcc:unverified]

The common subgroup E_i[4]≅F_i[4] has exponent4. Howe–Lauter's
Definition2.1 and the upper bound4 therefore also give actual gluing
exponent4 for these isogeny classes. Native kernel Frobenius fixed
counts are1,1,4,16 over degrees1,2,3,6. They agree with
F²-F+1=0 modulo4 and hence F³=-I,F^6=I;the fixed subgroup of F³
is precisely the four2-torsion points. [pcc:unverified]

## Theorem 5:all six C_i and T_i Jacobians are geometrically distinct

The old ordinary CM-field argument b05286 gives
Hom(E_i,F_j)=Hom(F_j,E_i)=0 over the algebraic closure for every i,j.
In each Jac(C_i) and Jac(T_i),the actual embedded E-type and F-type
elliptic subvarieties are therefore unique. A geometric unpolarized
abelian-variety isomorphism must preserve those types. For i≠j,its
restriction would give E_i≅E_j,contradicting j(E_i)=12,87,47.
[pcc:unverified]

For the same i,the canonical addition map from those two intrinsic
elliptic subvarieties has degree4 on Jac(C_i),via P_i,and degree16 on
Jac(T_i),via K_i. An isomorphism of the surfaces and its restrictions
to the two subvarieties give a commuting addition square with all three
vertical maps isomorphisms. Its two kernels are therefore isomorphic,
so their degrees must agree. Since4≠16,this is impossible. The argument
also distinguishes every cross-index C_i/T_j by the previous j test.
Hence all15 pairs among the six unpolarized geometric abelian varieties
are nonisomorphic. This is a written geometric proof with explicit old
embedding/classification/degree dependencies,not a native isomorphism
solver or full formal construction of the varieties. [pcc:unverified]

## Formalization and preserved failures

QuarticGluing.duplication_ordinate_numerator proves the ring identity.
sum_kernel_projection_injective derives projection injectivity from an
actual injective factor map. kernel_transport_bijective constructs the
kernel bijection from an explicit commuting square,bijective factor maps
and injective surface map. quotient_kernel_saturated and
saturated_equal_rational_span give the exact lattice implications with
torsionfree quotient and denominator hypotheses exposed. All five pass
the core Lean4.35.0 gate with zero trusted native proofs;actual geometric
objects,polarization,Tate exactness,cardinalities and classification remain
written bridges. Scientific signing remains separate. [pcc:unverified]

Static first attempt9bfde8 failed on coordinate typing/Hyp API before any
native execution. Followup02a50b retained missing core Bijective and missing
hex dependency;algebra style failureb93d1e retained the unchanged imported
SHA source. The scoped SHA style allowance changes no reference bytes;
all new code remains under strict Clippy. All raw failures,sources and
statuses remain immutable,as do the previous30-test/149-declaration reports.
Next is actual4-gluing transport along the nondual7 chain and a bounded
elliptic-cycle closure check,with full accounting still admission-blocked.
[pcc:unverified]

## Exact proof and certificate map

| Conclusion | Declaration / claim | Exact scope |
|---|---|---|
| Duplication numerator | QuarticGluing.duplication_ordinate_numerator / d6878f | All commutative rings;actual curve substitution explained above. |
| Kernel projection | QuarticGluing.sum_kernel_projection_injective / 8b6f8c | Additive target with explicitly injective second factor. |
| Intrinsic kernel transport | QuarticGluing.kernel_transport_bijective / c7cd22 | Explicit commuting square,bijective factor maps,injective surface map. |
| Quotient saturation | QuarticGluing.quotient_kernel_saturated / 47fb74 | Scalar-linear quotient and torsionfree target. |
| Primitive full intersection | QuarticGluing.saturated_equal_rational_span / 0669e7 | Inclusion,saturation and denominator-clearing hypotheses. |
| Native identity check | IDC1he156b6adc8f9cdd5 / 1176bb | Existing checker32issuer+64extra evaluations over2^61-1;exact Lean-source digest pointer;23 tests pass. |
| Actual geometry,degree,Tate index and six surfaces | Theorems3–5 above | Written global/Picard/classification bridges,not fully formal geometry. |

All five Lean statements are individually bound and locally pass. The
written census is154 declarations across31 modules. The initial binding
invocation lacked lean/lake on PATH and returned not_covered;that full
receipt is retained alongside the corrected localpass. The same immutable
first claim was reused;no ledger record was rewritten or duplicated.
New workflow checks pin both imported identity/SHA sources,run all23
algebra tests,and reproduce the exact committed certificate in a separate
job. Gate/source hashes and raw evidence reside under evidence/quartic-glue-*
and evidence/algebra-complete-03. [pcc:unverified]
