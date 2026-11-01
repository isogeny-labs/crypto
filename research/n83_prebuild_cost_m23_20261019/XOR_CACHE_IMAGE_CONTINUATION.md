# Frozen continuation after missing-image launch failure

The first `xor-cache-v1` card started at 2026-10-20 01:08:07 UTC and
stopped with `FAILED_OR_INTERRUPTED exit=1`. The first wrapper wrote its
start time and binary hash, then Docker rejected the absent pinned image.
There is no worker/solver receipt, container execution or natural attempt
from this launch. The partial directory is retained unchanged.

The exact image digest from the host card has now been restored:
`sha256:8ed3c0a996841537f75607e7d1de2114d8150391f75792e8da9268738547e73f`.
Docker resolved it to amd64 image id
`sha256:24b8580b081b53113046267a47ad12858ad97752c9735e1125fcdb7a62d1d2c9`.
The pull log is retained with the launch-failure records. This repairs an
environment prerequisite; it changes no solver, point set, query or limit.

This continuation is committed before retry. Run
`prepare-xor-cache-image-restored.sh` once to stage
`/home/ec2-user/n83-m23-20261019/xor-cache-v1-image-restored` from the exact
previously pinned binaries and scripts, verify all hashes at the new
paths, and inspect the pinned image before launch. Start the same shell
card with that new root. Require all four constructed controls and their
separate replays, then all sixteen public attempts in the original order.
No additional retry is implicit. The [original protocol](XOR_CACHE_PROTOCOL.md)
and [binary/host card](XOR_CACHE_HOST_CARD.md) remain in force.
