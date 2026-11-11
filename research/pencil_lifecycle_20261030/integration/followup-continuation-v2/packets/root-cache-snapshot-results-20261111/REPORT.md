# Root cache snapshot renewal — terminal evidence

## Result

The bounded snapshot attempt reached a terminal `passed` receipt. The native
worker returned exit code 0 in 3.1129651480005123 seconds; the controller
completed in 3.461129906005226 seconds and the caller in 3.629120421013795
seconds. The systemd result is `success`, the execution status is 0, and the
owned cgroup was empty or absent at terminal observation.

The attempt reserved and charged 60 worker-seconds at 1 CPU and 1 GiB, with
swap disabled and `TasksMax=32`. Campaign accounting moves from 3,720 to 3,780
charged worker-seconds, leaving 125,820 of 129,600 worker-seconds. No automatic
retry was authorized. The preserved receipts contain no dollar-cost value, so
the standardized cost field remains null rather than inferred.

## Preserved cache manifest

The exact `root-current-cache/v1` receipt is preserved as
`raw/native/current-cache.json`:

- SHA-256: `9eae2a2fc003071692c5058e52b6e712f2d3ee0ad87e5c2086bc08240479c850`
- Receipt size: 593,986 bytes
- Inventory: 2,165 files in 415 directories
- Aggregate file bytes: 2,618,767,578
- Initial manifest SHA-256: `c5178d2138d79f1ea8cac1fc96cb8d968913ba403d68ee66fa36afdd60fdf3f2`

The cache roots are local physical paths under
`/var/tmp/pencil-root-materialize-t1953-20261109/cache-stage-v2/`. They are
recorded as reproduced from published sources. This packet preserves the exact
manifest and small receipts only; it does not contain or claim upload of the
2,618,767,578-byte cache body.

## Initial-manifest comparison

The comparison receipt records 33 added paths, 1,395 changed paths, one missing
path, and 737 unchanged paths. Its attribution boundary is retained verbatim in
the raw JSON: the comparison is to the initial target-only manifest, and
additions in other mutable roots are not attributed solely to the failed
compiler. The snapshot result itself records `compiler_invoked=false`.

## Publication binding

Two distinct publication readbacks are preserved:

1. The pre-execution source binding reports commit and remote head
   `36b55a2fb224720fd043b5728cb15c84f79646d1`.
2. The caller observation publication reports commit and remote head
   `e5e262cafa7f05c1c5fa7769d999447fc4bc69d5`.

The equal commit/head pairs are evidence of the remote readbacks at their
recorded observation times; they do not assert that the cache body was uploaded.

## Artifact map

- `summary.json`: standardized result and accounting record.
- `metrics.csv`: one metric per row with source pointers.
- `graph-data.csv`, `make_graph.py`, `timing.svg`: editable graph inputs and SVG.
- `current-cache-pointer.json`: exact manifest pointer and publication scope.
- `raw/`: exact native, controller, caller, and binding receipts.
- `raw-receipts.tar.gz`: deterministic archive of `raw/` only.
- `MANIFEST.json` and `SHA256SUMS`: packet integrity indexes.
