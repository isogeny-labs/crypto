# F631 target order certified; opposite target excludes the source prime

Source freeze1d492f663149, checkpoint20261116T015539Z preceded the once-only two-model gate. Exit0/stderr empty; both fixed targets and all sixteen positive witnesses are retained. No scientific rerun or model/witness-cap adjustment occurred.

## Independent target-group certificate

Kernel0's actual quartic has597counted F631points, so a1=−35. Standard genus3Weilcoefficient bounds put its Jacobian order in [230993922,243613922]. This interval contains exactly one multiple of the verified prime59296481:237185924=4ell.

Using the plane-quartic canonical-class group law, distinct from the source's Cantor implementation, the first16pointdifferences P_i−P_0 are annihilated by237185924. Every cofactor4projection is nonzero and killed byell. Thus a valid nonzero ell-order point proves ell divides the target cardinality; the unique interval multiple certifies #J_target=237185924. Four fixed associativity/inverse controls pass, and ell−1does not annihilate the prime-order witness. Ell exceeds the proper-factor Weil bound, admitting F631simplicity, not absolute simplicity.

run/TARGET-CERT.txt records the interval, prime, unique order, first witness's point indices, canonical class key and Dinf. This supports replay in the same target arithmetic. No independent-host or all-input formal implementation verification is claimed. The stdout label independent_Cantor_source=false means the candidate uses no sourceCantor arithmetic; the positive distinct_plane_quartic_arithmetic=true flag identifies the separate target engine.

## Strong opposite-sign negative control

Kernel1's actual quartic has667points and a1=+35. Its Jacobian order lies in [258865262,271485262]. The first possible ellmultiple index is5and the last is4: there is NOellmultiple in the interval. Therefore the source prime59296481cannot divide this target Jacobian's order. This excludes the source prime subgroup on that model, beyond a mere point-trace mismatch. It does not say the geometric quotient has no interpretation over an extension or through another route.

## Formal interval arithmetic

Intervals.lean checks source_sign_unique and opposite_excludes over natural numbers using the explicit interval/divisibility hypotheses. LEAN.txt/EXIT-LEAN.txt retain outcome/axioms in the pinned Lean4.19/mathlib environment. It does not formalize Weil bounds, Jacobian cardinality, point-count or group-law correctness. The nonzero witness supplies divisibility through the standard group-order argument; target smoothness/genus and geometric order bounds remain mathematical premises.

## Scope and next work

The source-sign target now has an actual verified prime subgroup and certified cardinality matching the source. This is stronger than point-trace matching, but equal orders and a subgroup witness DO NOT establish an explicit source-to-target map. Actual calibration, the two public input images, transfer identities, complete IC and scalar verification remain.

No expected target DLP scalar, old image or logarithm table was supplied. No DLP or timing comparison was performed. The candidate and all certification/control costs remain chargeable. Genus2/4 and the full mathematical tenfold objective remain open; previous failed cold-cost comparison unchanged.
