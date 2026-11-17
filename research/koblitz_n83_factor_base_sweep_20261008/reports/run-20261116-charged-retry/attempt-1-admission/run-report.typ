#set page(paper: "us-letter", margin: (x: 0.75in, y: 0.72in))
#set text(font: "Helvetica", size: 10pt)
#set par(leading: 0.7em)
= K600 charged-build retry admission timeout

*Source:* e906c79d8a78  ·  *Status:* admission cap  ·  *Scope:* no-work check

The frozen admission ran 2026-11-16 04:25:15–04:28:04 UTC. Its 120-second soft cap returned exit 124; the Docker client/container remained until an operator stop. The wrapper wrote exit 124 after a 169-second outer interval. The raw log is zero bytes, so the slow individual gate cannot be identified.

#image("admission.svg", width: 90%)

After the stop, the named container was absent, the new charged-build output directory was absent, and host source stayed clean. The frozen protocol, raw log, receipt and exact source commit are retained beside this report. A new admission needs a hard-kill watchdog and progress markers while preserving every original source/input/host check.

This attempt has no Rust build, solver arm, relation, rank, descent, matched rho or total IC measurement. It cannot select a factor base.
