#set page(paper: "a4", margin: 17mm)
#set text(font: "New Computer Modern", size: 9pt)
= N83 smaller-base m=4 controls
_Verified control result · 2026-10-19 · small-m4-v1_

Both K2048 and D12 passed exact import, a selector-and-phase-pinned four-summand SAT control, group/row checking, and separate general-field replay on Linux x86_64 and macOS arm64. Each checked row has rank one. All six SSH preflight/control/replay containers exited zero without OOM. The SSH card started at 18:54:41 UTC.

#image("controls.svg", width: 100%)

#table(columns: (1fr, 1.7fr, 1.7fr), inset: 5pt,
  [*Measured field*], [*K2048*], [*D12*],
  [Columns / points], [2,048 / 339,968], [2,001 / 332,166],
  [Variables / clauses], [84,966 / 900,062], [84,590 / 883,894],
  [Import / model (ms)], [469.546 / 75.336], [454.976 / 70.489],
  [Root / search (ms)], [108.535 / 0.334], [105.260 / 0.290],
  [Cgroup peak (bytes)], [226,369,536], [217,911,296])

One observation per base. The seven x blocks, selectors and phase-zero choices were pinned; both controls returned with zero decisions/conflicts and three XOR passes. The different constructed targets and shared L0 host provide no admitted timing ratio or base ranking. CPU 31 had a one-CPU quota, hard 4 GiB memory, zero swap and no worker network. Other research and migration workloads shared the host.

The common-public-point K1,D1,D2,K2 card is in progress at this control boundary. Its first unpinned public arm entered search and emitted progress. This report and archive cover completed controls only. Natural relation yield, useful rank, full IC cost and matched same-point rho remain open; candidate and best-runtime fields stay null. Canonical leaderboard/boundary graphs are unchanged because this is a constructed correctness gate.

== Sources and reproduction

Study: `research/n83_prebuild_cost_m23_20261019/`. See `SMALL_M4_PROTOCOL.md`, `SMALL_M4_HOST_CARD.md`, `SMALL_M4_HOST_CARD_V2.md`, this directory's `RUN_REPORT.md`, `controls-data.json`, and `local-replay/`. Extract and hash-check the archive via `small-m4-v1/controls/README.md`.

Worker commit: `611aad60e4d9952d6239a3382a77184da3640eeb`. Control archive SHA-256: `076c60d6ba10127e9d07be2e6ccab5240f7dcc9356c763f2f96f5b86556d03ae`. Native binaries and all exact input hashes are recorded in the host card and archive.
