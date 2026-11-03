#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= p=13 two-dimensional IC pilot
Run ID: `run-20261022T0626Z-quartic-2d-p13`. Intel Xeon 6975P-C, 32 logical CPUs; shared-host L0. One target `[271]G`; 72 base points, six 24-fold columns. Seven queries yielded 4,041 exact signed decompositions, seven selected relations, rank seven and verified scalar 271. Independent replay checked all queries, rows and factor logs. A/A non-timing diff is empty.

#table(columns: (1fr,1.2fr,1.1fr), inset: 4pt,
  [phase], [$F_p$ multiplications], [group operations],
  [model], [650], [0],
  [base], [216,201], [0],
  [input], [3,560], [35],
  [eigenvalues], [6,478], [62],
  [orbit map], [149,262], [1,422],
  [pair table], [495,864], [5,112],
  [relation stream], [7,062,416], [72,897],
  [answer check], [162,265], [1,548],
)

Matrix: 460 modular row operations. Wall: 30.75 ms, shared-host L0. Hash work, calibrated rho and the polynomial relation system remain unpriced. Raw data, commands and source hashes are in `README.md`.

#image("p13_relations.png", width: 100%)
