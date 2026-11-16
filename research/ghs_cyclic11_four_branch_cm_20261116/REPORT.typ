#set page(paper: "a4", margin: (x: 18mm, y: 18mm))
#set text(font: "New Computer Modern", size: 10pt)
#show link: underline

= Four-branch cyclic-11 carriers: bound and exact F5 leads

_Research report, 16 November 2026. Conditional result; publication priority unresolved. Claim standing: unverified._

== Exact finite classification

For a smooth cyclic degree-11 cover of the projective line with four distinct geometric branch points in characteristic 3 or 5, write its nonzero inertia as $(a,b,c,d)$, with $a+b+c+d = 0 mod 11$. Riemann–Hurwitz gives genus 10. The signature is $s_i = -1 + sum_e < -i e / 11 >$. Because 3 and 5 have order five modulo 11, Bouw's upper bound is $f_p(J C) <= B = 5 min_(i in R) s_i + 5 min_(i in N) s_i$, where $R = {1,3,4,5,9}$ and $N = {2,6,7,8,10}$. [pcc:unverified]

The frozen native enumeration and a Lean finite certificate cover all 910 ordered admissible inertia triples. The histogram is 280 rows at $B=0$, 360 at $B=5$, and 270 at $B=10$. Exactly the $B=10$ rows have inertia in two opposite pairs ${u,-u,v,-v}$; these represent 15 sorted patterns. The counts are of ordered exponent rows, not distinct curves. [pcc:unverified]

#image("bound-histogram.svg", width: 100%)

== Conditional CM restriction

Let $E$ be an ordinary elliptic curve over $F_(p^5)$. Assume its restriction of scalars $A = "Res"_(F_(p^5)/F_p) E$ is simple over $F_p$, and assume a nonzero $F_p$-homomorphism $A -> J C$. If $B <= 5$, the image is an ordinary dimension-five factor and uses all available ordinary p-rank. Over $F_(p^5)$ it is isogenous to $E^5$. The deck group acts on that factor, so $"Q"(zeta_11)$ embeds in $M_5(K)$ for $K = "End"^0(E)$. The degree constraint forces $K = "Q"(sqrt(-11))$ and $t^2 + 11 m^2 = 4 p^5$ for the trace $t$ of $E$. This geometric bridge is a paper argument outside the Lean finite certificate. [pcc:unverified]

Exhaustive Hasse-range arithmetic yields only $t=plus.minus 31$ at $p=3$ and $t=plus.minus 57$ at $p=5$. All are fifth-degree traces of ordinary elliptic isogeny classes over the prime field. Therefore a non-descended ordinary class under the stated simple-restriction and map hypotheses requires $B=10$, hence opposite-pair inertia. This is a necessary criterion; it supplies no elliptic source or subgroup map. [pcc:unverified]

== Next exact falsifier and evidence

For opposite-pair inertia, the rational form $y^11 = x^a ((x-1)/(x-lambda))^b$ has the involution $(x,y) -> (lambda/x, delta/y)$ with $delta^11=lambda^(a-b)$. An exact $p=3$ candidate yielded only a nonordinary quotient trace; the complete rational F3 opposite-pair screen had only a descended ordinary trace. The F5 arm produced two ordinary, non-descended isogeny-class leads detailed on the next page. Dihedral Jacobian decompositions are established prior art. [pcc:unverified]

Primary sources: #link("https://doi.org/10.1017/nmj.2022.12")[Ozman–Pries–Weir (2022), Lemma 2.1 and Theorem 2.3]; #link("https://arxiv.org/abs/1609.01562")[dihedral Jacobian decompositions]. Run artifacts are `PROTOCOL.md`, `screen.rs`, `RUN.stdout`, and `SOURCE.sha256`. `FourBranchFinite.lean` passed the focused proof gate: 22 theorems, zero trusted exceptions under Lean 4.35.0. Full assumptions and evidence boundaries appear in `RESULT.md`.

#pagebreak()

= Two exact F5 carrier and source pairs

_Carrier and elliptic equations are exact. The proposed transfer is identified only at the isogeny-class level. Claim standing: unverified._

