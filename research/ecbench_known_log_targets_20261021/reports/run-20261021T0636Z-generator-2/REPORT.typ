#set page(paper: "a4", margin: 16mm)
#set text(font: "DejaVu Sans", size: 8pt)
#set heading(numbering: none)
= Arbitrary-target known-log study
#text(size: 10pt)[run-20261021T0636Z-generator-2]

This report displays the native recorded-value summary. Each workload is one
public target with no supplied logarithm; cold setup is charged on every run.
The session recorded #text("{'verified': 288}").
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
[15.697],
[40],
[40],
[10.628],
[11.222],
[16.564],
[abscissa-aa],
[15.697],
[40],
[40],
[10.628],
[11.222],
[16.564],
[known-log],
[15.697],
[40],
[40],
[7.542],
[8.104],
[13.162],
[known-log-aa],
[15.697],
[40],
[40],
[7.542],
[8.104],
[13.162],
[rho-neg],
[15.697],
[40],
[40],
[1.8596],
[1.9786],
[3.0496],
[transported],
[15.697],
[40],
[40],
[7.671],
[8.2371],
[13.332])

== Evidence
Session: #text("/workspace/crypto/research/ecbench_known_log_targets_20261021/sessions/20261021T0636Z-generator-2")

Records SHA-256: #text("756006e86bbc8b03c0b7b7ccb78dc0492b4c1436604649be6bf5212ec591460b")

Source files: summary.json, audit.json and its SHA-256 file, resources.json,
comparison JSON, COMMANDS.txt, and the sealed session records. Replays here
are on the measuring host; they are not independent hardware validation.
