# E12a gate-file recovery, 2026-10-28

Seven already-written E12a gate files are preserved here byte-for-byte and in
canonical S3. They contain **24 gate-result rows and four input rows**, totalling
**7,647 bytes**. This is artifact recovery, not a new experiment, a gate replay,
or independent scientific validation. It adds **zero target draw records** and
changes no target-degree conclusion.

## What was missing and what was recovered

The frozen parallel wrapper runs `e7/validate.sh` before the E12a target cell.
That validator writes six result JSONLs and one concatenated input JSONL under
`/work/fall-degree-source/research/fall_degree_bounds_20261008/e7/validate/`.
The periodic uploader archives `/work/out` plus available host/resource metadata.
Consequently the E12a archives retained `gate1.log` but omitted those seven raw
files outside `/work/out`. The earlier persistence report's phrase "original
gates ... saved" must be read with this limitation; this recovery closes that
specific raw-file gap without rewriting the historical report.

| Preserved file | Role | JSON objects | Bytes |
|---|---|---:|---:|
| `A_m3_13-5.jsonl` | Gate results | 4 | 1,140 |
| `inputs_m4_9-3.jsonl` | Gate inputs | 4 | 888 |
| `m3_5-2.jsonl` | Gate results | 4 | 1,065 |
| `m3_7-3.jsonl` | Gate results | 4 | 1,125 |
| `x_m3_11-4.jsonl` | Gate results | 4 | 1,141 |
| `x_m3_13-4.jsonl` | Gate results | 4 | 1,140 |
| `x_m4_9-3.jsonl` | Gate results | 4 | 1,148 |

All seven files are in [files/](files/), with the source-host hash list retained
as [source-files.sha256](files/source-files.sha256). The recovery checked the
existing job ID and original deadline before copying regular, bounded files.
The capture checked their hashes again after archiving. Both SSM commands
returned `Status=Success`, `ResponseCode=0`:

- Inventory: `838abbae-fe26-489d-b63e-606d6ac5803f`.
- Capture: `97767f60-934f-4d6e-bc56-1afbe883d888`.

The exact requests, submission responses, completion receipts and shell commands
are retained beside this document. They bind job
`20261028-021829-fd-e12a-r6a-20261028-r3-0c1d7f`, instance
`i-0f1e979a30bc4f52b`, frozen source commit
`371035216942660c4615dfbb88d85a14afaf5c16`, and original deadline
`1793326711` (`2026-10-30T02:18:31Z`). `instance.json` is a historical observation,
not a claim of the instance's present state. No new solver or gate was run; no
instance, worker policy, job, script or deadline was changed by this recovery.

## Durable copies and local checks

[canonical-durability.json](canonical-durability.json) records the data archive's
fresh S3 GET, SHA-256 and byte comparison at **10:19:50 UTC**. Its immutable URI is:

```text
s3://setec-astronomy-artifacts/research/fall-degree/e12/recovered-a-gates-20261028/8126bf3ce30513a463960b15877667228c1869c36fba99394717d82d0884b5d8/gates.tar.gz
```

The compressed archive is **1,096 bytes**, SHA-256
`8126bf3ce30513a463960b15877667228c1869c36fba99394717d82d0884b5d8`.
[provenance-durability.json](provenance-durability.json) records a separately
preserved **4,685-byte** provenance archive, freshly downloaded and compared at
**10:22:12 UTC**, SHA-256
`0d6823fe2dca0aee512c42b6cd76d2b6a753e8786921a972fe3b396e4672ea5a`.
Its full immutable URI is in that receipt. The corresponding PUT/GET responses
are also retained. No cloud request was repeated while packaging this evidence.

Local `sha256sum`, `cmp`, `tar` and `jq` checks establish source-file hashes,
archive/readback equality, the exact seven archive members, retained-file byte
equality, JSON parsing, and the row/byte counts above. They do **not** establish
that any recorded solution count, closure rank or degree is mathematically
correct. [manifest.json](manifest.json) lists every copied artifact with its
hash and size; [checks.json](checks.json) records the preservation checks.
From this directory, the retained bytes can be checked without a solver:

```sh
sha256sum -c evidence.sha256
(cd files && sha256sum -c source-files.sha256)
for path in files/*.jsonl; do
  jq -e -s 'length == 4 and all(.[]; type == "object")' "$path" >/dev/null
done
```

## Boundaries and remaining work

The request to save these existing raw files to S3 is verified for this bundle.
It does not establish complete capture of every intermediate computation.
E12a target completion and its degree remain separate from these earlier gate
files. Formal independent experiment review, solution recount, closure replay
and Lean validation were **not performed**; no official hypothesis decision,
timing comparison, or new budget allocation is made. The total campaign cap
remains USD 200; this preservation report makes no verified-billing claim.

The separate [E12c report](../interrupted-c-20261028/README.md) retains one
uncertified window record. Its `basis_bytes=30212692424` describes in-memory
closure storage, not an excluded checkpoint file. These recovered E12a files
neither contain that basis nor restore E12c's interrupted solver state. File
snapshots and solver-memory checkpoints remain distinct.

Sources checked for this preservation work: the frozen
[parallel wrapper](https://github.com/isogeny-labs/crypto/blob/371035216942660c4615dfbb88d85a14afaf5c16/research/fall_degree_bounds_20261008/e12/run_par.sh),
[gate validator](https://github.com/isogeny-labs/crypto/blob/371035216942660c4615dfbb88d85a14afaf5c16/research/fall_degree_bounds_20261008/e7/validate.sh),
[persistence contract](../continuation-20261028/S3_PERSISTENCE.md) and uploader,
and [AWS object-integrity documentation](https://docs.aws.amazon.com/AmazonS3/latest/userguide/checking-object-integrity.html).
These sources explain the omitted path and byte-integrity checks; they supply
no independent validation of the recorded mathematics.
