#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 10pt)
= N83 K600 preflight prepare: host-exec stop

*Observed:* 2026-11-16 07:40:43 UTC, exit 126 before lock creation. The Linux ARM64 supervisor could not execute directly on macOS. No worker or S3 transfer was launched.

#image("diagram.svg", width: 100%)

Raw evidence: `stderr.log`, `exit.txt`, `start.utc`, `end.utc`, `command.txt`. The protocol and source build are linked in `REPORT.md`. Equal second-resolution endpoints do not give a useful elapsed-time estimate.
