# Primary N83 AWS-agent resume handoff

This handoff is stacked on the source and evidence in [PR #2551](https://github.com/aburan28/crypto/pull/2551), whose last reviewed commit is
`acfcd8adfad8d45ddf9c5f799f9385120dce6cf4`. Check out this PR's head
and require that commit to be an ancestor before using the instructions below.
The workload is the primary `a=0` Koblitz GF(2^83) curve
`icv1-f2m83-tm6151469093347-debefd74`, with its 81-bit subgroup. The
diagnostic `a=1` subgroup has a different workload and must have separate
results.

## What is ready

The [four-arm source receipt](../external-xor-20261025/receipt.json) and
[provider admission receipt](../linux-xor-panel-20261025/admission.json) bind
the trial-zero, all-affine first-pattern hash/Gray 5+1 and 4+2 XCNFs.
The exact objects, already uploaded and independently fetched with matching
byte hashes, are at:

`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/linux-xor-panel-20261025/inputs/`

| Object | SHA-256 | Bytes |
| --- | --- | ---: |
| `hash_5_1.xcnf` | `2c6dba1feabb28bc6b064268a1f53ebe3b0ecd8aa731d26b31016830ce431022` | 13,156,532 |
| `gray_5_1.xcnf` | `220eaeb04a43ee8c853b637d6255420e2e341c6bb3e5f3588ca1295529b327e9` | 12,396,243 |
| `hash_4_2.xcnf` | `9d8392af80cb17e4da99d616933d43feb718cbe897a4cfffbbe5eedbaccfbe69` | 13,156,822 |
| `gray_4_2.xcnf` | `a39dc89ea7c8c8ea0c6f54cd5900707537cbdc1307b90fc457bc8b50a3ae0bce` | 11,636,244 |

The two K600 pool archives and the K600 base archive used to build these
formulas are under
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/a0/objects/`.
Their compressed BLAKE3 and SHA-256 digests are in the
[export protocol](../external-xor-20261025/PROTOCOL.md). The public manifest
and corpus are `pilot-01/manifest.json` and `pilot-01/probe-corpus.json`.
Exact write-once copies of those three compressed archives and the four
`export.json` files are now under this panel's `inputs/` prefix. The
[11-object manifest](../linux-xor-complete-inputs-20261025/manifest.json) and
[fresh-download receipt](../linux-xor-complete-inputs-20261025/receipt.json)
bind every object to its source SHA-256 and, for archives, compressed BLAKE3.
The [independent replay](../linux-xor-complete-inputs-20261025/audit.py)
checks XCNF, archive, export-metadata, source-receipt and S3 receipt identity.
The local source-pinned [wide-XOR run](../local-yield-frontier-20261025/RESULT.md)
ended at its 1,350-second process watchdog without a terminal solver result.
It is a censored L0 access observation; it cannot rank these bases.

The intended EC2 profile `n83-xor-panel-20261025` permits `GetObject` on
this panel's `inputs/*`, `PutObject` on its `outputs/*`, and SSM core access.
The mirrored archives and export metadata are therefore readable without
changing the role. It **does not** permit reading back `outputs/*`; an
independent output auditor needs separately authorized read access, or a
reviewed policy change, before any uploaded result can be accepted. The
attempted `us-west-2` `c7i.2xlarge` launch
returned the provider's `Blocked` account-admission error after a successful
dry run; it allocated no instance. Admission must be established on an
eligible account before allocating a new run. The old dry-run response is
not evidence of present admission.

## First actions on an admitted host

1. Pin the checked-out Git commit, Linux image, CPU and memory limits,
   isolation level, worker and solver binary versions and SHA-256 digests,
   and an all-stage cost envelope. Keep Linux measurements separate from the
   Darwin receipts. Freeze arm order, A/A controls, flags, caps, and stop
   rules in a new committed protocol **before** timing.
2. Fetch all 11 inputs to a new directory with the committed command below.
   It checks the S3 checksum headers, downloads, compressed hashes and
   source metadata. It does not claim that a new EC2 launch succeeded. The
   Python dependency is pinned in the bundle's `requirements.txt`:

   ```sh
   set -euo pipefail
   BUNDLE=research/koblitz_n83_factor_base_sweep_20261008/verification/linux-xor-complete-inputs-20261025
   python3 -m venv /tmp/n83-input-venv
   /tmp/n83-input-venv/bin/python -m pip install -r "$BUNDLE/requirements.txt"
   INPUT_DIR=/tmp/n83-xor-inputs
   /tmp/n83-input-venv/bin/python "$BUNDLE/fetch.py" --out "$INPUT_DIR"
   FORMULAS=/tmp/n83-xor-formulas
   mkdir "$FORMULAS"
   for arm in hash_5_1 gray_5_1 hash_4_2 gray_4_2; do
     mkdir "$FORMULAS/$arm"
     ln -s "$INPUT_DIR/$arm.xcnf" "$FORMULAS/$arm/instance.xcnf"
     ln -s "$INPUT_DIR/exports/$arm/export.json" "$FORMULAS/$arm/export.json"
   done
   ```

   After building and pinning the Linux worker binary, run
   `verifier_gate.py --inputs-dir "$INPUT_DIR" --worker "$WORKER"
   --worker-sha256 "$EXPECTED_WORKER_SHA256"` with the same Python
   environment. The gate rebuilds the `hash_4_2` XCNF from the fetched
   archives and metadata, then must reject a deliberately incomplete
   synthetic SAT assignment at the model parser. The retained
   [Darwin control](../linux-xor-complete-inputs-20261025/verifier-gate.json)
   passed with the source-pinned worker; the Linux binary needs its own gate.

3. Run the separately frozen, bounded Linux comparison on one isolated host.
   Keep per-arm raw stdout/stderr, exit status, solver terminal status,
   exact final conflict count when present, process and CPU time, peak
   memory, source and binary hashes, and censorship reason. The
   [Linux panel protocol](../linux-xor-panel-20261025/PROTOCOL.md) describes
   matched-arm and A/A gates. The local 25,000-column matrix admission
   result is a separate solver-setting axis, because it changes which XOR
   matrices are used; do not pool it with default settings as an A/A arm.
4. Upload generation-specific output keys with `If-None-Match: *`, retain
   SHA-256 receipts, then have a separate reader fetch and independently
   replay the exact bytes. Any complete SAT assignment must pass the
   worker's `verify-cross-xcnf` check against the byte-identical mirrored
   base/pool archives in `INPUT_DIR/archives/`, the checked-out public
   manifest and corpus, and `FORMULAS/<arm>/`. Its original CNF/XOR, field,
   orbit-domain, and curve-group checks are required. A rejected or
   incomplete model is not useful relation yield.
5. Continue only from verified natural partials to graph closure, useful
   rank, linear algebra, unseen-target descent, and a matched same-target
   rho control. Charge construction, S3 transfer, failed search, verification,
   and every later stage to cold one-target total runtime. The
   [v4 sweep grid](../../PAIRED_FRONTIER_V4.md) is address coverage, not
   measured coverage; unsupported and unrun combinations retain explicit
   dispositions.

The current `selected_best_total_runtime` is `null`. A timeout, conflict
cap, provider block, SAT model without group replay, or solver-stage timing
alone cannot turn it into a factor-base selection.
