#set page(paper: "a4", margin: 16mm)
#set text(size: 9.5pt, font: "DejaVu Sans")
#set heading(numbering: none)
= Native Koblitz cached lifting
2026-10-30 · Engineering implementation candidate · Generic decomposition track

#block(fill: rgb("fff0dc"), inset: 10pt, radius: 4pt)[
  *Measurement blocked.* Local checkpoint upload lacks an approved uploader.
  Actions run 41082358257 also failed: the upload secret is not configured.
  All build/experiment jobs were skipped. No native speed ratio is claimed.
]

#image("pipeline.svg", width: 100%)

== Implementation
Lazy, curve-local byte tables evaluate the existing odd-degree halftrace. The
explicit factor-base parameter `halftrace_tables=1` preserves the first root,
ordered points, signed orbit maps and existing relation/LA implementation.
Default recipes retain the original point-lifting path. Initialized immutable
tables are shared by clones. No field-wide table or new Python path is introduced.

== Verification and accounting
The original release suite completed: *5447 passed, zero failed, 123 existing
ignored*. Final-source cached/ordered tests, both native E2E examples and strict
library Clippy pass separately. The suite predates only the documented parity
lint cleanup. Later main merges preserve the cache but expose successive upstream
parser errors; the latest is in binary_semaev_chain_sat.rs. Exact source
scope and failures are retained under `development/`. Test times are not benchmarks.

The frozen comparison has eight independent one-target workloads, the same
base and seed in both arms, strong-rho and IC A/A controls, six alternating
measured rounds and one warmup. Its canonical protocol is prospectively frozen.
Of 280 planned scientific executions, zero were attempted. EKS Spot L2 and
independent replay are mandatory; cold cache construction is included.

== Limits and sources
The degree-31 invariant subspace has ord_31(2)=5; it does not discharge the
degree-83 confidence gate or establish transfer to ECC2K-130. Existing coarse
Artin–Schreier solve counters alone do not price the cache delta. External
arithmetic comparisons have not been measured. The scoreboard and leaderboard
remain unchanged because no admitted performance result exists.

Saved baseline source: `3b34d504bc13ce668e86d8e68cdf57e3fbaba89a`.
Scientific source pin: `7c58661935b79153ff74c1208d0592aa56ed04a1`.
The primary binary point-recovery contract is
#link("https://www.secg.org/sec1-v2.pdf")[SEC 1 v2.0, section 2.3.4]. Existing
`src/binary_ecc/f2m.rs` supplies the native trace/halftrace precedent.
The study's `README.md`, `PROTOCOL.md`, `protocol.json` and `BASELINE.txt` retain
source paths and evidence links. Same-binary flag comparison does not establish
regression against the unmodified-source binary. P1 remains open and auto-merge
is paused. Access gap: `problems/native-koblitz-measurement-access.md`.
