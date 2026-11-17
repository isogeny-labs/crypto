#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:9.4pt)
#set par(leading:0.6em)
#set heading(numbering:none)
= Two unbalanced-transfer models over F101
2026-11-12 · model phase A · native34 / algebra23 / two bound Lean identities

The complete changed suite passes34 tests,zero failures/ignores/filters;
oldguards,formatting,strictClippy and all622 source/fixture/engine hashes
pass. Full02/925b02 retains whole82,084-byte output as standardXZ9,716,
with raw/archive hashes,source snapshots,status and exact roundtrip.
Selected01/feff57 is one diagnostic with33 filtered. [pcc:unverified]

#image("transfers.svg",width:100%)
Actual equations and transported two-graphs. Both leading and constant
coefficients are nonsquares;no normalization or retwist is applied. Solid
boxes are native model/cover/graph checks. Dashed arrows are global written
quotient maps;whole Jacobian evaluation is phaseB. [pcc:unverified]

#image("counts.svg",width:100%)
Recorded exhaustive F101 curve counts,not sampled values or timings. Both
models have N1=120,N2=10436 and Jacobian order12317. Both infinities and
both x=0 points exist over the declared extension and are checked even though
none is F101-rational. All six Weierstrass roots are checked. [pcc:unverified]

== Exact covers and graphs
For f=c6*x6+c4*x4+c2*x²+c0,the direct quotient uses (x²,y) and cubic
coefficients[c0,c2,c4,c6];the reciprocal uses(1/x²,y/x³) and[c6,c4,c2,c0].
Short coordinates are X=a*u+b/3,Y=a*v,followed by X→s²X,Y→s³Y.

#table(columns:(1.3fr,1fr,1fr,1fr,1fr),inset:4pt,
  [Cover],[Target],[a],[b/3],[s],
  [C10 direct],[F0],[61],[70],[43],
  [C10 reciprocal],[E1],[89],[31],[43],
  [C20 direct],[F0],[18],[25],[33],
  [C20 reciprocal],[E2],[40],[15],[33])

All coefficient identities and every rational curve point image pass.
The source graph is independently extracted from[2]Q0 using actual T0
Cantor pulls and compared with C0's old root pairs. Actual transported
graphs give Gamma21/Gamma6;the four-graph Q20 is A6 up to relative sign.
One-factor7 four-transport is Galois stable but fails anti-isometry in
96/256 product-pair comparisons,all nonidentity values-1. This is the
credited CT44 obstruction in actual coordinates. [pcc:unverified]

#pagebreak()
= Global proof, formal scope and next maps
Canonical product pullbacks P_i have degree4 and norm[2]. Their exact
transported kernels give quotient maps F10 P0=P10(alpha,id) and
F20 P0=P20(alpha2 alpha,id),of separable degrees7/49. The adjoint norms
descend diag(7,1)/diag(49,1),so they are nonuniform on the full Jacobian.
Degree coprimality preserves both109/113 DLP subgroups. The written proof
128eb1 uses actual covers,Picard pullbacks,optimality and the quotient
property;it does not cancel maps in finite point groups. [pcc:unverified]

These C10/C20 models are geometrically distinct from the earlier six by
their intrinsic factors and canonical sum degree4 versus16. They are two
additional explicit presentations inside the fixed72-Jacobian catalog;
its previous figures and counts retain their scope. [pcc:unverified]

== Faithful local formal checks
UnbalancedCovers.direct_short_quotient/1bde23 and
reciprocal_short_quotient/d03d51 prove the universal cleared cubic-to-short
identities over every commutative ring. Both statements are exactly bound;
gate01 passes two/zero trusted native. The project has171 written
declarations in34modules;the new bulk compiled count is still pendingCI.
Actual variety/Picard/quotient-degree bridges are written,not LeanAVtheory.
[pcc:unverified]

Algebraf404e3 passes23 tests and11 input guards. Seven certificates are
emitted;the five old certificates replay byte-identically. New
IDC1hdb5d8c25ad6d1262 and IDC1h15a63844aad1bcd0 each have32issuer+64extra
checker points over2^61-1 and the final Lean-source digest. Evaluated
certificates and local Lean acceptance supply no scientific signature.
[pcc:unverified]

== Exact remaining implementation
PhaseB must adapt the existing sextic chart to nonsquare leading
coefficients over Fp12,compute cover norms from the actual input Mumford
divisor,and execute both maps/duals/recovery on all12317 rational classes.
The actual7/49 kernels,Frobenius and all exceptional fibres stay required.
UNBALANCED_PHASE_B_NOTES.md preserves formulas and falsification controls;
the predicted uniform-norm mutant counts have not been executed.
[pcc:unverified]

PhaseC must construct T20 from the actual image of Q20 in C20,identify its
unique Richelot matching,and check the full primitive-four-glue transfer.
Global model existence and graph matching do not complete these maps.
No computational advantage,solver cost or timing claim is supplied by this
model-construction component. [pcc:unverified]

== Sources and reproduction
Sources:Bruin–Doerksen0902.3480 Lemmas2.4/2.6,Definition2.7,Theorem3.2;
Galbraith25/26;Milne degree/polarization/quotient results as cited in
ISOGENY_CYCLE_PROOF_V2.md;exact externalCT44cc5d5541,original blob retained.
Characteristic101 conditions checked;characteristic0 Proposition2.8 unused.
KB MCP absent/wrapper unavailable/atlas encrypted;no novelty inference.
[pcc:unverified]

Native command:cargo test --release --offline --locked -- --nocapture,
15-minute bound plus oldguards/fmt/strictClippy and622 before/after hashes.
Rust1.99.1,Lean4.35.0. The focused workflow's algebra-certificates job
replays23 tests and compares seven exact outputs.
REPORT.md links raw custody,proofs,statement bindings and full sources.
Editable DOT/SVG,recorded counts/renderer and this PDF are retained beside it.
Root51d ownCI allfourjobs successful2c4744;consumerf656 fullCI pending.
All reported scientific results remain unsigned. [pcc:unverified]
