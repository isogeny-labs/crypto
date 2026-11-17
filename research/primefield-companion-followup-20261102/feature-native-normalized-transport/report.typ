#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Normalized division-value transport
2026-11-05 · T-1703

The new primitive transports fixed-index division values across public shifted points. Let T_n(R) = psi_n(R) c(R)^(n squared), using the existing source normalization. It uses the geometry lanes already needed by the half-product source. [pcc:unverified]

#image("pipeline.svg", width: 100%)

== Cancellation and recurrence
The ordinary division-polynomial addition identity contains the denominator (x(P)-x(R))^(n squared). Raising the existing seed recurrence to n squared cancels it. The resulting identity is

T_n(R+P) T_n(R-P) = (x(nP)-x(nR)) T_n(P)^2 T_n(R)^2.

This is conditional on the stated seed law and inherited phase-domain assumptions. The note derives this reduction, not a new proof of the derivative-normalization law. It uses algebraic specialization of the addition identity, with no analytic computation or characteristic-changing map. [pcc:unverified]

If the two predecessor pairs are [A,B] and [C,D], delta = x(nP)-x(nR), and K = T_n(P)^2, the next pair is [delta K A^2 D, B^2 C]. This requires six products and one coordinate subtraction, with no inversion or large power in the recurrence step. [pcc:unverified]

== Exceptional points and checked scope
Public setup retains anchors at O,P,-P,2P. Anchor precedence prevents zero predecessors across identity. At most offsets zero and one use direct initialization when they are not anchors. Geometry construction, setup, initial phases/values and allocation remain costs. [pcc:unverified]

Fifteen native tests pass. Complete subgroup fixtures over F5/r7, F7/r13 and F509/r479 check 1,490 normalized values against direct jets. Eleven length-16 shifted windows check 528 further values, including identity crossings. Scale-invariance, bounds and invalid inputs are tested. Eight source/data hashes bind the run; four compiler warnings remain. [pcc:unverified]

A separate checker imports no transport code, reconstructs public points by affine addition, recomputes values with frozen jets, accepts all 1,490 rows and rejects a changed finite numerator. It was authored in the producing session. [pcc:unverified]

== Integration dependency
The current residual expects unscaled values with factored seed dependence. It cannot substitute these normalized pairs directly. Next factor that residual and reuse initial phases/geometry, then account for the complete source. This artifact is the transport primitive. The original 4,563-case cohort is unchanged; no cloud or timing panel ran.

== Evidence and sources
Claims PCC-20261105-421d0b and PCC-20261105-cef8e8 have local verdict not_covered and no signed standing. Sources: saved seed/geometry code, #link("https://dlmf.nist.gov/23.10.E3")[DLMF23.10.3], and Stange2011 Sections3-4. KB unavailable: wrapper executable absent. No novelty-priority claim.
