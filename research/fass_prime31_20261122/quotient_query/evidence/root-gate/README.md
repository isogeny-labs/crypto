# Separate root-library gate

The one permitted root command was `cargo test --release --lib --locked --offline -j 1`, under a 600-second deadline, 6 GiB virtual-memory cap and one test thread. It ended with cargo exit 124 during dependency compilation. Stdout is empty; stderr has no compiler-error or build-finished event, and no root tests ran. This remains an **incomplete root-library gate**. [pcc:PCC-20261123-69256a]

The full retained pre/post receipts, command, toolchain, stdout/stderr and inventory are in [attempt1](attempt1). All 1,769 recorded materialized input hashes were unchanged. The broad tracked-tree inventory also lists 2,192 unmaterialized paths; it does not claim that all of those paths are required compiled inputs. The root call used the recorded working-tree sources, separate from the immutable standalone snapshots.

The full-root timeout is not reported as a test pass or an identified source defect. No second root call was launched and the timeout/resource envelope was not increased. The existing build cache remains available for a separately bounded future development gate. The standalone component's 95-case qualifications and persisted-reader controls have their own evidence and scope.
