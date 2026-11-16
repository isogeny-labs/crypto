#set page(paper: "a4", margin: (x: 18mm, y: 18mm))
#set text(font: "New Computer Modern", size: 10pt)
#show link: underline

= F5 cyclic-11 genus-five factor screen

_Research report, 16 November 2026. Source-bound finite result; PCC standing unverified._

== Population and method

The frozen population contains all 90 exponent pairs $1 <= a,b <= 10$ with $a+b != 0 mod 11$ in `y^11=x^a(x-1)^b` over F5. The three geometric branch points are $0,1,infinity$. The log-residue histogram screen and a separate direct eleventh-power point recount over F3125 agree for every row. `SCREEN.json` and `VERIFIED.json` retain the full results. [pcc:unverified]

The genus is five. The fifth-extension trace of the candidate elliptic factor appears in `L(T)=1-t*T^5+3125*T^10`, with `t=(3126-N5)/5`. `F5Arithmetic.lean` checks the order-five and trace-recurrence arithmetic without axioms. The geometric genus and point-count implementation are external to that Lean file. [pcc:unverified]

#image("traces.svg", width: 100%)

Exactly 60 rows have $t=-75$ and 30 have $t=90$. Both are nonzero multiples of five. The elliptic trace classification allows only $t=0$ for supersingular elliptic curves over F3125, while ordinary elliptic traces are prime to five. Hence this rational three-branch family has no eligible elliptic source factor. This does not classify four-branch covers or other genus-five carriers. [pcc:unverified]

== Prior work and next domain

Cyclic-cover signature and Jacobi-sum methods are established. Priority of this exact 90-row table remains unresolved. The broader order-five p-rank-zero exclusion for ordinary sources is in `../ghs_cyclic11_p_rank_bound_20261116/THEOREM.md`. [pcc:unverified]

Primary sources: #link("https://doi.org/10.1017/nmj.2022.12")[Ozman–Pries–Weir (2022)] and #link("https://pure.mpg.de/pubman/item/item_3596867_6/component/file_3655796/Agathocleous-Joux-Taufer_Elliptic%2Bcurves%2Bover%2BHasse%2Bpairs_2026.pdf")[Agathocleous–Joux–Taufer (2026), Waterhouse trace cases]. The exact finite program is `screen.py`; `verify_direct.py` is the separate historical recount. A native reimplementation is required for any extension of the screen under the repository's current language rule.
