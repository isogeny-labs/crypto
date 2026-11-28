# Parent delivery record and replay

Before producer work, `ParentDeliveryFile::admit` reserves a 16 KiB terminal
encoder and a root-only storage allowance against the original work, memory,
trace and deadline. It grants no fresh budget after a parent failure. The record
describes the boundary before its own encoding/storage and excludes its own hash
from the described items. The writer preserves a returned root acknowledgement
before checking its return-time deadline.

The record binds the original attempt, producer ledger before ownership,
post-storage ledger, ownership quote, payload lengths, parent stop/error and
per-artifact attempted/admitted states, storage quote, capture acknowledgement,
root metadata and dedicated-witness indicator. Producer state and first stop
remain distinct from parent storage state. `delivery_complete` means delivery of
the described artifacts, including raw diagnostics; it is not solver completion.

Ownership metadata now has versioned conservative 512-byte reservations for each
owner and its containing object, plus exact identity lengths. Admission rejects
a compiled layout exceeding those bounds. This avoids making replay's accounting
formula depend on the verifier's Rust object layout. Work tariffs and the original
limits remain unchanged; the modeled byte reservation is not an RSS measurement.

The strict reader binds the parent record's manifest hash and original attempt,
matches its before-ledger to independently supplied producer evidence, validates
both ledgers under original limits, recomputes owner/storage quote deltas and
checks ordered admission/acknowledgement states. Claimed capture acknowledgements
must exist in the authenticated capture prefix. Claimed root acknowledgements
must match that capture identity, the manifest and the actual root file hash.
No root acknowledgement may precede admission and capture acknowledgement.

Six actual F4/F5 positive/negative/capped fixtures write and verify the record.
Rehashed mutations cover ledger totals, storage quotes, capture acknowledgement,
root hash, admission flags, completion, producer phase, ownership bytes and
unknown fields. A tenth mutation per fixture removes an artifact and subtracts
its ownership, storage and trace charges consistently. The reader binds row
presence to canonical manifest entries and requires the full canonical inventory
for an unstopped completed producer, including a committed witness when present.
This closes omission attacks that arithmetic-only checks cannot detect. Genuine failed-input and root-collision controls also write and
verify their delivery records. A complete raw delivery preserves the producer's
failure; a root collision preserves one capture acknowledgement and zero root
acknowledgements without claiming complete delivery.

Runtime activation remains pending worker registry integration, central claim
wiring, original-bound CLI fixtures and full validation. The before-ledger must
come from authenticated original producer evidence, never from the parent record
being checked. Earlier failures before ownership transfer require the retained
prefix path; this record does not manufacture missing artifact owners.
