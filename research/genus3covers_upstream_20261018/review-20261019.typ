#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
= Genus3Covers: ingestion review
2026-10-19 · crypto PR #2241 · representation and provenance only

The immutable upstream revision is
AndrewVSutherland/Genus3Covers, revision
`221e74b7097a386527d3ebed68aa181b68954fd0`.
All nine vendored/decompressed files match its exact Git blobs. The MIT
license and the original Magma/example/catalog bytes are preserved.

#image("review-20261019.svg", width: 100%)

== Reader correction
The old native reader accepted [0,0] as a degree-one field polynomial.
It now rejects zero leading coefficients, including negative-zero spellings
and padded high-degree zeros. Nonzero arbitrary-precision coefficient strings
are retained. The native importer tests pass on all 654 example rows and
the adversarial representation controls; malformed input exits 2 without
emitting partial JSONL. This is a degree/shape check, not an irreducibility
or geometry certificate.

== Research boundary and graph impact
These exact counts describe candidate rows in a pinned file, not verified
covers or sampled performance. Magma is unavailable and was not run. No
finite-field model/map, subgroup transfer, ECDLP cost, timing or speedup is
promoted. Canonical covers/browser/scoreboard/timeline and benchmark claims
are unaffected; adding such claims still needs their native proof and
admission gates. The required full release library suite passed: 4,396
passed, zero failed, 118 existing ignored, no filtering. Final-head GitHub
CI remains pending.

Sources, metadata TSV, upstream SHA-256/blob receipts, editable figure and
reproduction commands are beside this PDF in REVIEW-20261019.md and the
original ingestion README.
