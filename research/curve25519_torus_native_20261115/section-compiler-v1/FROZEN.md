# V1 source and native evidence

Do not edit the13 new source/protocol inputs in SOURCES.sha256, the prior41
inputs, the three composite archives, native custody/control subjects or their
claim records to match future code. A source change needs a new versioned
snapshot, prospective qualification and fresh controls. [pcc:unverified]

The composite54 inputs are old30/58112-byte source-v1.tar.xz, old11/13500-byte
delta and new13/21360-byte delta. SOURCE_QUALIFICATION.json gives exact hashes;
no monolithic54-input archive is asserted. Ten compiled-in new input hashes
match. The retained immutable executable is
/workspace/curve25519-native-binaries/section-compiler-control-v1,
SHA25646e75f76f081edbed17b1bb0ec9500368d79bf60f2501c77cf3352f0b5b448d7.
Before and after the fixed call, all54 source hashes and that binary match.
[pcc:unverified]

Local source6f4987bd455e69e61c3449ef89a3a4025c6076dd preceded the call;
remote source e1591a201d3957cd59b27b5f2b3545e34b8d194f followed it after the
tracking-PR merge and Git-transport failure. Every source blob is identical;
the full publication tree equals local preservation merge df90afa2. This is
not an independent or signed execution receipt. [pcc:unverified]

Mutable publication/checker metadata may advance without changing frozen
source or claiming that old hashes describe newly edited Rust. Full native
archive publication is still pending and cannot be inferred from a Git commit.
