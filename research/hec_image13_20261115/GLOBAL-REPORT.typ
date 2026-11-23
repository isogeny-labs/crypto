#set page(paper:"a4",margin:17mm)
#set text(font:"Helvetica",size:10pt,fill:rgb("233746"))
#set par(leading:.5em)
#text(size:22pt,weight:"bold")[Global cubic and quartic certificate]

Both genus-3 quotient images · Mathematical supplement · 15 November 2026

*All eight interpolated cubics per quotient are global identities.* For each image, the recorded quartic is the unique nonzero quartic, up to scalar, globally singular along that image. This follows from a source-checked dimension certificate and the exact recorded ranks, without repeating the scientific run.

#image("GLOBAL.svg",width:100%)

*Input already established.* Each quotient A=J/V carries a symmetric principal line bundle M. The eight coordinate sections form an even basis of H⁰(A,2M). Frozen source b1fb249426b6 gives quadratic rank 36, cubic rank 112 and quartic-gradient rank 329. It also checks that every quartic partial lies in the fitted eight-dimensional cubic space and that those partials have coefficient rank eight.

*Tame inversion trace.* Work over an algebraic closure of F127. Inversion has order two and 64 reduced fixed points. A symmetric linearization of M has fiber eigenvalues ±1 at those points; on M⁶ all are +1. Each conormal eigenvalue is −1 in dimension three. Donovan’s algebraic Lefschetz–Riemann–Roch theorem applies to this smooth projective variety and prime-to-characteristic automorphism. Its Brauer-trace local contributions are 1/8, so the exact trace is 64/8=8 in the Witt fraction field. Teichmüller lifts of ±1 are those same integers; this is not merely a congruence modulo127.

*Even-section dimension.* Higher cohomology of M⁶ vanishes: finite étale multiplication by large m prime to127, trace splitting and projection formula inject it into the cohomology of (M⁶) raised to m²; Serre vanishing applies there. Riemann–Roch gives h⁰(M⁶)=6³=216. Inversion is diagonalizable, hence the plus/minus dimensions satisfy a+b=216 and a−b=8. Therefore a=112 and b=104. No Jacobian or complex-field assumption enters this calculation.

*Rank sandwich.* Cubics in eight variables form a 120-dimensional space. Their products of the even coordinate sections land in H⁰(A,6M)⁺, of dimension112, so the global relation kernel has dimension at least eight. It is contained in the sampled evaluation kernel, whose dimension is exactly eight. The two kernels coincide. Thus every recorded cubic is a global identity, also after algebraic closure. No projective-normality or statistical inference is required. The full-rank quadratic matrix likewise excludes every nonzero geometric quadric relation.

*Unique singular quartic.* Exact gradient membership now makes all eight partials vanish globally on the image. Euler’s identity, with 4 invertible, makes the quartic itself vanish there. Any globally singular quartic would belong to the one-dimensional sampled gradient kernel; therefore the recorded one is unique up to scalar. This proves containment in its singular locus, not scheme equality or absence of extra components.

*Provenance and boundary.* The original empirical-only raw flag and candidate report remain unchanged. This explicit post-run proof supplies the extra global argument; it is not independent validation or a benchmark-standing promotion. Original 160 training plus 32 holdout points per image, 1,088 successful holdout checks and 64 corruption detections are retained. Curve recovery, same-field/Serre conditions, divisor transport and complete costs remain. The full mathematical 10× genus 2/3/4 objective stays active.

*Primary sources.* Donovan, The Lefschetz–Riemann–Roch formula (1969), Sections 3–4 and Corollary 5.5; Kieffer, Isogeny graphs of abelian varieties over finite fields, Theorem 1.1.16/proof; prior pinned descent and exact-rank artifacts. Full hypotheses, links and proof are in GLOBAL-CERTIFICATE.md.
