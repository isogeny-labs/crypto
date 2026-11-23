#set page(paper: "a4", margin: 17mm)
#set text(font: "Arial", size: 10pt)
#set par(leading: 0.6em)
#set heading(numbering: none)
#set table(inset: 5pt, stroke: 0.4pt + rgb("ccd4dc"))

= N83 five-byte ID30 high-K screen
*17 October 2026 · historical execution · partial and censored*

The ordered K4096, K4608 and K5086 screen has one completed arm.
K4096's recorded geometric target-online interval is 22.466 ms on 128
public points, compared with 16.405 ms for the frozen K3072 control.
The larger base reduced mean probes, but its recorded online interval
increased. K4608 timed out during construction; the stop rule left K5086
unrun. No candidate is selected for the separate fresh holdout.

#figure(image("observations.svg", width: 100%), caption: [Recorded complete-arm
values and the original paired interval. The interval describes the 128
target pairs on this shared local host; it does not establish a qualified
wall-time advantage or account for selection across experiments.])

#table(columns: (0.8fr, 1.5fr, 1fr, 1fr),
  [*Arm*], [*Outcome*], [*Online, ms*], [*Recorded process interval*],
  [K3072 control], [Prior independent replay], [16.405], [491.832 s cold],
  [K4096], [128/128 targets replayed], [22.466], [1,500.010 s cold],
  [K4608], [Construction timeout], [Unknown], [1,800.734 s outer wall],
  [K5086], [Not run after stop], [Unknown], [Unknown],
)

The K3072/K4096 paired geometric ratio is 0.730229, with recorded descriptive
95% interval [0.544154, 0.979931]. K4096 is faster on 56 of 128 points.
Mean probes fall from 58,734.66 to 36,347.24. These are different accounting
quantities: fewer probes do not by themselves establish a timing gain.
No fastest-external-tool comparison was performed.

== Memory and censoring
K4096 retains 679,936 usable points and 4,096 columns at full rank.
Its ten-GiB root table plus approximately 7.133 GiB of states account for
17.133 GiB allocated. The child maximum RSS receipt is 14,980,890,624 bytes;
allocation and resident memory remain distinct.

K4608 emits no complete base dump, summary or target rows before its
1,800-second cap. Its observed 20,909,326,336-byte `RUSAGE_CHILDREN` maximum
is cumulative across children and provides an upper-bound observation for
that child. The timeout is retained as a censored setup outcome. Neither
its online time nor its completed index allocation is known.

#pagebreak()
== Frozen workload and eligibility
Curve `EC1N83Ckb1a1haacb69ee9f37`, root11 layout, five-byte slots and the
explicit `KIC_INDEXED5_ID30=1` mode are fixed. The plan orders K4096, K4608,
then K5086, with a 1,800-second cap for each arm and a stop after the first
incomplete or unverified arm. Rank seed 29, four rank threads and one
target thread are fixed. The 128-point workload is secondary and shares
setup within each process. The control is a retained prior measurement.

The hardware tier is `ordinary_local_exploratory`. This is a shared macOS
diagnostic without a matched multi-instance isolation or calibration
claim. Cold process totals, target-dependent phases, and the timeout's
outer-wall receipt are reported separately.

The prospective holdout reserves 24 distinct public subgroup points.
It requires at least two consecutive eligible ID30 arms before preparing
exact manifests and a separately gated comparison. Only K4096 is eligible,
so the selector returns `INSUFFICIENT_VERIFIED_ID30_ARMS` and an empty
selection. The holdout remains unmeasured.

== Source and custody
The historical source snapshot is
`8db7be2666fee80e39dc013ea1b25f7ba46ba330`.
The executed Rust example SHA-256 is
`8a2fe55dd90dc33892c197d27260182bf783fc8212fb485d8297daf2bbba8e12`;
the release executable SHA-256 is
`faaac22aa6228c8d486875d6947fa654417d1122191fbe0eb9bde59bb45eb98f`.
The frozen screen plan SHA-256 is
`ce79d7c6ebadb7a5d38a4d4b9609dabda3e3a224c4a7409862533159f0f07938`.
Later four-byte index implementation work uses a different source and
does not change this historical execution's producer identity.

`provenance.json` lists the retained raw files and their hashes. The
`raw/` directory copies the control, plan and small command/result receipts
without changing their bytes. The full base dump and target outputs remain
under `/Volumes/SSD990/crypto/artifacts/n83_root11_id30_screen_20261017`.
The archived K4096 analysis records independent base, relation, scalar and
phase replay of all 128 targets. Report completion checks receipt custody;
it adds no new timing measurement.

== Interpretation and reproducibility
The frozen screen remains `PARTIAL_OR_CENSORED`. This report records the
measured regression and the stopped arms. It adds a dated benchmark entry,
with no newly admitted primary leaderboard or performance-history point.
The original follow-ups examine the capacity edge near K3072, cooperative
target cancellation, and a four-byte K4096 index; those experiments have
their own protocols and evidence.

Source result and protocol: `../../results/RESULT.md`, `../../PROTOCOL.md`.
All plotted values: `../../results/summary.json`.
Render from this directory with `python plot.py` and
`typst compile --root ../.. report.typ report.pdf`.
