# 2026-10-28 — completed E12b report handoff

## What was done

- Re-downloaded the completed job archive from S3 and verified SHA-256 and
  all four raw rows. Copied the identical archive to the canonical artifact
  bucket with create-only semantics and verified another full download.
- Committed and pushed raw rows/provenance before producing the report.
- Independently recomputed the fixed lower-median reading, checked retained
  gate references, and added a read-only archive/record verifier.
- Published a new censoring-aware graph, editable sources and inspected PDF.
- Appended the completed continuation to `RESULTS.md`, preserving historical
  partial reports. Included this new raw file in the existing count exporter.
- Verified live E12a/E12c output snapshots by full download and SHA-256 at
  06:02 UTC; both still contained zero completed target draws. They remain
  the original running attempts; no duplicate job was launched.

## Decisions made

- The registered statistic is the second-smallest of four, not the average
  of two central values. Censored rows score cap plus one; they remain
  censored in the dataset and graph.
- Report “mixed” as frozen-rule arithmetic only. No independent closure
  correctness, general theorem, statistical significance or Lean extension
  is claimed. Do not pool old draw 0 with its repeated identity-gate record.
- Preserve the user's total $100–$200 envelope with $200 as the ceiling;
  this reporting step launches no compute instance. The budget record
  already reserves existing E12 and proposed pilot costs conservatively.
- The separately owned native dataset producer remains authoritative for
  its implementation and pilot submission. Its release binary and 5016
  passing library tests are software validation, not produced dataset cells.

## Open threads

- E12a degree-5 and E12c N=84 window runs remain in progress, with output-file
  snapshots rather than resumable closure-memory checkpoints.
- Check required CI/reviews for this report's exact PR head before declaring
  merge complete. The native ML pilot still has no observed portable receipt
  or completed cell as of the 06:01 UTC inventory.
- Independent closure validation and the broader research/dataset goal
  remain unfinished. No automatic retry or larger run is authorized here.

## Errors

- Conductor T-703 had been cancelled before claim and could not reopen.
  A fresh scope check was clear; T-705 owns this completed report scope.
- System Python lacked boto3; the S3 download/check used the existing AWS
  CLI instead. This changed no worker or cloud configuration.
- The first PDF graph clipped its long axis label. Regenerated it with a
  two-line label and inspected the replacement before publication.
- Independent record review found that host/rusage files sit outside `out/`
  in the archive; corrected the file count and whole-command CPU scope.
  The verifier now parses the exact bytes it hashed, avoiding a second open
  that could read a concurrently replaced archive.

The shared daily session log is reserved by another task; this scoped handoff
retains the required project record without editing that owner's files.
