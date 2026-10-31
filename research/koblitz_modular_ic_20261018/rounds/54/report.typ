#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= R54: actual RR fiber and four binary degree-20 evaluators
The complete normal-RR fiber is now identified directly by source matrices.
Four closed source points have binary residue fields and complete function
evaluators. Their orbit traces give a source-bound sign-minus null result.

== Full source and inverse products
All 528 retained multiplication columns bind the unit, lambda, V and actual
descended nu actions. At a^5+a^2+1=0 the idempotent selects the actual
supersingular base point. The 16 monomials lambda^i*nu^j, i, j=0, 1, 2, 3, 
form a complete basis of that component. Both ranks are 16 and
#raw("K F = E; F K = I16; Mlambda K = K L; Mnu K = K N",block:true)
Both descended equations vanish as full matrices; all border products pass.
A changed basis entry is detected. Source: fiber54-01.
#image("construction54.svg",width:100%)

== Explicit equations and larger residue fields
The actual border equations are
#raw("lambda^4+b*lambda+b^2=0; nu^4+b^3*nu+b^4*(lambda+1)=0",block:true)
With w=nu/b, full ideal products give
#raw("lambda=w^4+w+1; P=w^16+(1+b)*w^4+b*w+1+b+b^2",block:true)
P has four irreducible quartic factors over F32. Their five coefficient
conjugates give four distinct irreducible degree-20 norms over F2. Every
factor is checked independently by Frobenius/gcd tests. Each coupled point
has all 32 RR basis values and all 528 multiplication products with full
quotient witnesses: 2,112 products total.
#image("fiber54.svg",width:100%)
All plotted values are exact native data. Sources: fiber54-01, frobfiber54-02, 
flatpoints54-01. Operator ranks are on this F32 fiber algebra, not global
collector or Jacobian ranks. The complete anti-times-trace product is zero.
#pagebreak()
= Complete binary evaluators and restricted obstruction
For each point the coupled binary ideal (a^5+a^2+1, g_i(a, w)) is identified
by both full ideal products with (N_i(w), a+rho_i(w)). All four fields have
degree 20 over F2. The rho degrees are16, 16, 18, 18. All128 RR values become
binary polynomials of degree below20, with full source conversion products.
The fresh checker independently replays all 2,112 RR products and checks
unit and actual source-coordinate labels. Altered values are rejected.

== Frobenius and the failed anti-projected direction
On the complete 16-dimensional F32 algebra the native matrices give
#raw("T=Frob32; T^2=S; T^4=I; S(w)=w+1",block:true)
The operator U=I+T+T^2+T^3 has rank 4, and I-S has rank 8. Their full product
is zero. Thus these closed-point orbit traces are sign-invariant; they add
no directions through the sign-minus projection. A changed trace is detected.
No selected order-11 factor sign or global Jacobian action is inferred.

== Lean and complete workflow ledger
Five authored implications cover basis injectivity, component recovery, exact
range identification, order-four trace invariance and conditional anti-trace
annihilation. Compilation and separate audit exit0. Lean also generated
orbitTrace.eq_1: all six declarations audit with zero failing and zero trusted.
Actual arrays, field arithmetic and normal-RR geometry are external premises.
#table(columns:(1.15fr,.7fr,1.8fr),inset:5pt,[#text("Workflow")],[#text("Status")],[#text("Receipt")],[#text("fiber-lean54-01")],[#text("accepted")],[#text("compile 0; audit 0; 6 declarations")],[#text("fiber54-01")],[#text("accepted")],[#text("exit 0; 0 error lines")],[#text("flatpoints54-01")],[#text("accepted")],[#text("exit 0; 0 error lines")],[#text("frobfiber54-01")],[#text("rejected")],[#text("exit 0; 25 error lines")],[#text("frobfiber54-02")],[#text("accepted")],[#text("exit 0; 0 error lines")])

Four workflows accepted, one rejected. The first primitive run used the
reserved identifier degree; point checks were skipped and its late marker is
rejected. The corrected attempt changes only local names. Immutable copied
drivers and all 29 historical source/protocol byte pins are verified.

Native limits: 4 GiB virtual, 1200 s, one Singular/FLINT thread, seed54. Lean 4.34.1, 
mathlib d13f23b, -j1/-M4096, 1200 s separately for compile/audit, no virtual cap.
Only native wall times are recorded; complete CPU times were not recorded.
No accepted producer is repeated for publication. No comparative speed claim.

== Next construction
Finish compressed global target checking through Frobenius powers and the
QR/G, N syzygies, then the actual level-11/degree-128 source map, F2 descent
and intended 11 action. Test the larger A=0 quotient of dimension 80 and its
possible CRT coordinate model before claiming that global quotient. Determine
actual cusp/infinity poles before declaring a complete principal divisor.
Keep proper polarization, full saturation, larger 112/991 and reverse-class lanes.

Sources: R50/R51 complete RR algebra, R52 actual nu action, R53 descended fiber; 
native linalg.lib; Stacks0BJF; standard finite-field Frobenius/gcd criterion.
See CM.md, PROOF_MAP.md, protocol.json and results.json for premises and receipts.
