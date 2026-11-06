# N83 live S3 factor-base custody, 2026-10-25

This is a current-object custody audit for the S3 prefix
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/`.
The audit listed the prefix, made fresh `GetObject` calls for every recognized
`.jsonl.gz` archive copy, streamed and hashed each compressed object, and saved
the listing and per-object receipts in [`evidence-01`](evidence-01/). A second
independent GET round for the 77 content-addressed `/objects/` keys gave the
same BLAKE3, SHA-256, and byte length for each key. The script's saved-receipt
check and four positive/tamper tests passed.

| Custody item | Observed result |
| --- | ---: |
| Objects in complete prefix listing | 133 |
| Downloaded archive copies | 80 |
| Unique compressed BLAKE3 digests | 77 |
| Primary `a0` content-addressed objects | 50 |
| Diagnostic `a1` content-addressed objects, different curve arm | 27 |
| Cached Linux XOR panel input copies of `a0` objects | 3 |
| Content-addressed objects downloaded again with matching bytes | 77 |
| Compressed bytes in 77 content-addressed objects | 310,367,458 |
| Archive copies with S3 SHA-256 checksum | 20, including 3 cached inputs |
| Archive copies with S3 BLAKE3 user metadata | 58 |
| Archive copies with neither extra S3 checksum nor BLAKE3 metadata | 2 |
| Archive copies with SSE-AES256 | 80 |

All 80 downloaded copies matched the BLAKE3 digest embedded in their key,
their listed and returned ContentLength, any S3 checksum or BLAKE3 metadata
that was present, and the expected AES256 encryption header. The three cached
copies were byte-identical to their content-addressed `a0` objects. The two
objects without extra S3 checksum/metadata still passed independent downloaded
content hashing against their content-addressed names. `a1` counts are
separate because that curve arm is not interchangeable with primary `a0`.

To replay custody against **current** S3 state with AWS CLI credentials that
can list and read the prefix, install the pinned `requirements.txt`, then run:

```sh
python3 -m pip install --target /private/tmp/n83-s3-audit-pydeps -r research/koblitz_n83_factor_base_sweep_20261008/verification/live-s3-full-custody-20261025/requirements.txt
PYTHONPATH=/private/tmp/n83-s3-audit-pydeps python3 research/koblitz_n83_factor_base_sweep_20261008/verification/live-s3-full-custody-20261025/audit_s3.py --online --out /private/tmp/n83-s3-audit-current
PYTHONPATH=/private/tmp/n83-s3-audit-pydeps python3 research/koblitz_n83_factor_base_sweep_20261008/verification/live-s3-full-custody-20261025/audit_s3.py --check --out /private/tmp/n83-s3-audit-current
```

The checked-in first and second listing/receipt rounds bind the 2026-10-25
observation and support offline receipt comparison. `--check` on those files
verifies saved internal consistency; it does **not** fetch current S3 content.
`--online` is the fresh byte replay and fails on an unknown archive name,
missing object, changed content address, size/checksum mismatch, or missing
expected encryption. The audit proves custody of compressed archive bytes. It
does not rederive every base point, verify every point's curve/subgroup
membership, establish relation yield, or select a minimum total-runtime base.
The latter remains `selected_best_total_runtime: null` pending the bounded
solver-setting panel and complete one-target online accounting.
