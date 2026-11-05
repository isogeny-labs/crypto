# Polynomial residual and divisor acceptance

The composed Lean model now connects canonical polynomial negation, padded addition, multiplication, monic remainder and divisor acceptance. This is a source-shaped value proof; no scientific experiment was executed.

Residual construction: five main declarations establish that the stored output is canonical and decodes to f minus v squared. Twelve controls and helper declarations passed. Acceptance: six main declarations and eight controls passed. Separate axiom readback covered 31 declarations across the two components, using only propext, Classical.choice and Quot.sound.

Assumptions: prime modulus with 0 < p < 2^31; canonical input words; monic divisor. The validity bridge also requires u.length = d+1 and v.length <= d. Completeness requires fuel at least the constructed residual length. Soundness of a successful acceptance does not require this fuel bound. The equivalence retains the native degree and leading-word guards explicitly.

Controls include empty inputs, unequal padded lengths, trailing zeros, full cancellation, identity, linear and quadratic acceptance, nondivisibility, excessive degree, nonmonicity and insufficient fuel. All failed development attempts remain archived: residual1, controls1 through controls4 in the residual producer; guards1 in acceptance. Final checks have unchanged pre/post input bindings.

Native boundary: list semantics model the sliced u and v records. This does not establish equality of ignored record tails, allocation success, instrumentation-counter safety, dispatch correctness or compiled Rust equivalence. Imported compiled dependencies remain a trust boundary. Independent replay passed 11 main declarations, 20 producer controls, six reviewer controls and 37 axiom readbacks. A reviewer boundary case confirms the external v-length premise is necessary. The initial reviewer reduction failure is preserved and excluded from passing evidence. Source-model fidelity is scoped; the full review workflow and official scientific transition remain incomplete.

The scientific campaign deadline was 2026-10-31 06:00 UTC. No new cohort rows or measured cost ratios are supplied by this proof packet. The full 1,260-target requirement remains unchanged.
