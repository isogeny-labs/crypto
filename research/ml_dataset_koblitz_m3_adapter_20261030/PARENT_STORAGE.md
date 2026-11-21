# Admitted parent capture and root storage

`persist_parent_artifacts` consumes no producer buffers and creates no new
allowance. It uses the existing `ParentRetention` session and the admitted
original output root, attempt binding and capture limits supplied by its caller.
The API remains test-only until original-bound runtime dispatch is integrated.

Each item receives a quote based on its actual retained byte length. The model
reserves hashing/metadata work, capture metadata/event buffers, the store's 1 MiB
file-hashing buffers, and live/root temporary and final output bytes. These are
declared conservative reservation units, not hardware instructions or measured
RSS. Metadata is not silently treated as free and existing limits are unchanged.

Inside the admitted call, the actual thread-local bound sink must match the
original attempt and limits. A non-witness artifact goes through borrowed live
capture. Its acknowledgement is recorded immediately, before root storage.
`storage::write_bytes` then writes or checks the immutable root artifact, and its
returned artifact identity is retained immediately. An error or a late return
stops later items while preserving existing payload owners and acknowledgements.

A witness requires its exact dedicated identity, byte length and digest to
match the retained typed publication receipt. It skips generic live publication
and only writes/checks its root representation. Canonical replay authenticates
that receipt against the capture record. The storage function does not infer
trust from a producer-supplied JSON claim.

Root-file collision after a successful live publication retains the live
acknowledgement, original root bytes, payload buffer and parent error. Repeating
the stopped operation cannot reset the session or discard accounting. Completed
items are not republished by this function. Durable file integrity is checked
again by canonical/manifest replay before a dataset label is accepted.

The existing six F4/F5 positive/negative/capped fixtures now transfer their owners
and write through this storage path before the combined arithmetic/capture and
outer-outcome verifiers. Positive fixtures check that dedicated witness event
count remains one. Additional controls cover real raw failed-input capture/root
storage, a root collision after live acknowledgement, wrong attempt and missing
trace headroom with no output or capture-head mutation.

Remaining integration: save/authenticate parent accounting and storage terminal
state; transfer root artifact metadata into the worker registry without duplicate
publication; central replay and original-bound runtime/CLI dispatch. Storage
software controls do not constitute a new scientific dataset campaign.
