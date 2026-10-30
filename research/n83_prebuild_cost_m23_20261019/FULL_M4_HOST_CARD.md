# Full-base common-public panel: pre-execution host card

This card is committed before executing `full-m4-v1`, under the previously
frozen [full-base protocol](FULL_M4_PROTOCOL.md). The authorized existing
SSH Spot host is reused; no instance or other job is changed.

## Prerequisites satisfied

The preceding cache card ended at **02:00:10 UTC on 2026-10-20**.
All sixteen natural attempts returned `UNKNOWN_search_cap` at exactly
10,000 conflicts; all four matched cells passed exact search-work and
visit-conservation checks. Its native audit passed 294 files. Audit
SHA-256: `da7f5df53baaeba77091cb7aea843375b019adaf65533a20a416d634ae71f100`.
This establishes the bounded implementation gate; natural yield/rank
remain censored and timing observations remain L0.

The next native build/test ran **02:01:21–02:01:35 UTC**, after the prior
timed card ended. `full-m4-linux-build/` retains clean source status,
source bundle verification, both successful exits, compiler version,
tests and build logs. Release example tests passed **5 + 2 + 2 + 2**.
The library/dependency tree remains identical to the candidate whose
complete Linux library suite passed **4,403 / 0 / 118 ignored**.

## Exact executable card

- Worker/replayer source: `d9ef42301cc3d22f437e1f89304e673836afb6a3`.
- Source bundle SHA-256:
  `0adffcc69171d9cff28f026dfdc624cb055c1be4d30519fea3491a54da043150`.
- Worker SHA-256:
  `d4324884fdd52ee05a82e2f053708d320d8255591ec61640b700a8a6684a0486`.
- Replayer SHA-256:
  `1728ef0d3423ba61bab8ec98db3f4d06d256eb05ec0b12b04a6ce812c77f1425`.
- Card SHA-256:
  `6da91e69eb5671e514e627ecdbf6ebd7383c738a86f652989e73c86994c76270`.
- Docker wrapper SHA-256:
  `ae6148f425e7cad875012316bef78f2784e216519e441a13f0a34fbee4f2b1ae`.
- Docker image:
  `amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f`.
- New output: `/home/ec2-user/n83-m23-20261019/full-m4-v1`.

`prepare-full-m4.sh` refuses existing output, verifies both build/test
exits and the exact detached source, copies artifacts, and checks ten
SHA-256 bindings including both base objects/manifests, public corpus and
the preceding audit. `run-full-m4-card.sh` checks the manifest again and
refuses a missing, partial or unmatched cache prerequisite.

Before natural queries, it requires four full-base constructed controls,
four independent general-field replays, and two native off/on comparisons.
Those actual controls have not been replaced by the shell fixture tests.
On success it runs the eighteen natural attempts, with identical public
indices and limits across U/D. Every returned row is separately replayed.

## Launch once

After running the committed preparation script:

```sh
cd /home/ec2-user/n83-m23-20261019/full-m4-v1
nohup sh run-full-m4-card.sh "$PWD" > card.log 2>&1 < /dev/null &
printf '%s\n' "$!" > card.pid
```

CPU 31, one CPU quota, hard 4 GiB / zero swap, no worker network,
read-only root/inputs. No scalar sidecar. Natural work is capped at
eighteen times 1,800 seconds; controls/replays have separate process caps.
All failures and caps remain retained, and interruptions need a separately
frozen continuation. Shared-host observations remain L0. No final
completion time, natural rank, pipeline winner or automatic chat update
is promised by this card.

The PR's pending solver merge conflict does not change these pinned
sources. A later merge resolution requires its own validation and cannot
be substituted into this already frozen measurement.
