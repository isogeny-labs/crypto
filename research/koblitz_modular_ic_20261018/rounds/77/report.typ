#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= R77: the missing infinity zero of b
The quadratic source equation becomes $U^2+U=t+t^3$ at $t=1/z$, with two etale infinity places $U=0,1$. Exact native quotient checks give $N(B)=t^5$ and $N(B+t^3)=t^3(1+t)^3$ for $B=1+(1+t)U$. Three pinned Lean polynomial and degree theorems compiled and passed an axiom audit with zero failing and zero trusted.

#image("infinity77.svg", width: 100%)

At $U=0$, both factors are units; the norms force orders five and three at $U=1$. Since $b=t^-7 B(B+t^3)$ and $c=t^-4(B+t^3)$, the proper pole degrees in the degree-32 source are 112 for $b$ and 80 for $c$. The second infinity place also contributes a zero divisor of degree 16 for $b$, correcting R76's inference from finite zeros alone. Individual degree-32 infinity branches remain to be split.

One native and one Lean run passed. Eleven historical source-byte pins and five donor hashes match. The additive R76 erratum preserves its sealed original evidence.
