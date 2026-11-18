#set page(paper: "us-letter", margin: 0.65in)
#set text(font: "New Computer Modern", size: 9pt)
#set heading(numbering: "1.")

= K13 archived clock boundary

_Post-hoc diagnostic; no new trial or admitted speed claim._

The source is session `ECBS1hc322a24b6610`, SHA-256
`dc16c97ae21a250cc1d47237231d9642960c100dc3fdb68f4dd210af171bff44`.
It contains two public K13 targets and two measured rounds per arm. The script
checked all 48 records and summed the 32 measured rows. Each arm contributes
four measured rows. The record's clocks and work flags are producer assertions.
This PDF restates the source-bound `REPORT.md`; [pcc:unverified].

#image("clock_boundary.svg", width: 100%)

The horizontal axis is logarithmic. Blue is the recorded solve clock; orange
is complete process wall minus solve. Values are sums across four measured
rows, in milliseconds. Matrix arms occupy seconds on this scale. The
batch-m3 solve sum is 1.529 ms; its outside-solve sum is 41.414 ms. F6-IC's
corresponding sums are 150.783 ms and 43.047 ms. These are L0 macOS values,
not an x86 qualification. [pcc:unverified]

== Interpretation boundary

The A/A batch arms have identical reported `bitop/v1` work for every matched
cell, while their process clocks differ. The four batch-m3 solve clocks are
263,292 to 505,042 ns, below the 1 ms admission floor on this host only.
Inside the producer's model, relations account for 13,112,963 of batch-m3's
20,596,967 total bitops; the clock boundary shows why that phase priority
does not directly rank complete-process wall time at this K13 scale.
The frozen EKS v3 screen and full F4/F5/F6 panel remain unrun. An instruction
audit is still needed before the producer's model-work flags can support a
priced ranking. [pcc:unverified]

See `PLAN.md`, `AMENDMENT-1.md`, `SUMMARY.json`, `analyze.py`, and `REPORT.md`
for selectors, exact sums, failure history, and limitations. The independent
arithmetic audit of reported scalars is in
`../rootfind_k13_l0_independent_audit_20261116/REPORT.md`.
