# Owned replay caller and export integration

This proposed route adds explicit owned replay policy/receipt CLI flags, rejects ambiguous mixed replay requests, detects owned captures from configuration or reserved artifact roles/paths even when report labels are absent, and exports restricted row records through a retained central owner. Unsupported requests pass a separate no-execution/no-overclaim guard.

Prior work inspected: existing sat/search/affine replay callers, actual v5 owned worker selection, existing shared original-cell admission, bounded wire decoder and incremental BQI bridge. The route reuses these contracts rather than adding a second arithmetic proof algorithm.

Focused validation: complete proposed caller and clap-derived DatasetArgs pass rustc metadata typechecking using cached dependencies. The executable harness replaces surrounding command/central types with explicit structural fixtures and removes only the final clap-parser-specific test because the cached clap artifact contains metadata only. Eight caller controls and four actual types.rs controls pass. This verifies caller admission and receipt behavior, not original-cell mathematical replay. The first harness compile failed because its transformation removed the fixture enum Copy derive; the correction and failure log are retained.

Added controls prove run-binding refusal remains sticky after the file is repaired, an expired owner cannot restart, and all three competing replay policy kinds refuse before executing the command. Existing controls cover terminal receipts, exclusive starts, nested invocation, bounded input reads, and retained charges after refusal.

This packet has not received a full crate build or native CLI execution. Prior successful full typecheck applies only to commit 11ad50cd86e6, not this proposal. No scientific dataset or cloud job was created.
