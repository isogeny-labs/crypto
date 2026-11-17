#set page(paper: "a4", margin: 18mm)
#set text(font: "Liberation Sans", size: 9pt)
= p=37 full two-dimensional exact-oracle IC
Run ID: `run-20261022T0717Z-quartic-2d-p37`. One target `[271]G`, 624 physical points, 52 folded factor columns. Fifty-three residual queries yielded 2,833,126 complete signed decompositions, rank 53, and verified scalar 271. An independent point-keyed signed-pair oracle replayed all queries and selected rows; the A/A non-timing diff is empty.

#table(columns: (1.5fr,1.4fr,1.4fr), inset: 4pt,
  [phase], [$F_p$ multiplications], [group operations],
  [model], [859], [0],
  [base including axis relifts], [2,567,679], [0],
  [input], [4,741], [46],
  [eigenvalues], [16,618], [156],
  [orbit map], [1,877,044], [17,524],
  [full pair table], [37,708,944], [388,752],
  [53-query stream], [4,003,923,826], [41,277,322],
  [answer checks], [2,004,307], [18,854],
)

Matrix: 369,021 modular row operations. Total L0 time 20232.07 ms is shared-host diagnostic only. Hash and allocator cost, polynomial S5 relation finding, matched rho and GAE pricing remain open. Full commands, source hashes, raw trace and independent audit are linked in `README.md`.

#image("p37_relations.png", width: 100%)
