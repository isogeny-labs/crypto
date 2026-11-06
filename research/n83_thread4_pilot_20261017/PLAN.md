# K2816 target worker-count pilot on the current N83 source

This exploratory one-target run tests four cooperative cancellation workers on
the already exposed N83 point used by the current-source K2816 control. The
four-worker configuration retains the exact 467,456-point base, 2,816 folded
columns, root11 dynamic index, source SHA-256
`8109fb33374ce2aa66c6a5a0b6e144d3b0143168ac756f9e0bd70af60a911829`,
and all target-solving flags except `KIC_TARGET_THREADS=4`.
The pilot reuses the earlier archived release executable, SHA-256
`c93716a49d7db070666af021d1add78df3529a23d674e53e50ea816f8b5ca04a`,
whose prior source receipt binds it to that exact IC source.

The new [candidate manifest](generate_candidate.py) derives an IC1 identity
from the previously verified two-worker manifest by changing the thread count
and target worker policy:
`IC1N83Ckb1a1fb467456PDP4rootRCguidedLAgaussTDdirectISO0h62c132d4bafe`.
The producer starts a new process with empty caches
and a 900-second cap. The verifier replays the factor-base enumeration,
four-point target relation, subgroup scalar, exclusive online phases, exact
policy flags, source and binary digests, and archived base hash.

The point was exposed before this pilot and the local macOS host has no Tier 0
or Tier 1 noise gate. Its timing is an algorithmic diagnostic. A favorable
result will motivate a sealed, fresh same-point Tier 1 comparison with the
two-worker policy; it will not select a factor base by itself.

Run from the repository root:

```sh
python3 -m research.n83_thread4_pilot_20261017.pilot \
  --binary /Volumes/SSD990/crypto/artifacts/n83_highk_cancel_pilot_20261017_raw/executed_bin \
  --output /Volumes/SSD990/crypto/artifacts/n83_thread4_pilot_20261017_raw
python3 -m research.n83_thread4_pilot_20261017.verify \
  /Volumes/SSD990/crypto/artifacts/n83_thread4_pilot_20261017_raw
```
