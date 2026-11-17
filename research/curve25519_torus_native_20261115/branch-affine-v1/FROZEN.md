# Version discipline and identities

Core98inputs = unchanged64-input affine context (43+6+6+9) +34 new inputs
(six Rust/Cargo files and28 exact public requests). Five source archives
round-trip to the corresponding source bytes; no monolithic98archive.
New34-input delta:18960bytes, SHA-256
`f0735ae1100e76b91587221b0e662a2ad5ba30c99552c1cd00c5e49694d76ae8`.
All35 compiled bindings match, including the unchanged old engine template.
[pcc:unverified]

Retained core executable: `/workspace/curve25519-native-binaries/branch-affine-control-v1`,
3432840bytes, SHA-256
`87e5872e4f5743884237b56b740a27be6f03c2e1127adde1ccf60c149ebcfb4f`.
Core qualifier SHA-256
`b7e0e8d7649c3336b05ac17c821e59a029caaf55878bf8fc1b239edfeebfe9a7`.
All old and new core sources/fixtures are unchanged after qualification.
A later edit requires a new version/qualification; never edit old receipts to
match a new working tree. [pcc:unverified]

Whole core raw tree:247files/110509160bytes in280300-byte `native-v1.tar.xz`,
SHA-256 `5332de1c3b6981ab664cc6e71b1c162996ac7ef812a572a382af5382b27896ad`.
Member sizes/hashes are in NATIVE_ARCHIVE.json. Keep local raw bytes and the
archive immutable; store migration is not license to alter either.
[pcc:unverified]

The post-call reader is separate from those98inputs. Its one Rust source,
39 build-cache rlibs, locked Cargo context and compiler flags are in
AUDIT_BUILD_MANIFEST.json. Source SHA-256
`378d9e4b2acfb5b553196c9a125cd477c362120af12a7b4ed0aa8fbf0a1f0b53`;
executable `/workspace/curve25519-native-binaries/branch-affine-postcall-audit-v1`
SHA-256 `2b246aa058d7310484a85a22bef245d9ebe3b869331b790f2c6a476ba41296f2`.
Its complete separate audit round is11020bytes, SHA-256
`8f3d5e746ad5c09acdab2bbc8cbaa7ba6e537a95d3cab2f26a54c6e0b248e455`.
This is not a split of the core archive or a reconstructed pre-call receipt.
[pcc:unverified]

Native PCC0c76bf and reader PCC10062c remain unsigned/unverified. Conditional
coefficient and lifetime checks do not certify execution, counters, search
completeness, primality or full cost. Original criteria, consumer source pin,
old native/report/Lean bindings and all published histories stay unchanged.
