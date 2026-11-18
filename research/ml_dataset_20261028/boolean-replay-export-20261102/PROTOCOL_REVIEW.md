# Independent protocol-extension correction review

Exact proposal boolean-protocol-extension-correction-20261102: manifest08d7d46324383f7ded2ff66e883dc5c7d32d6e3dc3f15f8615a1583612bad481; patch43068081071edabdb5d909ad7e119ffa2ebb6293cd578607b1f20eba14932782. All manifest file hashes verified. Read-only source review; no build/test or production edits.

No blocking defect identified in the tiny correction. Runtime authorization now reads only value[extensions][dataset_channels]; a top-level-only request cannot satisfy it. Equality compares the full typed requested object, including requested row and optional SAT request, with no compatibility fallback. The original file digest, duplicate-key parser, protocol schema tag and canonical freeze-body digest checks remain intact. Plans without channels follow the unchanged absence path.

The new unit test invokes real verify_protocol. Its positive extension body passes the native checks by construction; mismatch cases change row, remove SAT, remove the extension or use top-level-only authorization. Both file and canonical body hashes are freshly recomputed, and each mismatch requires the precise authorization error, so stale file custody cannot satisfy these controls. Legacy absence is separately exercised. The minimal test protocol is explicitly a native verifier fixture, not proof of official full-schema protocol validity; actual experiment protocol/schema admission remains separate.

Tests are unexecuted in this review. Source correction accepted for the parent's normal exact-source integration gates.
