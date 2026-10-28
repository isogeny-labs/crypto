#set page(paper: "a4", margin: 20mm)
#set text(size: 10pt)
#set heading(numbering: none)
= Fixed n83 factor-base points
#text(fill: rgb("287d9c"))[Complete enumeration · ell = 27 · 28 October 2026]

Observations from saved point data. This report records enumeration and validation; it makes no degree-of-regularity or performance claim.

== What is fixed
Each of the three original curves has one base, reused unchanged for every m from 2 through 9. The x-domain is the integers 0 through 134,217,727; binary-field integers encode polynomial coefficients. All distinct affine y-lifts are saved. Infinity is omitted and no subgroup filter is applied.

#table(columns: (1fr, 1fr, 1fr, 1fr), inset: 6pt,
 [Curve], [Liftable x], [Stored points], [Compressed bytes],
 [Koblitz], [67,110,393], [134,220,785], [1,329,389,614],
 [Binary], [67,113,567], [134,227,133], [1,329,453,079],
 [Prime], [67,106,828], [134,213,656], [1,849,491,803])

#image("counts.svg", width: 100%)
Exact counts from local-r4: 128 shards per curve, each covering 1,048,576 candidates. No sampling uncertainty applies to these enumeration totals.

== Curve identities
Binary field: GF(2^83), modulus z^83 + z^45 + z^2 + z + 1. The equation is y² + xy = x³ + ax² + b. Koblitz: a = 0, b = 1. General binary: a = 1, b = 2^82 + 0x51b.

Prime field: p = 4835703278761981908090881; y² = x³ + 2x + 3. These are the prior study's exact curves. Subgroup membership is not asserted.

== Verification and access
All 384 shards passed a post-write read of every candidate: complete valid sign pairs for liftable x, and no-lift checks for omitted x. Distributed samples also use independent slow arithmetic. Arithmetic and altered/deleted-record controls pass. All compressed file hashes match the manifests. The first and last shard per curve were downloaded from the content-addressed S3 store, hashed and verified again.

Start at ../data/index.json. It fixes each family's manifest hash for all m. Point files are data/FAMILY/part-NNNN.bin.zst. Each decompressed file has a 32-byte header followed by 20-byte little-endian records: u32 x, u128 y. The README provides hydration, verification and CSV export commands.

#pagebreak()
== Execution and provenance
Run: n83-fixed-factor-base-e27-20261028-r4 / local-r4. Completed 02:09:19–02:11:33 UTC on the existing Spot workspace, six native worker threads, 8 GiB virtual-memory limit, 3300-second producer timeout. These timestamps describe execution, not a performance comparison. No container was used for R4.

R1 never scheduled because of Spot quota and was cancelled. R2 was rejected at admission for an odd CPU request. R3 started but failed shared-bucket authorization before enumeration. The pre-data R4 protocol amendment records the existing-host fallback. No permissions or quotas were changed.

#text(size: 8pt)[
Source revision: a3d2c86829dcb70723fee72a5f2b4566d63cf8ad\
Native binary SHA-256: dea894c5bd3cf7e718a235c2a9335f6a2b2896036c0118bacbe22fc103d1e55b\
Protocol R4 SHA-256: 3e373a7c5550d747c4584cac7ec51eb86d7d046e57d044c327e5009645c6c674
]

Raw archive: #raw("s3://setec-astronomy-artifacts/runs/n83-fixed-factor-base-e27-20261028-r4/local/", block: true)
Content-addressed point objects are referenced by the three data/FAMILY/.artifacts.json manifests. The archive retains producer logs, controls, source identity, certificates and the successful terminal receipt. The experiment export binds all 384 outputs to protocol R4.

== Scope and remaining work
Generation is complete. The earlier sweep used varying factor-base sizes; its degree results do not measure this fixed base. A degree sweep using these fixed manifests remains separate work. No end-to-end index-calculus, subgroup, independent research-review or speed claim follows from enumeration.

== Sources and evidence
- ../data/index.json and the three family manifests: exact counts, curve identities and hashes.
- ../experiment.export.json and ../protocol-r4.json: run binding and pre-data scope.
- ../validation/: exhaustive certificates, arithmetic controls, file hashes and sampled remote readback.
- #link("https://www.secg.org/sec1-v2.pdf")[SEC 1 v2, section 2.3.4]: affine point lifting.
- #link("https://cacr.uwaterloo.ca/techreports/2011/cacr2011-07.pdf")[Taverne et al., section 2]: binary-field carry-less arithmetic and half-trace methods.

The quantitative figure is generated from the saved manifest counts by ../src/bin/report.rs. Historical plots and leaderboard graphs remain unchanged: this dataset adds no degree or performance measurement.
