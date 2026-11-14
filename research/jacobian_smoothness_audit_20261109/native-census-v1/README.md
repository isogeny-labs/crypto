# Native Mumford calibration and census source

Status at source freeze: both executables build; no curve invocation or new
population has run. Calibration and full census require separately frozen,
committed canonical protocols before their affected calls. [pcc:unverified]

The primary Rust driver reuses the exact upstream prime-hyperelliptic
Mumford/Cantor backend from cryptanalysis commit
fe751b09b08bc274d60cb22cfba25b13983b2b6e. Three source files and their MIT
license are retained unchanged under src/vendor. This is an immutable
experiment source snapshot, not an arithmetic fork. Git blobs and SHA-256
digests are in [sources.json](sources.json). [pcc:unverified]

For each declared monic quintic, the driver retains a nonsquarefree rejection
or enumerates the complete actual reduced-pair model. It evaluates every
ordered Cantor addition, requires closure, checks all group axioms on the
whole table, and enumerates/deduplicates every cyclic subgroup by membership.
It records identity-included smooth counts and exact integer cross-products;
the first odd-prime, noncharacteristic proper counterexample is an explicit
record, not an asymptotic inference. [pcc:unverified]

The separate C++program uses its own small-field polynomial arithmetic,
Euclidean/Bezout implementation and Cantor algorithm. It independently
enumerates every pair, compares every actual addition and subgroup, and
checks the entire group table. Direct Fp/Fp²point counts provide additional
genus-two order and smooth-count controls. Per-curve counts are retained.
The JSON used for analysis must encode exactly the checked TSVbytes.
This is two languages on one host, not another machine or a signed verdict.
[pcc:unverified]

The exact F3control has8pairs/7smooth, and the new complete F5control
has16pairs/all16smooth. Both have Lean proofs. Stoll2014Example4.19
provides a further pre-data nonzero-v control for the full F3census:
on y²=x⁵+1, the predicted ambient counts are10/8and the cycle of(X,1)
has order5with all5members smooth. The latter is a source-based prediction,
not yet a new Lean formalization or an observation. [pcc:unverified]

Build only:

```sh
cargo build --offline --release
g++ -std=c++17 -O2 -Wall -Wextra -Wpedantic cpp_counts.cpp -o target/mumford_cpp_counts
```

Execution uses a fresh output directory and refuses existing data files.
Run from the repository root. The Rust `census` command checks that all
native source/lock/vendor files equal the supplied committed source bytes:

```text
mumford-census-audit census P all|fixture OUTPUT_DIR SEED REPLICATE SOURCE_COMMIT PROTOCOL_SHA256
cpp_counts OUTPUT_DIR/audit.tsv OUTPUT_DIR/cpp-check.json
mumford-census-audit analyze OUTPUT_DIR OUTPUT_DIR/cpp-check.json
```

The protocols, execution receipts, raw artifacts/export and report must
record every failure or timeout. Elapsed intervals at this nonisolated local
host are scheduling diagnostics only. User authorization permits existing
local capacity with zero paid compute. Full geometric Picard identification,
scaling/large-prime asymptotic claims and the original independent-machine
requirement remain separate. [pcc:unverified]
