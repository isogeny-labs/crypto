#set page(paper:"a4",margin:18mm)
#set text(font:"DejaVu Sans",size:10pt)
= Anchored quadratic classes in descent
16 November 2026 · Unselected source review · No experiment or result

The current Nagao materializer constructs a monic degree-six quotient
S=(C²H−A²)/u_R over Fp, then refuses it unless all six roots lie in the
supported Fp base. A four-root quotient with a remaining irreducible
quadratic is discarded at that check. [pcc:unverified]

#image("flow.svg",width:100%,height:87mm,fit:"contain")

For R=(u_R,v_R), the actual function is A+C*y with C=x+μ and
A=−xv_R+λ0u_R+λ1xu_R−μv_R. Exact norm/support, coprimality and point
checks would establish R+ΣsP·B+cD·D=0. An existing principal anchor
ΣsA·B+cA·D=0 gives the signed cancellation

#align(center)[R=−ΣsP·B+cD·cA·ΣsA·B.]

The target row retains coefficient−b and RHS a when R=aG+bQ. Both
original group equations and their source witness must be replayed before
kernel projection, affine target determination and elliptic verification.
Canonicalization identifies D with−D only with the retained sign; U alone
is not a class identity. [pcc:unverified]

Variants: one anchored quadratic; two anchored quadratics with quartic
factoring; affine graph joins for unanchored classes; or a larger retained
anchor pool. Match rate, extra proof/factoring work and complete solve cost
are unknown. More partials alone is not a result. The active41experiment
is unchanged; no next protocol or measurement is selected here.
[pcc:unverified]

Primary source: Joux–Vitse, CoverDecomp.pdf, §§3.1–3.3. Source review:
jv_cover::weil_forms and nagao_decompose. The KB is unavailable; this is
not a novelty or asymptotic claim. Public solver/forms/counting APIs make a
separate Rust component possible before library integration.
[pcc:unverified]
