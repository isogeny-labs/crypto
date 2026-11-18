# Degree-and-root certificate lemma

Mathematical deduction [pcc:unverified], not a novelty claim.
Let f be monic degree5, u monic degree2, deg(v)<2, and u divide f-v².
For t in the basefield set w=v+t*u. Then u divides f-w² and the quotient
h is monic degree3. Let x1,x2,x3 distinct with u(xi) nonzero and
yi=w(xi), yi²=f(xi). Hence h(xi)=0. The degree/root bound forces
h=product(X-xi), so f-w²=u*product(X-xi) identically.

This establishes the same principal-divisor identity as explicit polynomial
multiplication on the disjoint-support domain. It is not equivalent for
unrestricted overlapping support without multiplicity qualification.
Counterexample: u=X(X-1),v=1,t=0,
f=1+X(X-1)(X-2)(X-3)(X-4), supportx0,2,3. Allthree points satisfy
y=1 and lie on f, but the quotientroots are2,3,4. Omitting u(xi)!=0
would falselyaccept the claimed factorization. Keep that guard.

Verifier validates everycachedanchor once using Mumford divisibility,
checks degree/root/nonintersection conditions perrow, and the unchanged
folding verifier checks supportpoints lie oncurve. Together these are
the lemma's hypotheses. No probabilistic identity test or sampled
evaluation substitutes for its degree argument. Repeated residualroots
are explicitly unsupported/rejected; producer already uses three distinct
nonintersecting supportx values. Anchor u itself may have repeatedroots.

Background: divisor/Pic0 relation conventions in Milne chapterIII and
existing principal-pencil source; https://www.jmilne.org/math/CourseNotes/AV.pdf .
Only elementary divisibility and root bound used. Action andwholeprime
premises unchanged; the nativeactioncheck is not removed by this lemma.
