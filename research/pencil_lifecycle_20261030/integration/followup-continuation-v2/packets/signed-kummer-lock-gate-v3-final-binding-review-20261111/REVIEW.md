# Signed-Kummer lock-gate v3 final-binding review

Reviewed at `2026-11-11T00:53:50Z`. No blocking final-binding defect was found.

The release binds the exact source/review publication at commit
`6475e141f9c32a8e8d4a9f8bb929b095b52fd017`; those five source files and the
v3 review match the commit byte-for-byte. That commit is an ancestor of the
current remote PR head `dd5b007040da6f52dfa819c8502c69b9a4787e50`.

The campaign ledger charges the two 600-second attempts and the 2,400-second
pivot attempt, then reserves exactly 60 seconds for this gate: 3,600 charged,
60 reserved, and 125,940 unreserved under the 129,600-second ceiling. The
prior pivot completion hash matches, all three pivot phase owners are terminal,
there are no active pencil units, the new workspace is absent, and the target
unit is `not-found/inactive/dead`.

The exact release hash is
`024105c388aab653a3e045ecd04cd802a6592f9a2529c804d77de30fe4a86459`.
The command binds that hash, permits one attempt, disables automatic retry,
and wraps the offline launcher in a 59-second timeout with a one-second kill
grace. T-2128 was still claimed under the bound fencing epoch at review time;
the saved lease readback expires at `2026-11-11T01:01:18.124092Z` and must
remain live through publication and execution.

This is a final-receipt/admission review only. It does not rerun the v3 source
review, generate a lock, compile Rust, or establish a completed gate result.
The final-binding/release bytes were not yet present in source publication
commit `6475e141...`; root subsequently published them at remote head
`dd5b0070...`, which is the revision that must be retained for the invocation.

