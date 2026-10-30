#set page(paper: "a4", margin: 22mm)
#set text(font: "Liberation Sans", size: 10pt)
= Tian quartic Jacobian arithmetic gate

*Receipt written:* 2026-10-20T07:05Z UTC. *Command:* `tian-quartic-km` from `native/Cargo.toml`. *Host:* shared Intel Xeon 6975P-C; L0 operation counts only.

The native evaluation-space Khuri–Makdisi implementation constructed Tian's published quartic over F523 and the published classes S,T. It checked T=1987S, rS=0, inverse, commutative and associative controls. The known scalar 1987 is an input to this certificate; the run is not a destination-side DLP solver. The raw receipt is `correctness/tian-quartic-km-v2.json`.

At M+0.5S+10I, one measured quartic doubling costs 13318526 base-field multiplication equivalents, 14.043× the mean of five complete source-rho controls. Computing the deliberately known scalar 1987S costs 212.418× that mean. The complete price grid is `results/tian-km-arithmetic-gate-price-grid.csv`.

#figure(image("arithmetic-gate.svg", width: 100%), caption: [Recorded arithmetic gate ratios to the five-seed mean source-rho cost; no transfer DLP attack ratio is shown.])

The source-to-Jacobian map, quotient kernel verification and a destination DLP solver remain unexecuted. No wall-speed or external-system comparison is claimed.
