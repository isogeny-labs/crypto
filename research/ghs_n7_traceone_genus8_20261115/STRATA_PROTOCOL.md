# Frozen small-field isogeny-stratum check for the known genus-eight selector

Frozen 15 November 2026 UTC before the trace-distribution run. This is a follow-up to the exact q=2/q=4 S8 coefficient census in `PROTOCOL.md` and `screen.json`. It asks whether the small-field S8 examples contain isogeny classes outside those obtained by base change from the selected base field. A positive result is an exact-curve lead inside Teske's published family, not a new GHS construction or a demonstrated attack advantage.

## Prior work and scope

[Teske's primary presentation](https://www.lix.polytechnique.fr/Labo/Francois.Morain/Crypto/IsogenyParty/teske.pdf) gives the n=7 magic-number-four component selector, genus-seven/eight target, and isogeny-walk extension. The repository's `docs/teske161-selector.md` and KB `source:da469498ce3305b539134232c1172760` implement the selector over F_(2^161). KB searches for exact F128 genus-eight trace-one order strata and genus-eight F_(2^14) classes found the selector and generic records, but no exact small-field classification; absence from that search is not a priority claim. The KB revision is `9141e386254340a9bdba3036f2bb706a`, last refreshed 2026-11-14T21:44:48Z.

## Frozen prediction and controls

Use the exact fields and selected a from `screen.py`: F2[z]/(z^7+z+1), a=1; F2[z]/(z^14+z^5+1), a with mask 5107. Enumerate precisely the previously defined S8 set and count each source curve once. Record, for each q, source trace `t=q^7+1-#E`, group order, coefficient count, full absolute F2-field count, and the lexicographically first b mask in each trace stratum. The coefficient totals must remain 14 and 378, the full-field totals 14 and 364, and every q2/q4 group order must match the saved mod-eight histogram. Reject the run on any mismatch.

For each selected base field, independently calculate the seventh-power Frobenius traces of all possible ordinary base-field elliptic curves in the same absolute-trace-one a branch. At q=2 this set is `{−13}`. At q=4 it is `{87,251}`. A source trace in this set means base-field-isogenous by the finite-field isogeny theorem; a source trace outside it is not base-field-isogenous. This is an isogeny-class test, not a test of exact coefficient descent, and does not determine GHS cost.

Stop after the q2/q4 trace census. If any outside-base-field class appears, select one exact full-absolute-field equation with a prime subgroup for a later explicit map and same-subgroup attack-cost protocol. Do not call the curve or family newly vulnerable from this screen.
