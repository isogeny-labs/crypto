#set page(paper: "a4", margin: 16mm)
#set text(font: "Libertinus Serif", size: 10pt)
#set par(leading: 0.5em)
#align(center)[
 #text(size: 18pt, weight: "bold")[Fixed-label oriented CM lifts]
 #linebreak()
 Exact finite reduction · P-192 primary; five-curve certificate replay
 #linebreak()
 2026-11-08 · T-1888 · PR 3674
]
= Derived search object
For an ordinary prime-field curve, work in the Frobenius order $ZZ[pi]$, with $pi^2-t pi+p=0$, $n=p+1-t>0$ and $d=4p-t^2>0$. Fix the existing rational-subgroup label $mu$ and selected signed Frobenius eigenline. For $alpha=a+b pi$, put $s=a+b$. Substitution and completion of the square give
$ N(alpha) = s^2+(t-2)s b+n b^2, $
$ 4n N(alpha) = (2n b+(t-2)s)^2+d s^2. $
Hence every lift satisfies $N(alpha)>=d s^2/(4n)$, with $s=mu+j n$. This is an exact algebraic identity and bound, without a class-number estimate.

#image("flow.svg", width: 100%)

= Orientation and nearest coefficients
Let the distinct roots modulo the odd prime $ell != p$ be $rho,rho'$ and assume $rho != 1$. The chosen kernel is annihilated exactly when
$ s+(rho-1)b equiv 0 quad (mod ell). $
Set $b_0 equiv -s/(rho-1) quad (mod ell)$. To retain the opposite eigenline require $b_0 != 0$; if $b_0=0$, omit this sum. All remaining coefficients have $b=b_0+ell z$. At fixed $s$, only the square changes, so the minimum occurs at the one or two nearest integers to
$ z_* = (-(t-2)s-2n b_0)/(2n ell). $
Retain both on a tie and set $a=s-b$. The opposite eigenvalue is $b(rho'-rho)$, which is nonzero modulo $ell$. The exceptional $rho=1$ case needs a separate branch; all five current roots differ from 1.

= A complete finite interval
Given any existing oriented lift of norm $N_0>0$, every candidate of norm at most $N_0$ satisfies
$ abs(s)<=B, quad B=floor(sqrt(floor((4n N_0)/d))). $
Enumerate only $s=mu+j n$ in this interval and the nearest coefficients above. The old candidate is feasible, so this covers a global norm minimum within the specified order and orientation. A replay certificate must include and verify the interval's square inequalities, both integer endpoints, every admitted sum and nearest pair, the winner and all label/kernel checks. There is no heuristic omitted tail.

#pagebreak()
= What the point action establishes
The global endomorphism has the straight-line action $alpha(P)=[a]P+[b]pi(P)$. On the rational subgroup it is exactly $[s]P$, with scalar label $s mod n=mu$. Charge the group operations actually used. Changing the CM lift's norm does not by itself change this scalar action or provide a runtime improvement.

Annihilating the certified kernel gives a factorization $alpha=psi phi$ through its separable quotient. The residual degree is $N(alpha)/ell$. This existence statement supplies neither expanded quotient polynomials nor a cheaper point evaluator for $psi$.

The existing explicit closed dual route still proves $sigma eta phi=[ell]$ by full polynomial identity and differential sign. The charged rational-subgroup correction $[mu/ell]$ reaches the desired label; it is not thereby identified with the full global quotient. A new route must retain its endpoint isomorphism and full point action separately from its scalar label.

= Qualification and scope
The native GMP integer replay independently checks the five existing P-192, P-224, P-256, P-384 and P-521 certificates. It recomputes trace, norm, completed square, rational label, selected/opposite eigenline and quotient degree. Fifteen altered norm/label/orientation controls are rejected; final orientation controls substitute the other valid Frobenius root. This is software qualification against published inputs, not an executed minimization, benchmark or formal proof.

The first Boost-based compile failed because its headers were absent; its source and failure are preserved. The corrected implementation uses GMP 6.2.1 and retains dependency/build/check receipts. Invocations have a 120-second ceiling, one process/thread and 2 GiB address-space ceiling. No cloud job or cryptographic clock was launched. The separate modular-recoding report retains the previously frozen 30,780-row count census; it is unchanged by this derivation.

The reduction covers $ZZ[pi]$. A larger endomorphism order may have elements with denominators in this basis; extending the bound needs a certified ring index. The admissible set fixes one eigenline at $ell$; separability and horizontal factorization at other primes remain route checks. Adding restrictions needs a new completeness argument. No minimal-norm five-curve result or efficient quotient is claimed here. Future comparative execution must publish and freeze its source, inputs, finite resources, tie rule and full evaluation costs before data.

= Sources and next construction
The standard degree quadratic form and rational Frobenius fixed-point identity are checked in Sutherland's MIT 18.783 Fall 2023 Lecture 7, pages 1-2. Endomorphism operations and dual cancellation are in Lecture 6, sections 6.2-6.3. Both primary PDFs were read; the completed-square reduction is derived above. No novelty claim is made.

The next construction is a complete integer certificate enumerating this finite interval for the unchanged labels and orientations, followed by a costed executable quotient or other closed route. The current finite-bound derivation is complete at its algebraic scope; neither that next search nor its runtime evaluation has been performed.

#text(size: 8pt)[Primary sources: math.mit.edu/classes/18.783/2023/LectureNotes6.pdf and LectureNotes7.pdf. Full derivation and access gaps: ../../CM_LIFT.md. Inputs: ../../fixtures-v2/published-curves.json and marked-relations.json. Replay: ../../lift_identity.cpp and ../../qualification/lift-identity-v3/. KB unavailable: no loaded MCP tools; migrated scripts/research-kb wrapper absent, exit 127. No successful prior-art absence claim or new ecdb query is inferred.]
