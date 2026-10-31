# Producer input receipt adoption

The producer receipt preserves input capture/root acknowledgements and separate checking/trace admission outcomes. It moves the original input buffer and actual root Artifact into parent ownership, including a fully encoded input retained after a later producer stop. Parent persistence does not repeat the producer input attempt. Its storage charges remain outside the parent reservation interval.

Receipt memory and 512 modeled attachment work units are admitted in the original publisher budget; the existing parent owner size guards remain unchanged. The receipt verifier checks identity, capture sequence, original file metadata and modeled reservation transitions. Its earlier entry balance is not independently reconstructed: producer_entry_anchor_verified remains false. No instruction-count or fall-degree claim follows from these checks.

Both-engine controls cover success, root collision, post-root deadline and checking admission followed by trace rejection. They check pointer-preserving transfer and no duplicate input event; semantic mutations rehash the outer metadata before testing binding, sequence, digest, quote, origin and inventory rejection. Failures before admitted input hashing remain raw retained input, not adopted canonical input. Runtime dispatch, worker registry transfer and central partial replay are subsequent integration gates.

Source review: existing capture payload_parts idempotency, parent delivery ledger/manifest checker, original-limit Budget and immutable storage routines. Implementation review proposal retained separately. Validation is tracked against the exact commit; no cloud run is launched by this change.
