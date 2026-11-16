#set page(paper:"a4",margin:18mm)
#set text(size:10pt)
= Pivot/swap replay: retained failed attempt
Native terminal result: failed. No repeated attempt was launched for this report.
#image("flow.svg",width:100%)
== Observed compiler result
EliminationField compiled. PivotSwap failed at line 59 in swap_involutive: after a = b substitution, simplification left a nested conditional depending on i = a. Two unused-simp-argument warnings were also retained. The subsequent controls, 17-name readback and expected False rejection were not reached.
== Custody and accounting
Pre-custody passed in 45.61097849800717 seconds; compiler phase failed in 42.47305457800394 seconds; post-custody passed in 40.05902259401046 seconds. Native total elapsed time was 128.24891154497163 seconds. All three terminal-owner checks were true.
The full reserved 2400 worker-seconds is charged. Campaign cumulative charge is 3600 seconds and remaining allowance is 126000 seconds according to the coordinator's terminal reconciliation. These charges are distinct from measured phase elapsed times. No scalar-operation or rho cost ratio was measured.
== Retained evidence
The complete native workspace and outer supervisor files are archived without modifying the originals. Every archived file has a recorded byte count and SHA256; the canonical remote archive was downloaded and hash-verified. The failed proof, stdout and unreached controls remain explicit. Any proof correction is a separate prospective successor and requires independent review and admission.
