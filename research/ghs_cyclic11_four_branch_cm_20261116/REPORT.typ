#set page(paper: "a4", margin: (x: 18mm, y: 18mm))
#set text(font: "New Computer Modern", size: 10pt)
#show link: underline

= Four-branch cyclic-11 carriers: an ordinary-source filter

_Research report, 16 November 2026. Conditional result; publication priority unresolved. Claim standing: unverified._

== Exact finite classification

For a smooth cyclic degree-11 cover of the projective line with four distinct geometric branch points in characteristic 3 or 5, write its nonzero inertia as $(a,b,c,d)$, with $a+b+c+d = 0 mod 11$. Riemann–Hurwitz gives genus 10. The signature is $s_i = -1 + sum_e < -i e / 11 >$. Because 3 and 5 have order five modulo 11, Bouw's upper bound is $f_p(J C) <= B = 5 min_(i in R) s_i + 5 min_(i in N) s_i$, where $R = {1,3,4,5,9}$ and $N = {2,6,7,8,10}$. [pcc:unverified]

The frozen native enumeration and a Lean finite certificate cover all 910 ordered admissible inertia triples. The histogram is 280 rows at $B=0$, 360 at $B=5$, and 270 at $B=10$. Exactly the $B=10$ rows have inertia in two opposite pairs ${u,-u,v,-v}$; these represent 15 sorted patterns. The counts are of ordered exponent rows, not distinct curves. [pcc:unverified]

#image("bound-histogram.svg", width: 100%)

== Conditional CM restriction

Let $E$ be an ordinary elliptic curve over $F_(p^5)$. Assume its restriction of scalars $A = "Res"_(F_(p^5)/F_p) E$ is simple over $F_p$, and assume a nonzero $F_p$-homomorphism $A -> J C$. If $B <= 5$, the image is an ordinary dimension-five factor and uses all available ordinary p-rank. Over $F_(p^5)$ it is isogenous to $E^5$. The deck group acts on that factor, so $"Q"(zeta_11)$ embeds in $M_5(K)$ for $K = "End"^0(E)$. The degree constraint forces $K = "Q"(sqrt(-11))$ and $t^2 + 11 m^2 = 4 p^5$ for the trace $t$ of $E$. This geometric bridge is a paper argument outside the Lean finite certificate. [pcc:unverified]

Exhaustive Hasse-range arithmetic yields only $t=plus.minus 31$ at $p=3$ and $t=plus.minus 57$ at $p=5$. All are fifth-degree traces of ordinary elliptic isogeny classes over the prime field. Therefore a non-descended ordinary class under the stated simple-restriction and map hypotheses requires $B=10$, hence opposite-pair inertia. This is a necessary criterion; it supplies no elliptic source or subgroup map. [pcc:unverified]

== Next exact falsifier and evidence

For opposite-pair inertia, the rational form $y^11 = x^a ((x-1)/(x-lambda))^b$ has the involution candidate $(x,y) -> (lambda/x, delta/y)$ with $delta^11=lambda^(a-b)$. At $p=3$, the exact candidate $y^11=x(x-1)/(x+1)$ has proposed invariants $u=x+2/x$ and $v=y+1/y$; a native coefficient check yields $u D_11(v,1)=u^2+u+1$. Its identity certificate, smooth quotient replay, and elliptic-source subgroup map are pending. Dihedral Jacobian decompositions are established prior art, so method novelty is not claimed. [pcc:unverified]

Primary sources: #link("https://doi.org/10.1017/nmj.2022.12")[Ozman–Pries–Weir (2022), Lemma 2.1 and Theorem 2.3]; #link("https://arxiv.org/abs/1609.01562")[dihedral Jacobian decompositions]. Run artifacts are `PROTOCOL.md`, `screen.rs`, `RUN.stdout`, and `SOURCE.sha256`. `FourBranchFinite.lean` passed the focused proof gate: 17 theorems, zero trusted exceptions under Lean 4.35.0. Full assumptions and evidence boundaries appear in `RESULT.md`.
