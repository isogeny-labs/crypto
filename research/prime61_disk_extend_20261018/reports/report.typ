#set page(paper: "a4", margin: 17mm)
#set text(font: "DejaVu Sans", size: 10pt)
#set heading(numbering: none)
#set par(justify: false)
#let data = json("summary.json")
#let phases = data.phases
#let final = phases.last()
#let delta = data.new_since_checkpoint
#let n(v) = str(v)
= Prime61: larger checkpointed experiment
2026-10-18 · native capacity and capped relation-stage observations

The fixed public synthetic experiment over $p=2^61-1$ continues from the
verified 256-source checkpoint. Counts below are cumulative. The first
row is inherited; all earlier evidence remains frozen. Every measured
curve receives twelve seeded streams with the same collector limits.

#table(columns: (1fr,1fr,1fr,1fr,1fr), inset: 5pt,
  table.header([Expanded],[Discovered],[Maps],[Measured],[Streams]),
  ..phases.map(v => (n(v.expanded),n(v.discovered),n(v.edges),n(v.measured_curves),n(v.runs))).flatten()
)

#image("progress.svg", width: 100%)

The final committed population has #n(final.target_trials) target trials,
#n(final.relations) relations and #n(final.verified_solves) verified solves.
New since the input checkpoint: #n(delta.discovered) discovered curves,
#n(delta.edges) maps, #n(delta.runs) streams and #n(delta.target_trials)
target trials. The separate 192-stream collector replay is a control,
excluded from these totals.

The live database occupies #n(final.database_bytes) bytes. The maximum
reported native-worker peak RSS in the phase receipts is
#n(data.worker_peak_rss_kib_reported) KiB. Compiler, test, native audit,
Lean and renderer memory are excluded from this worker statistic. These
are observed values on this prefix, with no full-class resource
extrapolation or admitted wall-speed comparison. Full IC cost, S and
speedup remain unknown.

There are #n(final.pending) discovered sources pending. The selected-prime
component remains open. Neither this prefix nor its successful checks
establish a class-wide factor-base ranking or completed enumeration.

#pagebreak()
= Workload, controls and durable state

Worker source is pinned to
`b68341cd72b51acf9c5ce6c4f755351cc83053db`, with Rust 1.99.1 and system
SQLite. This is the same validated source as the input checkpoint. The
new protocol was frozen before extension; unrelated current-main
compilation failures remain recorded in the preceding study.

The unchanged spec fixes $a=p-3$, $b=111$, prime subgroup order
`2305843010818082053`,
`G=(1,1796364306280342617)` and
`Q=(992726311114892094,558260110083040833)`.
The checking scalar 539365399 is public synthetic data; the collector
receives G and Q. Degrees are 3,7,11,13,19,23,53,61. Each curve uses a
fresh low-x base with three inverse pairs, twenty requested relations,
and seeds 2026101600 through 2026101611, capped at 100000 target trials
per stream. There is one first-parent transported workload per curve.

The inherited snapshot was preserved locally from the previous verified
Drive round trip. Before expansion, the native verifier checks every
inherited map and source roster and replays 192 collector streams; its
logical dump must match exactly. Every batch then checks all maps,
expanded-source rosters, points, relations, scalars and run flags. The
separate native prefix audit compares all 11330 inherited immutable rows;
only expanded/measured flags may advance. Raw commands, outputs, errors
and exits, including any caps, remain in `results/runs.tar.xz`.

Expansion chunks have at most 32 sources; collection chunks at most 64
curve IDs, each with a 300-second outer limit. Full verification has a
900-second limit for its larger checked population. A timed-out
transactional phase may resume against committed state with a new log;
no stream limit is reduced. The worker uses an 8 MiB SQLite cache,
disabled mmap and disk-backed temporary storage. Disk data still grow.

Both closed batch snapshots are packaged as immutable Drive archives,
metadata-read back, downloaded, manifest-checked and compared byte for
byte. The final downloaded database produces the same native logical
dump. Sanitized receipts omit private Drive identifiers; small archive
round trips do not establish full-class quota. No remote compute or
post-session experiment is provisioned.

