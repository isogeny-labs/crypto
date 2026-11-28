# Inherited ecbench spec repair audit

## Conclusion and scope

The failed original-lock run 45096187034 / job 137436389923 encountered a pre-existing malformed `src/cryptanalysis/ecbench/spec.rs`. The file at CI source `bf56de345c02ef6f1c55bfa047087b0e880cb09e` has Git blob `af585eb7a0b4b8e1a63bea08d1ae6f3222c32702`, identical to main as fetched, the earlier Prime31 checkpoint, and the faulty merge identified below. The new arithmetic examples did not compile in that run.

The proposed repair is a focused restoration: **289 inserted lines, zero deleted lines** relative to that blob. It restores the phase-policy implementation and damaged test fixtures while retaining the binary field-profile implementation. It adds no new arithmetic algorithm, no test exemption, and no validation bypass.

The candidate parses with Rust's formatter. This is a syntax result only: the complete root-lock build and test workflow must be rerun before claiming compilation or test success. The original failed CI receipt must remain intact.

## Git provenance

All sources were read through the GitHub connector. Local files were checked with `git hash-object`, matching the fetched blob identifiers.

| Role | Commit | spec.rs Git blob |
| --- | --- | --- |
| Failed CI source | bf56de345c02ef6f1c55bfa047087b0e880cb09e | af585eb7a0b4b8e1a63bea08d1ae6f3222c32702 |
| Earlier Prime31 checkpoint | 65f413e2ca1014a28e5ba6bb456d599f020b8f29 | af585eb7a0b4b8e1a63bea08d1ae6f3222c32702 |
| Merge introducing this malformed blob | 52e986e114c97b7a46eb156e55b7553e1d5023b1 | af585eb7a0b4b8e1a63bea08d1ae6f3222c32702 |
| Merge parent 1: phase-policy lane | 58356b85fbbb5bd3e2457f15816aac59f57090c5 | 8dcd2a368e81d7c2e2bc0fcb901aa9b484f8d600 |
| Merge parent 2: main / binary field profile | 05714baf67a5eb8a77f53563050fddfef7ab7a07 | 837b2bc18a4f51722f95af27bafd53d7043f9f14 |
| True merge base | ad2a014c0022cf000845014499a9649b6c2cef2d | 43b88239a9465a92c49442da73ad9fd0c1701842 |

The [culprit merge](https://github.com/isogeny-labs/crypto/commit/52e986e114c97b7a46eb156e55b7553e1d5023b1), titled “Merge main into codex/ecbench-canonical-panel-20261118”, is dated 2026-11-22T23:31:19Z in GitHub metadata. Its first-parent predecessor already contains the intact phase tests and policy validator; the other parent contains the intact binary-profile tests. A direct three-way merge using the recorded base produces **three conflicts**: the Spec fields, the identity/validation region, and the appended test region. The retained `spec.threeway.rs` records that reconstruction.

The observed deletions are consistent with an incorrect conflict resolution, but the repository evidence does not establish which editing process caused them.

## What was missing, and what is restored

The damaged test region lost repeated `#[test]`, `&json!({`, schema/measurement/workload fields, function arguments, `.unwrap()` / `.err()` chains, and closing macro/function delimiters. The malformed JSON fixture *source* is not merely invalid test data: deleted Rust structure prevents the module from parsing.

The merge also removed production phase-policy code while leaving a call to `validate_curve_policy(&spec)?`:

- `Spec::require_current_curve_policy()`.
- `validate_curve_policy()`, including the existing phase/panel restrictions.
- Nonlegacy `curve_policy` participation in `Spec::identity_view()`.

The candidate restores those exact lane-parent implementations. It retains the current `plan()` verbatim and retains the main-parent binary-method detector, `validate_field_profile()`, and `plan_for_new_run()` verbatim. Both nonlegacy curve-policy identity and optional field-profile identity are present. No production policy condition is changed.

All **15 distinct parent test functions** are present, including the two phase-policy/smoke tests and the binary-profile test whose attributes or bodies were damaged. A textual preservation receipt records the function-level comparisons. Test preservation is not a claim that tests have run.

### Necessary schema constant reconciliation

In the main parent, `SPEC_SCHEMA` meant `"ecbench.spec/v1"`. The phase-policy lane introduced `SPEC_SCHEMA = "ecbench.spec/v2"` and named the historical constant `LEGACY_SPEC_SCHEMA`. Therefore the restored `binary_groebner_spec()` fixture uses **LEGACY_SPEC_SCHEMA** in its schema field, preserving the original parent's actual v1 fixture value. Its assertions and other fixture contents are unchanged. Keeping the renamed v2 constant there would change the fixture's meaning and fail parsing for an unrelated missing curve policy.

This is the only semantic integration adjustment to a copied parent fixture. It preserves the existing historical-replay versus new-field-profile test boundary; it does not change a production gate.

## Files and verification

- Candidate: `spec.restored.rs`.
- Reviewable repo-path patch: `restore_spec_rs.patch`.
- Candidate Git blob: `ed85acbb242a11a1412da7c842cef70872235dcc`.
- Candidate SHA256: `25c33ec0e00eb358a9ef896cc92a09c108478a43d2a061a3ac389ed9ae41c176`.
- Patch SHA256: `0f88db6da738654f2d150d048ad5f1d1dea5d575f48501188ea6b0456d54603d`.

Parser: `rustfmt 1.11.0-stable (0ef1bc3860 2026-11-09)`.

```sh
rustfmt --edition 2021 --emit stdout --config skip_children=true spec.restored.rs
```

Exit **0**; stdout retained separately as `spec.restored.rustfmt.rs`. The candidate itself is the minimal restoration, not a wholesale formatter rewrite.

The same command against `spec.bad.rs` exits **1**, with mismatched delimiters at **964:45** and **1038:46**, reproducing the relevant original failure locations. Full stderr is retained in `parser_bad.log`.

`git apply --check` succeeds against an isolated tree containing the exact malformed source. Applying the patch there yields a byte-for-byte match to `spec.restored.rs`. The documentation checkout did not contain this source path, so no source file in the actual shared repository checkout was modified.

Nine textual preservation checks passed; their exact names and the 15-test inventory are in `receipt.json`. These checks establish the stated restoration provenance, not runtime correctness.

## Integration requirement

Apply the supplied patch as an explicit inherited build prerequisite, retain the initial failed original-lock CI artifact unchanged, and rerun the original required build/test workflow. Do not label the arithmetic examples compiled or verified until that workflow produces the appropriate evidence. No remote source, branch, PR, workflow, or artifact was changed by this audit.
