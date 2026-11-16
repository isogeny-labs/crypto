# Frozen native indexed-circuit v1

Native claim PCC-20261116-b3227f pins the protocol, complete original-system
context, augmented system/witness/trace, source qualification, qualifier and
raw logs. Producer evidence is unsigned/unverified. Do not edit pinned bytes.

The49-input composite context is the unchanged43-input v2 source archive
(55368 bytes, SHA2568c69c14ac4723a1609b8a35202be08e711b28172ec70231543bb2bafd372feb9)
plus the six-input34384-byte source-v1-delta.tar.xz
(SHA256c19bcfc224c111f8c3b14ab99f2575aac8c94e36467906ce30a191542e457eff).
The delta has five new crate inputs and the original full-system gzip;
there is no monolithic49-input source archive.

Retained immutable executable:
/workspace/curve25519-native-binaries/indexed-circuit-control-v1,
1565216 bytes, SHA2561bd10ec12e3ae459dde65cd14c5f69772edd1c6e00b8fe137dc85a0763b39bfc.
qualifier-v1.sh is a separately copied0555 qualifier; its pre-call hash
e118cd25d4c08b8aee251d7a2a3d4b0285f0044af945ef22c1a0dc2ab5926fb9
was captured in QUALIFIER_PRE_CALL.json before executing it. The Rust
pre-call file additionally commits compiled-in source and executable hashes
before control evaluation. BUILD_MANIFEST, QUALIFIER_PRE_CALL and final
SOURCE_QUALIFICATION are distinct records; no post-call reconstruction is
presented as a pre-call receipt.

No native source changed after the fixed release test/fmt/strict Clippy and
qualification. All43 dependency working inputs remain unchanged. Old v1/v2,
trace/reference/solver archives, original criteria, native outputs and direct
Lean bindings retain their bytes. Future source changes require a new
versioned qualifier, archive and evidence rather than calling v1 current.

The sibling lean-indexed-v1 project has its separate source/configuration
and direct proof binding; its endpoints are conditional algebraic contracts,
not a proof of this native compiler. Frozen report bytes have their own
report claim PCC-20261116-b20167; direct Lean claim is PCC-20261116-77b508.
Both remain unsigned/unverified. See QUALIFICATION_SCOPE.md for the full
limits and NEXT.md.
