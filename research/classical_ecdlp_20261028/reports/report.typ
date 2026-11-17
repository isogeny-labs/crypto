#set page(paper: "a4", margin: (x: 17mm, y: 16mm), numbering: "1")
#set text(font: "DejaVu Sans", size: 9pt, fill: rgb("18283b"))
#set par(justify: false, leading: 0.6em)
#set heading(numbering: none)
#show link: set text(fill: rgb("126c77"))

#text(size: 21pt, weight: "bold")[Classical ECDLP followups]
#parbreak()
#text(size: 12pt)[APS sources and an independent guess-and-determine archive]
#parbreak()

Import date: 28 October 2026. Historical bundles: 26–27 October 2026.
Scope: classical methods, public synthetic fixtures, correctness and provenance.

The independent historical implementation recorded 60 solved known-answer
instances on six small prime-order curves. Six complete witnesses also have
successful Magma 2.30-1 receipts. The active Rust path checks the saved answers
and witnesses using the repository's existing W12 matrix/zero-minor engine.
This evidence establishes no performance advantage over rho.

#image("classical-routes.svg", width: 100%)
#text(size: 8pt)[Figure 1. Source/contribution map. Arrows indicate the investigation's
method routes, not measured improvements or a claim of direct derivation.]

== Sources and access

- #link("https://doi.org/10.1016/j.ffa.2018.01.009")[Amadori–Pintore–Sala,
  FFA 51 (2018), 168–182]: one usable labelled relation avoids the usual
  factor-base logarithm linear-system stage. Accepted manuscript read.
- #link("https://eprint.iacr.org/2017/1262")[McGuire–Mueller, ePrint 2017/1262]:
  direct summation-polynomial evaluation and signed-point sums. Public author
  definitions executed unchanged with our driver in six small Magma checks;
  direct paper-PDF retrieval failed.
- #link("https://doi.org/10.1007/978-3-030-00434-7_19")[Kudo et al., CANS 2018,
  377–393]: hybrid exhaustive/Gröbner acceleration and its limitation.
  Publisher and precursor abstracts available; full chapter not audited.
- #link("https://doi.org/10.1515/jmc-2019-0029")[Yokoyama et al., JMC 14 (2020),
  460–485]: conditional bounds for a specified naive setting. Full text read;
  not a universal impossibility result for APS.
- #link("https://unire.unige.it/handle/123456789/6368?show=full")[Lanteri, 2023]:
  public abstracts document C++/CoCoA implementations; thesis restricted.
- #link("https://doi.org/10.1016/j.ffa.2024.102452")[MultiSolve, 2024] and its
  #link("https://doi.org/10.3934/amc.2026024")[oracle-based successor, 2026]:
  possible solver technology, demonstrated on other cryptanalytic applications.
  No APS-over-rho improvement is established by these sources here.
- #link("https://arxiv.org/html/2607.09814v2")[Mahalanobis, 2026, v2]:
  guess-and-determine zero-minor route. It uses matrix algebra, unlike the
  original linear-algebra-avoidance objective. The #link("https://github.com/isogeny-labs/crypto/tree/main/research/lit2026_w12_flagged_claims_20261011")[prior repository W12 audit]
  already implements and assesses this method.

The full 14-record catalogue and individual access limits are in
`historical/aps/sources.json`. The historical README access snapshot for the
2026 paper predates the subsequent independent replication.

Flow: 19 source files imported; 60 saved instances and six full witnesses
replayed; six public-target native regressions; zero new performance cases.
No historical case was excluded. The source-linked study README records
remaining access and performance gates. Conductor's local control plane was
unavailable; no task lease is claimed.

#pagebreak()
== Verified population and finite count diagnostic

For each curve below, the archive contains ten distinct trial IDs. All curves
have prime full group order, cofactor one and non-anomalous order. The exact
generators and all public targets are in the README and raw archive.

#table(
  columns: (1fr, 1fr, 1fr, 1fr), inset: 5pt, stroke: 0.4pt + rgb("cbd5df"),
  table.header([Field p], [a, b], [Group order r], [Saved cases]),
  [251], [1, 4], [271], [10],
  [1021], [1, 6], [991], [10],
  [4093], [1, 22], [4049], [10],
  [16381], [2, 16], [16567], [10],
  [65521], [1, 35], [65761], [10],
  [1048573], [1, 14], [1048193], [10],
)

#image("candidate-counts.svg", width: 100%)
#text(size: 8pt)[Figure 2. Exact finite enumeration for one outer guess, h=6,
d=2 or 3. Source: the frozen `probability-audit.json`; native enumeration is
recorded in `validation/native-replay.json`. No sampling uncertainty applies to these finite
counts. This does not prove a general probability law or asymptotic cost.]

The existing W12 engine already distinguishes candidate multiplicity from
distinct sets. No new counting theorem, algebraic identity or Lean proof is
claimed. The historical probability calibration is a separate scalar sample;
it is not an EC timing experiment.

== Replay and limits

The native checker validates all 60 known-answer fixtures and 120 recorded arm
answers, recomputes six curve orders, and checks six complete witnesses:
evaluation/kernel ranks, K*M=0, labels, signature kernels and index maps,
zero determinant, elliptic point sum, and scalar recovery. Negative tests
corrupt a kernel, label and minor-column map. A separate native regression
uses the existing W12 solver on six trial-zero public targets; its RNG and
shortcut paths differ from the historical Python implementation.

The original timing data remain historical Python diagnostics against simple
three-part Floyd rho. Host, isolation, affinity, A/A calibration and strongest
matched rho metadata are missing. No timings were remeasured, no new performance
experiment was run, and no benchmark or leaderboard row is admitted.
The stronger prior W12 audit remains the repository's relevant comparison.

== Evidence and reproduction

Study root: `research/classical_ecdlp_20261028/`.
`HISTORICAL.sha256` covers 19 byte-preserved files.
`historical/guess_determine/results.json` contains the raw 60-case archive;
the two `witness-magma` and `magma-author-code-check` receipts retain CAS status.
`validation/` holds native replay, solve and development-check receipts.

#text(font: "DejaVu Sans Mono", size: 7.6pt)[
cargo test --release --locked --example classical_ecdlp_archive \
cargo run --release --locked --example classical_ecdlp_archive -- check \
cargo run --release --locked --example classical_ecdlp_archive -- solve
]
