#set page(paper: "us-letter", margin: 0.7in)
#set text(font: "Helvetica", size: 10pt)
= N83 K600 zero-trial preflight: executable-selection stop

*Observed:* 2026-11-16 08:17:25–08:17:28 UTC; outer exit 1, native `PRODUCER_FAILURE_worker_exit`, worker exit 1, no timeout. A fresh 3,025,798-byte S3 GET matched the frozen SHA-256 before container launch. The mounted mixed-orbit worker did not implement `primary-store-v3-orbit-chain-cold` and printed usage instead. No circuit or worker summary was created.

#image("diagram.svg", width: 100%)

Raw: `launch.json`, `native-outer.json`, `worker.stderr.log`, `S3_INPUT.json`, `failed-lock-H1.json`, wrapper timestamps and exit. Source and corrective amendment are linked in `REPORT.md`.
