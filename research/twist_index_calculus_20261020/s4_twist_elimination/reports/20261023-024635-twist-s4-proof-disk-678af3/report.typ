#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Theorems compiled; audit failed")

#text("Job 20261023-024635-twist-s4-proof-disk-678af3. Native execution and evidence only; no admitted elapsed-time comparison.")


== #text("Exact execution")

#text("Protocol commits: 509df89c5e, 3ff04c056, 40fbae5a7, e6fc49a7d, as applicable to each stage. The actual executed source snapshot is retained in this job's raw tree.")


#text("Host: c7i.large Spot. Started 2026-10-23T02:47:37Z. Terminal phase failed, exit 1. All launched instances terminated. Exact command: command.sh (command.sh), SHA-256 f23bc8c6d70cbf46659650f813b42ae8f6238fdd6815743f306f023b6dc7abee. Host, environment and rusage receipts are in the extracted raw job tree.")


== #text("Recorded observations")

#text("Both theorem artifacts compiled and print only standard axioms. The separate audit failed because its subprocess could not locate lean on PATH. Whole-job exit 1; no audit pass credited.")


#image("graph.svg", width: 100%)

== #text("Scope and limits")

#text("Structural and collector processes have an 8 GiB address-space cap and 1,200 s watchdog where invoked by the recorded command. Fixed native source and exact raw inputs are retained. No seed or failed stage is dropped. Proof-wrapper setup and extraction do not use the theorem process's cap. Counts shown are recorded native values, with no estimated intervals or timing ratios. F4 row-reduction counters are not complete solver or index-calculus costs. The final prefix A/A checks deterministic output, not timing isolation. Full cost, five-round isolated timing, strongest matched rho calibration and msolve/Magma comparison remain separate uncompleted gates.")


#text("Round report (../../REPORT.md). PDF (report.pdf).")

