# Software validation

The isolated fixed-base runner's five native tests pass. Eight exact small
positive/inconsistent controls pass through the actual pinned Singular runtime.
The full repository cargo test --release --lib -- --test-threads=4 run passes;
see the archived complete log and the recorded result line below. No root source
or tests were edited for this experiment. Existing tracked sparse fixtures had
already been hydrated for the preceding dataset task.

Cargo fmt --check and bash syntax checks pass for the new standalone project
and wrappers. Actual container inspection records read-only input/executable
mounts and4CPU/16GiB runtime limits. Arithmetic/control tests do not establish
tractability or degree values for the large target systems.
test cryptanalysis::partial_enlargement::tests::every_order_computes_the_same_closure ... ok

test result: ok. 4966 passed; 0 failed; 122 ignored; 0 measured; 0 filtered out; finished in 759.47s
