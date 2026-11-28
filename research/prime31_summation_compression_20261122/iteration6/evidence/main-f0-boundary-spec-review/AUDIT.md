# Current-main boundary/spec rebase review

## Disposition

Reviewed main `f0ca664eca0081a30adfcc79dcf62a32de40d2af` using the exact
materialized inputs in `main-f0/`. **No boundary source change is needed.**
All eleven lines from the historical restoration already exist upstream.
The candidate is an exact copy of current main, and the minimal patch is
intentionally empty: zero insertions and zero deletions. Do not apply the
historical patch a second time. [pcc:unverified]

The complete updated boundary and spec modules both parse with exit zero
and empty stderr. This review does not execute tests, type-check the
workspace, validate the new table's behavior, or establish a passing CI
build. [pcc:unverified]

## Source identities

| File | Current Git blob | SHA-256 |
| --- | --- | --- |
| src/cryptanalysis/ic_boundary.rs | 0b0d7fb149307d484075529bfb9852621aaa4ff0 | ee70c40d611bea2a86e782e8513d6e18be49ca575e1159a42512afb18dc346ce |
| src/cryptanalysis/ecbench/spec.rs | 6878f0b872ef6349324e7ba43c8a6d61d7ffcedf | d073eb9f7fb448a7a8fa3d0e3cb7b141e6267153ade2ae74aa8da2250a634dca |

The boundary candidate has exactly the first row's identity. It contains
25,340 lines and 999,509 bytes. The read-only spec copy has 1,407 lines and
57,343 bytes.

## Boundary overlap and upstream preservation

The historical reviewed candidate was Git blob
`d57e6de59cc01de91e860d3ee32dcd849bbdbd67`, derived from the malformed
`9cf97a9f56846ff39b574967a49a0703733ae028` by eleven insertions. Its
source-history audit remains in `ci_ic_boundary_audit/`; its receipt has
SHA-256 `2c63edc5f9324d11bb03cc6eeb5e5992d4d681c1ed87d432c303c9605b347b86`.
That evidence is unchanged.

The first 469,052 bytes of the historical restored candidate, covering all
restored methods, are identical to current main. In a separate scratch copy,
the exact historical patch passes reverse-check, reverse-application,
forward-check and forward-application; all four exit zero. The round trip
returns exactly the original current-main bytes. This confirms that all
eleven restoration lines are already present with their original context.
[pcc:unverified]

Beyond that already-restored prefix, upstream adds the `TwTable`, `TwEntry`
and `TwVacant` implementation and its tests and adjusts existing table-test
iteration expressions. The diff from the historical restored candidate is
286 insertions and five deletions; it is retained in
`boundary-upstream.diff`. No portion of that upstream change is altered by
this review. [pcc:unverified]

All 100 historical test names remain. Current main adds these four names,
bringing the attribute-aware inventory to 104:

- `matches_hashmap_with_exact_hint`
- `matches_hashmap_when_the_hint_is_too_small`
- `matches_hashmap_above_the_prefault_threshold`
- `edge_keys_round_trip`

The current and candidate files are byte-identical, so all current method
bodies, test attributes, assertions and accounting code are preserved.
No conclusion about execution of those tests is drawn. [pcc:unverified]

## Read-only spec review

Compared `spec.rs` against the earlier restored Git blob
`ed85acbb242a11a1412da7c842cef70872235dcc`. The upstream diff is 88
insertions and 12 deletions, confined above the test module. It adds the
`PairedCurvesV1` execution-order/seed policy, its input checks, workload
grouping and match branches. The diff is retained in `spec-upstream.diff`.

The entire existing `#[cfg(test)] mod tests` section is byte-identical to
the historical restored source, including all fifteen test functions.
The complete current module parses successfully; no inherited syntax
omission was found in this bounded comparison. The new policy's semantics
and its consumers outside this file require the ordinary compilation and
correctness gates. `spec.rs` was not edited. [pcc:unverified]

## Reproduction and limits

Run `node ci_main_f0_rebase_audit/audit_rebase.mjs` with the recorded local
input snapshots to repeat the source-only checks. The script asserts exact
Git blob identities, byte preservation, historical patch round-trip,
test inventories and spec-test-module equality, then parses both complete
direct modules using:

    rustfmt --edition 2021 --emit stdout --config skip_children=true <file>

Parser version: `rustfmt 1.11.0-stable (0ef1bc3860 2026-11-09)`.
Child-module resolution is skipped; the direct modules are parsed in full.
The empty `restore_ic_boundary.patch` documents the no-op result and should
not be passed to `git apply` as a new repair.

`receipt.json` records the current-main commit, both exact input identities,
historical audit and patch hashes, parser outcomes, all fifteen spec test
names, four added boundary tests and the round-trip commands. All writes
were confined to this new audit directory. No original input, repository
audit file, PCC record or remote state was modified. No cryptographic
computation, test run or experiment was performed. [pcc:unverified]
