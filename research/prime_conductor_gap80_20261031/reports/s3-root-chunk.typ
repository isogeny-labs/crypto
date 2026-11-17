#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 10pt)
= S3-backed native root-chunk fixture

1 November 2026. The native producer writes a bounded stream of baby-root
coordinates for the frozen 80-bit kernel. A 4,096-root fixture occupies
131,072 bytes; PARI independently checked every coordinate. This is a
development/storage check, not an isogeny performance experiment.

#image("s3-root-chunk.svg",width:100%)

== Exact data and checks

The source and kernel are the unchanged values in `../parameters.json`.
The fixture covers ordinals 0 through 4,095, with kernel indices
2 × ordinal + 1. Each x-coordinate occupies 32 unsigned big-endian bytes.
`../s3-chunks/chunk-manifest.json` binds the field, kernel, interval,
encoding, producer source and blob digest. `independent-check.json` records
the separate PARI arithmetic check for all 4,096 roots.

Local checks passed: byte-identical regeneration, offset-resume agreement
with the original tail, rejection of the count/window bounds, and exclusive
output creation preventing accidental overwrite. No wall-time advantage
or peak-RSS measurement is inferred from these correctness checks.

== Storage and completion evidence

The authorized artifact-autofix workflow uploads the 128-KiB object to the
existing write-once bucket, reads back the whole object and commits its
SHA-256 pointer. The creation-time manifest records upload as pending;
only the workflow's pointer commit establishes completed S3 readback.
Retrieve with the repository artifact tool and verify the adjacent checksum.
No credential values or memory dumps belong in this fixture.

== What S3 does and does not establish

Bounded root preparation avoids allocating the entire root vector. It does
not make the existing contiguous FLINT polynomials operate on S3. A full
external-memory schedule for product/remainder trees, intermediate manifests,
resumption and complete point evaluation is still required. S3 is object
storage, not process RAM or transparent swap.

The full baby window contains 427,154,957,034 roots. This simple encoding
would require 13,668,958,625,088 bytes before intermediates. Only the small
fixture has been generated; no full-window job or multi-terabyte upload was
started. Future measurements must include construction, transfer, I/O,
serialization, failed epochs and verification, following the existing native
EKS/checkpoint rules and `../MEASUREMENT_PROTOCOL.md`.

Sources: existing `sqrt_velu.cpp` / `CONSTRUCTOR.md` in the related CM study,
the checked `search80.c` point arithmetic, the PARI elliptic-curve API and
`docs/artifacts.md` upload/full-byte-readback contract. No new algebraic
identity, completed crossing or endpoint measurement is claimed.
