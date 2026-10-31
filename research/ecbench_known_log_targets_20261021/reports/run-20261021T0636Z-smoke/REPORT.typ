#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 8pt)
#set heading(numbering: none)
= Arbitrary-target known-log study
#text(size: 10pt)[run-20261021T0636Z-smoke]

This report displays the native recorded-value summary. Each workload is one
public target with no supplied logarithm; cold setup is charged on every run.
The session recorded #text("{'verified': 20}").
Native work is priced equally on all arms at the three displayed weights.
Uncounted work or failed recoveries block rankings. A/A controls are retained in
the table. Raw wall times are descriptive unless the harness admits them at L2.
This report makes no standard-curve, asymptotic, or fastest-tool claim.

#image("cost.svg", width: 100%)

The figure displays native per-curve means, not a fitted growth exponent.
Read paired uncertainty and admission from the comparison JSON beside this
report. The plotted values are copied directly from summary.json.

#table(columns: (2fr, 1fr, 1fr, 1fr, 1fr, 1fr, 1fr), inset: 3pt,
[Arm],
[log₂ r],
[Runs],
[Verified],
[S(0)],
[S(0.1)],
[S(1)],
[abscissa],
[11.807],
[2],
[2],
[28.15],
[29.058],
[37.23],
[abscissa-aa],
[11.807],
[2],
[2],
[28.15],
[29.058],
[37.23],
[known-log],
[11.807],
[2],
[2],
[25.059],
[27.163],
[46.092],
[known-log-aa],
[11.807],
[2],
[2],
[25.059],
[27.163],
[46.092],
[rho-neg],
[11.807],
[2],
[2],
[2.5393],
[2.6788],
[3.9343])

== Evidence
Session: #text("/workspace/crypto/research/ecbench_known_log_targets_20261021/sessions/20261021T0636Z-smoke")

Records SHA-256: #text("199d8f6447534bf124cc87847168964e1c8012b984064df9af757e522bfba748")

Source files: summary.json, audit.json and its SHA-256 file, resources.json,
comparison JSON, COMMANDS.txt, and the sealed session records. Replays here
are on the measuring host; they are not independent hardware validation.
