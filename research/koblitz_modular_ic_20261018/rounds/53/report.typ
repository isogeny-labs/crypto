#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= R53: exact defect supports and marked correspondence coordinates
== Complete source-bound trace geometry
All 32 generator traces and all 1, 024 Gram entries agree with an independent
companion-power and quadratic-base trace calculation. Both full source and
discriminant-change products pass; changed entries are rejected. All eight
predictions match in discrim53-02, with
#raw("fA=z^5+z^2+1; fC=z^5+z^3+z^2+z+1",block:true)
#raw("disc(B)=z^76(z+1)^24 fC^12; Norm(Delta)=z^11(z+1)^22 fC",block:true)
#image("defect-support53.svg",width:100%)
Measured complete determinant exponents, not a sample or a timing comparison.
Source: runs/discrim53-02/native.log. The last two adjunctions remove defect
only at fA. Final coordinate defect remains at z=0, 1. Completed local-ring
identification needs branch valuations and residue-field descent.

== Actual denominator and marked model
The composed membership gives the full witness
#raw("(G,N,psi3,alpha)*(Vx,Vy,U,Vz0)^t = Delta^16",block:true)
Alpha is the actual R52 norm coefficient, and the G, N, psi3 quotient has
dimension 64. The complete witness is retained in raw custody. Generic
coefficient denominators and exceptional fibers remain explicit obligations.
#image("construction53.svg",width:100%)
#pagebreak()
= Full integral fiber versus the localized compact equations
At c=1 (A=0), the source equations give
#raw("Q=lambda^4+b*lambda+b^2; G=Q^4; N=b^3*Q^2",block:true)
The prediction N=0 failed. A fresh native check verifies both full ideal
products identifying (G, N) with (Q^2). This compact specialization leaves nu
free: its two-variable quotient has infinite dimension (vdim=-1).

The full three QR equations instead give a finite etale algebra of dimension
16, with a Jacobian-minor unit witness and complete ideal products. This holds
over F2(b) and at the actual F32 base point
#raw("a^5+a^2+1=0; V=(1+a^4+a)/(a+a^2); c=1; b=1+1/a",block:true)
The original base equation and b, c formulas are checked directly. The lambda
quartic factors with degrees 1, 1, 2 over F32, so it has four distinct geometric
roots, two of which are F32 rational.

== The actual twisted fiber
Ordinary nu is descended nu+b*zeta at A=0, where zeta^2+zeta+1=0.
Elimination and both full augmented-ideal products certify descent. The
descended quotient has dimension 16, Jacobian unit, and leading monomials
lambda^4 and nu^4. Its standard monomials are lambda^i*nu^j for i, j = 0, 1, 2, 3.
Thus nu separates four points over each geometric lambda root. The ordinary
and twisted nu equations differ; both are retained, without identifying them.

This identifies the missing integral coordinate data at the projection
fiber. It is a finite-fiber result; direct binding to the normal RR fiber, 
the remaining completed local rings and proper correspondence are next.
Sources: supersingular53-01 and compactfiber53-01, including every full witness.

== Normalized map prefix and its cap
packet53-02 retains the composed inverse, the image line, normalized coordinates
and a native target-Delta unit check. The coordinate export is 78 KiB; the
Delta unit lift was not separately serialized. Direct multivariate modular
powers reached the original 1200 second cap(exit124) before the five target
equations completed. This is a computational cap, not a failed identity.
The next checker can use Frobenius squares and the retained QR/G, N syzygies, 
or the actual degree-8 level-11 base to reduce coefficient growth.
#pagebreak()
= Validation and next construction
The generic marked Tate change uses h=A*r+b, 
s=(r^2+a4+A*q)/h, D2=b+r+s^2+A*s and u=h/D2.
All t, h, D2, u units require full Bezout products. The complete curve change is
oldX=u^2*x+r and oldY=u^3*y+s*u^2*x+q, with
bnew=D2/u^2=h/u^3 and Anew=A/u. See CM.md for actual terminal acceptance.

== Complete terminal attempt ledger
#table(columns:(1.2fr,.65fr,2fr),inset:5pt,[#text("Attempt")],[#text("Result")],[#text("Native / Lean receipt")],[#text("alphaunit53-01")],[#text("accepted")],[#text("exit 0; 0 error lines")],[#text("compactfiber53-01")],[#text("accepted")],[#text("exit 0; 0 error lines")],[#text("discrim53-01")],[#text("rejected")],[#text("exit 0; 89 error lines")],[#text("discrim53-02")],[#text("accepted")],[#text("exit 0; 0 error lines")],[#text("packet53-01")],[#text("rejected")],[#text("interrupted; intermediate custody invalid")],[#text("packet53-02")],[#text("rejected")],[#text("exit 124; 0 error lines")],[#text("supersingular53-01")],[#text("accepted")],[#text("exit 0; 0 error lines")],[#text("tate53-01")],[#text("rejected")],[#text("exit 0; 106 error lines")],[#text("tate53-02")],[#text("rejected")],[#text("exit 0; 8 error lines")],[#text("tate53-03")],[#text("accepted")],[#text("exit 0; 0 error lines")],[#text("transfer-lean53-01")],[#text("rejected")],[#text("compile 0; audit 1")],[#text("transfer-lean53-02")],[#text("accepted")],[#text("compile 0; audit 0")])

Every failed implementation attempt is retained, including late markers after
errors. Exact native limits: seed 53, one thread, 4 GiB virtual, 1200 s. No accepted
producer rerun for publication. Lean uses 4.34.1, pinned mathlib d13f23b, -j1, 
-M4096, 1200s separately for compile/audit, without a virtual-memory cap. Native wall times are recorded in results.json; 
complete CPU times were not recorded. These are exact identity checks, not
statistical or performance comparisons.

packet53-01 has invalid intermediate custody: a live Bash-runner edit caused
interpreter re-entry into a second native invocation. The owned processes
were stopped, all retained files are rejected, and this exception is excluded
from valid historical executed-byte pins. A fresh workflow requires completed
Lean receipts before admission. All 12workflows are terminal, 6 accepted and
6 rejected; 24 valid completed-workflow input pins are verified separately.

The three Lean implications cover determinant change, composed alpha product
and the resulting unit. Actual arrays, trace semantics, source specialization
and geometric hypotheses are external inputs to those statements.

== Next decisive checks
Use the normalized line coordinates (L+s)/u and (L*r+N+q)/u^3. Verify all
target QR identities, the F2 descent and registered denominator specializations.
Then specialize the level11 base and certify intended subgroup action. The
two completed local rings, proper polarization, full quotient saturation and
arbitrary reverse-divisor evaluation retain their separate obligations.

Sources: R50 complete normal RR algebra; R51 line/collision/quotient functions; 
R52 complete nu closure and norm line; pinned donors.json. Established
isogeny model conventions: Sage elliptic-curve isogeny documentation.
Proper/function-field correspondence: Stacks Project tag0BXX. No novelty
claim is made for trace pairings, Bezout composition or Tate normalization.
