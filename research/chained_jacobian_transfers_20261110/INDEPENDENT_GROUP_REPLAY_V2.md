# Existing second Cantor engine: precise chart and replay scope

Scientific standing unsigned. This adds group-law replay for the TWO fixed
49-row Psi/Psi2 corpora, beyond their earlier flat divisor-validity/Frobenius
checks. It does not independently evaluate either isogeny or claim complete
arbitrary-class Jacobian coverage. Raw native acceptance is a separate
receipt; the argument below identifies exactly the implemented chart.
[pcc:unverified]

## Existing arithmetic and source custody

The existing generic Fld/polynomial/Hyp implementation comes from
crypto:src/cryptanalysis/jv_cover.rs, last changed at
cfa4ef08bccfb509c89f8d0bb0c6c5baf662ff11 and unchanged in root ecf190beda.
Full source SHA256 is
1ecd62c4282402e3411635e0487d0bb36a5d57e15a547f352a7cab1fda7309cc.
The initial retained lines40–52 and464–690 are byte-exact:the generic field trait,
polynomial operations and odd-model Cantor methods through u128 scalar
multiplication. An impl-closing brace supplies framing after the omitted
BigUint scalar method. No retained arithmetic function is changed.
StrictClippy02 rejects the original tuple return signature's type complexity.
The final version adds only its identical Bezout tuple alias and substitutes
that name in the pxgcd signature;arithmetic bodies stay unchanged. The
source-extraction test reconstructs the original bytes and permits exactly
those two type-only edits. legacy.sha256 binds both sources. Initial source
and failed lint receipt are retained separately. [pcc:unverified]

A thin Fld implementation delegates operations to the existing flat FpR
engine. It imports no producer tower, balanced addition, closed Richelot
map or elliptic kernel generator. The witness parser reuses the accepted
flat replay parser. The two inputs are the frozen actual Psi/Psi2 TSVs,
with source models T0=[31,75,74,54,71,55,1] and
T1=[95,26,19,27,94,91,1]. All nonidentity rows have monic degree2 support,
degree<2 ordinate and balanced-plus0;the identity is degree0/plus1.
Unsupported row shapes are rejected,not silently reinterpreted.
[pcc:unverified]

## Monic quintic without an ordinate square-root assumption

Let the smooth monic sextic be y²=F(x). Choose the FIRST sorted branch
root a in the existing F101⁶ field for which u(a) is nonzero on every
displayed row. The native certificate checks all six roots and every row;
failure to find such a chart would be retained as a chart failure.
Put L=F'(a),nonzero by smoothness,and

    H(z)=z⁶ F(a+1/z),
    w=L/(x-a),   V=y*w³/L,
    G(w)=L⁴ H(w/L).

F(a)=0 makes H degree5,with leading coefficient L. Consequently G is
MONIC of degree5,without requiring L to be a square. The equation
V²=G(w) follows by substitution;the inverse is
x=a+L/w,y=V*L/w³. These are birational inverse charts and extend to an
isomorphism of smooth projective curves. Native code checks the exact
degree/leading coefficient and squarefreeness of G. Seven distinct w
values check its degree≤6 defining polynomial identity as an additional
coefficient-evaluation control,not as a density argument on the Jacobian.
[pcc:unverified]

For u(x)=x²+b*x+c,set A=u(a). The support transforms to the monic

    U(w)=w²+[L(2a+b)/A]w+L²/A.

The exact identity

    w²u(x)-[A*w²+L(2a+b)w+L²]
      =[(x-a)w-L][(x+a+b)w+L]

proves the support relation. [pcc:unverified]
A andL are checked units. U(0)=L²/A is
nonzero,so the inverse chart is defined in the full finite support algebra.
For v(x)=v0+v1*x,the transformed ordinate is

    W(w)=v1*w²+[v(a)/L]w³ modulo U.

It equals v(x)w³/L in that algebra. The inverse coordinate substitution
and these algebra identities retain repeated support and its ordinate jet;
they do not choose individual support roots. KernelChart.lean formalizes
the cleared/monic support and ordinate implications over arbitrary
commutative rings with explicit inverse/chart hypotheses. Completion and
Picard functoriality remain written. [pcc:unverified]

## Balanced baseline and induced group map

Each nonzero exported balanced class is D_finite-K,where
K=infinity_plus+infinity_minus. The sextic infinities map to
B+=(w0,V=L²) and B-=(w0,V=-L²). The new quintic's unique Weierstrass
infinity W corresponds to the branch point(a,0). The rational function w
has divisor B+ +B- -2W. Thus the image of K is linearly equivalent to2W,
and the mapped class is exactly the odd-model Mumford class(U,W),with
baseline2W. There is no missing infinity translation. The identity maps
to the odd-model identity. The projective curve isomorphism induces a
Jacobian group isomorphism,so comparing these representatives in the
second Cantor engine tests the original displayed classes' additions.
[pcc:unverified]

## What is checked and what remains

For BOTH corpora,the bounded replay checks49 valid/distinct transformed
classes,all7-annihilators and inverses,and ALL2401 pairwise sums against
the indexed Z7×Z7 sum law. The first two-test receipt01 passes,including
exact source extraction,PCC42b1b7. The existing flat replay separately
checks coefficientwise Frobenius in the original coordinates. Frobenius
is not naively applied coefficientwise on one fixed nonrational branch
chart,which would move the quintic model. [pcc:unverified]

Final version03 passes the complete8-test standalone suite and strict
Clippy with the tuple-equivalent type alias,PCC9ebcf7. Initial byte-exact
source/receipt and the type-complexity failure02/PCC647a5a are immutable.
Four KernelChart Lean statement bindings/axiom checks pass,PCC2026a6;
this formalizes the support/ordinate algebra,not projective/Picard theory.
[pcc:unverified]

This is a second pre-existing Jacobian group algorithm with an explicit
coordinate bridge,not a signed scientific verifier. The map-kernel
identification still uses the written global square/degree and producer's
actual elliptic-map killing controls. It is not independent isogeny-map
evaluation,whole rational-domain replay,
large-prime/catalog coverage or a benchmark. All prior limited replay
claims remain frozen and keep their original scope. [pcc:unverified]
