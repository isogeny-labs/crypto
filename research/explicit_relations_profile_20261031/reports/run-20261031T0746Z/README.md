# V2 external-profile supplement

Raw records were pushed as `5d4713f59fa3` before rendering this report. Both
pages were visually inspected. Graphs are the byte-identical native wrapper
outputs; the table presents values from the preserved Nsight summary JSON.
The directory uses the worker launch minute as run identity.

`render-report.py STUDY_DIR` renders into this fixed fresh report directory and
refuses an existing destination. The Typst source and original plots are also
provided for direct reproduction elsewhere. Python is presentation only.

Actual GPU service SHA is recorded in results-v2/instruments.sha256; native
wrapper manifests identify the bash launcher. Unsupported counter attempts
are retained alongside successful timelines.
