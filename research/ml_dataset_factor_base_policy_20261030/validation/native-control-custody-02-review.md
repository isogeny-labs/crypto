# Parent review of copied-artifact alias rejection

Review found a gap after revision01 was frozen: separate canonical directories
can still contain hard links to the same inode. The original-after-write check
would detect damage too late. Revision02 checks link count exactly one on every
mutation path, including manifest and worker registry, after read-only preparation
and before any write. The exclusive-namespace assumption remains required.

The sixth test creates a real file hard link, retains both directory snapshots,
requires the hard-link diagnostic and verifies both snapshots remain unchanged.
Previous five tests and all nine fixed recipes are preserved. Registry resealing,
result/outcome equality, aggregate caps and semantic-verdict separation are
unchanged. Formatting passed. This is parent static review; execution is pending.

Revision01 and candidate07 remain retained, unexecuted. Candidate08 carries only
this helper delta relative to07; all25 library overlay records match passing06.
Source references: revision01 contract/source/review, acceptance.rs checked_file,
copy_tree and snapshot helpers, and std::os::unix::fs::MetadataExt::nlink API.
No change to mathematical algorithms or measurement protocol is introduced.

Tooling note: explicit +1.98.0 rustfmt was unavailable; no component was installed.
The installed default rustfmt 1.10.0-stable formatted and checked the source.
Build validation continues to use the frozen runner's Rust1.98 configuration.
