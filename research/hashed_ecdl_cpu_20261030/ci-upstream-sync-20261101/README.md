# Upstream sync and standalone CPU lockfile

Merged upstream main 5ac38ef4e916ff9535afb9738fa4970816e59103 without rebasing
the evidence branch. Merge commit: 8d0b93868a3da588c0b3ebee31ea0887f11f4b03.
Upstream repair ff83a15cedf0138205218f1520e47153d56c27ab restores the exact
checkpoint capsules, including the two host.json files missing from PR2980's
evidence_audit_pr shards 1 and 8 at head ad51eafc2aca. Those records were
imported through the merge, not reconstructed in this lane.

The first locked standalone CPU build failed because upstream added sha2 to
hashed-ecdl-tensor while the standalone CPU Cargo.lock lacked its dependency
closure. Retained initial-lock-failure.log records this failure. An offline
package update added eight packages and preserved the existing locked versions.
The updated lockfile is committed with this validation.

After the update, 46 standalone release library tests, strict all-target
release Clippy, formatting and the complete-example build pass. Commands use
--offline --locked and crates/hashed-ecdl-cpu/Cargo.toml, with build outputs
outside the checkout. No root crypto or CUDA build was run locally.

frozen-inputs-diff.log is empty: round20's frozen protocol/runner/build and the
entire round21 package are unchanged from ad51eafc2aca. The actual round20 job
used the previously frozen executables, not this merged source. Current-head
hosted checks must still confirm the evidence audits and other repository gates.

The initial git fetch failed with a pack write error. A retry with one pack
thread and no auto-maintenance succeeded. A broad repository reservation was
rejected due to unrelated owners; scoped CPU paths were checked clear and the
merge was confined to this isolated PR worktree. No other owner's workspace
or running job was changed.
