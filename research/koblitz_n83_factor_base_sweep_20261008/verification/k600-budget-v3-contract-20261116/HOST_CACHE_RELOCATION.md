# Reversible host cache relocation, 2026-11-16 UTC

During isolated budget-controller validation, the macOS boot volume fell
from roughly 7.2 GiB free to 3.0 GiB while a separate Rust build was active.
The first controller compile was stopped with exit 130. This is a host resource
stop, not an algorithm result or a passing test. `TMPDIR` was subsequently
placed on the secondary volume. No source tree, frozen N83 input, result
receipt, or tracked output was deleted or moved.

After `ps` found no Cargo or rustc process and `lsof +D` found no open handles
in the selected generated caches, four inactive directories under
`/private/tmp/ic-suite-feature-integration/debug/incremental/` were moved to
`/Volumes/SSD990/tmp/n83-budget-boot-cache-quarantine-20261116/` with a
symlink left at each original path:

- `cryptanalysis_suite-3ox844cqultfl`
- `cryptanalysis_suite-19cofk28uh15o`
- `cryptanalysis_suite-11w7ymjs77dxa`
- `cryptanalysis_suite-1eephapdvt7yo`

Their destination total was 2,864,420 KiB; boot free space rose to 6,120,476
KiB. The inactive generated `debug/deps` directory was then moved to the
same destination with an original-path symlink. Its destination occupied
13,493,024 KiB after the cross-volume move; boot free space rose to
14,309,592 KiB. The destination volume had 37,822,620 KiB free. Different
source and destination allocation sizes were observed, so the receipt does
not equate `du` growth with unique bytes. The source symlinks and destination
directories were checked after each move. The caches remain available at
their original paths and may be restored by a later owner-controlled move.

After the first full-library attempt exposed missing sparse fixtures, the
inactive Cargo registry source cache at
`/Users/adamburan/.cargo/registry/src` was moved to the same secondary
quarantine as `cargo-registry-src` and symlinked at its original path.
`ps` showed no Cargo/rustc process and `lsof +D` found no open handle before
the move. The destination occupied 904,932 KiB; boot free space rose from
2,773,168 KiB after sparse materialization to 4,311,192 KiB at the next
inspection. The difference is an APFS allocation observation, not a claim
that this one directory uniquely caused every byte of change.

The second release example test invocation used a separate Cargo target and
temporary directory under `/Volumes/SSD990/tmp/`; its result is recorded
separately. None of these host operations changes the frozen K600 source,
input hashes, or the arm admission protocol's 5 GiB boot floor.

Before the merged-tree full library run, two more inactive generated locations
were relocated to the same secondary-volume quarantine. The remaining
`/private/tmp/ic-suite-feature-integration/debug/incremental` directory now
points to `incremental-rest`, and the idle
`/Users/adamburan/.rustup/toolchains/stable-aarch64-apple-darwin` directory
now points to `stable-aarch64-apple-darwin`. Each original path is a symlink
to the preserved directory. The pinned validation command uses Rust 1.99.1,
not the relocated `stable` toolchain. These moves were checked for active
Cargo/rustc processes and open handles before completion. They were not
deletions or changes to repository inputs.
