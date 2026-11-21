# Source session

T-2164, fence 1, 2026-11-11. Preserved the v5 source and terminal materialize
failure. Restored the exact eight overlay files declared by `overlays.json`
from v3, assigned fresh v6 workspace/unit identities, and added the overlay
subtree to the semantic seal. No compiler, test, resolver, or payload was run.
The first static closure check rejected rows lacking optional `pre_origin`; the
corrected checker requires the seven common keys, permits only that optional
eighth key, and then checks every file, size, type, and post-hash.
The final packet-closure check passed all eight overlays, all 50 hash-bound
driver source inputs, and 17 direct packet inputs as regular non-symlink files.
