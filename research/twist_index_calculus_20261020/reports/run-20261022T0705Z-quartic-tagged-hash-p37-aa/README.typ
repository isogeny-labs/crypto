#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= Action-tagged pair index · hash · p=37 · A/A
Run ID: `run-20261022T0705Z-quartic-tagged-hash-p37-aa`. Intel Xeon 6975P-C, 32 logical CPUs; shared-host L0. One known-answer target and 3 registered residual queries. Full and tagged tables returned identical complete signed decomposition sets on every query; 388,752 postings passed an independent group-addition audit. The non-timing A/A diff is empty.

#table(columns: (1.3fr,1.4fr,1.3fr), inset: 5pt,
  [phase], [$F_p$ multiplications], [group operations],
  [full setup], [37,708,944], [388,752],
  [tagged setup], [7,796,605], [32,396],
  [full queries], [226,616,754], [2,336,256],
  [tagged queries], [226,616,754], [2,336,256],
)

Retained requested bytes: full 91,007,248; tagged 46,196,696. Setup and query counts exclude common input preparation and the audit. L0 wall times are shared-host diagnostics, not isolated speed evidence. Whole IC, polynomial relation finding, rho and external tools are not measured here. Source, command and raw data are linked in `README.md`.

#image("tagged_pair_index.png", width: 100%)
