#set page(paper: "a4", margin: 18mm)
#set text(size: 10.5pt)
= Aggregate transcript projectors
T-1792 • 2026-11-07 • PCC-20261107-82ce86 • unsigned producer evidence

The destination extractor now multiplies exact sign indicators directly in the split function algebra F_r[X]/(X^r−X). It avoids enumerating scalar-field values for compiler synthesis and avoids an explicit subset expansion. It is connected to the standalone public-point sign source with mandatory final group verification.

#image("flow.svg", width: 100%)

== Construction
For C=(X+a)^((r−1)/2), the indicators are (C²+C)/2 for sign+1, (C²−C)/2 for sign−1, and 1−C² for zero. Their product is exactly the candidate-set indicator, including every zero endpoint. Two coefficients give candidate count and first moment. After one constraint the count is below r, so its residue is the exact integer count. A singleton's first moment is its scalar. The empty transcript is explicitly excluded from this count rule.

== Software checks
Three selected native methods pass, with 157 imported methods filtered. There are 324 transcript-prefix checks over scalar primes 5, 7, 11 and 31, verifying count, moment, every function value and idempotence against a test-only enumerated reference. Repeats, contradictions, zero endpoints, caps and invalid inputs are checked. All 20 frozen public-point cases at p=509, r=479 recover and pass forward verification; the one-query control remains unresolved. No original-cohort rerun or timing panel.

Twenty-six dependency hashes are pinned. Normal dep-info excludes legacy source/transport paths and quartic/bitplane compiler modules. Test and normal builds retain 16 and 28 warnings. No build or selected test failed. Checks and proof are producer artifacts, not independent review.

== Cost and next construction
Each update requires O(M(r)+r) scalar-field work and retains a coefficient state of dimension r, plus multiplication temporaries. A public inverse-coefficient array costs O(r) setup. Source queries retain their existing cost/domain restrictions; this adapter does not yet use shared windows.

A nonempty proper candidate indicator necessarily has degree r−1 because its leading coefficient is minus its nonzero count. Degree trimming cannot compress it. Its other coefficients encode candidate power sums; next investigate conversion to a low-degree root polynomial once the count falls, charging conversion and subsequent updates explicitly. This is a representation constraint, not a general circuit lower bound.

== Sources and replay
DERIVATION.md gives identities, costs and the validation command context; run validate.py with a new output directory. Existing quotient constructor and CONTINUATION.md guided the approach; KB and the older lifted-moment report were unavailable. Primary sources checked: Sage quotient-ring documentation and van Dam–Hallgren–Ip's shifted-character problem paper. No novelty claim or quantum algorithm.
