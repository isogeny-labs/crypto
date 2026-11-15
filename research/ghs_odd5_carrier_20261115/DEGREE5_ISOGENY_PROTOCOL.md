# Trace +26 alternate-target degree-five screen

Frozen 15 November 2026 before execution. This screen uses the same `Y/F3` and
`K=F3[a]/(a^5+2a+1)` as `DEGREE5_THEOREM.md`. It tests each normal form
`E_b/K: v^2=u^3+2u^2+b` of trace +26 from the committed
`class_count.json`, omitting the already-screened b-mask 3. The remaining masks
are `27,30,103,139,167,182,194,230,233`; mask digits are the five base-3
coefficients of b in powers of a, least significant first. Every target has
order 218 and one K-rational nonzero 2-torsion point.

The source is `degree5_isogeny_screen.sage`. Invoke once per mask as
`sage degree5_isogeny_screen.sage MASK`, capture stdout JSON and stderr without
truncation, and require exit zero and a source SHA-256 bound to the committed
`.sage` file. Reject masks outside the nine frozen values. Keep the exact
denominator shapes and 14,446,377 logical assignments from `DEGREE5_PROTOCOL.md`.
For each target recompute the finite 2-torsion roots, the CRT residue, and the
identity `f*C^2=B*(A^3+2*A^2*B+b*B^3)` for every degree-five numerator survivor.

This protocol is a complete degree-five K-map exclusion for each target only if
all nine runs exit successfully, match the expected counts, and have empty
`hits`. Any hit requires independent replay and subgroup/map verification before
claiming a transfer. A zero-hit result is an exact-target obstruction, not a
general family classification. The nine curves are isogenous over K by their
common ordinary trace; this alone supplies no explicit isogeny or F3 transfer.
