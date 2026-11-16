# C71: odd-prime conditional orbit roots and five NIST fields

Hypothesis: the monic degree-12 common-shift resultant can be solved exactly over odd prime fields, retaining all finite original intermediates, and full-field factorization may avoid large interval scans. The supported input fixes three source offsets; this is not a search over independent source coordinates.

Implemented gcd(R,X^p-X), bounded seeded linear-factor splitting, explicit canonical interval filtering, polynomial-GCD auxiliary recovery and exact Tonelli-Shanks roots. Exhausted splitting or excessive free-auxiliary output raises unresolved, never false UNSAT. Characteristic three requires no division by three. A complete finite-difference interval scan is the stronger specialized comparator.

Independent literal enumeration passed 189 small systems and 310,149 shift/auxiliary checks; 427 square-root checks include zero and high two-adic cases. The zero-coefficient singular stress control was retained. All five source-identified standard NIST curve fixtures passed proven-prime, generator/source/target membership and exact equality between factorization and complete interval scanning. Each returned one original conditional root. These are exposed planted correctness fixtures, not hard holdouts.

| Field | Factor solve phase, s | Difference solve phase, s |
| --- | ---: | ---: |
| P192 | 0.053669145 | 0.005842029 |
| P224 | 0.065101214 | 0.006968820 |
| P256 | 0.071056023 | 0.006526834 |
| P384 | 0.149818792 | 0.006639835 |
| P521 | 0.372562669 | 0.008021991 |

Uniform factor-first superiority is refuted on these small intervals: it is slower in all five fields. These are single development phase timings, not cold end-to-end or genuine F4/F5/F6 comparisons. Prime proofs, fixture generation and startup are outside these phase numbers. No qualified ratio is claimed.

Two frozen 90-second experiments ran through checked resume/run-ready under 4-GiB per-process inherited address-space limits and one-core settings. The full worker lifetimes were approximately 0.6843 and 0.9776 seconds. Runner peak RSS was 518,688 and 511,952 KiB, respectively; inherited parent/pre-exec footprint makes these diagnostic lifetime peaks, not isolated solver-memory estimates. Raw outputs, source/input hashes and resource evidence remain preserved.

Primary source review found upstream already implements classical S3, recursive Sylvester resultants and affine-line restriction. The basic resultant/line-restriction construction is retired as a novelty claim. This checkpoint contributes a wide-prime exact conditional backend and correctness evidence; it establishes neither a novel generalized algorithm nor a fiftyfold gain. The full five-source GF(2^83) search, independent-source coverage, odd-extension performance and matched F4/F5/F6 panel remain open.

Next: preregister a five-field interval-size ladder with factorization versus exact finite differences, then freeze a separate cold timing/memory panel and field/domain dispatch rule. In parallel research scope, seek nonenumerative image-cell certificates or sparse block elimination that reduces the true independent-source search, rather than crediting conditional orbit arithmetic as a global reduction.

Budget: the coordinator exceeded the 900-second session cap during analysis/checkpoint closure. Worker limits were preserved. This is an execution-budget failure and is recorded, not a change to the scientific gate. The scientific goal remains active.
