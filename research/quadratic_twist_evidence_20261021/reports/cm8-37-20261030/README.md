# F37 twist transfer and complete size-two policy frontier

2026-10-30, same PR2381. [Inspected three-page PDF](report.pdf), [recorded-count graph](frontier.svg), [map diagram](bridge.svg), [native audit](../../native/cm8-37-20261030/audit.json), [validation/export/controls](../../validation/cm8-37-20261030/README.md), [Lean source and proof map](../../lean-cm8-37/README.md).

The new fixture is E/F37: y²=x³+7x+19 and its actual nonsquare d=35 monic twist T: y²=x³+28x+33. Both have38 rational points. The preregistered lexicographic generator rule gives P=(1,8), U=(32,29), order19. These exact unregistered model tuples identify the synthetic inputs; no registry alias or endomorphism-ring classification is asserted.

## Verified reduction to the original DLP

For w=1/(x−4), Phi(x,y)=(x+18w,y(1−18w²)); origin and K=(4,0) map to origin. For v=1/(x−29), D(x,y)=(28(x+35v),14y(1−35v²)); origin and target kernel(29,0) map to origin. The inverse on the cyclic19 subgroups is I=[10]D. C++ and independently implemented Rust verify every38+38 point image, both compositions=[2],1444 addition pairs per map (2888 total), and all19 scalar images/inverses. The false[9]D differs on all18 nonzero subgroup points and an altered inverse payload rejects101/no receipt.

New Lean definitions use actual Mathlib nonsingular points and these coordinate formulas. The unmodified scoped gate passes **19 public named theorems /22 audited declarations**, zero failing or extra trusted/native axioms; only propext, Classical.choice, Quot.sound. Lean4.34.1/Mathlib d13f23b723b8a846827a245b89c10fc7d3f11612 are pinned. The source-DLP equivalence is for an arbitrary actual Q with explicit existential membership in <P>: Phi(Q)=nU iff Q=nP, n:Fin19. The unknown index is not passed to recovery. Scalar transport, cyclic injectivity, subgroup addition/inversion, nonsquare coefficients and a necessary short-change coefficient obstruction are checked. This is not generic isogeny classification or a fresh combined/global audit. Native addition covers the whole38-point groups; the new Lean addition theorem covers the19-subgroups. The nonzero2-torsion kernel explicitly refutes global inversion.

## Complete bounded solver result

All36 size-two subsets of nine unsigned subgroup abscissae per curve, with all36 matched and36 pullback controls:144 arms. Both original/reduced encodings and caps0..8 produce2592 configurations and46656 separate configuration-target attempts. They repeat18 original-source points across policies, encodings and budgets.

| Independently reconstructed result | Count |
|---|---:|
| Verified original-source recovery runs |19440|
| Blocked public-log setups |1512|
| Policy/target/budget comparison records |3779136|
| Higher rank and lower requested cap, recovery admitted |55230|
| Such records beating strongest canonical source policy |0|
| Genuinely changed corruption payloads rejected101/no receipt |10|
| Shared backend tests passed |22|

55230 counts pair records, not distinct targets or wins. Lower requested cap need not imply lower actual solving degree. The complete source frontier absorbs every unmatched gain. No stationary useful-yield, intrinsic regularity, runtime, rho, fastest-external or complete-cost gain is established. The smaller F13 cohort is not a matched speed reference.

The new versioned F37/r19 instrument receives S3 plus both product-membership equations. Exact original-input witnesses and bounded Buchberger checks certify all generators/S-pairs before root extraction. Matrix degree and expanded verification-witness degree fit each rung. Above-bound shifts are denied before construction; an unknown cap admits no roots or rows. An already-computed smaller witness is retained only for an identical normalized polynomial within one query solve. Exhaustive reference roots do not control candidate stopping.

Actual ordered sign lifts are checked. Public setup derives logs only from admitted relations and verifies the base points; incomplete rank blocks all18 targets. Recovery receives actual unknown Q, makes fresh shifted solves, preserves all failed/identity shifts and verifies the scalar on original E/P. Output-reference binding is after each fresh solve, unpriced, and supplies no arithmetic answer. Direct enumeration is a separate generic decomposition reference with no named matrix degree. Setup, allocation, field/group and verification work lack a calibrated total price. Size3..9 bases are outside this new preregistered size-two cohort; the old F13 full scope stays frozen.

## Evidence and deviations

The first complete native attempt passes. Nine census corruptions and one false-inverse payload genuinely change/re-hash and reject101/no receipt. All2592 canonical case hashes, exact membership, raw commands/exits, and altered payloads remain. Nine full corrupted trees reconstruct byte-identically from canonical cases and17 changed overlay files/removal lists. Raw107 files restore to exact hashes and repack to identical9668-byte archive. The canonical export has planned3/attempted3/completed3 full-contract arm records, no exclusions: producer/auditor each invoke transfer+census, controls are nested, not independent statistical samples.

Initial Lean setup failed before elaboration cloning a new Mathlib checkout. An exact manifest and ignored existing dependency cache fixed setup without modifying gate/trust. Attempt02 records explicit gateexit0/PASS/22 audit despite outer PTYexit1; unchanged-source attempt03 explicitly exits0/PASS. Both records remain. The first report overflowed; exact source/PDF/log remain, followed by a clean three-page render inspected page by page. The first export used string notes instead of schema-required note objects; exact failed export/log remain, corrected metadata passes without scientific changes. A [dated erratum](../../CM8_37_20261030_METADATA_ERRATA.md) preserves one copied prior-F13 prose ordering phrase in the frozen JSON; question, arms, parameters, Markdown, sources and assertions already declared the new full F37 scope. No criterion or data changed.

All18 prior identity records remain unchanged; no new universal polynomial identity or fake group certificate. Root/library/workflow/registry/catalog/benchmark/leaderboard and old frozen outputs are unchanged. The5140-test library suite remains600s/124 incomplete; no redundant rerun. Conductor127/no lease, missing1921 registry records, naming, required exact-head CI, review, integration, geometry, probability and full-cost gates stay open. Keep the PR open, non-draft, auto-merge off.

## Next bounded step

The next discriminating test is the **complete size-three cohort** on this verified route:84 canonical bases per curve plus matched/pullback controls. It needs a new versioned instrument and preregistered contract. Cubic membership leaves S3's degree≤2 per variable unchanged, so original/reduced equality must be an explicit control. It is not implemented or executed here. The actual twist advantage over the strongest source remains open. A null size-two comparison does not abandon the program.

The atlas query for this step matched isogeny-transported relation yield, curve-presentation invariance and decomposition probability. No novelty/disposition promotion. Source: [primary Sage isogeny blob](https://api.github.com/repos/sagemath/sage/git/blobs/972f41162b3b237a216a3f1a53b1386e301ac2c6), `compute_codomain_formula`/`compute_vw_kohel_even_deg1` lines238–311 and `__velu_sum_helper`2183–2229, plus prior actual CM8Transfer/CM8Dual/ActualRowYield and bounded-witness sources. Only source excerpts are stored; no Python crypto was executed.

Study-local graphs are updated. Canonical cost/leaderboard graphs have no validated complete-cost row to update. Global progress exporter is not tracked in this PR head; daily global integration remains open rather than adding unrelated main tooling. Scientific window13:40–14:40UTC, two existing local workers,600-second command caps, no paid compute or arbitrary result upload. Approved exact-branch checkpoints succeeded before launches; private token and broad credentials are absent from repository evidence.
