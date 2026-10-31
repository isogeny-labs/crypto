#set page(paper: "a4", margin: 15mm)
#set text(size: 10pt)
= R52: exact coordinate closure and larger source field
The complete nu adjunction stabilizes at index length 180, with least recovery
denominator z^5(z+1)^9. An explicit degree 3 line norm supplies generic packet
transport. Complete good fibers certify a degree 112 larger source field by
monic specialization; all 3,810 stored factorizations are freshly checked.
#image("leads52.svg", width: 100%)

== Complete coordinate algebra
Starting from S=F2[z][V, lambda] inside the retained rank 32 normal-chart B,
all 32 actual nu-multiplication columns and their full source products pass.
Both matrix products at every adjunction step and final stability pass.
#image("closure52.svg", width: 100%)
The exact determinant is z^60(z+1)^120. The full 32x32 recovery product uses
the least scalar z^5(z+1)^9; predecessor nonzero remainders prove minimality.
A fresh process reconstructs every closure and source-action product.
#table(columns: (1fr, 4fr, 1fr, 1fr), inset: 5pt,
 [Prime], [Nonzero elementary exponents], [Zeros], [Length],
 [z], [1x4, 2x6, 3x6, 4x4, 5x2], [10], [60],
 [z+1], [1x4, 3x4, 4x4, 6x4, 7x4, 9x4], [8], [120])
All 14 local levels retain both products. The complete types match the candidate
(7,11) semigroup model (30 gaps, conductor 60), but actual completed local-ring
identification still requires branch valuations and residue-field descent.
The four coordinates recover the normal source on z(z+1)!=0, under the retained
normal-RR bridge. Omitted fibers, infinity and proper polarization remain tasks.
#pagebreak()
= Explicit line norm and the larger marked-23 field
== Actual generic line transport
For f=y+lambda*x+nu and the degree 3 kernel {0, Q,-Q}, the full polynomial product is
#raw("NormNumerator = alpha*Ynum*H^3 + beta*Xnum*H^4 + gamma*H^6",block:true)
Here H=x+t and alpha is the original line cubic evaluated at t. Consequently
Y=(beta/alpha)X+gamma/alpha on alpha!=0. Changed intercept is rejected. The
sign action preserves alpha, changes beta by A*alpha and gamma by b*alpha.
The fresh checker reconstructs the actual translated line functions and Velu
coordinates before checking the norm product and all 3 sign products.

Next certify alpha on the actual QR packet, normalize the marked point to
Tate form, check all 3 target QR equations and gamma/F2 descent, including alternate
charts and exceptional branches. The intended 11 primary action needs the
actual integral/odd-adic correspondence module.

== Larger source: two reducible patterns suffice
The retained marked23 base field K has degree 7 over F2(x). Its actual twisted
QR polynomial is monic degree 16 and nu is linear with nonzero A²Delta. Both
fixed probes enumerate every coefficient-field element and every smooth
linear base root with registered denominators and A*Delta nonzero.
#table(columns:(1fr,1fr,1fr,1fr),inset:5pt,[Field degree],[Good fibers],[8+8],[2+14],text("7"),text("224"),text("224"),text("0"),text("8"),text("264"),text("88"),text("0"),text("10"),text("880"),text("220"),text("440"),text("11"),text("2442"),text("242"),text("484"))
The small range has 488 good fibers and no 2+14 pattern. Scaling gives 3,322 good
fibers, 462 patterns 8+8, 924 patterns 2+14 and 2,024 odd-degree factors over F2048.
Every 3,810 source specialization and each stored factor's irreducibility are
freshly checked. Full factors and source coordinates are retained.

At a checked smooth base place, the local ring is a DVR and G's coefficients
are integral. A monic global factor is integral there, so reduction preserves
its degree. The 8+8 fiber allows degrees 0, 8, 16; the 2+14 fiber allows 0, 2, 14, 16.
Only 0 or 16 survive, proving irreducibility and degree 112 over F2(x). A degree-dividing-10
base place rules out constant extension 7. An odd-degree 11 packet place then
rules out a nontrivial constant field dividing 16, so the full constants are F2.
The DVR/monic-specialization and place interpretations are external standard
arguments in CM.md; the polynomial implication is formalized separately.
Normalization of the 112 field and its selected 991 action are the next checks.
#pagebreak()
= Receipts, formal scope and continuation
Terminal attempts: 17; accepted: 12; rejected: 5. All failed attempts remain in raw custody.
#table(columns:(2fr,1fr,2fr),inset:3pt,[Attempt],[Status],[Native receipt],text("annihilator52-01"),text("accepted"),text("exit 0"),text("closure-lean52-01"),text("rejected"),text("compile 1; audit not run"),text("closure-lean52-02"),text("accepted"),text("compile 0; audit 0"),text("closure52-01"),text("rejected"),text("exit 0; native errors"),text("closure52-02"),text("accepted"),text("exit 0"),text("fibers23-52-01"),text("rejected"),text("exit 0; marker absent"),text("fibers23-52-02"),text("accepted"),text("exit 0"),text("fibre-lean52-01"),text("accepted"),text("compile 0; audit 0"),text("filtration52-01"),text("rejected"),text("exit 0; native errors"),text("filtration52-02"),text("accepted"),text("exit 0"),text("fresh52-01"),text("accepted"),text("exit 0"),text("largefibers23-52-01"),text("accepted"),text("exit 0"),text("normline52-01"),text("accepted"),text("exit 0"),text("rect-lean52-01"),text("accepted"),text("compile 0; audit 0"),text("reload52-01"),text("rejected"),text("exit 0; native errors"),text("reload52-02"),text("accepted"),text("exit 0"),text("semigroup52-01"),text("accepted"),text("exit 0"))
Nine declarations compile and pass the axiom audit with zero failing or
trusted declarations. Closure52 proves range/stability/least-closure and
unit-scalar implications plus the pattern intersection. FibreIrred52 proves
polynomial irreducibility from the degree restrictions and the 7/10 constant
divisor step. RectBridge52 covers the actual rectangular generator products. Actual native matrices, DVR specializations, source geometry and
selected-factor actions are external; PROOF_MAP.md gives the precise boundary.

Native runs: seed 52, one Singular/FLINT thread,4 GiB virtual and 1200 s each.
Lean4.34.1/mathlibd13f23b,-j1,-M4096,1200 s separately compile/audit, no VM cap.
A native exit 0 or a late success marker never overrides error lines or a missing
required marker. The first fiber loop was manually stopped after a documented
continue-semantics bug; the corrected complete range was run once. Other
rejections retain their source, logs and partial certificates.

Evidence: results.json, receipt-validation.json, CM.md, PROOF_MAP.md and the
per-stage frozen protocols. ARTIFACTS.md records immutable byte custody and
fresh download/repack; that is separate from the native fresh checker and
external independent semantic review. Standard sources and exact references
are recorded in CM.md. Existing sealed R27-R51 records are unchanged.

Continue with actual cusp valuations/completed subrings, normalized QR/Hecke
transport,112 field normalization and 991 action. Preserve quotient saturation,
proper polarized embeddings, node187 coupled divisors,132 section RR intersection
and arbitrary reverse collectors, with Klein n3/samefield n5 controls.
