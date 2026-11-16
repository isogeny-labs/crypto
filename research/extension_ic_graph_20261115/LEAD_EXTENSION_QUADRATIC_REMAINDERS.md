# Unselected next lead: keep a quadratic divisor remainder

Static source review only. No implementation, protocol, yield measurement,
performance estimate or novelty claim. Trial39 remains unchanged/frozen.
[pcc:unverified]

`TwoLargePrimeSieve::verified_relation` removes the small Fp roots, calls
`low_degree_roots` on the remaining factor and then requires every root to
belong to the full Fp-abscissa base. `low_degree_roots` refuses a quadratic
with nonsquare discriminant in Fp. `relation_from_hit`, the original
full-smooth extractor, also requires all m roots in Fp. Thus the current
reduced-base graph path still admits only relations whose full support is
inside that original Fp universe. [pcc:unverified]

Candidate: after extracting seven Fp roots at m9, retain an irreducible
quadratic remainder U as a Mumford divisor D=(U,V) over Fq=Fp² rather than
splitting it into Fp roots. Actual source conventions are
A=a0+t·s·a1, C=sqrt(s)·B, ordinate y=−A/C and F=A²−C²H in Fp[x]. For a monic
quadratic U dividing F and coprime to C, compute V=−A/C modulo U; require
V²=H modulo U. The proposed relation is the seven signed small-point
classes plus D, derived from the principal function A+C·y. The degree-nine
pole and all support/multiplicity/exception conditions must be checked.
This construction and its cheap verification still need implementation and
independent Jacobian controls. [pcc:unverified]

Use a canonical D or−D descriptor, with its orientation carried as a signed
coefficient. Equal verified remainder classes could become graph vertices;
original partial relations attach small-point terms to one such vertex,
and repeated descriptors can cancel. D may represent a composite divisor,
so call it a remainder-class vertex, not an asserted prime divisor. Do not
identify conjugate points or classes without proving the required equality.
[pcc:unverified]

Competing hypotheses: additional accepted partials and cheap polynomial
certificates reduce collection work; or the larger vertex universe, few
collisions, verification and filtering cost erase that benefit. A future
matched-cohort test must include both successful full-Fp rows and all
remainder partials, unique descriptors, collisions, projected rank, complete
certificate replay, actual algebraic work and failures/caps. More partials
alone are not success. A full target solve and matched original IC/rho
reference remain separate obligations. [pcc:unverified]

Initial verification obligations: exact norm identity including the source
line's sqrt(s) convention; monic/squarefree/coprime support; every selected
point and sign; V²−H modulo U; principal-divisor equality checked against
independent Jacobian arithmetic on controls; canonical orientation and
collision combinations; protected descent target/RHS; dynamic vertex
labels and bounded resources. Replacing Jacobian checks by polynomial
certificates requires an explicit correctness argument and controls, not
silently removing verification costs. [pcc:unverified]

The existing m8 option also has a different finite B-space: source
`b_space` gives ceil(p/2), whereas m9 has p². Lower degree alone cannot be
assumed to provide enough independent relations; measure rank/yield if that
variant is selected. No m8 experiment is selected here. [pcc:unverified]

Sources checked: actual jv_sieve.rs SieveSetup/b_space/relation_from_hit/
relation_from_roots and jv_sieve_two_large_prime.rs low_degree_roots/
verified_relation; JV primary sections3.2/3.3. KB unavailable; exact/negative
receipts in validation/extension-remainder-kb-*.log. The lead is standard
large-prime/remainder cancellation in the existing cover setting, without
an asymptotic or general-curve claim. [pcc:unverified]
