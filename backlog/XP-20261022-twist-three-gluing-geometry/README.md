# Indecomposable gluing, a genus-2 curve and explicit maps

**Current status, 2026-10-23: partial / blocked / unfunded.** Four parameter presentations per source and explicit degree-three maps pass native/PARI checks. Toy divisor transfer and 24 distinct source-verified destination recoveries pass. Polarized graph identification, second-map transfer, formal bridges and full costs remain open.

[Current bounded results](../../research/twist_routes_1_3_20261021/reports/followup-20261023/E19.md) · [Raw publication/extraction status](../../research/twist_routes_1_3_20261021/execution-v2/README.md). Local phase receipts do not complete the original experiment.

The original 2026-10-22 design and full requirements follow. Its “not started” wording records that historical design date.


**E19: proposed, blocked, not started.** Route 2; priority P1.

[Full finite design and controls](../../research/twist_routes_1_3_20261021/proposals-20261022/README.md#e19).
[Parent route card](../XP-20261021-twist-cover-transfer/README.md).

## Question and decision value

Does an E18 anti-isometry yield an indecomposably principally polarized quotient with an explicit same-field genus-2 model and subgroup transfer?

A fully checked toy model becomes an E24 arithmetic input and a future destination-solver input. A failed anti-isometry closes only that candidate. P-521 construction success remains a public-parameter result, with no NIST DLP execution.

## Inputs and method

Process every E18-certified anti-isometry on the three positive toy models, up to the four candidates reported per model. Retain every rejected candidate. Attempt at most the four public P-521 candidates only after the same formulas and checker pass the toy stage.

Construct the graph quotient of E times its twist, certify the induced principal polarization and geometric indecomposability, descend a smooth genus-2 curve to the declared base field, and provide quotient/pullback/norm maps with their actual degrees. Prove each composition identity and the exact kernel intersection with the source prime subgroup; normalize recovery by the actual coprime degree. Do not infer degree or injectivity from the numeral 3 alone.

## Controls and evidence owed

Include a documented reducible/product-polarization example, a non-anti-isometric graph and identity/ramification cases. Compare independent point counts and divisor-map replay. A product principally polarized surface, matching j-invariant or characteristic polynomial cannot pass the smooth-curve certificate.

Polarization and kernel certificates, curve equation, smoothness/genus/infinity checks, explicit rational maps, exceptional charts, exact subgroup order, scalar recovery and all failures. Initially measure construction and map costs only; no Jacobian-IC improvement follows from existence.

## Prerequisites

Explicit gluing/polarization/model/map constructors and independent replay are missing.

Dependent designs: E18.

## Resources and cost derivation

Proposed pilot: one CPU, 4 GiB and 3,600 seconds per anti-isometry. Stop that cell on a singular model, failed descent, failed map identity or resource cap; retain the failed certificate.

These are proposed process caps, not calibrated runtime/disk estimates. Full
core-hour and wall-hour estimates remain null until a pilot and runnable
protocol exist. Funding is zero, below the existing funded seed cards; no
allocation, lab publication or spending is performed. The card retains an
intentional failing command and is not runnable.

## Before ready and before done

- [ ] Implement native Rust driver and an independent checker.
- [ ] Freeze exact inputs, source, controls, seeds, counters, complete cost intervals and resource estimates.
- [ ] Commit and hash PROTOCOL.md; pin merged code; fill checkpoints and real verifier/run argv.
- [ ] Validate the ready card and preserve every failed or inconclusive cell.
- [ ] Land the bounded result, independent replay, report, diagram and inspected PDF.
- [ ] Meet the stated decision criterion; distinguish construction, stage and pipeline results.

Novelty and any IC speedup remain unestablished. The original E00–E17
requirements are preserved. This card records a proposal, not a completed run.
