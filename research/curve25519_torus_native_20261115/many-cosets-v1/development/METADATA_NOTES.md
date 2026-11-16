# Metadata development notes

An early byte-retention audit selected every `git diff --name-only HEAD`
path and incorrectly expected that list to contain only the mutable README.
It also included newly staged source/configuration files. The assertion
stopped before writing a preservation result. Corrected the selection to
existing-file statuses D/M/R/T; the resulting retained result is
PRESERVATION.json. All source/qualifier/attestation hash comparisons preceding
the assertion had matched. No frozen scientific file was changed to satisfy it.

A report-spacing edit also initially used repository-relative paths while
the cwd was the report directory. The edit failed without changing bytes;
the renderer then ran on the same earlier source. Repeated the layout edit
from the repository root and inspected both pages of the final PDF. Rendering
logs are retained. No computation or claim was inferred from that path error.

Incremental artifact preflight passed. `git diff --cached --check` reported
four whitespace advisories confined to raw rust-test-01.log (one final
blank line) and raw report.log (two trailing spaces and one final blank
line). Preserve their bytes; no source or prose formatting issue was
reported. These are advisories, not a claimed passing diff check or hosted
merge gate. Existing frozen raw-log advisories from the prior version also
remain unchanged.