Let $K=F_5[z]/(z^5+4z+1)$ and $q=3125$. The following rational models have the involution $(x,y) -> (2/x,1/y)$ and quotient coordinates $u=x+2/x$, $v=y+1/y$. Their smooth involution quotients have genus five by the two-fixed-point Riemann–Hurwitz calculation. The integer numerator identities for the involutions have evaluated certificates `IDC1ha61ebaac2a79b72e` and `IDC1h01d6f75e07ec20c7`. [pcc:unverified]

*Trace 2:* $C_2: y^11=x(x-1)/(x-2)$; $E_2/K: y^2=x^3+x+1+z^2$. The F3125 character count is $N=282$, giving $|Q_2(F_3125)|=3116$. Native point enumeration and independent PARI/GP give $|E_2(K)|=3124=4 dot 11 dot 71$. [pcc:unverified]

This quotient also has the explicit smooth hyperelliptic model
$ H_2: W^2=(v-2)(D_11(v,1)-1), quad v=y+1/y, quad u=x+2/x. $
The rational map uses `W=(2u-D11(v,1)-2)/P5(v)`, with $P_5=v^5+v^4+v^3+2v^2+3v+1$ in $F_5[v]$. PARI/GP independently checks the square factor $D_11-2=(v-2)P_5^2$, squarefreeness, genus five and $X^10-2X^5+3125$ as its Weil polynomial. The quotient and factor identities have evaluated certificates `IDC_Q2.json` and `IDC_H2.json`. [pcc:unverified]

*Trace 24:* $C_24: y^11=x((x-1)/(x-2))^9$; $E_24/K: y^2=x^3+2x+2+z+2z^3$. The count is $N=262$, giving $|Q_24(F_3125)|=3006$. Native enumeration and PARI/GP give $|E_24(K)|=3102=2 dot 3 dot 11 dot 47$. [pcc:unverified]

For $m=1,2,3,4$, both quotients have $|Q(F_(5^m))|=5^m+1$. At $m=5$, $|C(F_3125)|=4+11N$ and the Frobenius-orbit formula gives $|Q(F_3125)|=(|C(F_3125)|+3126)/2$. Newton identities yield $L_Q(T)=1-t T^5+3125 T^10$. PARI/GP confirms the corresponding characteristic polynomials $X^10-t X^5+3125$ are irreducible; the native and Lean arithmetic confirms both traces are ordinary and outside the F5 fifth-trace list. By #link("https://pazuki.perso.math.cnrs.fr/index_fichiers/Tate66.pdf")[Tate's finite-field theorem], each quotient Jacobian is in the isogeny class of the displayed elliptic Weil restriction, conditional on the quotient count derivation. [pcc:unverified]

The quotient map $pi:C -> Q$ gives $pi_* pi^*=[2]$, so its pullback is injective on the order-71 and order-47 quotient subgroups. For trace 24, $"disc"(X^10-24X^5+3125)=5^30(24^2-12500)^5$ is prime to 47; integral Tate thus proves existence of a degree-prime-to-47 isogeny from the elliptic Weil restriction to $J Q$. No evaluation formula is known. For trace 2 the discriminant is divisible by 71, leaving this criterion inconclusive. Trace 24 still needs a smooth quotient/zeta replay. Measured full GHS-versus-rho costs are required for a vulnerability claim. The exact pairs are new to this project; prior dihedral work covers the geometric family, and publication priority for these F5 instances remains unresolved. Recent #link("https://arxiv.org/abs/2611.10255")[Kummer/Chebyshev] and #link("https://arxiv.org/abs/2611.14198")[hyperelliptic/Chebyshev] papers concern different displayed equations. [pcc:unverified]

Evidence: `F3125_RUN_V2.stdout`, `ELLIPTIC_RUN.stdout`, `GP_REPLAY.stdout`, `HYPERELL_REPLAY.stdout`, four identity JSON files, and the focused 22-theorem Lean gate. Details, carrier-side maps and per-candidate prior-work checks are in `RESULT.md`, `MAPS.md` and `PRIOR_WORK.md`.
