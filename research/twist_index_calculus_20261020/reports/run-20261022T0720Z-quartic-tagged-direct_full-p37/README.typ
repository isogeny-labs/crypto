#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= Action-tagged pair index · direct_full · p=37
Run ID: `run-20261022T0720Z-quartic-tagged-direct_full-p37`. Intel Xeon 6975P-C, 32 logical CPUs; shared-host L0. One known-answer target and 53 registered residual queries. Full and tagged tables returned identical complete signed decomposition sets on every query; 388,752 postings passed an independent group-addition audit. The non-timing A/A diff is empty.

#table(columns: (1.3fr,1.4fr,1.3fr), inset: 5pt,
  [phase], [$F_p$ multiplications], [group operations],
  [full setup], [37,708,944], [388,752],
  [tagged setup], [7,796,605], [32,396],
  [full queries], [4,003,562,862], [41,273,856],
  [tagged queries], [4,003,562,862], [41,273,856],
)

Retained requested bytes: full 91,007,248; tagged 47,721,464. Setup and query counts exclude common input preparation and the audit. L0 wall times are shared-host diagnostics, not isolated speed evidence. Whole IC, polynomial relation finding, rho and external tools are not measured here. Source, command and raw data are linked in `README.md`.

#image("tagged_pair_index.png", width: 100%)
