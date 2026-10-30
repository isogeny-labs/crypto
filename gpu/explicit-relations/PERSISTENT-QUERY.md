# Optional GPU-resident right search — device validation pending

`make cuda-persistent ARCH=sm_120` retains the exact dense table, wide index
and queried CUB scratch path, but replaces the host's reset/search/readback
cycle for every chunk with two kernels and one 24-byte result readback per
relation target. It is an experimental option, not the ordinary build.

A grid of at most eight blocks per multiprocessor visits disjoint 256-tuple
tiles at grid-size strides. One lane loads the current minimum right ID using
a device-scope relaxed atomic; a block stops only when its entire tile is
above an existing witness. Every ID below the final minimum is therefore
searched. There is no cross-block barrier, spin wait or requirement that all
blocks be resident. All lanes, including tails and infinity, participate in
the normalization barriers. An explicit end-of-iteration barrier protects
shared normalization storage before reuse. Loop progression avoids overflow.

The first kernel atomically minimizes the global right ID. A second kernel,
ordered in the same stream, reconstructs that right tuple and performs the
same exact lower bound to find the minimum duplicate left ID. Separate IDs
avoid packed-product overflow. The host checks the bounds and independently
checks the relation. Tables and targets have the same semantics as before.

Each block accumulates its actual valid tuple constructions and adds one
counter at exit. `visited` records search tuples actually computed, including
any speculative work above the final witness. It can depend on scheduling;
it is neither a deterministic prefix nor a complete arithmetic-operation
ledger. A complete miss must count every right tuple exactly once. The final
witness reconstruction is separate from this search counter. Results remain
deterministic; only the counted speculative work can vary.

Query tests cap the grid at two blocks to force multiple strides, a partial
last tile, late minima, complete misses, alternating hit/miss resets and an
empty reprepare. Existing full-coordinate CPU comparisons, duplicate/parity
keys and large-key/index boundary checks remain. The future validation panel
must run memcheck, racecheck and synccheck before any timings. No CUDA build
or performance data for this option has yet been obtained.

Literature checked before implementation:
- NVIDIA's grid-stride loop guidance:
  https://developer.nvidia.com/blog/cuda-pro-tip-write-flexible-kernels-grid-stride-loops/
- CUDA device-scope atomic references and memory semantics:
  https://nvidia.github.io/cccl/unstable/libcudacxx/extended_api/synchronization_primitives/atomic_ref.html
  https://nvidia.github.io/cccl/unstable/libcudacxx/extended_api/memory_model.html
- Existing search_right, normalize_block, exact indexed lookup and the frozen
  v11 contract. This is conventional GPU scheduling, not a new ECDLP method.

The active v11 run is pinned to its prior commit and does not use this option.
A separate frozen matched experiment is required before any performance claim.
