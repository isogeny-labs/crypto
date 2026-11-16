# Exact N83 V5 ordered-orbit K600 zero-trial preflight

Freeze this protocol and `run-preflight.sh` before execution. This is
an implementation, input and resource admission for **one** declared
V5 cell, ordinal `26960859236`. Its exact route is the primary `a=0`
K600 hash base (600 signed-Frobenius columns, seed `2026100801`),
six summands, zero split bits, signed-Frobenius plus ordered-first-pair
symmetry, SAT CDCL, binary enumeration, zero large primes, one thread
and modular BigUint linear algebra. The earlier [K600 preflight](../../reports/run-20261116-k600-preflight/attempt-6-zero-trial-pass/REPORT.md)
used `orbit-chain-s3` without ordered-first-pair symmetry; it is a
reference for staging, not this route's result.

## Frozen sources, input and hypothesis

- Linux ARM64 exporter source snapshot
  `cedc6dc8ea85bacf8b3b687177cd98cb0ba9ecff`, binary SHA-256
  `87fdd7af02c140467b72225960c2c88be1af443b028d2ac99a8b25b611fa1236`,
  image `sha256:2d67b100bee2048d32ea8feb5170be2b13c61d6c70949fbc21c0ffbbff1a4573`.
- Read-only archived v3 bundle hashes: `manifest.json`
  `d2095084767804af1525665307c9ad1c9eb8dbee2fbdb5adcac9f849e765d119`,
  `replay.json` `60725bf905ed7ccf3ebef24fbee98cda8b32422075a8708e268e60f2e7916843`,
  `storage.json` `8b8452192a6344e8afd98737765e5d944a02efca57f37eae17251f092c2fa98b`.
- Public fixture hashes: `probe-corpus.json`
  `760ae5c2e88615152e2569653a0e0c0cf3b2f84691f2b0731247f9ee3759dcbd`,
  `probe-validation.json`
  `1c6d3a45c8b4654ff208751258fa2e34f0cc60e1c30ba48e9214db62be60472f`.
- Read-only store-index SHA-256
  `298b5c1396b58a1b4025813be092934be6f65e34544856d92f1edadabeabd28b`.
  A fresh GET under `ecc-challenges-new` account `104186811093`
  must return 3,025,798 bytes with SHA-256
  `3e85c9e419fcddd3c9faed3c84470bf96fcad9178e48e1d73629369335841ecf`.

The hypothesis is that the exact Linux V5 adapter accepts these
inputs and returns `PREFLIGHT_ONLY` for zero trials with strategy
`ordered-orbit-chain-s3`, zero relation/SAT work, and no solver or
target result. A failure is retained with its cause. This does not
measure relation yield or select a base.

## Limits, controls and stop

Use `primary-v5-retained-chain-cold` with `MAX_TRIALS=0`, 125,000
maximum variables, 350,000 domain clauses, one model, 10,000
conflicts, a 240-second worker budget and 4,096 MiB memory cap. Run
in the pinned image with one CPU, zero swap, 128 PIDs, no network,
read-only root/source/inputs/index/cache/binary and a fresh output
directory. Set `RAYON_NUM_THREADS=1` and the pinned source and
artifact-store environment. The host GET cap is 90 seconds and the
Docker outer cap is 300 seconds. Stop if either SSD990-2 or the boot
volume falls below 2 GiB available. Preserve the exact commands,
identity, S3 metadata and GET hash, cgroup and disk records, worker
summary, terminal status, Docker inspection, cleanup, and failures.
Do not start a relation trial in this packet.

The pass gate requires both prior exact-binding negative controls,
this run's source/input/resource and `PREFLIGHT_ONLY` checks, and no
OOM, timeout or disk stop. The positive and natural-query arms remain
separate future protocols with native model replay. This preflight's
interval is stage diagnostic only; the eventual cold single-target
cost must also charge construction, archive custody, build, stopped
attempts, relation work, rank, target work and verification.
