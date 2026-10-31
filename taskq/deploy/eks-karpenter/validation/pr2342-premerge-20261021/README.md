# PR 2342 pre-merge integration evidence

The GitHub-built executable passed a real EKS L2 run and independent S3 verification.
This run addresses the pre-merge live integration hold; the complete GitHub `main`
Action, including OIDC publication, remains a separate post-merge requirement.

- Build: [39527210297](https://github.com/aburan28/crypto/actions/runs/39527210297).
- Selected source: `e862ee96a5d1109ef284a8ddf8e21528d2f988af`.
- Reviewed publisher: `abe6b1b52fe976a05b9382655be21b2db2af2034`.
- Observed: 108/108 L2 executions, 90/90 matching native replays, 10 admitted A/A pairs,
  no retries, and all 19 S3 manifest payload hashes verified.
- Host: M8a Spot, eight physical/logical CPUs with no SMT; pod four CPUs and 4 GiB.
- Regional capacity during the run: 92 of 96 Standard Spot vCPUs. CI pools cap at 48.

[Report and diagram](report/report.pdf), [readable report](report/REPORT.md),
[frozen protocol](protocol.json), [verification](verification.json),
[launcher result](client/result.json), [raw artifact archive](artifacts.tar.gz), [payload manifest](artifact-SHA256SUMS).
Exact S3 prefixes, ECR digest, executable SHA-256, and Job/Pod UIDs are in verification.json.
The trusted publisher ran on the authenticated harness using a temporary EKS Docker
daemon. The temporary publisher and identity probe pods were removed, and the task's
EKS administration grant was revoked after capacity repair. Native computation ran
only in the EKS benchmark pod; Python was used for receipt checking and report layout.

The source artifact was downloaded from GitHub, hash-checked, and copied by the
reviewed Dockerfile without execution during publication. The VPC launcher checked
its actual pod image, executable hash, selected spec, L2 grade, replays, and A/A gate.
A second download verified every S3 payload against the manifest and repeated the
identity checks. To recheck the retained payloads locally:

```sh
tar -xzf artifacts.tar.gz
(cd artifacts && sha256sum -c SHA256SUMS)
```

This is integration/isolation evidence on the public 20-bit fixture, with no algorithm
speedup or security claim. Algorithm leaderboards and curve graphs are unchanged.

The archive preserves every original payload byte, including trailing whitespace in the process-status receipt; no frozen artifact was reformatted.
