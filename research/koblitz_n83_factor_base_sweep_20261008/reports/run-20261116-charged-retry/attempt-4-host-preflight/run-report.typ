#set page(paper: "us-letter", margin: (x: 0.75in, y: 0.72in))
#set text(font: "Helvetica", size: 10pt)
#set par(leading: 0.7em)
= K600 charged-build log-name preflight rejection

*Source:* fd6a7849d77a  ·  *Status:* host exit 102  ·  *Scope:* before Docker

The source-correct no-work admission passed 2026-11-16 05:25:51–05:31:14 UTC, verifying the current producer, plan, archive inputs, source and cgroup. The subsequent build launcher rejected an old attempt-2 log filename before it created a Docker container or new build output.

#image("preflight.svg", width: 83%)

The admission's raw log and receipt are retained beside this PDF. The build printed the existing log path and exited 102 before its wrapper, so no new build log or receipt exists. A versioned corrected launcher uses distinct attempt-3 names and requires a new same-commit admission.

This is a preflight naming error. It does not measure build-stage time, relation yield, rank, descent, matched rho or total cold IC cost.
