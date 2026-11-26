#set page(paper: "a4", margin: 18mm)
#set text(font: "DejaVu Sans", size: 9.5pt)
= Half-trace elimination: engineering preflight
TZH-20261123. Current merged adjoint solver, two frozen field-builder binaries; identical fixed queries and all per-query setup charged.

#table(columns: (1.7fr,1.1fr,1.4fr,1fr), [Fixed d12 case], [Arm], [Whole cold Ir], [Output],
[n83/planted-1], [baseline], [27,362,118], [Found],
[n83/planted-1], [elimination], [22,063,082], [Found],
[n83/planted-1], [baseline-aa], [27,362,115], [Found],
[n83/uniform], [baseline], [33,333,898], [Refuted],
[n83/uniform], [elimination], [28,074,028], [Refuted],
[n83/uniform], [baseline-aa], [33,333,954], [Refuted],
[n131/planted-1], [baseline], [39,971,165], [Found],
[n131/planted-1], [elimination], [30,900,678], [Found],
[n131/planted-1], [baseline-aa], [39,971,162], [Found],
[n131/uniform], [baseline], [44,023,408], [Refuted],
[n131/uniform], [elimination], [34,999,782], [Refuted],
[n131/uniform], [baseline-aa], [44,023,460], [Refuted],)

All12 pilot calls complete with valid outputs; planted targets are Found and replay. Baseline and elimination phased/unphased partition sums satisfy the frozen0.1% tolerance. These are unsigned producer checks; no primary improvement conclusion is drawn from the pilot. [pcc:unverified]

CPU0,2GiB child limit, zero retries/paid resources. The measured engineering durations are used only to estimate whether the complete calibration/pilot/freeze/90-row primary panel fits600seconds. The original case selection,5% cold criterion and no-regression/A/A rules are unchanged.

Evidence: calibration.json, preflight.json, preflight-gate.json; protocol.json; checks/qualification-claim.txt. Primary instruction summaries will be computed by the frozen Rust analysis, after source/protocol publication. Profile discovery is separately recorded in profile.pdf.
