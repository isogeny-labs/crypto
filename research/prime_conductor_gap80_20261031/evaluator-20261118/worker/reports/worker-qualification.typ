#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
= Native worker packaging qualification
18 November 2026 · PR 4690, follow-up to merged PR 4680.

The corrected standalone amd64 FLINT/PARI worker runs as UID/GID 1000 and emits
the archived fixed-fixture bytes: 42 map inputs, 68 known-answer endpoint
recoveries and six coverage rejection controls. These are unsigned producer
observations. Statement PCC-20261118-8b3a31 has no independent mathematical
standing: not_covered. [pcc:unverified]

#table(
  columns: (0.6fr, 1fr, 2.4fr), inset: 5pt,
  [Attempt], [Source], [Outcome],
  [A], [3cedf8af6c], [Image exported; loader exits 127 because libgmpxx.so.4 is absent],
  [B], [1df0b39628], [Runtime dependency added; fixture exits 0, stderr empty, archived bytes match],
)

Attempt A's build-exit JSON records zero, although the orchestration tool
reported one. Both observations and the explicit loader failure are retained.
Attempt B used a new receipt directory. No failed input or output was replaced.

The base manifest and FLINT/PARI versions are pinned. The maintained GitHub
publication receipt and pre-call manifest precede each build. The replay uses a
600-second limit, 2 CPUs, 1 GiB, 64 PIDs, no network, read-only root and a 64-MiB
temporary mount. These are ceilings, not measured requirements; no scientific
timing or host-isolation claim is made.

#image("worker-status.svg", width: 100%)

The image embeds source, binary and package manifests. Its source hashes match
the unchanged native evaluator and log-norm helper. The local Docker image ID
is not a published registry manifest digest. The image archive is not committed
or uploaded to S3. [pcc:unverified]

#text(size: 8pt)[
Local image ID: #raw("sha256:3dcd2c0e240753cd315aadb61dd30375d31c4cabdecf0688156f46c286f56b49", block: true)
Native executable SHA-256: #raw("3c16dd628912f1f37c16903447f03f513cd63574e197f246245b383833cebfd7", block: true)
]

#pagebreak()
== Supplied 47-bit control: prepared, not run
The prepared case is archived case 1007 over the 96-bit prime field
#raw("78125000378223625457769953627"), with candidate prime conductor ratio
#raw("125000000302579") (47 bits). The input is the archived root quadratic
twist and contains its independent second ell-torsion basis point, kernel
generator and infinity. Frozen source snapshots and exact-string consistency
checks accompany the input; no new large-case curve computation took place.

The proposed transport subgroup order is ell, not the original curve's 96-bit
prime order. The recorded nontrivial Weil pairing supports the independence
obligation to replay. This subgroup order equals the isogeny degree: a
coprime-degree argument is unavailable, the dual map is not an inverse on this
subgroup, and recovered scalars must be checked on the source.

The proposal specifies scaled-floor coefficient comparisons and post-run
nonzero/order/coverage checks. Its ceilings are one attempt, 4 CPUs, 96 GiB,
3600 seconds, USD 5 total. They are not measured requirements or an admitted
launch request. Current account/lane remaining and reserved spend and live
capacity must be established first. No large-case map, ECDLP trial or timing
measurement was run. [pcc:unverified]

== Goal obligations
#table(
  columns: (2.4fr, 1.6fr), inset: 5pt,
  [Obligation], [Status],
  [Local package and fixed fixtures], [Exercised; unsigned statement evidence],
  [47-bit supplied case], [Input and proposal only],
  [Registry image and trusted launcher], [Open],
  [Live budget, reservations, capacity], [Not established],
  [80-bit quotient and full point map], [Open],
  [Full-window work/memory and durable resume], [Open],
  [Matched scalar/ECDLP timings and IC comparison], [Open],
)

The prepared 47-bit case does not replace the minimum 80-bit crossing. Its
field bit length is separate from its conductor-prime bit length. The existing
80-bit source/kernel and frozen measurement protocol are unchanged.

== Evidence and next action
The source-linked text report is ../QUALIFICATION.md; raw build/loader/fixture
outputs, source-publication receipts, pre-call and image manifests are under
../validation/. The proposed input, exact source snapshots and checks are under
../qualification47/. The corrected Dockerfile is byte-identical to the tested
build input; source, local image identity and any later registry digest remain
separate provenance objects.

Sources are the existing complete evaluator, frozen upper-96 kernel/floor and
torsion-module records, cloud runtime Docker instructions, and the existing
IsoLab cost/access contracts. Shared research KB unavailable; this preparation
uses the scoped frozen sources. No novelty or eligible gap record is claimed.

Next: trusted worker publication, then read-only cost/capacity qualification.
Only after those obligations pass should the supplied-case proposal become a
bounded experiment. Existing coverage, traversal and benchmark figures have
unchanged scientific evidence; this packaging step promotes no benchmark row.
