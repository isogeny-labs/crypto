# Native guarded lift v1 evidence boundary

Native PCC-20261116-35f196 pins the protocol/derivation,49-input source
manifest/qualification, complete finite trace, controls/identities,
pre-call qualifier and final logs. Standing is unsigned/unverified.

The composite snapshot reuses the unchanged43-input55368-byte v2 archive
(SHA2568c69c14ac4723a1609b8a35202be08e711b28172ec70231543bb2bafd372feb9)
plus the new six-input7452-byte source-v1-delta.tar.xz
(SHA256c44fc3c521469ebad0f72cbc1d03b8be3c621f91cd4572db3b55f20dd219f17b).
There is no claim of a new monolithic49-input archive. Both archive
round-trips and six compiled-in source hashes match the working inputs.

Retained executable: /workspace/curve25519-native-binaries/trace-lift-control-v1,
SHA256dfbfcda19fde51588a3fc762e2dc68f1844472cc017da231154ee53a857e7fd3.
This local qualifier is not an independently signed execution receipt.

All old native/Lean/claim/report bindings remain unchanged. New code may
evolve only with a new versioned source snapshot, qualifier and checks.
Never alter frozen evidence to agree with later code. The adapter's three
evaluated identity records are not formal proofs of its semantics.
