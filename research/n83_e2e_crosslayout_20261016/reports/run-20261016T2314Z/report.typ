#set page(paper: "a4", margin: 17mm)
#set text(font: "Arial", size: 10pt)
#set par(leading: 0.6em)
#set heading(numbering: none)
#set table(inset: 5pt, stroke: 0.4pt + rgb("ccd4dc"))
#let data = json("../../ic1_rows_38976053815.json")
#let number(x, scale: 1) = str(calc.round(x / scale, digits: 3))

= N83 cold root-layout screen
*Run 38976053815 · 16 October 2026 · one exposed public target*

Six fresh IC processes completed under the frozen symmetric schedule, followed
by one same-point rho process. At K1104, the recorded root11 A/B mean was
119.446 seconds and the root22 mean was 117.806 seconds. The original screen
reports a 1.640-second (1.37%) reduction in this complete process interval.
The corresponding A/B spreads were 0.231% and 0.094%, both below the frozen
5% threshold. Root22 used about 530 MiB more peak RSS at K1104.

#figure(image("observations.svg", width: 100%), caption: [Six recorded
fresh-process observations, in execution order. The dot-plot axes are cropped
and labelled. Each K1104 layout has two observations; each K1120 layout has
one. No confidence interval was estimated. Source: the exact IC1 row record.])

#table(columns: (1.35fr, 1fr, 1fr, 1fr),
  [*Arm*], [*Cold process, s*], [*Online, ms*], [*Peak RSS, MiB*],
  ..data.rows.map(row => (
    text(size: 8pt, row.arm),
    number(row.cold_process_ms, scale: 1000),
    number(row.one_target_online_ms),
    number(row.peak_rss_bytes, scale: 1048576),
  )).flatten(),
)

The plotted and tabulated quantities are measured observations; conversion
from milliseconds to seconds divides by 1,000, and bytes to MiB divides by
1,048,576. The means, spreads and differences above are the original report's
derived summaries. K1120/root22 recorded 117.258 seconds versus 118.017 seconds
for root11, with one process per layout. This is a descriptive screen on an
already exposed target. The frozen result has a null promoted candidate.

#pagebreak()
== Frozen workload and accounting
The exact curve is `EC1N83Ckb1a1haacb69ee9f37`, workload `eebf98ee1031`.
At K1104 the usable base has 183,264 points and 1,104 folded columns; at K1120
it has 185,920 points and 1,120 columns. Both root layouts reconstruct the
same base at fixed K. Rank seed 29, four rank threads and one target thread
were fixed. The cap was 480 seconds per IC arm and 120 seconds for rho.
All six IC arms and the rho arm completed; no failed arm is omitted.

The measured cold process includes factor-base/index/log setup, relation-rank
work and the target phase. The target-dependent online interval is recorded
separately, including its five exclusive phases in the IC1 rows. The retained
same-point rho interval is 721.786 milliseconds. Cold IC process cost and rho
online cost have different boundaries and are not divided into a speedup here.
No fastest-external-tool comparison was run by this layout screen.

== Host and controls
The archived host identifies four visible AMD EPYC 7763 CPUs and 15.61 GiB
memory on a GitHub Actions allocation. The run records topology, affinity,
cgroup limits, telemetry and per-process resource counters. The analysis
admits its recorded hardware tier and A/B controls; this report adds no
stronger host-isolation claim. The workload uses one public target repeatedly;
it is not a fresh-target generalization result or a multi-target throughput test.

== Source, custody and replay
The execution commit is `cec93c657f49441f59d6408a3a81038f72a6ee97`.
The IC example SHA-256 is
`8a2fe55dd90dc33892c197d27260182bf783fc8212fb485d8297daf2bbba8e12`.
Source and both executable hashes are retained in `raw/source.json`.
Provider artifact `n83-e2e-crosslayout-38976053815-1`, ID 12080884073,
has ZIP SHA-256
`5556c506faaff4f54baac31010da3f0abde0747410045828acdcbb983ef6cb6b`.

The original independent replay exited zero, had empty stderr and reproduced
the archived analysis. During PR completion, the materializer rechecked the
archive digest, all 60 result members, exact source/host/target identity and
the retained replay logs; it reproduced four candidate manifests and six IC1
rows byte for byte. This was a custody and recorded-replay check, not a new
timing run. `provenance.json` records file digests and the retained archive
location; `raw/` contains unchanged analysis, host, source and preflight receipts.

== Interpretation and reproducibility
The original result is `SCREEN_COMPLETE`, with independent replay recorded
and no promoted candidate. A frozen fresh-point comparison is required before
selecting a layout for the primary leaderboard. The canonical IC leaderboard,
progress timeline and performance-gains figures receive no new admitted point
from this exposed-point screen. The benchmark entry records its bounded result.

Study result and protocol: `../../RESULT.md`, `../../PROTOCOL.md`.
All six rows and candidate identities: `../../ic1_rows_38976053815.json`.
Provider run: #link("https://github.com/aburan28/crypto/actions/runs/38976053815")[38976053815].
Run `python plot.py`, then
`typst compile --root ../.. report.typ report.pdf` from this directory.

