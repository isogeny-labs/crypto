#set page(paper: "a4", margin: 19mm)
#set text(size: 11pt)
= Homogeneous normalization seeds
Prime-field source construction · 2026-11-03

Prepared windows retain seeds as numerator/denominator pairs. The recurrence and a new homogeneous consumer carry these pairs to the sign predicate without converting them back to field scalars. Raw scalar-source functions remain reference paths.

#image("pipeline.svg", width: 100%)

== Construction and tradeoff
For the scalar update Delta times S times previous squared divided by older, use numerator Delta times S times previous numerator squared times older denominator, and denominator previous denominator squared times older numerator. This has six products and no inverse; the previous expression had four products and one inverse. Independent input rescaling preserves the quotient.

Scaled division values, recurrence squares, partial tails and singleton factors all propagate their denominators. The consumer adds denominator arithmetic; stored seeds use two residues. Other source inversions remain. A complete-cost comparison is still required to assess the combined tradeoff.

== Checked behavior
One optimized native build and one bounded invocation pass all 148 methods, none filtered. Checks include 20736 independently rescaled recurrence cases, nine scaled-source cases, two rejection cases, seed/phase comparisons and all eight complete recoveries. All 27 prebuild source/data hashes pass. Scalar raw-parts and recovery code are byte-identical to the prior component.

== Scope and next step
No comparative panel or cloud job ran. Next instrument the complete homogeneous path, including added denominator work. Previous comparison identities and the original 4563-case cohort remain unchanged. See DERIVATION.md and raw receipts. Standard homogeneous-scaling context is documented by #link("https://www.hyperelliptic.org/EFD/g1p/index.html")[EFD].
