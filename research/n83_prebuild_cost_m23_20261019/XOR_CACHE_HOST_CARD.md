# Pre-execution host card: XOR cache v1

This card is committed before running `xor-cache-v1`. The earlier
[protocol](XOR_CACHE_PROTOCOL.md) and source implementation fix the query
and correctness policy. The existing SSH Spot host is reused under the
owner's authorization, with no instance launch or other job changes.

## Fixed artifacts

- Native revision: `1de47a9d4186bff08abc84c224a111183e820106`.
- Source tree: `/var/tmp/n83-xor-cache-source`, detached and clean.
- Worker SHA-256: `a9b8599761c6cce8a169602086c3426af9f9f7a55a8108124daed56059ffb3bc`.
- General-field replayer SHA-256: `9ed4476c8869d0fc124a4f68392fc968071c1b804fa1688796ca2ab062e55861`.
- Card SHA-256: `4477ba28e581f3bccaaf4de1666026b9c67b2d127f5fc5cd2d5676d80106009a`.
- Docker wrapper SHA-256: `f5c24df168febec056033ddb374d7316185c3dd180e83339aadafe00ae5d7439`.
- Output root: `/home/ec2-user/n83-m23-20261019/xor-cache-v1`.
- Image: `amazonlinux@sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f`.

`prepare-xor-cache.sh` checks these binaries/scripts, source files,
manifests, compressed point objects and public corpus. It refuses an
existing run or input manifest. Binaries were built natively, and all
compilation/testing finishes before timed controls/search. The first
build's unused worker is preserved as `worker-first-build`; it was never
used for this card. `source-commit-first.txt` and both build logs preserve
that preparation boundary. The final worker adds an exact SAT-model digest.

## Required validation

The complete candidate Linux library suite passed at `628ec4b8d`:
4,403 passed, zero failed, 118 pre-existing ignored. The `src` tree and
dependencies are identical at the final revision (recorded Git identities
and empty diff). Final-revision example tests passed: worker 3, receipt
audit 2, general-field replayer 2. See [validation](XOR_CACHE_VALIDATION.md)
and retained logs. This card does not waive unrelated PR checks.

## Launch once

Copy the committed preparation script to the runner, run it once, then:

```sh
cd /home/ec2-user/n83-m23-20261019/xor-cache-v1
nohup sh run-xor-cache-card.sh "$PWD" > card.log 2>&1 < /dev/null &
printf '%s\n' "$!" > card.pid
```

The card first requires K2048 and D12 constructed controls under off/on,
with four independent replays. It then runs sixteen fresh public searches:
K2048 target 0, D12 target 0, K2048 target 1, D12 target 1, each in
off/on/on/off order. Each arm has 10,000 conflicts and a 1,790 s internal /
1,800 s outer wall cap, 4 GiB with zero swap, one CPU quota on CPU 31,
no network, and read-only inputs/root. Only the public target file is mounted.

All failures/caps remain retained. Returned relations require independent
replay; exact model/row digests and work counters are compared after
collection. Stop on invalid control/model/lift or missing progress. A
Spot interruption needs a new frozen continuation before retry. The maximum
natural-search allocation is sixteen times 1,800 s; completed conflict
caps can finish earlier. No completion time is promised.

The initial host snapshot records shared workloads. Treat this card as L0
unless stronger host evidence is actually obtained; no wall-time winner
or complete IC improvement is pre-authorized by these checks.
