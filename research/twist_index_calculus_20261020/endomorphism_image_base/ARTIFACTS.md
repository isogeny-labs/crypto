# Raw evidence and reproduction

The study retains every launched job, including failed builds, deadline
exits, Spot interruptions, quota refusals and contended intervals. The
original job objects remain in the repository's configured ephemeral S3
namespace. No generator, old freeze followup or EKS job was repeated as a
side effect of migration recovery.

## Content-addressed archive

The final `runs.tar.xz` is **28,241,048 bytes**, SHA-256
`1e0a2139c9a32571a9476e4b9579f73da91e93e4a8cc7c2143d2bf4536c36038`.
Fresh extraction, byte-identical canonical repacking, and a native replay
of all four committed CSVs passed; see
[the replay certificate](results/replay-certificate.json).

**Repository store registration is pending.** `ecbench store push` received
S3 `403 Forbidden` under the migrated worker identity. No store index entry
was created. The intended name is
`store:evidence/twist-index-calculus/20261023-image-base-runs.tar.xz`.
The failed attempt and the successful immutable backup upload are recorded
in `results/publication/`. Existing study jobs retain their original raw
objects. The verified canonical archive is also backed up in the existing
research artifact bucket; its S3 checksum and byte count match the local
archive. Fetch, verify and extract that backup from this directory:

```sh
aws s3 cp --region us-east-1 s3://ephemeral-runs-104186811093-us-east-1/jobs/twist-image-base-evidence/1e0a2139c9a32571a9476e4b9579f73da91e93e4a8cc7c2143d2bf4536c36038/runs.tar.xz runs.tar.xz
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
```

`runs/` is ignored. It contains the raw reconciliation, native analysis,
negative controls, baseline parity, resume-control records, every original
job tree under `runs/jobs/<id>/`, and frozen source bundles under
`runs/source-bundles/`. Recursively copied `previous-records/` trees and
redundant assembled cold trees are omitted because their originating job
trees and assembly provenance are retained. Repeated unpacked source trees
are represented by their exact original source archives, decoded from the
known source-bundle keys without executing an environment file. Source,
command, build and test records are retained; runtime environment files and
compiled `target/` caches are excluded. `pack-evidence.sh` stages on local
scratch and packs the canonical tree with the command below.
Every numerical source used by the report is also copied loose into
`results/`, so report links resolve without extracting an archive.

The manifest `runs/reconciliation/job-registry.json` enumerates all launched
study jobs. `results/job-observations/` and `reports/INDEX.md` include failed
jobs. Paths in original receipts are immutable historical paths. The
replay script changes only a newly generated location manifest to point to
the extracted tree; it does not rewrite original receipts or output files.

Archive packing uses the study's 2026-10-23 date:

```sh
tar --sort=name --mtime='2026-10-23 00:00:00Z' --owner=0 --group=0 \
  --numeric-owner --mode='u+rwX,go+rX,go-w' --format=gnu -cf - runs \
  | xz -6 -T1 > runs.tar.xz
sha256sum runs.tar.xz > runs.tar.xz.sha256
```

Packing the extracted tree with this command produces the same bytes.
The canonical archive exceeds the Git blob limit and remains outside Git.
Its repository store registration requires the existing scoped store
credential to be restored. Raw text is not compressed to evade that limit.

## Frozen inputs and revisions

- Frozen query tape: `../quartic_twist_mixed_v1.ndjson`, 3,009,728 bytes,
  SHA-256 `d50c500961344a1d5813a02dbba9019aef3309cb74180e99715ca693f5b8094c`.
- Every timed cold job uses the identical source bundle
  `runs/source-bundles/cold-source-v1.tgz`, SHA-256
  `61948c5aaae1e1451dee1760e619ffc4d585c11808c90a8cff60dce2bd5786c1`.
  Resume, analysis and reporting code were extended without changing that
  timed program. The resume supplement was committed before launching its
  replacement rounds.
- Original branch baseline: `c7cecbd07f437b600b5893be61ad0eedf38d2368`.
  Each job's `job.json`, `cmd.sh`, source tree and native source hashes
  preserve the actual snapshot, including uncommitted bundled code.
- Correctness-gate source bundle:
  `runs/source-bundles/root-gate-source-v2.tgz`, SHA-256
  `ad29a56205a3f80f34e2c05dd4da72101f84228017fac81498ed1434afe9dbf4`.
  It is a separate complete-library test snapshot, not a timed cold arm.
- msolve: official version 0.11.0, commit
  `cc044551f8e202d00691990fd3fd0b1226a88ea2`. Dockerfile, timer-precision
  patch, build checks, executable/library digests and full version logs are
  retained. No proprietary tool is bundled.

The actual input and clock boundary is specified in [BOUNDARY.md](BOUNDARY.md).
In particular, encoded public points are present for alignment; the program
recomputes and charges their lifts. It is source-audited, not process-isolated
blinding. Every record uses public scalar 271.

## Recompute without rerunning expensive experiments

With Rust, jq and the archive extracted, run:

```sh
bash replay-analysis.sh /absolute/path/to/new-replay-directory
```

This independently reconstructs signed relations and matrix rows from the
frozen references, checks the candidate-prefix audits and recovered points,
selects only chronological complete resource-valid eleven-execution rounds,
checks all cold deterministic traces and exact phase sums, and compares all
four committed CSVs byte for byte. The native summary retains every
per-target row, original A/A spread and five same-origin paired ratios.
Absolute evidence-path strings in JSON are historical provenance, not
numerical data to be silently rewritten.

The independent audit source is `native/src/image.rs`; the actual 210
post-run audit commands and data are in job
`20261023-170329-twist-image-audit-29b206`. Its scalar/group BSGS oracle and
matrix elimination are independent of the candidate relation finder.
`run-audit.sh` and the other native run scripts reproduce the command
structure. Re-execution of a timed experiment is separate from this replay
and requires the frozen protocol and its isolation gate.

For the external GB check, use `--image-external-report INPUTS JOB_OUT OUT`
with the original exported-input directory under `runs/external-inputs/`
and job `20261023-192006-twist-image-msolve-6e5855/out`. The exact directory
is recorded in the reconciliation manifest. This checks all 3,024 raw
reduced bases by equality, input reduction and S-pairs before summarizing
the eight fixed passes. It does not relaunch msolve or infer a complete
external DLP cost.

## Documents and software checks

`render_reports.py` only lays out the native values with matplotlib and
Typst. Run it with a scratch virtual environment containing `matplotlib`,
`typst` and `pymupdf`; inspect every PDF page before committing regenerated
reports. PDF regeneration embeds each matching 150 dpi PNG because a Typst
SVG import misplaced the diagram; source SVGs remain available separately.
Post-seal publication and document-inspection receipts stay loose in
`results/`; the earlier boundary documents and failed SVG import are
retained in `results/document-history/`. The study's figures and PDF copies in `benchmarks/` are diagnostics
with the same limits. They are not promoted into the gated benchmark index.

[VALIDATION_STATUS.md](VALIDATION_STATUS.md) records test outcomes and
remaining price/rho gates. No generated `VALIDATION.json` is asserted by
the existence of this archive.
The [proof map](proof/PROOF_MAP.md) gives the Lean declaration's exact
hypotheses and the concrete group, row and probability definitions still
missing from a full formalization of the mathematical discussion.
