# Exceptional transition controls

Read REPORT.md and report.pdf; flow.svg is editable. transitions.csv provides one row per invalid trace; summary.json retains the complete before/after records. fixtures/ contains five explicitly constructed controls with provenance sidecars.

Run `bash research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-exception-controls/check.sh` from the repository root. The checker reads existing native streams; it does not generate a new cohort. Restore content-addressed files with `python3 tools/artifacts/artifacts.py pull research/pencil_lifecycle_20261030/integration/followup-continuation-v2/signed-exception-controls` and the adjacent signed-replay packet before replay.

check-eVZy7TsO is the preserved expected pre-fix failure. check-myoPE6k9 is the passing corrected-source check. Review source bindings in each directory. publication-manifest.json binds this packet; remote-readback.json records fresh downloads of archived raw logs.
