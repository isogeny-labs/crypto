#set page(paper: "a4", margin: 18mm)
#set text(font: "Arial", size: 10pt)
#set par(justify: false, leading: 0.65em)
#set heading(numbering: none)
#set table(inset: 6pt, stroke: 0.4pt + rgb("ccd4dc"))
#let receipt = json("../../k4405_load80_local_3600_20261020.json")
#let provenance = json("provenance.json")

= K4405 load80: capped local control
*Run 20261020T1604Z - one attempt; zero completed index or target receipts.*

The release process reached its 3,600-second cap while this control had not
produced an index or target receipt. A process sample at 16:24:07 UTC observes
root-index construction. The completed capacity, exact output, and performance
of load80 remain unknown. This report preserves an existing run; no new
experiment was executed to prepare it.

#figure(image("stages.svg", width: 100%), caption: [Retained stages of the
censored attempt. There are fewer than two comparable completed measurements,
so this explanatory diagram replaces a quantitative comparison graph. The
horizontal spacing is not elapsed time.])

== Measured observations and configured limits
#table(columns: (1.3fr, 1.7fr),
  [*Quantity*], [*Recorded value and scope*],
  [Process status / exit], [TIMEOUT_3600S / -15],
  [Process wall, measured], [3,602.661142375 s; termination interval, not solver throughput],
  [Sampled peak RSS, measured], [13,126,172,672 bytes; sampling is not a bound on an unobserved peak],
  [RSS cap, configured], [27,917,287,424 bytes],
  [Time cap, configured], [3,600 s],
  [Outputs retained], [stdout, stderr and target-output files are each zero bytes],
  [Index / target receipts], [null / null],
  [Replicates and uncertainty], [One attempt; no spread or confidence interval recorded],
)

== Workload, host and interpretation
The exact curve is `EC1N83Ckb1a1haacb69ee9f37`, K = 4405, load = 80.
The command invokes `koblitz_orbit_dlp_fast construct:83:1:4405` on the
retained public target with command argument `29`; the full command and all
environment flags are in `raw/input.json`. Its expected scalar is
8171303969165027. No complete output exists for an equality check.

The sample identifies ARM64 and macOS 26.6 (25G70). No host isolation
attestation or matched load75/rho control is retained for this attempt.
The sample's physical-footprint figure and the controller's sampled RSS are
different counters and are not substituted for each other. The elapsed time
is a censoring observation; it establishes neither a speed ordering nor a
completed capacity measurement. K4500 and K4549 are not promoted by it.

#pagebreak()
== Identity and independently retained provenance
The original capped-control JSON is unchanged. Its source commit is
#raw(receipt.input.source_head), with example SHA-256
#raw(receipt.input.source_sha256). The binary SHA-256 is
#raw(receipt.input.binary_sha256). Both were checked against the historical
Git source and local executable while assembling this package. The executable
remains external; it is not embedded in this report or committed here.

The files below are copied byte for byte from the original local paths and
match the hashes already pinned in the receipt. `provenance.json` records the
complete file hashes and sizes. This verifies the binding of retained bytes;
it does not independently rerun the capped computation.

#table(columns: (1.5fr, 0.6fr),
  [*Retained file*], [*Bytes*],
  ..provenance.files.map(f => (raw(f.path), str(f.bytes))).flatten(),
)

== Frozen dispatch and canonical figures
The associated two-point protocol records that its former Pod was absent at
2026-10-20T15:58:56.317004Z. Its existing deadline check rejects dispatch from
those frozen bytes. A future comparison needs a new allocation/protocol,
an authorized budget, complete capacity receipts, and a provider-absence
watchdog. This report does not allocate infrastructure.

The study's `capacity_projection.json`, canonical IC scoreboard,
`docs/ic/progress-timeline.json`, and performance-gains figures receive no new
point: the attempt yields no admitted completed performance comparison or new
curve/map identity. The retained timeout does not change the previous load75
candidate identities. No benchmark performance row is warranted by these bytes.

== Sources and reproducibility
- Frozen receipt: `../../k4405_load80_local_3600_20261020.json`.
- Exact raw input, target, process result, empty outputs, and process sample:
  `raw/`; their SHA-256 bindings: `provenance.json`.
- Study interpretation: `../../README.md`.
- Frozen comparison protocol:
  `research/n83_load80_online_pair_20261020/PROTOCOL.md`.
- Editable diagram source: `stages.svg`. Build this PDF with
  `typst compile --root ../.. report.typ report.pdf` from this directory.
