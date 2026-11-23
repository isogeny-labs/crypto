# S3-free artifact recovery

`index.json` maps unchanged manifest SHA-256 digests to recovery sources.
`tools/artifacts/artifacts.py pull` uses these entries when reading from Git.
Every result passes the usual manifest byte count and SHA-256 check before
installation. No manifest or reproduction lock was changed.

- `cf1ca93d708e020ca89257bd883002849c4b878185545cfa29c1f9ea09f3db9b`:
  current backend leaderboard, 8,451,778 bytes. The compressed bundle was
  recovered from the local content-addressed cache and its uncompressed
  digest and length were checked against `docs/backend-leaderboard/.artifacts.json`.
  Gzip uses `mtime=0`; compressed size is 684,523 bytes.
- `16b0fd0e9a68483556b6d2e714de96e25004dfd5c423398bf74b3171d30ce6b5`:
  archived backend leaderboard, 8,451,778 bytes. Existing historical Git blob
  `b9a57729f87d975c319aa8a8de80fd017dd3a288` has the required bytes; it was
  previously tracked at `docs/backend-leaderboard/leaderboard.json`.

The claim-n13-v2 records need no recovery entry: their original bytes remain
in the current commit's full Git history, blob
`6b369c427087e8485d7569edc4d6daa433ed32b9`, SHA-256
`0b5e0124732f9739d84883dc27baaf662c31bf6bf49e47fec0eaf2fe7813e495`,
1,798,468 bytes.

These are retained historical inputs, not newly generated measurements.

## Weak-curve Lean CI input (2026-11-21)

`research/weak_curve_properties_lean_20261011/root_lib.stdout` is the
346,379-byte original Git blob `d01cc9689c2222005990eae14232afa2a13305c9`
from the parent of migration commit `d2f715a221cc06f373cdc24346560943001782b6`.
Its SHA-256 is the unchanged manifest digest
`762ca86fbcec6dbb4dcf0bebf42875421b6936bda04b2ea4b4510d69be87cc07`.
That migration commit is outside the maintained PR head's ancestry, so a
HEAD-scoped history search cannot discover the blob on CI. The catalog maps the
digest to a deterministic gzip copy of those exact original bytes. The normal
manifest size and SHA-256 checks still run before the file is installed.

## Shallow CI checkouts (2026-11-18)

CI checkouts are shallow (`fetch-depth: 1` or `2`), so the full-history
search in `pull` cannot see the commits that moved files into the store,
and every S3-free pull in `curve-names`, `claim-corpus` and
`validation-gate` failed with "no Git-history object matches manifest
SHA-256". Of the 782 store entries those workflows pull, 623 (615 distinct
digests) were committed to Git before they were moved; each is mapped here
to the historical blob that holds its exact bytes, found at the moving
commit's parent (or at the commit that added its manifest). Every blob was
checked for the manifest's byte count and SHA-256 before it was listed, and
`pull` checks them again on every read. A partial clone
(`filter: blob:none`) fetches the blobs from its promisor remote on demand.
No manifest digest was changed.

The other 159 entries, all pulled by `validation-gate`, were never committed
to Git: their manifests were added in the same commit as the rest of their
run, without the file. They still need their original store.
