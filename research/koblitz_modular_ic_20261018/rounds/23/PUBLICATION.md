# Round23 durable publication

The repository artifact store is s3://setec-astronomy-artifacts/store.
Both explicit files were uploaded with SHA256 checksums through the standard
artifact tool;the local .artifacts.json contains content hashes and byte counts.
Fresh S3 GetObject downloads were byte-compared with both local files and their
SHA256 values rechecked. No write was made to the reports/ workflow-only prefix.

- runs.tar.xz:38,236 bytes;
  SHA25695955643a8139649f453fe204adb391275b15753b28377c877498c8a787fba2c.
- report.pdf:54,610 bytes;
  SHA2563744c38900e12fdbc88c7f637d880dccc14510bfc68bff21a06af77e9dc43642.

Hydrate with:

```
python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/23
cd research/koblitz_modular_ic_20261018/rounds/23
sha256sum -c runs.tar.xz.sha256
tar -xJf runs.tar.xz
sha256sum -c runs.sha256
```

The55-member scientific archive is write-once. Later software-validation and
publication receipts are separate;they are not appended to that sealed archive.
The source frame is5cf618dbff2c01b9675884a685fac477177ba716. source.sha256 also
binds the two immutable R22 raw input files,which hydrate from their own archive.
The generic experiment.export/v1 checker is absent at this revision;export.json
is explicitly the inherited exact-computation schema,not a false validator pass.

Both earlier primary failures and the lost ignored first-host raw logs are
recorded in FAILURES.md. Their absence is not concealed by the replacement
replay receipts. No finished G32/G33 or capped global normalization is rerun.

## Separate software-validation package

validation.tar.xz:86,448 bytes;
SHA25618cbc9d4ba2fd543ae8dad222ab0253c59becda544db37557c19f71da6879365.
It has29 additional raw files;science and validation together retain84 files.
validation.json reports two failed full runs and their passing targeted replays;
it explicitly does not claim a single clean full-suite pass. It does not modify
the sealed mathematical export or its55-member archive.

For native regeneration on a new checkout,also hydrate and extract Round22's
own archive first;the exact input circuit and saved closure live there:

```
python3 tools/artifacts/artifacts.py pull research/koblitz_modular_ic_20261018/rounds/22 research/koblitz_modular_ic_20261018/rounds/23
(cd research/koblitz_modular_ic_20261018/rounds/22 && sha256sum -c runs.tar.xz.sha256 && tar -xJf runs.tar.xz && sha256sum -c runs.sha256)
```

REPLAY_NATIVE.sh requires the configured source-checkpoint transport and native
Singular4.4.1 (override SINGULAR_BIN for another installation path). Each call
uses a fresh attempt name and retains the inherited4GiB/1200s limits.
