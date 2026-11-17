#set page(paper: "a4", margin: 1.6cm)
#set text(font: "Helvetica", size: 10pt)
#set par(justify: true)

= N83 K600 v3 local-store construction control

*Run:* 20261115T1209Z. *Source:* `498efdc53e821edb24fcebf0826ff94549ce266b`.
*Host:* macOS arm64 Darwin 27.2.0, shared-host L0. The protocol was committed
before execution at the same source revision. One primary `a=0` K600 base was
constructed and checked. This report records construction and local custody,
not a relation search or a cold IC runtime comparison.

#align(center)[#image("custody-flow.svg", width: 100%)]

_Sequence diagram, n=1: the single control has no comparable second arm. Timings
are recorded L0 component walls and do not form a complete IC cost._

== Recorded result

#table(
  columns: (2fr, 3fr),
  inset: 5pt,
  [*Check*], [*Observed value*],
  [Producer and independent importer], [Both exit 0; v3 replay, storage and both imports PASS],
  [Complete base], [600 signed-Frobenius columns; 99,600 subgroup points],
  [Exact-set control], [Point-set BLAKE3 `ac664f5f0c7122c5...25ffffee21` matches the frozen historical K600 set],
  [Compressed object], [3,025,798 bytes; SHA-256 `3e85c9e419fcddd3...9335841ecf` in object, index and fresh pull],
  [L0 construction / generic replay], [1,102.783 / 753.807 ms, one sample each],
  [L0 local push / fresh pull], [36.091 / 22.700 ms, one sample each],
  [L0 total producer process], [1,963.477 ms, one sample; not total IC runtime],
)

At admission, the filesystem had 9,723,520 KiB free and one-minute load was
10.54. The touched release example passed 26/26. The 600-second construction,
900-second outer and 300-second independent-import caps did not fire. The
producer and independent import used separate caches. The `rust-objcopy`
stripping warning did not change either terminal exit.

== Evidence and limit

Protocol: `verification/k600-v3-local-custody-20261115/PROTOCOL.md`.
Result and raw receipts: `verification/k600-v3-local-custody-20261115/RESULT.md`,
including the retained compressed object, manifests, replay, storage,
imports, local index, stdout, stderr and exit files. Diagram source:
`reports/run-20261115T1209Z/custody-flow.dot`.

The temporary `file://` remote is not S3 custody. The exact S3 destination
and writer remain pending. No natural relation, useful rank, column logs,
target descent, matched rho or cold end-to-end `S` was measured. The
selected minimum-runtime factor base remains unset. No canonical runtime
or progress graph changes from this control.
