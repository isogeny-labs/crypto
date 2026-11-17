#set page(paper: "a4", margin: 2cm)
#set text(size: 9.5pt)
#set heading(numbering: "1.")

= Run report — twist obstruction, run-20261026T1456Z

*Observations only.* The interpretation is in `../../REPORT.pdf`.

#table(columns: 2, stroke: 0.4pt,
  [Command], [`twist-obstruction full reports/run-20261026T1456Z` (release build)],
  [Commit], [`ff3bf9d485ebb27350542f0203a6bac090f1835a` (harness and frozen `PROTOCOL.md`)],
  [Protocol sha256], [`ed94554d0e65fd814590bcc617439efc85d3a59c51c979fae97722f1055ef358`],
  [Seed], [`0x7715720261026` (splitmix64)],
  [Host], [Claude Code cloud container, Linux 6.18.44, 4 vCPU, unisolated, single thread],
  [Wall time], [269.0 s (recorded in `run.json`; not a result)],
  [Rows], [4969: 15 census rows and 4954 decomposition targets. Every requested row completed; there were no failures.],
  [Raw records], [`rows.jsonl` (sha256 `d8cab334…c41189`), `summary.json` (sha256 `4dd346bb…249f97`), `stderr.log`],
)

== Census (exhaustive, n = 7, 11, 13; 5 curves each)

Every P ∈ E(F\_q) was paired with every Q ∈ φ(E′(F\_q)). Measured:
- 356,633,920 pairs in total.
- Q not 2-torsion: 356,530,036 sums, of which *0* are σ-fixed.
- Q ∈ {O, T}: 103,884 sums, of which all are σ-fixed.
- Count checks, *0 mismatches*:
  - |σ = −1 part| = \#E′(F\_q), with \#E′ counted independently on the twist model;
  - φ(E′(F\_q)) lies in the σ = −1 part;
  - \#E + \#E′ = 2q + 2.

== Decompositions (curve model E\_a and twist model E\_(a+1), same x(R))

All values are measured counts from `summary.json`. "Off" means a solution
with at least one coordinate whose σ-parity is opposite to R's. The
interval is a 95 % Clopper–Pearson interval, derived by `summarize/`.
"Heuristic" is the solution-weighted 1 − p\_E^m (derived).

#table(columns: 9, stroke: 0.4pt, align: right,
  [arm], [n], [m], [targets], [solutions], [off obs], [off pred], [off fraction \[95 %\]], [heuristic],
  [A], [11], [2], [1200], [4836], [0], [0], [0 \[0, 0.0008\]], [0.737],
  [A], [13], [2], [1200], [4415], [0], [0], [0 \[0, 0.0008\]], [0.765],
  [A], [17], [2], [800], [3178], [0], [0], [0 \[0, 0.0012\]], [0.750],
  [A], [19], [2], [600], [2339], [0], [0], [0 \[0, 0.0016\]], [0.754],
  [A], [11], [3], [300], [6582], [416], [416], [0.063 \[0.057, 0.069\]], [0.871],
  [A], [13], [3], [300], [11377], [463], [463], [0.041 \[0.037, 0.045\]], [0.893],
  [A], [17], [3], [200], [4256], [0], [0], [0 \[0, 0.0009\]], [0.863],
  [A], [19], [3], [150], [6389], [0], [0], [0 \[0, 0.0006\]], [0.874],
  [C1], [11–19], [3], [170], [39861], [18042], [18042], [—], [—],
  [C2], [11–19], [3], [34], [1098], [95], [95], [—], [—],
)

Decision-rule counters, summed over every arm and both models. *All are 0.*
- violations (off-parity sub-sum ∉ {O, T});
- witness failures;
- targets where the observed off set ≠ the predicted off set;
- targets where the x-set differs between E\_a and E\_(a+1);
- point solutions missing from the explicit-polynomial roots;
- unexplained extra roots.

Explicit-polynomial cross-checks:
- S₃ on all 3800 m = 2 targets: 14,768 roots, equal to the point sets.
- S₄ = Res\_X(S₃, S₃) on 105 m = 3 targets: 2910 roots, equal to the point
  sets, with no degenerate extras.

Every tally (pure, 2-torsion, off\_O, off\_T) is identical on the curve model
and the twist model for every arm. The p\_E measured over A targets is
0.46–0.51 by family.

Positive controls:
- C1 (x\_R planted in V): the predicted set is non-empty on 170 of 170
  targets, and observed = predicted on every one.
- C2 (planted T-pairs): 95 off\_T solutions in total, all predicted.

#figure(image("fig1_off_parity_fraction.svg", width: 100%),
  caption: [Arm A, run-20261026T1456Z. Observed fraction of solutions with an off-parity coordinate (95 % CP) beside the independent-lift heuristic. N = solutions per family.])
#figure(image("fig2_predicted_vs_observed.svg", width: 62%),
  caption: [m = 3, curve model. Per target: predicted off-parity count (built from the target and V only) against the observed count. Arms A, C1 and C2; there are 1154 targets.])
#figure(image("fig3_mechanism.svg", width: 100%), caption: [Diagram of the eigenspace decomposition the run checks.])
