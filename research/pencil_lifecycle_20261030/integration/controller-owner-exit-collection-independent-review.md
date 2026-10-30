# Independent source review: controller owner-exit collection

Reviewed 2026-10-30, read-only. Patch SHA256 b170e5a4bb2bf42c054e05949a8ce8dd28c30420b79f2545102cfb466ea170df against retained base ec633a6af0e9475731f0b7f9757828bfdd6b6baab6918ec6d082803a6cd0ec91. No application, compilation, fixtures, cloud requests or scientific execution performed.

No concrete blocking defect found within the documented collection/custody contract.

The existing call chain checks request identity, pinned Pod UID/instance and runtime image digest/ready container before fetching supervisor status; status checks request/attempt. The added fetch retains that chain. Fixed argv and integer-derived path introduce no shell interpretation. Existing kube() wraps kubectl with timeout 180 seconds, kill-after 5 seconds and request-timeout 30 seconds. Readers retain bounded stdout/stderr prefixes while draining; the transport timeout remains relevant to their completion.

Raw captured bytes are preserved without reserialization, digest-addressed, including malformed or truncated nonempty output. Only transport success plus bounded schema/request/attempt/null checks marks an exact receipt. Derive rejects duplicate known binding fields; unknown observation fields remain unvalidated raw evidence. This is not general semantic verification of the owner record.

Exclusive temporary creation/hard-link publication and file/directory synchronization preserve differing bytes and failures. Existing identical files are checked and synced. This assumes the controller-owned local receipt directory is not concurrently replaced by an adversary: the existing-file reopen is not an O_NOFOLLOW/openat path-walk guarantee. Kubernetes placement checks and exec are also separate requests, not an atomic UID-preconditioned exec. Neither boundary is newly represented as cryptographic authentication.

Collection precedes failed/completed/verified branches. Remote unavailability remains unknown evidence without invalidating a mathematical result; local retention failures propagate. An existing payload failure remains first in the reported error. Already-terminal controller entry or vanished Pod does not synthesize a receipt. Top-level JSON outputs enter the existing EXIT upload mechanism without new storage/IAM routes.

Runtime utility availability (head), deployed image behavior, actual transport custody and fixture execution are unverified here. Receipt presence does not establish reap, full cost closure, active worker time, or execution qualification. Three authored tests are source-reviewed only. Patch application root is taskq (diff paths begin deploy/); no application was attempted.
