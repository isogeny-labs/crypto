# Inputs freeze (amendment A1): pending

Stage 0 has **not** run. Nothing below exists yet; this file is the place the
freeze is filed, before any measured run (PROTOCOL.md §3).

When `run/00-build.sh`, `run/01-classes.sh 24`, `run/01-classes.sh 28` and
`run/02-specs.sh` have run at the frozen commit, record here, commit and push:

- the commit and `inputs/tools.sha256` (volcano-rho, ecbench, repsel);
- `inputs/classes/t24.selection.log` and `t28.selection.log` (accepted and rejected seeds, with reasons);
- `sha256sum inputs/MANIFEST.sha256 inputs/jobs.tsv`;
- the Iso Lab publish receipt `inputs/image.json` (image digest, source revision, ecbench SHA-256);
- the masking nonce, drawn now with `od -An -N8 -tx8 /dev/urandom`, as the line `mask nonce: <16 hex>`;
- a statement that no IC cost of any study curve has been seen.

Amend `protocol.json` with an `amendments` entry `A1` (`filed_before_affected_data: true`) naming this file and those digests.
