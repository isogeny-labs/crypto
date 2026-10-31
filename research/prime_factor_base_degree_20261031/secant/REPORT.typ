#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: none)
#align(center)[#text(size: 18pt, weight: "bold")[Split graph factor bases: an exact degree reduction]]
#align(center)[2026-10-31 · two-summand prime-field theorem · external novelty unverified]

== Derived result
Let $E: y^2=f(x)=x^3+A x+B$ over a prime field of characteristic greater than three. A polynomial $h$ of degree $d>=2$ must satisfy $h(T)^2-f(T)=a^2 product_(v in V)(T-v)$ with $2d$ distinct rational roots and nonzero leading coefficient $a$. Retain both signs above every root.

The dense exact system has homogeneous degree $2d+1$. The secant/graph system below has homogeneous degree $d+1$ per branch, retaining all four sign branches. This is a precise presentation-specific reduction; no iterative F4 degree or running-time improvement is inferred.

#figure(image("degree.svg", width: 100%), caption: [Exact derived values for the fixed fixture and matched control. No experimental error bars apply.])

== Exact system and completeness
For a finite target $R=(r,s)$ and each pair of signs $epsilon,eta$, use three variable coordinates $x,z,m$:
$ m^2-x-z-r=0, quad x z-r^2-r m^2-A-2m s=0, $
$ epsilon h(x)-m(x-r)+s=0, quad eta h(z)-m(z-r)+s=0. $
Write the first two polynomials as $g_1,g_2$, $L(T)=m(T-r)-s$ and $c=s^2-r^3-A r-B$. Exact integer coefficient expansion verifies
$ f(T)-L(T)^2=(T-r)((T-x)(T-z)-T g_1-g_2)-c. $
It supplies curve membership and the chord/tangent multiplicities. Complete splitting then forces the two abscissae into the exact base. Conversely every relation with finite target has a finite secant or tangent in one of the four branches. The identity target is handled separately by inverse pairs.

The identity record is `IDC1h621ed1a51b5f3ca8`. Its v1 certificate is replayed; exact expansion over the integers is checked in addition. This classical identity is not presented as novel. Full proof: `SECANT_GRAPH.md`.

#pagebreak()
== Degree proof and explicit fixture
For $d>=3$, the top ideal is $(m^2,x z,x^d,z^d)$. A surviving monomial has no simultaneous positive powers of $x,z$, both exponents less than $d$, and exponent of $m$ at most one. Its degree is at most $d$; $x^(d-1)m$ survives. Hence the first zero homogeneous component is degree $d+1$. Lean checks the universal exponent argument; the polynomial and group bridges are written.

For $d=2$, the top ideal is $(m^2,x z,x^2-alpha m x,z^2-beta m z)$. Every cubic monomial vanishes and the quadratic quotient has dimension two. Thus the degree is three. A native certificate checks all ten cubic monomial identities and a quadratic lower dual witness in each of the four branches.

The explicit model `icv1-fp5-tm1-a77cc6af` has $p=17,A=B=2$, full group order 19 and generator $(5,1)$. Take $h(T)=4T^2+9T+9$ and $V={5,6,10,16}$. Exact factorization is
$ h(T)^2-f(T)=16(T-5)(T-6)(T-10)(T-16). $
The signed base has eight points and four columns, `FB1hed5b19904cc8`. Its dense degree is five and secant degree three. The same-size control has abscissae ${0,5,6,10}$, oriented interpolant $8T^3+6T^2+7T+6$, exact additional membership equations, and secant degree four. Its FB1 is `FB1h66fbb7ed3fb6`. Registry and EC1 joins remain pending in the fixture metadata.

== Verification and open obligations
Twenty-six standalone package tests pass, including seven new secant tests, two homogeneous-certificate tests and nine reused identity-certificate tests. Strict Clippy, formatting and release build pass. Four new Lean declarations check without custom axioms, sorry or native evaluation; the full polynomial-ring proof is not formalized.

All four sign branches are charged. The candidate uses one more variable than the dense presentation. Complete splitting, base construction, natural relation yield, F4 behavior, width, cost and a growing-size ladder remain unmeasured. No P-256 or 17-term result follows from this two-summand theorem. The experiment checkpoint gate failed for missing upload access; no panel was launched.

Prior work: #link("https://people.maths.ox.ac.uk/petit/files/16PKC_primeECDLP.pdf")[Petit–Kosters–Messeng, PKC 2016] and #link("https://math.mit.edu/classes/18.783/2023/LectureNotes2.pdf")[MIT 18.783 group-law notes], plus the repository's 24-design register, PKM notes and symmetric implementations. Riemann–Roch/Nagao antecedents were identified; some primary full texts were inaccessible. Literature-wide novelty and independent mathematical review remain open.
