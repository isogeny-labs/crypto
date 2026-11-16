# Independent Boolean causal export V2 correction review

Exact manifest a4f876019ee66f23a7c2e73cd1131c8d2b28537691064334b6ea62f8b863c621; full patch d7565a02eca31387c7ffe03858c08dc4ffd098445642bae8b7e1558a87dbd47b; V1→V2 delta576d1f526c9bee59038c6dda8ccc5ab458fc51fccdb5650fbcf5b2e4d5cd5d1b. All8 base/proposed hash pairs verified. Only binary_m3_boolean_replay.rs and export.rs differ from the previously reviewed V1 source. V1 review remains immutable at boolean-causal-export-independent-review-20261102/REVIEW.md, SHA a9c8da129e72b12d57732ba044bc07710923b230ffa3438ebb46580c42eda3da.

## Conclusion

All three V1 findings are resolved at source-review scope. No new blocker identified in these corrections. No build, test execution, production edit or shared target use occurred; this is not executable integration evidence.

1. Final release deadline: production into_rows delegates to a private helper with Instant::now. It prebuilds the bounded restricted Unknown record under the existing prepaid allowance, checks the original absolute deadline, serializes/parses the positive feature and record, then checks the same deadline again. Expiry returns the prebuilt no-feature/no-target/no-receipt Unknown. No encoding or allocation follows the final decision on that exhausted branch. The private scripted-clock test first establishes a positive baseline, then returns start on the first clock call and deadline on the second, asserts exactly two calls, and requires Unknown with absent feature/target/receipt. This exercises the actual post-encoding branch, rather than merely constructing an already-expired seal. Its synthetic source is correctly labeled as an encoding control, not original/proof admission evidence.

2. Failure reason: reserve_projection_output returns ReplayBudget on cumulative reservation failure and ReplayDeadline on subsequent deadline refusal. It reuses the same ReplayAdmission without new caps or clock. Source tests assert the deadline case retains its reservation and the exhausted-work case remains ReplayBudget. When both conditions hold, reservation failure retains the existing budget-first precedence; the reported distinction for the previously conflated deadline-only case is now correct.

3. Policy metadata: structural_allowlist_version uses the actual validated factor_base_policies axis. Valid legacy v1 with no axis remains v6; valid v2 with explicit policy remains v7; requested v3 with explicit policy is v7; requested v3 without a policy axis remains v6. The added test validates each plan before checking the version. This changes the missing v3 metadata behavior without changing legacy valid-plan bytes or public projection fields.

## Preserved scope and pending gates

V1's reviewed private-seal provenance, original-only public allowlist, restricted target/proof/cost storage, same cumulative historical admission, final custody, unknown/zero suppression and conditional legacy file behavior are unchanged by this delta. Exact V9+CLI dependencies remain necessary. Compile and execute original16384 both-twist/F4/F5 nonzero replay/export controls, final-encoding expiry tests, semantic corruption, valid alternative-proof invariance, actual zero-source coverage and output hash/join checks before treating export as functioning dataset evidence. The root's separate integration gate is not claimed as validation of this unapplied source.
