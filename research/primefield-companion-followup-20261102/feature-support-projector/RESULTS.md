# Bounded support-projector extraction

The new optional extractor computes the common-root indicator in the finite
phase quotient, reads its trace moments, reconstructs the exact candidate
polynomial, and uses a singleton image in the existing rational scalar chart.
Runtime takes public points/observations, not a supplied image or scalar.
PCC-20261108-80d4d8 is unsigned producer evidence. [pcc:unverified]

Validation01 passes528 subset-projector controls, including22 singleton
indicators with allL coefficients nonzero. Newton reconstruction and idempotence
are exact for every subset in the selected phases. It reconstructs144 transcript
annihilators equal to the direct three-mode oracle and passes1176 phase-membership
checks. Across those144 cases, maximum entered monomial pairs are67512,
maximum retained node-coefficient slots15558, and maximum temporary support20.
These dimensions are not total runtime or peak memory. [pcc:unverified]

The public adapter checks all19 targets on E/F17 with subgroup order19 and ten
fixed targets on E/F257 with subgroup order281. Of29 cases,18 recover through
projector-derived singleton images and11 through verified identity/endpoint
exits. All29 scalars are checked against the public points; no cases are
unresolved or capped in this set. The adapter receives no fixture image Y.
The29 nominal recovery calls use332 source API queries and689538 projector
monomial pairs. The27 guard checks add27 source queries, for359 including
guards. Source
construction/calibration and other arithmetic are additional charged work.
All27 nonidentity single-call guards pass. No timing ranking. [pcc:unverified]

The separate large-field construction uses fixture-generated observations at
r998244353,m7,L71303168. Under64 terms per sparse map,100000 retained node
slots,2000000 monomial pairs and output degree8, it refuses on the term cap
at14621 entered pairs,5724 retained slots and maximum map support64.
This is a retained bounded refusal, not a large public-point recovery run.
Four independent cap controls (terms, stored slots, pairs, output) and a cyclic
truncation counterexample pass. [pcc:unverified]

The derivation gives coefficient_support*d>=L for a nonzero projector with d
surviving images. In particular a singleton hasL nonzero coefficients. Thus
small survivor count cannot justify an implicitly small sparse indicator.
The bound applies to this explicit representation, not every root extractor.
Next investigate direct trace/moment queries; ordinary low-degree truncation
fails because high-degree products wrap into low coefficients. [pcc:unverified]

Fresh overflow-checked build/check complete within90/60-second limits.
No compile/assertion failures occurred. Ten copied files and15 compiler
dependencies are hash-bound. The prior recovery adapter and4563-case cohort
are unchanged; this standalone component did not rerun the inherited suite.
Independent signed review remains pending. No cloud work was submitted.
