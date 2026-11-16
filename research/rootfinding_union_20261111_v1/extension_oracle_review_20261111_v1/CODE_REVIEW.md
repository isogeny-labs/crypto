# Independent static source review

Decision: ADMITTED_COMPONENT_ONLY after one pre-freeze correction. No compilation or scientific execution was performed by this reviewer.

Reviewed the complete new worker, proof and input rule. The primal and dual block layouts implement the stated coordinate maps, with the required transpose for ordinary coordinate functionals. The membership checker instead computes field products of basis elements through the independent closed-form table oracle. Exact Bareiss determinants and bounded elementary row operations test full-basis equality, including a membership-preserving doubled-row counterexample. Metric controls merely scale/unscale fixed bases; no LLL repair or reduction benefit is claimed.

The candidate uses generic convolution/reduction and repeated synthetic division. Its separate oracle builds addition/multiplication tables with explicit pair formulas, enumerates all 27 original ordered triples, and indexes their elementary-symmetric coefficients. All q³ monic cubics are compared, covering all multiplicities and unsupported/partially supported cases. These are full tiny-domain root sets, not just witness membership checks. Exactly one declared affine functional is exercised across 243 feature tuples per field; all possible functionals are not claimed.

Static review caught an initially incorrect GF4 witness: the odd-field helper models theta²+c and cannot represent theta²+theta+1. The author corrected the witness before freezing to separate binary polynomial multiplication/reduction modulo 0b111. This was a code-review correction, not a failed experiment or a change to frozen evidence.

The fixed loops bound fields, domains, coefficient cases and 128 basis transformations. The 200000 counter measures predicate checks, not all individual field/integer operations. Every predicate checks the internal work deadline. Full root-set records are serialized and exclusively written before the final component flag; the final serialized report is checked at 84/88 seconds, with an external 90-second parent-process deadline covering startup and stdout. Per-process self RSS is a peak process metric, not aggregate descendant memory. No subprocess is launched.

Admission does not assert that the experiment passes; the runner must execute it. It establishes neither an elliptic chart nor original-system completeness, large-field solve, novelty or timing superiority. The coordinates and support routine are correctness components for an extension-field route.
