# Artifact publication handoff

The complete core archive `native-v1.tar.xz` is selected by the repository's
actual artifact policy: 146,256 bytes, SHA-256
`ef5b561a184672f36ac9c2e12963f23d14d56aa731ea6e3d79f68968b3e2a875`.
It must stay byte-identical, including historical receipts and failed paths.

Local maintained push attempt 01 failed because AWS CLI was absent. An isolated
CLI was installed outside the repository; attempt 02 failed at object metadata
lookup with `Unable to locate credentials`. These attempts establish neither
remote absence nor upload success. Logs are retained under `development/`.
No credentials were exported, and no policy, exemption, workflow or remote
object was changed.

The maintained same-repository `artifact-autofix` workflow is the documented
fallback (`docs/artifacts.md`, `.github/workflows/artifact-autofix.yml`). It uses
trusted base tooling/policy; the PR provides data bytes. Publish the real archive
path with the source/report, then require the workflow's subsequent trusted
`.artifacts.json` receipt to bind the exact hash and size before claiming
storage success. Store: `s3://setec-astronomy-artifacts/store`.

Parent #4447 has 31 check runs and unresolved main journal/ledger conflicts.
The isolated new crate/report therefore use the separate `curve25519-affine`
area lane, stacked on the parent's `lane/curve25519-torus` branch. Parent
conflicted paths are not resolved by this child. Its initial trusted feature
base and candidate policy bytes match SHA-256
`e34962a1c1260120af47265b11d33531acc6a4d3aa938073d2aea356c3bd94d3`.
Current main has a legitimate additional keep entry and policy SHA-256
`120f74a3280a69a700e0d38093c1a089bdf9ccf95b0670a72beb5f6bdb27917e`;
do not revert that update on later main integration. No policy was edited to
make this payload pass. The child waits for actual parent integration, then
retargets to main and must satisfy its actual current-head gates and reviews.

Source publication: regular [PR #4456](https://github.com/isogeny-labs/crypto/pull/4456),
initial published source merge `6c7a1f4266f8d4ae43a77a8e0ce08600a2d2d906`
from local `58633ba01fbd5f97d1382e40de04b157f4126b8f`. All 77 owned paths
and the complete initial tree match. `PUBLICATION_RECEIPT.json` records that
Git identity, not store publication; PCC-20261116-0bd924 remains unsigned.
[pcc:unverified]

**Pending:** genuine receipt and trusted upload commit; source PR publication
does not itself establish upload. Do not invent a receipt, rename/split data to
avoid policy, export credentials or remove remote objects. Keep inspected local
bytes hydrated if the maintained bot removes the tracked payload.
