#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Terminal arithmetic in coefficient construction
T-1892 · Prime-field coefficient path · 2026-11-08

The scalar power helper skips its final unused square. A shared reverse-prefix helper avoids multiplication by the leading prefix1 and the final unused accumulator update. Compiled coefficients, ordinary/shared streams and progression evaluation use the new ending.

#image("flow.svg", width: 100%)

*Power identity.* Positive exponent n now uses bit_length(n)−1+popcount(n) multiplication calls. Exponent0 retains zero calls. An independent fixed64-bit left-to-right oracle checks values, including full-width exponents.

*Reciprocal identity.* Before the final index, the accumulator already equals the first denominator's inverse. Assign it directly. The tail needs 2·max(N−1,0) multiplications; the caller's prefix construction and inverse verification remain. The compiled path also reuses its denominator vector.

*Qualification.* Focused tests exercise 17,556 powers, 96 batches and 464 inverse values, with three malformed-prefix controls. Shared progression qualification remains: 234 setups and 151,020 evaluation values. The native receipt records inherited recovery output equality separately.

*Accounting.* A positive power saves one multiplication; a nonempty integrated reciprocal tail saves two. A batch using both changes saves three by source derivation. Fused/neighbor-specific helpers and source-field arithmetic are separate; no full-recovery or runtime saving is inferred from this local count.

*Evidence.* Reversible source edits and compiler dependency hashes bind the implementation. The exponent95 instrument control is explicitly amended from13 to12 products; old evidence is preserved. Producer assertions await independent signed review. No comparative panel, cloud or timed workload ran.

*Next step.* Rebind the matched width panel to a common updated implementation; retain source setup and outside-engine arithmetic as separate costs. Further structural compression remains part of the active research objective.
