#set page(paper: "a4", margin: 17mm)
#set text(size: 9pt)
#set par(justify: true)
#show raw: set text(size: 7.6pt)
= #text("Collector built; test build failed")

#text("Job 20261023-025202-twist-s4-f4-prefix-9b3f7b. Native execution and evidence only; no admitted elapsed-time comparison.")


== #text("Exact execution")

#text("Protocol commits: 509df89c5e, 3ff04c056, 40fbae5a7, e6fc49a7d, as applicable to each stage. The actual executed source snapshot is retained in this job's raw tree.")


#text("Host: c7i.large Spot. Started 2026-10-23T02:53:11Z. Terminal phase failed, exit 101. All launched instances terminated. Exact command: command.sh (command.sh), SHA-256 220b42e75d7a718da4f1f7ac0cee2309a1e4968a1341b721993b7a2d35c68263. Host, environment and rusage receipts are in the extracted raw job tree.")


== #text("Recorded observations")

#text("The bundle omitted an include_str certificate fixture required by the existing tests. No collector ran. The corrected bundle retains the test and its input.")


#image("graph.svg", width: 100%)

== #text("Scope and limits")

#text("Structural and collector processes have an 8 GiB address-space cap and 1,200 s watchdog where invoked by the recorded command. Fixed native source and exact raw inputs are retained. No seed or failed stage is dropped. Proof-wrapper setup and extraction do not use the theorem process's cap. Counts shown are recorded native values, with no estimated intervals or timing ratios. F4 row-reduction counters are not complete solver or index-calculus costs. The final prefix A/A checks deterministic output, not timing isolation. Full cost, five-round isolated timing, strongest matched rho calibration and msolve/Magma comparison remain separate uncompleted gates.")


#text("Round report (../../REPORT.md). PDF (report.pdf).")

