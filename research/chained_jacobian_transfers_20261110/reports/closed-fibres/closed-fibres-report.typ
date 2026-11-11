#set page(paper:"a4",margin:16mm)
#set text(font:"DejaVu Sans",size:10pt)
#set par(justify:true)
= Closed Richelot fibres and the actual Ψ kernel
2026-11-11 · CT23 · unsigned research checkpoint

The new evaluator recovers actual Richelot images without splitting
quadratic source support. Its targeted comparison passes192 inputs in
each R0/R1 direction. The separate kernel test computes49 actual T0
classes R0(Ker Φ), checks every2401 addition,7-annihilation, geometric
dualR0 norm2 and Frobenius eigenvalues5/6 over the existing F101⁶ field.
PCCdb5cea/b0738e; the all-domain comparison remains pending here.
[pcc:unverified]

#image("closed-fibres.svg",width:100%)

== Exact finite-algebra elimination
For valid u=x²+a·x+b, reduce the existing correspondence
 g1(x)h1(z)+g2(x)h2(z) to A(z)·x+B(z). Put
 R=B²−a·A·B+b·A². The identity

  (A·x+B)(B−a·A−A·x)+A²u(x)=R

shows R vanishes in the fibre algebra. If R has degree4 and A is a unit
moduloR, x=−B/A gives mutually inverse algebra maps between
 K[z,x]/(u,A·x+B) and K[z]/R. This preserves nilpotents and multiplicities.
For u=(x−t)² the resultant is (A·t+B)². No finite-tests-to-global
inference is needed for this identity. [pcc:unverified]

If source_scale·target_scale·v(−B/A) is also a unit, recover the target
ordinate by the original Richelot equation. Runtime checks source u,
the reduced correspondence and the target curve moduloR. The resulting
four-point divisor minus2K has raw balanced plus=−1 and uses the
unchanged reduction previously used by Cantor addition. Lower-degree
supports and infinities reuse the existing completed fibres.
[pcc:unverified]

Degree drops and nonunits reject the chart explicitly. Prime inputs retain
the old completed map as fallback; extension inputs report a boundary.
This does not assert that an unsupported point or map does not exist.
[pcc:unverified]

#pagebreak()
= Kernel, proof scope and continuing checks

== Actual49-point geometric kernel
The retained global square is Ψ·R0=R1·Φ, with degΨ=49. R0 has2-primary
kernel, hence is injective on the49 geometric7-torsion points KerΦ.
Their images are49 distinct points killed byΨ and therefore its full
geometric kernel. The new chart computes the actual balanced T0 divisor
classes in the existing F101⁶ field. [pcc:unverified]

Native02 checks every image,7-annihilation, all49 geometric dualR0
compositions against[2], all2401 additions and Frobenius5/6. The independent
flat-field elliptic route supplies actual7-kernels and checks Φ killing.
PCCb0738e pins a partial process receipt that already contains this test's
PASS/ok; its separate full-domain test is not thereby a pass.
[pcc:unverified]

== Faithful partial Lean proof
Six ClosedFibres.lean identities pass gate and all local declaration
bindings, rootPCCa97f41: quadratic remainder, resultant norm identity,
linear/quadratic source recovery under explicit inverse, repeated-support
resultant and cleared ordinate recovery. Total source census103, zero
trusted native. Full coordinate-ring, scheme/Picard and projective geometry
remain the written bridges in CLOSED_FIBRES.md/rootproof0e5bbf.
[pcc:unverified]

Primary source: Kunzweiler, DCC92(2024), §3.2 Propositions7–8. Its later
degree5/rational-Weierstrass assumptions are not imported as coverage for
our split sextics. Existing native/consumer/generic Richelot and balanced
engines are reused. KB retrieval is unavailable; no novelty is claimed.
[pcc:unverified]

== Current scope and next action
The complete four-domain12317-class comparison is still running. Require
old/closed equality, both dual2 identities, bijections, paired additions,
coefficient descent and wrong-model rejection. Exercise actual geometric
chart boundaries, retain the old evaluator, then integrate and run the
complete changed guard and109-source/Psi controls before publication.
[pcc:unverified]

Read CLOSED_FIBRES_STATUS.md, src/closed_fibres.rs and immutable evidence
with the closed-fibres prefix. This branch starts readyPR1490/994869034.
Parent CI and both frozen-ledger forks remain separate obligations.
Next CT24 retains the further nondual7-hop. S3 admission/livebudget block
benchmarks and paid work. The current result concerns explicit evaluation
and geometric kernel data; development elapsed times do not measure cost.
[pcc:unverified]
