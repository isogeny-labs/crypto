# Main-f0 framework repair: pinned-source revalidation

**The existing four-line patch applies to the exact current-main source without rebasing.** On pinned main `f0ca664eca0081a30adfcc79dcf62a32de40d2af`, a direct GitHub file fetch confirms `src/cryptanalysis/ic_framework/mod.rs` has blob `8c6fbde25df09ad504877f780f4bdbf6acbbd002`, byte-identical to the previously audited baseline.

The candidate remains `/workspace/scratch/2f841b0b9a19/ci_ic_framework_audit/mod.restored.rs`, Git blob `f4f2cce4118d1510fd961cc6d6e0fc0354c257e1`, SHA-256 `4c8ae812e9f9233797e0131d132f13189cd7157f21318798556d0b57e96b06d0`. The patch remains `ci_ic_framework_audit/restore_ic_framework.patch`, SHA-256 `c009c70e9b24ff832f78f38f0c6235d99d7635e1ea0f4aaaddeb28a560bda74e`.

## Fresh checks

- The entire source difference is **four inserted lines and zero deletions**: the missing logical-OR/`values`/bounds-predicate in `x_index` validation, and the closing brace of the `lean_layout` oracle gate. No arithmetic, relation generation, accounting, test, module declaration, or other policy code changes.
- The complete restored factor-base validation guard is byte-identical to the guard in parent `3636b1520a93311096377b035726fa7bc25009a1` (file blob `84104546f291a009ecfb0a0728d52c57528680ff`). Inconsistent x-coordinate index entries therefore continue to be rejected.
- The complete restored `lean_layout` guard is byte-identical to the guard in parent `8104713e049d162577ef18cbf943e20fda3d2e36` (file blob `3680ec50b0ab9826b9d56d83e570b4016973825a`). A factor base with its index elided still requires `mitm-frobenius-layout-counted`.
- All **35** `#[test]` declarations are unchanged, and the entire source suffix beginning at `#[cfg(test)]` is byte-identical. This is a static inventory, not a passing test count.
- A fresh full-module parser check with `rustfmt --edition 2021 --emit stdout --config skip_children=true` fails on the pinned baseline (exit 1) and succeeds on the candidate (exit 0, empty stderr). The formatter output is retained only as audit evidence; the candidate is not reformatted.

The historical provenance remains in `ci_ic_framework_audit/AUDIT.md`: merge `732322c96041d1be47db7789ec702658cedff8bb`, with the two parents above and merge base `ad2a014c0022cf000845014499a9649b6c2cef2d`, dropped these lines while combining adjacent independent validation changes. The retained three-way union reconstructs the complete malformed merge plus exactly the four missing lines.

## Scope and reproduction

Run `python /workspace/scratch/2f841b0b9a19/ci_main_f0_framework_review/recheck.py` to repeat the exact source identities, whole-diff restrictions, both parent-body comparisons, test preservation, and current/candidate parser controls. `receipt.json` records the results and parser commands. SHA-256 of that receipt: `d4aba083ece580161b875b5dffa2961f9e0712bd51cc555451b0fb38e721efee`.

This is a revalidation by the original framework-repair investigator against a newer pinned main, not an additional independent author review. Root already reviewed the earlier candidate independently. No remote resource, checkout source, lockfile, workflow, ledger, or test was changed by this revalidation. No Cargo build or cryptographic execution ran. The next original-root-lock CI must establish type checking and test results at its own recorded checkout.