Pinned-source library/example tests, native diagnostic tests, Lean
build/axiom checks, archive recovery and report inspection are recorded
in the README and `results/`. Host isolation is unqualified: five
visible Intel Xeon Platinum 8573C CPUs, four-CPU cgroup quota, 16 GiB
memory. Canonical performance graphs remain unchanged; this study adds
prefix/finite-proof evidence without an admitted performance result.

*Sources:* `PROTOCOL.json`, `spec.json`, `continue.sh`, native worker
sources in the pinned commit, raw archive and compact verification
receipts, `visual.rs`, `reports/summary.json`, and the study README.

#pagebreak()
= Selected-prime coverage: what the diagnostic adds

#image("genus.svg", width: 100%)

The native character calculation checks primitive positive forms for the
eight selected degrees and norms coprime to the discriminant. Independent
elimination and enumeration of all 256 subsets agree. Exact form data,
192 additional primitive representations and square-set controls are
retained in `results/genus-span.json` and native test logs.

The subgroup lower bound uses standard genus theory and the preceding
exact order-68162026 class certificate. It is a mathematical derivation,
not an enumerated vertex count. Full generation follows *if* the entire
class number equals 545296208; that number remains conditional after
the earlier certification timeout. No class-number upper bound is proved.

Lean 4.34.1 checks eleven finite/arithmetic theorems, including complete
XOR coverage, coefficient identities, the LCM and conditional size
implications. The repository axiom audit passes with zero extra trusted
axioms; standard foundations are recorded. Formalization is partial:
norm-to-character, ideal-class/genus, the old power certificate and CM
action bridges remain explicit obligations in `proof-map.json`.

The companion coefficient identity is
`IDC1h5d70c781a75cea1c`, with a pointer to the universal integer Lean
proof and 32+32 native evaluation checks over $FF_(2^61-1)$. It does not
certify class coverage. The existing fundamental-discriminant result
continues to exclude vertical base-field edges for this fixed class.

#link("https://github.com/aburan28/crypto/pull/2144")[PR 2144: study, immutable evidence, proof map and remaining obligations]

#pagebreak()
= Exact selected component: additive certificate

#image("component.svg", width: 100%)

The odd generator has exact prime order 34081013. Each torsion basis
element squares to the principal form and all sixteen subset products
are distinct. Odd-order and exponent-two subgroups intersect trivially.
The explicit relations show containment in both directions, giving
exactly 545296208 selected ideal classes. Standard ordinary maximal-order
CM theory transfers this size to the selected-prime curve component.
This is a mathematical certificate, separate from actual enumeration.

The frozen producer uses exact PARI 2.15.2 reduced-form arithmetic and a
5838-entry baby/giant table. A separate fixed-witness checker performs no
logarithm search and rejects both a corrupted exponent and a corrupted
torsion mask. Both paths use the same PARI library and host. No whole
class-number oracle is used. The prior power certificate also replays
byte identically.

The equivalent structure `C68162026 × C2^3` suggests unique coordinates
for a future streaming backend. Lean proves the finite encode/decode
bijection and count. The complete project has fourteen authored theorems;
the gate audits sixteen including two generated equation lemmas, with
zero extra trusted axioms. *Formalization remains partial:* the native
class arithmetic, group interpretation, cardinality argument and CM
bridges remain outside Lean. See `component-proof-map.json`.

This sharpens the preceding genus lower bound without changing its frozen
evidence. It supplies no whole-class upper bound and completes no
traversal. An index-free coordinate walker has not been implemented or
tested; this run still uses SQLite BFS and a growing disk index.

*Sources:* `RELATIONS_PROTOCOL.json`, `class_relations.gp`,
`verify_relations.gp`, `results/class-relations.jsonl`,
`results/class-relations-witness-check.json`,
`COMPONENT-CERTIFICATE.md`, `component_visual.rs`,
`proof/ComponentCoordinates.lean` and `results/component-proof-gate/`.
