# Parent artifact ownership transfer

The retained visitor supplies exact artifact counts, identity lengths and data
lengths without cloning the buffers. `quote_artifact_owners` computes the modeled
work and requested metadata allocation for the parent owner vector and strings.
Already retained data buffers are not charged as new allocations. This quote is
for ownership metadata only, not storage I/O, hashing or serialization.

`take_parent_artifacts` admits the quote through the original-budget parent
session, uses fallible exact reservations for the vector and identity strings,
and completes all metadata allocation before moving any producer bytes. A
metadata allocation failure leaves every original data buffer in place. Once
identities are ready, `mem::take` transfers each buffer into its parent payload.
The run records transfer and refuses another transfer or further exposure of
empty canonical artifacts. A post-transfer deadline failure retains the returned
parent value through the existing retention-call acknowledgement rule.

The five logical `partial/koblitz-m3-*.raw` names map explicitly to flat
`partial-koblitz-m3-*.raw` names. These are suitable identities for bound capture,
but the transfer itself does not publish them. The registry requires the exact
raw role and native encoding and rejects duplicate paths or canonical promotion.
The four canonical names remain unchanged. A published witness carries its
existing publication receipt beside the transferred payload.

Controls use actual positive, negative and input-persistence-failed native runs.
They compare buffer pointers and lengths before and after transfer, retained
publication receipts, exact quoted metadata charges, unchanged trace charges,
and the producer stop. A no-headroom control retains original ownership without
transfer. Registry controls reject changed raw roles, encodings and duplicates.

Remaining work: admit and perform live capture plus final root storage using
the actual transferred payload lengths; preserve dedicated witness publication;
record and authenticate parent accounting; central replay and runtime/CLI wiring.
The adapter remains test-only. Metadata ownership transfer is not storage or
dataset-generation evidence.
