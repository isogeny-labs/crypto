#set page(paper: "a4", margin: 18mm)
#set text(size: 10pt)
= Reflection-odd coefficient solver
2026-11-05 · T-1631

The cubic coefficient solve now uses reflection symmetry to retain only the target's odd sector. At r=479/q=16, the system has239 rows and344 feature-orbit columns; the reconstructed packet remains an exact identity in the original variable.

#image("pipeline.svg", width: 100%)

== Why the reduction is exact
Center the variable at c=(q-1)/2. Reflection X → -X-(q-1) becomes Z → -Z. It maps a feature indexed by S to epsilon^|S| times its reflected subset, where epsilon=chi(-1). The span is invariant and contains constants. Therefore X is representable exactly when the centered odd target Z is representable in the odd projection. Pair features, retain reflection-fixed odd features and discard fixed even ones. Solve only odd polynomial coefficients, expand orbit coefficients and subtract c.

== Checks
All99 optimized native tests pass with overflow checks, including the reference suite and36 small-field/window feasibility comparisons. The q15 odd control remains inconsistent with observed rank238. The q16 solve has observed rank239 and produces479 terms. All479 field inputs and eight source integrations pass. Independent unreduced-polynomial replay verifies the exact X identity and rejects a reflected-partner sign corruption. Twenty source/data hashes bind the run; eight compiler warnings are retained.

== Cost and scope
The augmented matrix contains82455 field entries, from the derived239×345 dimensions. The exact-run diagnostic records7227137 inner elimination updates, excluding other construction and source work. No comparative performance protocol or timing panel ran. The packet has one more term than the previous478-term packet and still requests16 source signs. Dense elimination remains; next investigate structure within the odd system. Original4563-case cohort unchanged.

== Evidence and prior work
DERIVATION.md, claims.json, control.json, source-lock.json, receipts and logs. The sign law is standard: #link("https://dlmf.nist.gov/27.9")[NIST DLMF27.9.1]. The order-two projection proof is given explicitly; no novelty priority claim is made.
