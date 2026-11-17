#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Complete coefficient profile")

#text("Job 20261023-022455-twist-s4-profile-30c47f. Native execution and evidence only; no admitted elapsed-time comparison.")


== #text("Exact execution")

#text("Protocol commits: 509df89c5e, 3ff04c056, 40fbae5a7, e6fc49a7d, as applicable to each stage. The actual executed source snapshot is retained in this job's raw tree.")


#text("Host: c7i.large Spot. Started 2026-10-23T02:25:42Z. Terminal phase done, exit 0. All launched instances terminated. Exact command: command.sh (command.sh), SHA-256 4bed16e7087aa3da986a2ee9633f4b594218d7746ef6ada274e438d39d79b28c. Host, environment and rusage receipts are in the extracted raw job tree.")


== #text("Recorded observations")

#text("All 46,200 finite peels match independently interpolated coefficients. All original and linearly reduced component degree profiles remain [4,4,4,4]. IDC and 13 native tests passed.")


#image("graph.svg", width: 100%)

== #text("Scope and limits")

#text("Structural and collector processes have an 8 GiB address-space cap and 1,200 s watchdog where invoked by the recorded command. Fixed native source and exact raw inputs are retained. No seed or failed stage is dropped. Proof-wrapper setup and extraction do not use the theorem process's cap. Counts shown are recorded native values, with no estimated intervals or timing ratios. F4 row-reduction counters are not complete solver or index-calculus costs. The final prefix A/A checks deterministic output, not timing isolation. Full cost, five-round isolated timing, strongest matched rho calibration and msolve/Magma comparison remain separate uncompleted gates.")


#text("Round report (../../REPORT.md). PDF (report.pdf).")
