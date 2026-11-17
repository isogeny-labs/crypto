# Primary N83 Linux XOR natural-query panel: bounded result

The frozen [protocol](protocol.json) ran on one GitHub-hosted Ubuntu 24.04 ARM
VM in [run 40181063181, attempt 1](https://github.com/aburan28/crypto/actions/runs/40181063181)
at protocol commit `1b34b3ae6cd634356ab3c5e858ceba45a033ed22`. The exact provider
artifact is ID `12594952985`, ZIP SHA-256
`d9702b1d34c19775fa76f45a8c203bef7cd50a3eebf2b3f24fb9d6c21ebc79de`
(149,279 bytes; provider expiry 2027-01-23T09:32:24Z). Its independent
[replay](run-40181063181-attempt-1/replay.json) passed, including provider
identity, complete eleven-object S3 input replay, positive solver control,
exact binary/input admission, all six raw streams and process receipts. A
forged ZIP with an appended SAT line failed the pinned digest check
([negative check](run-40181063181-attempt-1/tamper-check.json)). The provider
ZIP remains in GitHub Actions. The [retained manifest](run-40181063181-attempt-1/retained-manifest.json)
maps 30 selected files to their provider ZIP members and SHA-256 hashes;
raw solver streams are losslessly gzip-compressed in this repository.

All six arms returned `INDETERMINATE` at the 300-second solver cap, below the
one-million-conflict cap. None reported a SAT model, and there was no watchdog
or observed OOM kill. Values below are the provider's per-process L0 wall,
not an end-to-end IC cost:

| Frozen arm | Conflicts | L0 wall (s) | User CPU (s) | Peak RSS (MiB) |
| --- | ---: | ---: | ---: | ---: |
| hash51_A1 | 244,163 | 305.473 | 304.108 | 1,415.06 |
| hash51_A2 | 244,163 | 303.692 | 302.231 | 1,415.05 |
| gray51 | 180,993 | 305.615 | 304.482 | 1,400.93 |
| hash42 | 355,009 | 304.711 | 302.997 | 1,566.70 |
| gray42 | 299,459 | 301.539 | 300.091 | 1,385.88 |
| hash51_A3 | 221,955 | 303.796 | 302.438 | 1,403.46 |

The six solver processes sum to **1,824.826 seconds** of L0 wall. The provider
job ran from 09:32:40 to 10:07:35 UTC (2,095 seconds including checkout,
setup, fresh S3 fetch/replay, and upload). The VM exposed two CPUs/cpuset
`0-1`; each solver process used one thread. Linux cgroup CPU and memory maxima
were unavailable in the retained host observation. CryptoMiniSat was pinned
to `5.11.15+dfsg1-1.1build1`; the full flag list and memory/watchdog caps are
in the protocol. Process receipts preserve exact CPU, RSS, exit 15, conflict,
and stream hashes.

Adjacent hash 5+1 A/A executions reached exactly 244,163 conflicts. The later
hash 5+1 drift control reached 221,955 conflicts, 9.09% fewer at a similar
cap. This is a single-host repeat, not a variance estimate. The hash and Gray
exports also used different residual point domains (99,600 versus 16,102),
so their observed conflict counts cannot isolate enumeration order. The
5+1 and 4+2 exports differ in splitting as well. These censored observations
cannot rank factor bases by relation yield or total runtime.

The four formulas cover one fixed public first-pattern query on the exact
primary `a=0` Koblitz GF(2^83) curve. The experiment did not vary trial
coefficients or identity masks, produce a verified natural relation, measure
large-prime closure and rank, perform unseen-target descent, or charge a
complete cold one-target run against matched rho. The finite search grid is
therefore still largely unmeasured, and `selected_best_total_runtime` is
`null`. The next experimental gate is to generalize the external exporter to
new trial coefficients and identity patterns, then preregister a bounded
natural-relation yield/rank panel with full build/import accounting. The
eleven qualified factor-base inputs remain at
`s3://crypto-autoresearcher/factor-bases/icv1/etc/koblitz_n83_factor_base_sweep_20261008/linux-xor-panel-20261025/inputs/`.

To reproduce the independent provider replay, download artifact ID
`12594952985` without transforming its ZIP bytes, then run:

```sh
python3 audit_artifact.py \
  --artifact-zip /path/to/n83-linux-xor-natural-40181063181-1.zip \
  --expected-zip-sha256 d9702b1d34c19775fa76f45a8c203bef7cd50a3eebf2b3f24fb9d6c21ebc79de \
  --metadata run-40181063181-attempt-1/artifact_metadata.json \
  --jobs run-40181063181-attempt-1/provider_jobs.json
```
