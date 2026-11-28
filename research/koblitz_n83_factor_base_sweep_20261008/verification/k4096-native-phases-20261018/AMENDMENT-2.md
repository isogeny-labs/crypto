# Panel preflight correction after input validation failure

The scoped S3 input read succeeded on job
`20261019-020117-n83-k4096-phases-retry-00a4ff`; the downloaded compressed
object independently matched SHA-256
`ccc1196d75de406b1daba634b8011f08308c37bd1e674758f9efeabb0ff0bcad`.
The local staging script then exited 1 because its panel copy list omitted
`probe-corpus.json`, one of the four sidecars required by the frozen
`fresh-inputs.json` SHA-256 map. No build or solver search started. The raw
runner job, status, log, host, resource record, and S3 GET receipt are in
`launch/failed-panel-preflight/`.

The next attempt adds only that Git-tracked sidecar to the panel. It keeps
the same object bytes, public fixture, curve, arity, caps, resource envelope,
source instrumentation, and audit. The exact staged copy and hash preflight
must pass locally before relaunch. This failure is a launcher/preflight
result and remains outside relation-yield accounting.
