# Typed immutable ancestry index

This is metadata preservation and duplicate prevention, not scientific search.
All old files remain read only. Commands (coordinator only, after review):

    python -m research.typed_ancestry_20261115_v7.builder fixtures
    python -m research.typed_ancestry_20261115_v7.builder index

Outputs are exclusive FIXTURES.json, INDEX.json and INDEX_RECEIPT.json in this
fresh package. No stdout copies of state or raw scientific evidence are emitted.
INDEX.json is capped at256KiB. Index items preserve every historical ID, epoch,
status, registered/attempt fingerprint and normalized semantic digest.

The root is the active manifest's sealed-parent entry, checked against the
active state manifest hash. Closed parent formats are modern sealed_path with
state/journal/terminal hashes; path/receipt with matching journal, terminal and
path bindings plus receipt state hash; and the specific ledger_epoch_relocation_v1
that names the21October preserved parent. Unknown/incomplete formats, cycles,
missing identities for fingerprinted items, missing file bindings and corrupt
hashes fail before index completion. No exception handler skips a parent.

Every sealed state is streamed through SHA256 in1MiB chunks before parsing.
Only one parsed state is kept at a time under4GiB address-space limit; it is
released after compact fields are extracted. The372MB root state is never
copied to disk. The19.64GB journal is not read anew. Its binding comes from
the existing frozen canonical-terminal/suffix verification receipt, authenticated
by a successor ADMISSION_FROZEN file whose hash MUST already appear in a
verified child registration or expected-source identity. That admission binds
the relocation journal/terminal/path, exact historical verification-receipt
SHA89c5a0..., and exact rootstate SHA0d4d843b.... The actual rootstate hash is
then rechecked. Existing verification is explicitly inherited, not represented
as a fresh full-journal replay. No journal validation is bypassed by a path alias.

All four successor admission alternatives are fixed filenames; selection is
allowed only if its actual hash is in verified-child source bindings accumulated
while walking toward the root. Missing trusted admission is a concrete failure,
never permission to drop that root. Unknown parent fields do not imply a root;
explicit sealed_parent=None terminates the chain. One additional closed root
case is the exact LEGACY path and STATE_SHA above: its authenticated program
dictionary predates the sealed_parent key. Missing parent keys elsewhere fail.

The semantic function is byte-for-byte in meaning the v6 metadata comparison:
normalize matching experiment-ID arguments and the module argument, retain
other argv, code-hash multiset for.py/.cpp/.hpp/.rs, input hashes, timeout,
memory and CPU. Its canonical sorted compact JSON SHA256 is retained peritem.
This ignores renaming but is not a general semantic code-equivalence decision.
The fresh registration must compare BOTH raw runner fingerprints and semantic
digests against this full index before any manifest mutation or scientific run.
This index does not itself register or execute experiments.

Synthetic corrupt/incomplete receipt fields, unknown formats and escaping paths
are rejected by a declared separate fixture command before full indexing. Prior
failed adapters and receipts remain unchanged. No measurements or goal-complete
claim follows from successful metadata indexing.
