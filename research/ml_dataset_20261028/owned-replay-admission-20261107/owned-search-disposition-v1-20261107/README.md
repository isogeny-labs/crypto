# Owned-search capability disposition v1

Proposed source patch against activated HEAD ff02728b58311a34129d3472cac325334cc7b347. It adds `Option<ChannelDisposition<OwnedBooleanSearchRequest>>` to RequestedChannelDispositions with serde default/skip-none, and the explicit reasons RequiresActualOwnedSearchReplay and OutsideOwnedSearchProfile. The original function calls the actual owned worker supported predicate. Both execution_verified and training_export_admitted remain false for supported and unsupported requests. Exact request and finite policy are retained.

Absent owned requests serialize without the new key; the requested-disposition schema remains v1. Legacy v1-v4 channel disposition bytes are unchanged. The new field does not alter channel request validation or v5 exclusivity. There is only one struct construction in current src, patched here; the other occurrence is the definition.

Eleven tests pass: seven new controls plus four existing types tests. The harness compiles the complete proposed actual types.rs with real cached serde, the exact proposed disposition function, and actual extracted owned worker selection/supported/profile functions. The one-variant native policy enum is a structural fixture. Legacy SAT/affine eligibility functions are explicitly fixed-false structural stubs shared by the proposed and original-function compatibility controls. The old function receives only an owned_boolean_search:None initializer to compile against the expanded struct; skip-none omission preserves its bytes. Thus the v1-v4 control checks byte compatibility under the same fixture predicates, not real legacy execution eligibility. Existing legacy function branches are unchanged in the patch.

Controls cover supported binary/Koblitz across n23-31 and47-61, m2-15, both twists; exact request and typed serde roundtrip; unsupported family/size/solver/zero capacities; pattern boundary; missing channels; invalid mixed v5; and old v1-v4 byte compatibility. Full solvers.rs or worker execution was not compiled. No Cargo/dependency rebuild, cloud job, scientific record or repository mutation occurred.

`ROUTING_EXPORT_CONTRACT.md` identifies the still-required dedicated replay caller and restricted export integration. This patch does not fix shared-map SAT-initial misrouting or bypass verification, and does not claim fully functional owned dataset export.

Apply types.rs.patch and solvers.rs.patch only after checking the frozen base hashes. Complete proposed files and original .base files are included for review; source bindings and test receipts preserve custody.

Replay from the packet directory:

```
timeout 60 bash -c 'ulimit -v 1572864; ulimit -t 60; rustc +1.98 --edition=2021 --test harness.rs -L dependency=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps --extern serde=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps/libserde-cac565bd9dd1312a.rlib --extern serde_json=/var/tmp/crypto-ml-search-recovery-target-20261107/release/deps/libserde_json-10f254ba1c7de3e0.rlib -o disposition-tests'
timeout 30 ./disposition-tests
```

The harness references this packet's complete types.rs by absolute path. Relocated replay must explicitly change that path and preserve both harness hashes.
