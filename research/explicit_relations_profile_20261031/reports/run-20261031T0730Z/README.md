# V1 query-profile report

Raw data commit `b50e30d47458` was pushed before this report was rendered.
The graph is the byte-identical output of frozen `analyze.rs`; the table is
presentation of the native summary, with no new trial or statistical analysis.
Both PDF pages were visually inspected. The directory uses the worker launch
minute (2026-10-31T07:30Z) as the run identity; profile data followed preflight.

Reproduce into a fresh directory with the supplied `render-report.py` and a
Python environment containing the Typst binding. Optional second argument is
the study directory; default is inferred from this report's location. Python is
used only for presentation. Native measurement and analysis remain Rust/CUDA.
