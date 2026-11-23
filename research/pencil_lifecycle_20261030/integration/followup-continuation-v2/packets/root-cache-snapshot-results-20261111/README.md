# Root cache snapshot: terminal result packet

This additive packet reports the terminal result of the separately published
`root-cache-snapshot-renewal-20261111` source packet. It preserves the exact
native, controller, caller, and pre-execution binding receipts under `raw/`.

Scope is intentionally narrow:

- `raw/native/current-cache.json` is the exact 593,986-byte manifest receipt for
  the local cache roots. It identifies 2,165 files totalling 2,618,767,578 bytes.
- `raw-receipts.tar.gz` contains only the small JSON/text receipts in `raw/`.
- The 2,618,767,578-byte cache body remains at the receipt's local physical
  source and is **not** copied, archived, or claimed as uploaded here.
- `current-cache-pointer.json` records the exact manifest hash, local roots, and
  the two remote-publication readbacks associated with source binding and caller
  observation.
- `summary.json`, `metrics.csv`, `graph-data.csv`, `timing.svg`, and `report.pdf`
  are neutral derived views of those receipts.

No compiler, solver, target collector, cache traversal, or experiment payload
was run to prepare this result packet. Only existing receipt files were read and
copied, and report artifacts were rendered from them.
