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
