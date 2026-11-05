# Archive verification and rendering

Run `sha256sum -c manifest.sha256` from this packet directory. The nested `evidence/manifest.sha256` is the original source manifest and uses absolute private-source paths; the outer manifest binds the copied files independently. Source evidence is byte-preserved. Empty Docker configuration directories are excluded; no credentials or daemon configurations are published.

Run `python3 render-report.py` to rebuild the editable flow.svg and report.pdf from REPORT.md. Requires rsvg-convert and pdflatex. The diagram is an evidence-state diagram, not a performance plot. Original source scripts under evidence are archived for provenance, not a reproduction command: reconcile-followup.py issues Docker inspections and make-ledger.py rereads original private context paths. Do not execute them as part of archival verification. No build or scientific execution is authorized by this packet.

Inspect report.pdf visually after rendering. Rendering may leave report.aux, report.log and render.log; these derived logs are not scientific evidence. Source privacy review screened credential patterns and inspected selected Docker metadata/commands and historical receipts. Published owner labels, digests, paths and timestamps identify this task only.
