#set page(paper: "a4", margin: 17mm)
#set text(font: "Arial", size: 10pt)
#set par(leading: 0.5em)
#set table(inset: 5pt, stroke: 0.4pt + rgb("ccd4dc"))
= N83 root11: larger bases and the index ceiling
*Historical screen, 17 October 2026 · report completed 21 October*

K3584 completed all 128 public points. Its target interval was longer than the earlier K3072 reference, despite fewer recorded probes. K4096 failed the indexed-slot capacity assertion before setup; the frozen stop rule left K4608 and K5120 unrun. This is candidate-discovery evidence on an ordinary shared host, with no admitted performance improvement.

#figure(image("observations.svg", width: 100%), caption: [Recorded geometric target interval and complete process time. Each completed process shares setup across 128 points; the process time is not a cold single-target estimate. Baseline and candidate ran at different times.])

#table(columns: (1.0fr, 1fr, 1fr, 1.5fr),
 [*Arm*],[*Usable points*],[*Columns*],[*Outcome*],
 [K3072 reference],[509,952],[3,072],[Prior replay: 128 points],
 [K3584],[594,944],[3,584],[Completed; 128 points],
 [K4096],[Unknown],[Unbuilt],[Exit 101 at capacity guard],
 [K4608 / K5120],[Unknown],[Unbuilt],[Not run after stop rule],
)

== Comparison and resource accounting
The frozen summary records geometric K3072/K3584 target ratio *0.917294*, with descriptive 95% interval *[0.655474, 1.283693]*. K3584 was faster on 62/128 points. These paired point statistics do not remove host or chronology effects. No fresh same-point rho arm, matched A/A run, reserved-core isolation, or fastest-external-tool comparison was performed here.

K3584's mean probes were 51,096.328 versus the earlier 58,734.656. The packed root table allocation doubled from 5 to 10 GiB. K3584's root-state plus table allocation was 15.461 GiB; the observed child maximum RSS was 12,175,867,904 bytes. Allocation, RSS and useful work are distinct quantities. The 808.509-second process contains setup, rank work and all 128 targets; its outer receipt is 809.567 seconds.

== Frozen execution and interruption states
The ordered screen used K3584, K4096, K4608, K5120, each with an 1,800-second cap; root11 dynamic layout, five-byte indexed table, rank seed 29, four rank threads and one target thread. All 18 fixed KIC flags plus the dump path are retained in the command receipt. K4096 exited after 1.462609 outer seconds with the exact indexed-slot assertion. This is an implementation boundary, not a mathematical failure or a measured online interval.

The original orchestrator raised a post-producer import error after K3584 finished. Its successful raw producer was preserved and independently replayed before the sealed continuation attempted K4096. K3584 was not rerun by that continuation.

#pagebreak()
= Identity, exact bounds and remaining gates

The fixture is the recorded binary Koblitz curve y² + xy = x³ + x² + 1, over the field defined by x^83 + x^7 + x^4 + x^2 + 1. The legacy curve ID is `EC1N83Ckb1a1haacb69ee9f37`; subgroup order is 8,569,786,107,849,059. The generator coordinates are (6,614,244,651,398,184,860,292,015; 8,868,260,410,029,249,391,539,529). The archived generic group checker defines these values and replays the scalar law and subgroup membership.

== Index ceiling proof
For the stated symmetric layout the source count simplifies to (83 K² + K)/2. The current guard requires 83 K² + K < 2³⁰; the exact ceiling is K=3596. The separately flagged wider-ID variant uses 2³¹ and has ceiling K=5086. Both equivalences were independently checked with pinned Lean 4.35.0, using only propext and Quot.sound. These are integer-bound theorems: the source-to-formula connection was inspected separately, and no runtime or memory behavior is certified by them.

== Separate mid-K interruption and prospective holdouts
The later K3088 screen disappeared without a producer exit code or run receipt; the retained interruption receipt records observation at 20:45:21 UTC. Its target file was empty and no base or analysis exists. Status remains `INFRASTRUCTURE_UNOBSERVED_TERMINATION`. K3104, K3114 and K3136 did not run. This event supplies no factor-base ranking.

Two distinct 24-point prospective fixtures remain sealed and unmeasured: the high-K holdout and the later mid-K holdout. The high-K top-two selector reports insufficient verified high-K arms because only K3584 passed. Future execution needs its own authorized, frozen dispatch; this report launches nothing.

== Provenance and reproduction
Frozen plan SHA-256:
#text(size: 8pt)[`89812f798ad2625af7eb1069013abd21fe6d9b68af76bc0e4800dee753f11bb9`]

Executed Rust source SHA-256:
#text(size: 8pt)[`8a2fe55dd90dc33892c197d27260182bf783fc8212fb485d8297daf2bbba8e12`]

Release binary SHA-256:
#text(size: 8pt)[`faaac22aa6228c8d486875d6947fa654417d1122191fbe0eb9bde59bb45eb98f`]

`provenance.json` binds every report input to its original bytes. `raw/` contains the plan, summary, reference analysis, both producer receipts, capacity stderr, proof receipt, holdout manifests and mid-K interruption receipt. The original K3584 base is retained once as `../../results/k3584_base_dump.jsonl.zst`; target rows and commands remain alongside it. No frozen experiment file was rewritten.

The existing `verify.py` checks the original source/binary/input hashes, base reconstruction, four-point relations, recovered scalars and five target timing phases. `summarize.py` reproduces the archived descriptive comparison. See `review-validation.json` for the fresh reviewer replay outcome and exact commands. The directory label uses the retained command file's modification time; it is not an independent timestamp attestation.

Canonical scoreboard, leaderboard and curve-route conclusions receive no promotion: this screen lacks the required matched confirmatory evidence. The dated benchmark copy records the historical observations and failures without changing those conclusions.
