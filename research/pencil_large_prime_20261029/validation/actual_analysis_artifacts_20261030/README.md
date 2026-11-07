# Durable actual finite analysis outputs

The maintained artifact store contains both immutable snapshots:

- initial-null-review-analysis.json: first native analysis with absent coverage
  review; all finite price cells null. Preserved rather than replaced.
- covered-analysis.json: same records and pinned native analyzer after binding the
  independently reviewed finite source coverage. All300 measured pairs admitted
  in all36 price cells. Clock qualification remains unavailable.

Use the repository maintained artifact helper `pull` on this directory to hydrate
these files from the tracked .artifacts.json. Publication receipts record exact
S3 keys, SHA256, byte lengths and complete independent remote readback checks.
The helper stored objects and manifests but its final Git index cleanup met a
concurrent parent index.lock. That lock was not modified; raw files remain ignored.

Raw input files already reside in versioned existing run S3 objects. Retrieve
using ../actual_gate_results_20261030/versioned-raw-locators.json, whose entries
contain every bucket/key/version/byte length/SHA256. Verify before extracting or
reading. This avoids republishing duplicate raw run data.

Both analysis manifests are retained here. Their paths record the originating
host. To recompute elsewhere, hydrate the source-bound analyzer/review sources,
fetch raw artifacts from the locator manifest, and explicitly rebind paths to
those identical bytes; retain that relocated manifest as a new artifact. Run
native_price/target/release/analyze_cohort MANIFEST --p739-gate60. Path rebinding
changes manifest/output integrity hashes; it does not alter record bytes or
finite arithmetic. Compare exact cost fractions and semantic observations, not
raw serialized path-dependent output hashes. No solver execution is required.

The exact36-cell grid and compact covered observation receipt are in
../covered_gate_results_20261030/. These are finite model ratios, not measured
CPU-time or monetary ratios. Preserve prior failed-attempt resources separately,
including the nonrefundable180-second admission reservation, which is not a
measured worker-time observation. The frozen-v5 canonical metadata erratum is a
separate qualification matter and is not resolved by artifact publication.
