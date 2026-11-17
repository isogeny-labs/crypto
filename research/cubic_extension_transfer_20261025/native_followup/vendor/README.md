# Pinned native dependencies

* `quartic_ic.rs`: byte-preserved source from aburan28/crypto PR #2170,
  commit `e5d1f02aeee957f2517d2cee5afe6a63bb36c803`,
  Git blob `94a2465cfffe10d533b8e9f13a5629837b3bd908`.
  The original repository license applies. This is a dependency snapshot,
  not a replacement for that PR or an independently written quartic group law.
* `json.hpp`: nlohmann/json v3.11.3, `single_include/nlohmann/json.hpp`,
  Git blob `8b72ea6539f40e4e3f7762d17e30c67f2a8308ed`.
  MIT license and copyright are retained in the header.
* `identity_certificate.rs`: existing native repository checker from main,
  Git blob `114f1f4a3f2a1be027a9b046ef166259df49dd45`. The separate Cargo
  checker reuses its statement and point derivation exactly, with OpenSSL
  providing SHA-256. It checks the C++ issuer and adds a checker seed.

`build_native.sh` renames the reference CLI main, selects deterministic
standard-library hashing, exposes the already recovered scalar in ArmResult,
and appends the fixture adapter. Every transformation is explicit in that
script. The vendored source itself is unchanged. Operation counters and
algorithms come from the reference, with actual post-return verification
costs captured by an enclosing counter interval in the adapter.

FLINT and GMP are dynamically linked from the pinned SageMath 10.6 image;
they are compiled C libraries. No Sage or Python process runs the experiments.
Record the container image digest and compiler when reproducing.
