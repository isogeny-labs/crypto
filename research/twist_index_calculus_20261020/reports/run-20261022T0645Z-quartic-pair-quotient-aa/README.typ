#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= p=13 twist-quotient pair-table oracle · A/A
Run ID: `run-20261022T0645Z-quartic-pair-quotient-aa`. Intel Xeon 6975P-C, 32 logical CPUs; shared-host L0. On the same seven recorded residuals, the full and quotient tables returned identical complete signed decomposition sets. The full table has 5,112 entries; the quotient stores 426 orbit representatives and 350 canonical keys. Every pair-entry orbit has size 12. A/A non-timing diff is empty.

#table(columns: (1.3fr,1.2fr,1.2fr), inset: 5pt,
  [phase], [$F_p$ multiplications], [group operations],
  [full setup], [495,864], [5,112],
  [quotient setup], [544,746], [426],
  [full seven queries], [7,029,782], [72,576],
  [quotient seven queries], [96,036,422], [72,576],
)

This is a storage/work-size and operation-count result. Hash probes, allocated bytes and calibrated whole-method costs were not measured. Raw data, command and source hash are in `README.md`.

#image("pair_quotient.png", width: 100%)
