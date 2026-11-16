# Prior-work audit for the two F5 carrier candidates

Checked 16 November 2026. This is a targeted audit, not a completeness
claim about the literature. The shared research KB was queried at revision
`9141e386254340a9bdba3036f2bb706a` (last refresh 14 November 2026).
The local repository was searched for cyclic-11 four-branch trace 2/24
and these exact exponent rows. The existing F3 genus-10 screen contains
trace numbers 2 and 24 in different characteristic and curve populations;
it is not prior verification of the F5 equations below.

| Candidate | Exact equation and source | Targeted exact-query outcome | Classification |
|---|---|---|---|
| Trace 2 | `C/F5: y^11=x(x-1)/(x-2)`; `H/F5: W²=(v-2)(D_11(v,1)-1)`; `E/F3125: y²=x³+x+1+z²` | Searches for the curve equations and `X^10-2X^5+3125` located no direct publication match. Related Chebyshev papers below have different displayed models. | New exact triple in this project; publication priority unresolved. |
| Trace 24 | `C/F5: y^11=x((x-1)/(x-2))^9`; `E/F3125: y²=x³+2x+2+z+2z³` | Searches for the curve equation and `X^10-24X^5+3125` located no direct publication match. | New exact pair in this project; publication priority unresolved. |

The underlying signature and p-rank bound come from Ozman–Pries–Weir,
Lemma 2.1 and Theorem 2.3 (Bouw):
https://doi.org/10.1017/nmj.2022.12 . General Jacobian decomposition
through dihedral actions predates this screen:
https://arxiv.org/abs/1609.01562 . Odd-characteristic direct-cover
context is in Iijima–Momose–Chao:
https://eprint.iacr.org/2015/805.pdf . The same-Weil-polynomial isogeny
step is Tate's finite-field theorem:
https://pazuki.perso.math.cnrs.fr/index_fichiers/Tate66.pdf .

The LMFDB `av_fq_isog` table was checked for `g=5,q=5` and returned zero
rows. That is a coverage gap, not evidence that the classes are new.
Two papers posted in November 2026 are relevant to the trace-2 Dickson
model: [Kazemifard–Tafazolian](https://arxiv.org/abs/2611.10255)
studies `y^n=D_m(x,1)` and its automorphisms, while
[Tafazolian–Top](https://arxiv.org/abs/2611.14198) studies
`w²=D_d(v,1)` and its maximality. The displayed defining equations
and stated objectives differ from `W²=(v-2)(D_11(v,1)-1)`; this is
a targeted distinction, not a proof that the latter is absent from
all prior literature. Their Chebyshev/dihedral context further limits
any family or method novelty claim.
The targeted exact-equation web queries returned unrelated records and
general dihedral-cover work. The result is therefore labeled as an exact
carrier/isogeny-class lead inside a known geometric family. It is not
labeled a new attack method or demonstrated vulnerable curve family.

The decisive vulnerability checks are explicit maps on the order-71
and order-47 source subgroups and a complete same-subgroup
GHS-versus-rho cost comparison. The trace-2 quotient now has an
independent smooth hyperelliptic model and zeta replay; the trace-24
quotient still needs that replay. A broader bibliography and isomorphism
screen are needed before assigning publication priority.
