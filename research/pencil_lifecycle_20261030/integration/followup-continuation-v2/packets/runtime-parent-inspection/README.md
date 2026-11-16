# Base layer metadata

`python3 inspect_layers.py` reads the exact local archive in save-receipt.json and regenerates layer-observations.json. It checks the archive hash, image config and each layer before reporting headers. It does not call Docker or extract files.

`capture-first-attempt.py` is historical failed source; do not rerun it. `capture.py` is the corrected future capture script, not a second executed attempt. Running it contacts the local Docker daemon and creates a fresh local archive; it is not needed to replay this inspection.

The raw archive remains local, identified by digest and exact path. No remote custody is claimed. Docker image IDs and layer diff IDs are pinned; timestamps in newly exported archives may differ. Root mounted metadata and actual replacement-image qualification remain unobserved. See REPORT.md and binding.json.
