# Archive custody repair

The original review archive was committed directly despite the64KiB data-archive rule. This follow-up moves its HEAD representation to the official artifact-store pointer without rewriting history or changing proof records. The local hydrated archive remains available. A fresh remote download matched all bytes and every archive member hash and metadata.

Hydrate from the repository root with `python3 tools/artifacts/artifacts.py pull research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/solve4-native-narrow-inverse-review`, then check SHA256 2a607f2d4bc55bf2d90e5d39451f534ab18f3d7b48c10a5becf71b4629ac1af2 and extract `tar -xzf research/pencil_lifecycle_20261030/integration/followup-continuation-v2/packets/solve4-native-narrow-inverse-review/replay-evidence.tar.gz -C <fresh-directory>`. Exact URI, bytes, download command and member inventory are recorded in this directory. No Lean replay occurred.
