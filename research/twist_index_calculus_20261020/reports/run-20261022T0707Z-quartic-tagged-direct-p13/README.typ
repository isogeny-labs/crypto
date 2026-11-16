#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= Action-tagged pair index · direct · p=13
Run ID: `run-20261022T0707Z-quartic-tagged-direct-p13`. Intel Xeon 6975P-C, 32 logical CPUs; shared-host L0. One known-answer target and 7 registered residual queries. Full and tagged tables returned identical complete signed decomposition sets on every query; 5,112 postings passed an independent group-addition audit. The non-timing A/A diff is empty.

#table(columns: (1.3fr,1.4fr,1.3fr), inset: 5pt,
  [phase], [$F_p$ multiplications], [group operations],
  [full setup], [495,864], [5,112],
  [tagged setup], [154,922], [426],
  [full queries], [7,029,782], [72,576],
  [tagged queries], [7,029,782], [72,576],
)

Retained requested bytes: full 1,273,360; tagged 681,932. Setup and query counts exclude common input preparation and the audit. L0 wall times are shared-host diagnostics, not isolated speed evidence. Whole IC, polynomial relation finding, rho and external tools are not measured here. Source, command and raw data are linked in `README.md`.

#image("tagged_pair_index.png", width: 100%)
