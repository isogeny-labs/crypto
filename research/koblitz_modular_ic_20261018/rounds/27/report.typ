#set page(paper:"a4",margin:(x:16mm,y:15mm),numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
#set par(leading:.5em)
= Koblitz cover transfers — R27
*Proper source models, complete pole geometry and actual 11-torsion* \
2026-10-28 client date / 2026-10-29 UTC publication.

This report records exact native computations, kernel proofs and labelled
written geometric consequences. IC efficiency and relation-rank assessment
remain with the owner.

The original lambda, nu, S and N divisors are complete under the normal-curve
and DVR bridge, using all 20 cusp charts, the retained SS5/Z5 branches and a
checked simple lambda zero of degree 14. Their pole degrees are 64, 132,
308 and 462. All original residuals vanish at positive absolute precision.

#table(columns:(1.8fr,1fr,1fr,1fr),inset:5pt,stroke:.4pt+gray,
 [*Model*],[*Sections*],[*Product rank*],[*Quadrics*],
 [Source, genus 61],[68],[196],[2,150],
 [Quotient, genus 31],[34],[98],[497])

#image("round27.svg",width:100%)

The source divisor has degree 128; its complete RR basis contains 68 explicit
rational functions. ACTION reconstructs all functions at all 20 charts and
matches the original jet columns. The invariant 34-section space descends
to the quotient divisor of degree 64. Individually framed exports repair a
bulk-list writer that silently omitted its last element.

Every quadratic kernel and pivot is independently recombined. The finite-field
quadratic-generation bound identifies these equations with the prime saturated
ideals of the new P67/P33 embeddings. This does not prove saturation of the
older coupled quotient. See #link("https://ruudp.win.tue.nl/paper/65.pdf")[Marquez-Corbella et al., Proposition 11].

#pagebreak()
= Actual class, proper carrier and reverse controls
The actual principal relation matrix has determinant 1,477,212. Its mod-11
presentation leaves a T=6 line, but a presentation alone could map to zero
in the Jacobian. The completed RR space resolves that gap: the moved divisor
D′ has positive/negative degree 64, satisfies 11D′ = div(f), and its complete
membership matrix has rank 68 and zero kernel. Thus D′ is nonprincipal and
has exact order 11 in the actual source Jacobian.

The recovered class has T=6. Its desired CM-sign projection remains unknown:
the two residual systems overlap at 11. It is not an evaluated Koblitz factor
inclusion or a nonzero target image.

The checked free cyclic-10 tower gives a written proper mixed Prym carrier
of dimension 24, with polarization type (1 repeated 18, 5 repeated 6) over F2.
The actual integral correspondence E=(1−T⁵)(5−N₅) satisfies E²=10E and acts
invertibly on the residual 11-line. The Picard, duality and polarization
arguments are written; no explicit theta sections are claimed. The principal
Prym input in characteristic two comes from
#link("https://www.math.colostate.edu/~achter/math/prym1224.pdf")[Achter–Casalaina-Martin];
the complement argument uses #link("https://www.jmilne.org/math/xnotes/AVs.pdf")[Milne, sections 11–12].

REVERSE02 gives an integral 5-by-5 matrix witness for arbitrary ramified-cusp
coefficients after scalar 134,292. It is not restricted to forward images.
It does not evaluate arbitrary ordinary collector classes.

NODE01 checks all 80 actual Miller-function/conjugate instances across four
graphs. It verifies their proper-chart pole units, group-law lift identities
for both ordinary curves and an explicit nilpotent-unit certificate. A global
original-ring h-power membership certificate is still missing.

LOWGENUS01 excludes the particular genus-7 and genus-13 diamond quotients under
the documented modular decomposition and good-reduction bridge. Other
low-genus carriers remain open. The separate degree obstructions for the
trace-one choices 34, 80 and 110 are exact.

#pagebreak()
= Admitted map solve, proof audit and failures
The full completed RR basis produces 455 F32 equations for equivariant
elliptic coordinates, including nonconstant invariant adjustments and both
ordinary signs. Two independent source-jet specializations verify all 910
checks with zero failures. The admitted Gröbner solve then reaches the fixed
4 GiB virtual-memory cap and exits 14. It produces neither a map nor a unit
certificate. The full equation file is retained; the cap is not increased.

#table(columns:(2.4fr,1fr),inset:5pt,stroke:.4pt+gray,
 [*Native outcome*],[*Count*],
 [Accepted exact runs],[15],
 [Implementation rejections],[8],
 [Expected corrupt-input rejection],[1],
 [Unknown wrapper exit],[1],
 [Registered memory cap],[1])

The 26 native attempts retain every original input and log. Failures include
reserved identifiers, coefficient-field type conversion, a cross-ring control
lookup and the live-launcher edit that lost a native exit. Late success
markers do not override errors. The corrected map admission uses explicit
check counts and a hard stop before solver entry.

Nine proof directories contain eight compiler invocations and one gate refusal.
Final replay lean09 compiles 35 named theorems and audits 35 declarations,
zero failing and zero trusted-native. The old 18-theorem successful replay
is retained without double counting. Proper curves, RR completeness and
polarization theory remain written rather than Lean scheme formalizations.

R22 integral norm-zero recovery d=1 is retained without rerun. The root source
is unchanged since the prior validation: the full library run timed out at
1,200 seconds without a final summary; the targeted replay was 26 passes and
one inherited registry-contract failure. No green full-suite claim is made.

Next obligations are in R29/NEXT.md: colored source lifts and complete reverse
signed divisors; an actual F2 modular-factor transfer with a nonzero 11-image;
old coupled saturation; explicit determinant-19 theta equations; and the
independent model's global rational-function match.
