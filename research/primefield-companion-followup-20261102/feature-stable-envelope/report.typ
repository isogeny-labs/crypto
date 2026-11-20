#set page(paper: "a4", margin: 16mm)
#set text(size: 10pt)
= Minimal stable envelopes for orbit replay
T-1818 · 2026-11-07 · native producer evidence

A shrinking candidate polynomial G now has a computable minimal orbit-stable envelope inside the original lifted parent L. Start at G and repeatedly take the monic lcm with a rotated copy whose exponent doubles. At most ceil(log2 m) stages cover every rotation; stop earlier when stable.

Every intermediate divides L. The implementation checks divisibility, normalization and prospective lcm degree before multiplication, then computes A times B/gcd(A,B). Zero roots retain their multiplicity; constant1 is already stable.

#image("flow.svg", width: 100%)

*Exact closure checks.* 47,488 scalar subsets over primes7,11,13 and all eligible orbit widths. Whole coefficient vectors match independent root-set closure.23,744 subsets contain zero;47,196 output-cap rejections;16,935 envelopes equal the full parent. These no-shrink controls remain in the record.119 complete sign histories include zero targets and source values.

*Public integration.* All63 fixtures match fixed-parent replay in scalar, route and query fields. Each uses511 source calls and78 retained powers. Across51 retained orbit blocks,26 use a smaller stable modulus and25 retain the original degree. The block closures use40 lcm stages; upfront validation and stability tests are additional work.

*Cost.* Charge root-order validation, rotations/normalization, gcds, exact divisions and divisibility checks. The envelope may cost more than the work it saves. Both dynamic and fixed replay remain available; no timing or total-cost superiority is asserted.

*Evidence.* validation-02, generation.json and DERIVATION.md; passing validation-01 source/output snapshot retained before early exits and exceptional-history controls.15 input hashes match. Source domain and original4,563 cohort unchanged. No cloud work, full formalization or independent signed verdict.

*Next:* derive a degree-aware selector or cheaper image-presence construction before a frozen full-cost comparison. Keep unchanged-envelope cases as controls.

Sources: prior orbit-replay/NEXT_ENVELOPE, local native gcd/division code, FLINT nmod_poly conventions. KB unavailable; accessible official index and local fallback recorded. Polynomial lcm and invariant closure are standard algebra.
