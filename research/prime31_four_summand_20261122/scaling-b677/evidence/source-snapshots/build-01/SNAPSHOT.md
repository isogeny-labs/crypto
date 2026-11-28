# First-build source snapshot

Captured on 2026-11-23 UTC, before correcting the eighth Y7 coset constant.
These are direct byte-for-byte copies of the unchanged working files, not
reconstructions. Both source files were untracked when the first build ran, so
the final Git commit alone would not preserve their failing versions.

The hashes in `SHA256SUMS` were compared against
`../../build-01/pre-command-sources.sha256` and independently recomputed after
the copies were made. They match the first build's recorded inputs exactly:

| Original source | SHA256 |
|---|---|
| `native/b677_fixture.rs` | `93dfb0ab3e7d11912cf020e6c28990600fecd99ad2e3a52aaa8f47587ee54897` |
| `native/b677_controls.rs` | `ed992f0443bcba8c83f1b0c3dc39bbb0f3715e72ab4a32bddc6df5ef82839de5` |

These copies retain the rejected value `1852955275`. The native test failure
and the independent scalar diagnostic remain in the original immutable receipt
directories. The correction rationale is in `../../../SOURCE_CORRECTION.md`.
