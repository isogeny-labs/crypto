# Inherited ic_framework merge repair

## Result

The second original-lock CI failure, run 45097885135 / job 137441679059, stopped before tests at src/cryptanalysis/ic_framework/mod.rs:4528:3. The failed checkout was a4ddc035c0c7519d6bab7a438d8a168f74ae40db. Its missing delimiter was independently reproduced by rustfmt.

The same corruption remains in current branch fc184b03d47e13ec922c9b489f3b3f8a976a8bd9 and main as fetched. The current source differs from the failed checkout only by seven module-declaration lines; the malformed validation region is unchanged.

**Candidate repair: four inserted lines, zero deleted lines.** It restores three missing lines in the factor-base x_index bounds check and the closing brace of the lean_layout oracle-compatibility check. The entire direct module parses afterward. This does not claim type checking, passing tests, or a passing original-lock build.

## Provenance

| Role | Commit | mod.rs Git blob |
| --- | --- | --- |
| Failed CI checkout | a4ddc035c0c7519d6bab7a438d8a168f74ae40db | 886e2eea78ca8b895f8b317a4d0cab8093af460c |
| Current branch | fc184b03d47e13ec922c9b489f3b3f8a976a8bd9 | 8c6fbde25df09ad504877f780f4bdbf6acbbd002 |
| Culprit merge | 732322c96041d1be47db7789ec702658cedff8bb | a3ace1f95a97f1f75b72dae090a94f00f5754bc3 |
| Parent 1, input-check restoration | 3636b1520a93311096377b035726fa7bc25009a1 | 84104546f291a009ecfb0a0728d52c57528680ff |
| Parent 2, lean_layout gate | 8104713e049d162577ef18cbf943e20fda3d2e36 | 3680ec50b0ab9826b9d56d83e570b4016973825a |
| True merge base | ad2a014c0022cf000845014499a9649b6c2cef2d | 345e53baaa554e9cccae0751ae6a0edd754d17fb |

GitHub metadata dates the [culprit merge](https://github.com/isogeny-labs/crypto/commit/732322c96041d1be47db7789ec702658cedff8bb) to 2026-11-22T23:20:09Z, titled “Merge main into claude/bloom-hyperloglog-index-calculus-kh3pgr”.

Parent 1's blob is also the exact file at e3b934392b46f2b429254d085d986d9d31d6a205, titled “Restore the pipeline input checks a merge dropped”. Parent 2 contains the intact lean_layout compatibility gate. Subsequent merges, including 77d388738a48eb5a6e2abd80636c2e31cceab254, inherit the defect.

## Honest three-way reconstruction

The recorded parent/base files were reconstructed with:

    git merge-file --diff3 -p -L restoration_parent -L merge_base -L lean_layout_parent mod.culprit_parent1.rs mod.culprit_base.rs mod.culprit_parent2.rs

This produces exactly **one conflict**, at the location where both parents inserted independent checks after factor-base construction. The common-base part is empty. Retaining the entire first-parent check followed by the entire second-parent check resolves that conflict without changing their contents.

The resulting **whole reconstructed module is byte-for-byte equal to the actual culprit merge plus the four proposed lines**. This is stronger than merely guessing a delimiter: it establishes the complete intended parent union for the directly affected module. The current candidate preserves all subsequent source changes and adds only those same four lines.

The reconstructed conflict and union are retained in mod.culprit_threeway.rs and mod.culprit_union.rs. This evidence establishes a faulty conflict resolution, not its authoring mechanism.

## Restored behavior and preservation

The x_index condition regains:

- its separate logical-OR branch, starting with “|| fb”;
- iteration over map values;
- rejection if any flattened point index is outside fb.points.

The lean_layout gate regains its closing brace. It continues to reject any oracle other than mitm-frobenius-layout-counted when the factor-base hash index is elided.

Subgroup/cofactor validation, factor-base table-length checks, column and negation indexes, point_index checks, archive/Frobenius restrictions, rank-audit controls, solver selection, recovery checks, all current module declarations, and all 35 direct-module test functions are preserved. No test attribute or assertion is changed.

## Parser and patch receipts

Parser version: rustfmt 1.11.0-stable (0ef1bc3860 2026-11-09).

    rustfmt --edition 2021 --emit stdout --config skip_children=true <file>

- Failed-CI source: exit 1; original delimiter locations 561, 601, and 4528 reproduced.
- Current source: exit 1; corresponding locations shifted by the seven module-declaration lines to 568, 608, and 4535.
- Candidate: exit 0, empty stderr. All 4,539 lines of this direct module were parsed. Child-module resolution is deliberately skipped; this is not a workspace compilation.

An isolated tree containing the exact current blob passes git apply --check. Applying the patch there gives bytes identical to the candidate. The original failed CI artifacts remain untouched.

Deliverables:

- restore_ic_framework.patch — 786-byte repo-path patch.
- mod.restored.rs — complete candidate.
- parser_ci.log, parser_current.log, parser_restored.log — parser diagnostics.
- static_checks.json and receipt.json — preservation and provenance receipts.

Candidate Git blob: f4f2cce4118d1510fd961cc6d6e0fc0354c257e1.  
Candidate SHA256: 4c8ae812e9f9233797e0131d132f13189cd7157f21318798556d0b57e96b06d0.  
Patch SHA256: c009c70e9b24ff832f78f38f0c6235d99d7635e1ea0f4aaaddeb28a560bda74e.

## Existing regression-test assessment

A small related test is:

    cargo test --locked --test rust_extension_security malformed_plugin_maps_and_invalid_subgroup_bounds_return_errors -- --exact

Its source at the reviewed current commit has blob e4624c681aa9bae7573d381206b8056fe6974b35. It uses roster16 and base size 8, corrupts col_of, asserts the inconsistent-index rejection, and checks subgroup orders 0 and 1. It returns before relation solving. **It does not specifically exercise an invalid x_index entry**, since the earlier col_of condition already rejects its fixture.

The exact lean_layout rejection appears in the existing direct-module test generator_orbit_lean_sib_tau_levers_keep_the_solve_and_remove_work. That combined test also runs n23/n29/n37 end-to-end cases with max_trials=10,000,000. It is not recommended for this bounded prerequisite gate.

No narrower existing exact x_index/lean_layout regression was found in bounded source search. This is a search limitation, not a claim that none exists. No tests were added or executed by this audit, and no broad suite was substituted for the original required workflow.

## Integration boundary

Apply the four-line patch as a separate inherited-build prerequisite. Preserve both failed original-lock receipts and add the repaired source identity to the workflow evidence. The original current-root-lock CI must establish any compilation and test success.

No remote branch, PR, workflow, source, or artifact was changed by this audit. No actual shared-checkout source was edited; all candidate material is in this dedicated audit directory.
