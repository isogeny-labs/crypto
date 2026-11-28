# Worklist solver v1 freeze

Native PCC-20261116-fb7763 is unsigned/unverified. It binds the protocol,
source/qualification manifests, qualifier, full raw archive, final logs and
failed first fixture-binding attempt. [pcc:unverified]

Binary: `/workspace/curve25519-native-binaries/worklist-solver-control-v1`,
1470000 bytes, SHA256
`0ae79df0ef371e82fac4b1cd5fea6418b8e9b5fd20dc0533d076774bcce8b01e`.
Qualifier SHA256
`f12fbd55dc6d43a826e0ff62dd5d42cc2a6c05151bf18173b2b1026eabdd2399`.

The55-input context is COMPOSITE: unchanged43-input v2 source archive,
unchanged six-input indexed delta, and new six-input worklist delta. The
last has12952 bytes/SHA256
`f75bac5d5ec48202ee39a00cbdf0b7442a1c4b268cd78da2f28e33ea8333fb3a`.
There is no monolithic55-input archive. See SOURCE_QUALIFICATION.json.

The whole raw native tree is one10992-byte archive, SHA256
`a67942ca041409c948f6f9d345f34570b4ede31edb0889729dcaa266dad0d0b7`.
NATIVE_ARCHIVE.json lists each original member's byte count and hash;
local ignored native-v1 bytes remain available. First/final raw logs retain
their bytes and formatting; do not trim them to hide diagnostics.

Working Rust may evolve only with a new versioned source snapshot, qualifier
and evidence; never update v1 hashes/results to describe future source.
Prime-field validity remains external. Native replay is not a formal proof
of solver/compiler correspondence; no new Lean theorem is asserted here.
Partial scheduling counts are not timing, total cold cost or live RAM.
[pcc:unverified]
