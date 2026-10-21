# N83 six-arm finalist result: selected cold factor base

The frozen 32-point Tier 1 comparison selects the factor base with 183,762
usable points (`IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0he423fcbc5668`)
for fresh-process cold wall time among its six exact candidates. Its mean was
94.211 s, versus 165.713–431.514 s for the five larger bases. Every one of
the 192 independently replayed, same-point IC/rho pairs had a positive online
rho/IC ratio across four distinct GitHub Actions runners. The smallest ratio
was 3.01.

The primary one-target online selector remains `BOUNDED_UNKNOWN` with
`selected_candidate_id: null`. The 572,700-point base has the smallest
descriptive geometric mean online time, 5.680 ms, but its paired tests against
the 509,952- and 581,000-point bases have two-sided sign-flip p-values of
0.155 and 0.437. The prespecified familywise threshold is 0.05/15.

## Reproduction and provenance

- Provider [run 39160284543](https://github.com/aburan28/crypto/actions/runs/39160284543), attempt 1, source commit `f152eb7d5e5de660fb16902d2abfe7cbd3ff331a`.
- Four separate eight-point provider jobs passed the Tier 1 topology, resource,
  calibration-drift, and A/A gates. All four full archives passed independent
  local replay. [validated_rows.json](validated_rows.json) contains the
  per-point IC, rho, phase, resource, and replay records;
  [selection.json](selection.json) is the exact frozen selector output.
- [provider_jobs.json](provider_jobs.json) binds the distinct provider runners.
  [provider_zip_sha256.json](provider_zip_sha256.json) records the original ZIP
  digests and sizes. Its local `path` fields identify the collection machine;
  the content identity is each ZIP SHA-256.
- `pack_archive.py ARTIFACT_ROOT` checks every provider ZIP member against its
  extracted byte hash and writes a deterministic deduplicated raw archive.
  `verify_archive.py OUTPUT_DIR` independently restores and hashes every
  archive member, including all repeated factor-base dumps and replay logs.

The 16-point extended holdout uses this independently replayed result as its
frozen upstream input. Since the online selection is null, that protocol uses
its prespecified K2304 control; it includes the selected cold base and three
older frontier bases. Its dispatch is commit `ac892cfced2b3c998c8872eb169dce8cc4424f6`.
