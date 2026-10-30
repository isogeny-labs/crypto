# Native copied-control custody helper

Frozen before edits. Implements the bounded copied-cell mutation/resealing
component required by factor-base-native-acceptance-completion-plan-01/DESIGN.md,
slots1-5,7-10. Source contracts/literature already frozen there and in the native
factor-base/SAT contracts. No arithmetic algorithm or scientific claim changes.

Use the existing native controller's checked_file, bounded JSON decoder, SHA
helper and artifact_rows verifier. Operate only on an exclusive copied cell;
reject use against the original and reject symlinks/nonordinary files. Compute
all semantic mutations in memory, enforce per-recipe exact file allowlists,
validate both original outer manifest and worker registry before editing, then
update modified artifact hashes/sizes in the worker registry and outer manifest.
Removing a companion removes both its file and all corresponding registered rows.
Result mutations update manifest.outcome identically. Independently read back
both levels after writing; return precise mutated paths and before/after digests.

This helper alone does not admit a source as mathematically verified, execute a
negative CLI control, classify a semantic rejection, or implement all16 slots.
The eventual workflow must select only predeclared verified native rows, preserve
original snapshots, enforce overall copy/storage/command caps, record all16 slots,
and pin actual rejection stages. Slots4/10 retain their named deeper API gates.

Bound each JSON file at32MiB and total replacement payload at128MiB before writes.
No arbitrary caller-supplied path or replacement JSON; enum recipes construct only
the prescribed changes. A write failure retains the copied partial state and is
an error, never a successful negative. No source rollback/retry or worker launch.

Tests must show unrelated files and originals stay unchanged, both hash levels
are repaired, wrong/duplicate/symlink inputs fail before mutation, one-point input
is a missing prerequisite, removed companions disappear from both levels, and
result/manifest outcomes stay identical. Tests are software custody fixtures,
not generated curve instances or proof of the mathematical rejection stage.
