#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: none)
#set par(justify: false)
#let data = json("summary.json")
#let phases = data.phases
#let last = phases.last()
#let delta = data.new_since_checkpoint
#let n(v) = str(v)
= Prime61: continuation from a Drive checkpoint
2026-10-18 · capacity and relation-stage observations

The public synthetic experiment over $p = 2^61 - 1$ continues from the
verified 2026-10-17 checkpoint. The table contains cumulative populations;
the first row was inherited. These are finite prefixes of the selected-prime
component. The entire horizontal class has not been enumerated or certified.

#table(columns: (1fr, 1fr, 1fr, 1fr, 1fr), inset: 5pt,
  table.header([Expanded], [Discovered], [Maps], [Measured], [Streams]),
  ..phases.map(v => (n(v.expanded), n(v.discovered), n(v.edges), n(v.measured_curves), n(v.runs))).flatten()
)

#image("progress.svg", width: 100%)

New since the restored checkpoint: #n(delta.discovered) discovered curves,
#n(delta.edges) map records, #n(delta.runs) committed collection streams and
#n(delta.target_trials) target trials. The final population has
#n(last.relations) relations and #n(last.verified_solves) verified solves;
#n(last.pending) discovered sources remain pending. Every measured curve
has twelve capped streams. A capped stream is retained as a negative result.

The last live-database status records #n(last.database_bytes) bytes.
The maximum reported native-worker peak RSS across the retained phase
receipts is #n(data.worker_peak_rss_kib_all_phases) KiB. Compiler, test,
audit and renderer processes are excluded from this worker statistic.
These are capacity observations on this prefix, with no full-class
memory extrapolation or wall-speed claim. Full IC cost and speedup remain
unknown; no matched rho or external-tool speed comparison was performed.

#pagebreak()
= Inputs, controls and remaining obligations

The source is the exact previously validated revision
`b68341cd72b51acf9c5ce6c4f755351cc83053db`, built with Rust 1.99.1.
The initial current-main build failed in unrelated ecbench code;
`AMENDMENT.json` froze the fallback before new collection. Both source
revisions' build/test outcomes are retained and distinguished in the README.

The unchanged spec fixes $a=p-3$, $b=111$, prime order
`2305843010818082053`,
`G=(1,1796364306280342617)` and
`Q=(992726311114892094,558260110083040833)`.
The public checking scalar is 539365399; the collector receives G and Q.
Prime degrees are 3, 7, 11, 13, 19, 23, 53 and 61. Each curve uses a fresh
low-x base with three inverse pairs and requests twenty relations. The
twelve seeds are 2026101600 through 2026101611, each with 100000 target
trials. One first-parent transported workload is measured per curve.

The downloaded archive's SHA-256 and all internal manifest entries passed.
Before continuation, the native verifier checked the restored maps and
rosters, replayed the first sixteen curves' 192 collector streams, and
reproduced the archived logical dump exactly. The final immutable-prefix
audit checks all inherited payloads. Every batch receives complete native
map, source-roster, point and record verification; the optional collector
replay is the separate restored-control population, not extra headline runs.

Raw phase commands, stdout, stderr, exits and snapshots are preserved in
`results/runs.tar.xz`; unpack from the study directory with
`tar -xJf results/runs.tar.xz -C results`. Compact verification, prefix and
storage receipts sit under `results/`. The immutable final Drive archive
is separately downloaded and checked before its receipt is called passing.
Private Drive object identifiers are omitted from this public report.

The earlier fundamental-discriminant result excludes vertical separable
base-field edges for this fixed ordinary trace class. No new vertical-map
or algebraic-identity theorem is claimed. Exhaustive class structure,
generation by the chosen degree set, every class workload and full-run
storage/compute provisioning remain open. A merge or passing prefix does
not discharge those obligations. Candidate registry promotion is pending;
full EC1/UIDs and map witnesses stay in the raw logical records.

*Evidence and reproduction:* `PROTOCOL.json`, `AMENDMENT.json`, `spec.json`,
`continue.sh`, `audit_prefix.rs`, `visual.rs`, `reports/summary.json`, and the
study README. The SVG is generated only from native phase status records.
The historical pilot and performance dashboards are unchanged: this run
adds coverage observations without an admitted performance result.

#link("https://github.com/aburan28/crypto/tree/codex/prime61-disk-continue-20261018/research/prime61_disk_continue_20261018")[Source-linked study and raw-evidence manifest]
