#set page(paper:"a4",margin:16mm,numbering:"1")
#set text(font:"DejaVu Sans",size:10pt)
= Koblitz cover transfers — R32
*Local incidence tangent certificates, with failures retained*

This continuation tests a prerequisite for using R31's surviving degree83
diamond-twist divisors. It differentiates the original five equations in four
variables, then evaluates the complete Jacobian at every retained quadratic
point algebra. No earlier kernel search or point passage is repeated.

#image("round32.svg",width:100%)

At every geometric branch the matrix has rank3, hence the presented tangent
space has dimension1. Complete evaluated matrices and exact unit-ideal lift
witnesses are retained; no factor or denominator is discarded. The zero-minor
family stays proper as a negative control. This diagnostic does not establish
the scheme's local dimension, normalization or proper polarized carrier.

== Failed attempts and exact correction
Two initial invocations were refused before CAS because the resume handoff
lacked required headings. Restoring those headings changed no math or gate.
The first native recorder then failed its explicit witness reconstruction.
Its scalar loop used26 nonzero ideal generators to index a vector with42
slots, truncating coordinates when minors vanished. The runner rejected it
even though Singular returned0 and later printed a terminal marker.

The corrected native matrix product preserves all slots and must equal1
exactly. Both fields then pass. The failed input/log and both pretrial
refusals remain in the raw bundle; no failed run counts as accepted evidence.

== Formal scope and continuation
Two Lean lemmas prove generic unit-identity specialization and a nonzero
minor on every branch. Their assumptions are exact unit identities; native
matrices are not imported into Lean and the local-dimension hypothesis is
not formalized. Pinned Lean4.34.1/mathlibd13f23b and a separate transitive axiom
audit are retained. No proof-assistant geometry claim is made.

Next: establish the required local dimension/standard-smooth presentation,
bind the proper normalization, and evaluate the degree83 twist divisors in
the Jacobian and intended order11 factor. Nonzero formal divisor remains
distinct from nonprincipal class and nonzero modular-factor image.

Reference checked before code: Stacks29.35, Lemma29.35.14
(https://stacks.math.columbia.edu/tag/01V4). Reports are observations only.
No IC timing/rank, attack, global normalization or independent science verdict.
