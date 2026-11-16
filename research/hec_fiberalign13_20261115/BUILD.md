Cargo release/offline builds in tmp/target, TMPDIR=tmp, toolchain stable
1.99.1. Private msolve official v0.11.0/cc044551f8e202d00691990fd3fd0b1226a88ea2
rebuilt using AppleClang and the same matching FLINT/libomp flags as the
previous inverse study;71upstream tests pass. No global install or dylib edit.
Wrapper tmp/msolve/msolve selects the private libneogb; runtime/source/hashes
recorded. Cache is ignored/reproducible, not assumed present after sparse sync.

fiber_select.rs copies the prior admitted selector, returns first source/pair,
retains all exact/source controls and removes the old-matrix final comparison.
generator.rs copies the exact polynomial producer and reads a fresh public
lambda file instead of an embedded previous target vector.
recover_model.rs uses the fresh decoded pair, public adjacency and offset;
old original-D1/Bimage certificates are absent from its constructor/candidate.

pair_functions.rs copies only collection, small-kernel and reverse-trace logic
from hec_coldbuild13_20261115/pair_dynamic.rs; lp_trace.rs is the pinned existing
sparse LP implementation. No old translate-descent or oldmapper main is included.
The final47column core stays dense and is explicitly disclosed.
Conic geometry/exact arithmetic is reused unchanged from the conic descent gate.
No compile failure or scientific rerun in this integration so far; unused
helper warnings retained. Rustfmt skip_children avoids editing shared modules.
