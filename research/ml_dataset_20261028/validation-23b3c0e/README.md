# Native dataset budget software delivery — 23b3c0e

This packet records software validation for source
`23b3c0e527d621aebf6532db61744fab77867f48`. It is not a cloud corpus,
scientific benchmark, worker IAM check or campaign admission. Verified cloud
dataset cells remain zero.

`validation-initial.json` records the release build, strict maintained Clippy,
formatting,19 CLI checks,7 synthetic dry-admission cases and non-root isolated
container preflight. Full-library validation was still running at this initial
record; interrupted earlier attempts are retained with explicit explanations.
The image build-info similarly preserves the status at image construction.

The37 initial artifact references in `.artifacts.json` total42,458,703bytes.
Every object was freshly downloaded directly from the approved S3store, without
using the local content cache, and its length/SHA-256 checked.
`remote-readback-initial.json` binds those exact results. Raw logs, binary,
public build inputs, operator fixtures and reviews are restored through the
repository artifact helper. No credentials or Docker login state are included.

The illustrative admission packet deliberately includes invented evidence and
placeholder execution identities. It exercises software schemas and decisions;
it must never be used for real reserve/activate operations. A well-formed
rejection can exit0, so automation must check JSON `admitted`.

See the current [operator guide](../../../docs/ml-dataset/BUDGET-CLI.md),
[launch adapter contract](../../../docs/ml-dataset/LAUNCH-ADAPTER.md) and
[remaining coverage](../../../docs/ml-dataset/REMAINING-COVERAGE.md) for the
canonical-plan metadata, launch identity/resource, worker IAM and broader
solver/ML work still outstanding.
