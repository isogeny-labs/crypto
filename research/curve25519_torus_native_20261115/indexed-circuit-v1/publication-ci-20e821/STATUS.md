# Source-publication CI snapshot:20e821

Source/evidence publication20e82118494e438afedb83e1dfdfb3b2b27a3dcd
has local scientific parentd174823774609e0d5249bfa01824a91db59ebfa5
and prior lane parent50a5a4522593ab16d7b93b13032261657bdaef6e.
Tree3d77e1138717f164dd5c2d835889610db7705cd5 equals the scientific tree.
Hosted checkout746b3df0643acdb08d7467d355b37ba2af19c9df has the SAME
tree, with feature-parent21f5/source20e parents. [pcc:unverified]

The once-only19:58Z snapshot has11 check runs: review/migrate SUCCESS,
current/recompute FAILURE, validation gate/learning/parity IN_PROGRESS,
first PCC gate CANCELLED and its later run IN_PROGRESS, two sign jobs
SKIPPED. This does not infer later results or any result for a new head.
No explicit rerun, workflow dispatch, empty refresh commit or merge occurred.
[pcc:unverified]

## Newly observed artifact-access failures

- current job133570169265, run43944432135, failed19:56:10Z.
- recompute job133570169492, run43944432056, failed19:56:20Z.

Both complete decoded logs stop at maintained artifact hydration with S3
HeadObject403 Forbidden on existing backend evidence, before backend
validation. The workflow, helper, policy, root/backend artifact manifests,
backend catalog/reproduction lock and original replay-fixture manifest are
byte-identical Git blobs across feature-parent21f5, prior lane50a5 and this
source20e head. RETENTION.json records the exact blob IDs. [pcc:unverified]

decoded-logs.tar.xz retains BOTH complete decoded UTF-8 job logs losslessly,
with zeroed archive timestamps; hashes/readback are in RETENTION.json.
This is not a raw server-byte or independently signed execution receipt.
Private decoded originals are retained outside the repository. [pcc:unverified]

Restore CI read access to the referenced artifact objects through the
canonical service/owner. No credentials are exported, no bucket/policy or
verifier is changed, no old subject/pointer is altered and no remote object
is deleted. This check does not diagnose which credential or service policy
caused403, or assert that main's latest CI was inspected. No scientific
source failure or route exclusion is inferred. Existing missing806be3 is
a distinct historical PCC issue; do not carry it into an uninspected head.

The local native/Lean/report qualification retains its recorded limited
scope and unsigned standing. Subsequent metadata publication has its own
actual-head gate snapshot; it cannot inherit these source-head outcomes.
