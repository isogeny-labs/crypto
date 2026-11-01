# Lean gate integration and retained proof evidence

This normally merges the exact source heads of PRs 2357 and 2685 and the
Jacobian catalog repair, preserving their original evidence and histories.
The gate explicitly builds every tracked project Lean module, then audits
every built module. The omitted-module control still fails for sorryAx even
when its source is outside defaultTargets. All eight positive and negative
controls pass on both Lean 4.34.1 and Lean 4.19.0. Their raw output and
omitted-module receipts are retained. The first attempt found that the gate
was committed without an executable bit; that exact failure is preserved,
and the Git executable mode is now restored. No axiom restriction is relaxed.

Hosted job 122469853857 in run 40583796366 also completed the conditional
cover-transfer build (1,131 jobs) and printed only propext, Classical.choice
and Quot.sound for pull_injective_on_subgroup. Its five source, toolchain
and dependency inputs match both the retained October 22 receipt and this
checkout byte for byte. lean4checker was not enabled. This verifies that
conditional theorem under the stated pins; concrete per-cover additivity,
composition, kernel and subgroup obligations remain open.

The updated report uses its exact existing diagrams and contains 13 pages.
PDF metadata matches the Markdown coverage marker: 15,791 dated map rows and
979 EC1 bindings. A local asset-path mistake initially omitted the diagrams;
that output was discarded and the hash-verified diagrams were restored before
the final render. Regeneration and the independent verifier preserve all 17
assessment sections; only source-artifact provenance changes. The schema passes.
The exact new PDF is tracked under the existing canonical `*.pdf` keep rule
from owner commit 74a2e09bf78e5b4b6633805be9dd6111dd912bb8. Its obsolete
pointer and managed ignore entry are removed; the old content-addressed object
remains preserved in history. An initial plan to migrate the new PDF was
corrected after checking the current policy. No new policy or label exception,
resealed scientific input or prospective-timestamp claim is introduced.

A subsequent normal merge of main changed only source-artifact provenance in
the assessment ledger. Regeneration and the independent verifier pass again,
with all 17 scientific sections unchanged.
