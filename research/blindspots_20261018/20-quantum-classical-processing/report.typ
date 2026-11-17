#set page(margin: 1.6cm, numbering: "1")
#set text(size: 9.5pt)
#set heading(numbering: none)
= Item 20: classical pre- and post-processing for quantum ECDLP
Blind-spots run 20261018, Claude session 018QnLTPqFUP1K7iRhEfzuiu. Protocol `PROTOCOL.md` (commits ec9e958cd0, 2c4a39d6f5), cards `card-01..06.yaml` (d38cb6fb21), raw records `raw/*.jsonl`.

== Verdict: INCONCLUSIVE (level: constant)
The frozen rule makes the item INCONCLUSIVE if any control fails. The negative control failed in 21 of 567 single-run cells, all at m = 12 (observed null below the chance formula in each; the formula is a union bound that overcounts overlapping boxes: post hoc diagnosis, not a licence). All 8 selftest gates, the positive control, monotonicity, search/criterion agreement and the m = 16 and m = 20 null controls passed. Toy scale (m <= 20), resource model for n = 256, host isolation L0, no hardware claim, no wall-clock result.

#figure(image("graph.svg", width: 100%), caption: [Item 20 graph: (a) model band of the qubits x expected-Toffoli ratio to A0, n = 256; (b) single-run success, S16 nu = 0, B_eta = 0, 4000 runs per cell, Wilson 95%; (c) lattice recovery, 400 trials per cell. Source `raw/model.jsonl`, `raw/s16-nu0.jsonl`, `raw/lattice-m*.jsonl`; plotted by `plot_item20.py`.])

== Arm rows at n = 256 (P-256 and secp256k1), rule output per row
#table(columns: 5, align: (left, right, right, left, left),
[*arm*], [*ratio vs A0*], [*log2 work c=0 / 0.1 / 1*], [*bits to matched rho (c=1)*], [*rule output*],
[E1-0], [1.021], [10.21 / 10.21 / 10.24], [117.6], [MATCHES],
[E1-2^20], [0.966], [18.83 / 18.97 / 19.83], [108.0], [MATCHES],
[E1-2^30], [0.927], [28.83 / 28.96 / 29.83], [98.0], [ADVANCES],
[E1-2^40], [0.887], [38.83 / 38.96 / 39.83], [88.0], [ADVANCES, uncapped],
[E1-2^50], [0.848], [48.83 / 48.96 / 49.83], [78.0], [TRADES, capped],
[E1-2^58], [0.816], [56.83 / 56.96 / 57.83], [70.0], [TRADES, capped],
[E2-s2], [1.715], [6.97 / 7.95 / 10.40], [117.4], [REGRESSES],
[E2-s3], [2.205], [8.45 / 9.75 / 12.41], [115.4], [REGRESSES],
[W(A0)], [0.038 to 0.110], [28.46 at all c (tables)], [99.4], [ADVANCES (model band)],
[W(E1-0)], [0.039 to 0.111], [28.46 at all c], [99.4], [ADVANCES (model band)],
[W(E1-2^40)], [0.034 to 0.095], [38.83 / 38.96 / 39.83], [88.0], [ADVANCES (model band)],
)
Ratios are measured/derived from `raw/model.jsonl` (qubits x expected Toffoli, same band point). Success probabilities are Wilson bounds measured at m = 16 (p1 = 0.9928, p0 upper 0.6024). Against A0p the E1 and W rows are lower still (0.49 to 0.62 for E1) but that column is never decisive: it corrects an assumed success of 1, an accounting change. EH rows are a promise problem and carry no verdict. Matched rho: 0.5 log2(pi r / 4) = 127.83 bits (derived). These are distances remaining for the classical step, not a cross-method ranking; the section 5a band (0, 0.1, 1 GAE) leaves the order unchanged.

== Controls
Positive: S16, sigma 7, B_Delta 100: 0.998 (Wilson lower 0.9961 >= 0.97); sigma 0, Bt = 0: 0.601 [0.586, 0.616], inside [0.40, 0.90]. Negative: random pairs match chance at m = 16 and m = 20; fail at m = 12 (21 cells). Search vs closed criterion: 0 mismatches. BSGS cost: measured/formula 0.90 to 0.92 (B_eta 0), 0.41 to 0.49 (B_eta 1).

== Caveats
Success probabilities extrapolated from m = 16; lattice work from m <= 20; windowed rows are a model band whose own gate sits about 2.4x below Haner et al.'s low T-count edge, so read them as optimistic; E1-2^40 sits at the 2^40 cap at c = 1; P-256 and secp256k1 share one row; no depth, wall clock or hardware claims. Full detail: `README.md`.
