#set page(paper: "a4", margin: (x: 17mm, y: 16mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 10pt)
#set par(leading: .55em)
= Koblitz cover transfers: Round25
*An explicit original-source quotient and direct norms* \
2026-10-28 UTC. Exact algebra, native certificates and kernel proof records;
no IC cost, new relation rank or official scientific promotion claim.

== The actual quadratic action
In the unchanged degree16 incidence field L over
$E: v^2+v=u^3+u$, put $A=1+c$. The action
$sigma(lambda,nu)=(lambda+A,nu+b)$ squares to identity and exactly preserves
all five original equations, all eight registered entries and the literal
generic nu equation. Its invariants are
$Z=b+lambda^2+A lambda$, $U=A nu+b lambda$, $V=nu^2+b nu$.
Native reduction gives U and V as degree7 polynomials in Z divided by
$A Delta$ and $A^2 Delta$, respectively. The actual retained quadratic
extension identifies the fixed field as $K=F_2(E)(Z)$.

== The integral chart retains A=0
Thirteen polynomials Q are explicitly extracted from the original five
equations and eight entries, with exact pullbacks
$Q(A nu+b lambda,nu^2+b nu,b,c)=b^d P(nu,lambda,b,c)$.
Their b-powers are0,0,6,7,8,0,0,0,0,0,0,3,2. Full coefficients are in
registered-quotient.sing, copied unchanged from the accepted native output.

Let B be the quotient by Q1,...,Q5 localized at Q6,...,Q13. Since b=Q6 is
a unit, the original registered source algebra is explicitly
$T=B[nu] slash (nu^2+b nu+V)$, with $b lambda=U+A nu$.
This monic rank2 presentation has unit derivative b, hence is finite etale.
Writing each element as p+q nu shows that the sigma-invariants are exactly B.
No inversion of A is used, so the chart remains valid at A=0. These scheme
consequences use the written ring-isomorphism argument in CM.md; the full
proper boundary and proper polarized embedding remain open.

#image("round25.svg", width: 100%)

#pagebreak()
= Norms, verification and remaining acceptance
The exact quadratic formulas are
$"Tr"(lambda)=A$, $"N"(lambda)=Z+b$, $"Tr"(nu)=b$, $"N"(nu)=V$, and
$A^2 V=U^2+A b U+b^2 (Z+b)$.
For p,q in K, expansion gives the written general formula
$"N"(p+q nu)=p^2+b p q+V q^2$.
It supplies a function norm once arithmetic on H is available. An arbitrary
divisor-class reverse evaluator is still required.

== A necessary subgroup test
The proper normalized degree2 map has $pi^* pi_*=1+sigma$ on Picard groups
under the written curve correspondence. Thus its norm can kill the desired
order11 factor when sigma acts as minus1. The next acceptance check is an
actual subgroup or matrix certificate for sigma and F2 Frobenius. The
projector6(1+sigma) modulo11 would isolate a nonzero plus part if present;
no such factor action or modular-Jacobian identification is inferred here.

== All attempted work is recorded
#table(columns: (2fr,1fr,2.6fr), inset: 5pt, stroke: .4pt+gray,
 [*Record class*],[*Count*],[*Outcome*],
 [Native invocations],[8],[2 accepted,5 implementation rejections,1 expected rejection],
 [Gate-only refusals],[2],[No native input/output],
 [Storage prelaunch failure],[1],[/tmp inode exhaustion; task-owned /var/tmp used],
 [Lean attempts],[2],[1 rejected elaboration,1 accepted14-theorem audit])

The corrupted actual invariant has residual1. Native exit0 and a later
marker occur, but the wrapper rejects it as required. All accepted native
blocks require zero residuals, native exit0, marker and no error diagnostics.
The implementation failures are reserved identifiers or map syntax, not
mathematical obstructions. Caps stay4GiB virtual/1200s. No migration, new
EKS job, duplicate Diem, fifth task claim or PR merge occurs.

Lean4.34.1 / pinned mathlib audits14 new theorems,0 failing,0 trusted-native.
Invariant, inverse, involution, trace/norm and mixed identities are formalized.
The actual function-field, normal-curve, affine-scheme and Picard bridges
are written, not kernel formalized; see PROOF_MAP.md. R24 separately records
14 audited theorems, four rational infinity branches e1,1,7,7 and actual
coordinate/denominator valuations; its source and70-member archive are unchanged.

The integrated root library run hits1200s, exit124, with no full summary.
The final27-test diagnostic passes26 and fails1 inherited registry contract
(unknown ic.hyperelliptic). Exact fixture restoration changes no root code
or expectation. This software evidence is separate from algebra/proofs and
does not establish a green full suite or completed CI.

== Open construction requirements
Resolve A0,B0,C0,D0 and enumerate additional zero supports; complete global
divisors must precede a complete prescribed Riemann-Roch space and the F2
factor-map test. Node187 still needs source-bound membership/separator or a
complete coupled arc. Full quotient saturation/proper polarized embedding,
arbitrary reverse evaluation and ordinary460/367 images remain open. R22
integral augmentation recovery d=1 is preserved and not rerun.

Sources checked before approach: R22 generic/tower certificates,R23/R24
geometry and #link("https://stacks.math.columbia.edu/tag/09I7")[Stacks09I7],
#link("https://stacks.math.columbia.edu/tag/03PA")[03PA],
#link("https://stacks.math.columbia.edu/tag/0BXX")[0BXX]. Reproduce using
PROTOCOL.md, REPLAY_NATIVE.sh, REPLAY_LEAN.sh and VERIFY_RETAINED.sh.
