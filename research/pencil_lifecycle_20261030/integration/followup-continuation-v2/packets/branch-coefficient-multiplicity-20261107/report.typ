#set page(paper:"a4",margin:17mm)
#set text(size:10pt)
= Actual coefficientData branch multiplicity
[pcc:unverified] T-1732; 2026-11-07. One prospective Lean declaration. Kernel unverified and independent source review pending. No compiler, axiom readback or experiment ran.
#image("composition.svg",width:100%)
== Construction
The dedicated wrapper instantiates the prior generic support theorem with the actual coefficientData first component b, actual numerator, first and second supports, and actual residual. The polynomial norm identity is supplied by source_norm_identity from b nonzero and the two divisor congruences; it is no longer an input premise of the wrapper.
Separability together with the branch root supplies derivative nonvanishing through WeightedProjQuinticHead. The predecessor then returns all three support polynomials nonzero and the sum of their base root multiplicities equal to one. Its finite stalk-order argument is preserved; this wrapper adds no conversion from infinity to natural numbers.
== Exact scope and source checks
Assumptions: field K, rational branch root, separable quintic, actual coefficientData b nonzero, both divisor congruences, and support-product evaluation zero. No support is selected in advance. No singular-branch or infinity statement is added. No axioms, sorry or native evaluation are introduced.
Read BranchRootMultiplicity, its independent source review, BranchNormTransport, Nonbranch numerator/residual/source_norm_identity, and WeightedProjQuinticHead. Exact paths and SHA256 values are in input-bindings.json. The KB tools were unavailable; these primary sources provide the fallback. No novelty claim is made.
== Evidence boundary and next check
The new module imports the existing source dependency rather than changing frozen files. Elaboration and named axiom readback remain unrun; the complete import closure is not yet bound. Publish the source for independent fidelity review, then perform authorized kernel validation separately. See DERIVATION.md, declarations.csv, result.json and the editable composition.svg. This diagram expresses a dependency composition, not quantitative measurements.
