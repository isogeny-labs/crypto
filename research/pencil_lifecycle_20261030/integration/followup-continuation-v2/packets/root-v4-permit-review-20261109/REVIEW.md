# Root v4 permit source review

The corrected source passes this scoped source/protocol review. The worker retries exactly once only when a valid identity-matched permit has expired. It retains the original 10-second permit, 15-second lease margin and final wall-time expiry check. The parent verifies an attempted launch, current invocation and nonempty matching cgroup, then atomically publishes before cumulative checkpoint flushing. Cleanup, interruption and terminal paths do not mint a permit.

All three initial findings are resolved. Initial and corrected source snapshots remain retained. The prospective qualification has 24 fixed software cases including changed/missing cgroup, prelaunch ownership, expired second read crossing wall time, identity errors, bounded retry and publication ordering. No qualification or compiler was executed by this reviewer; the coordinator retains execution authority.

Reviewer disclosure: renew_materializer authored the v3 ancestor, but did not author this repair. The prospective review plan preceded inspection. The Rust candidate, patch, inputs, cache snapshotter and durable I/O helper are byte-identical to v3. The four ordered controls and full library invocation are unchanged.

The admission draft remains disabled. A fresh post-v3 mutable-cache inventory is required because v3 compilation changed the private cache. The immutable materializer manifest remains c5178d2138d79f1ea8cac1fc96cb8d968913ba403d68ee66fa36afdd60fdf3f2. Actual qualification result, cache receipt, ledger, lease and publication readback must bind the final admission. No runtime latency improvement is inferred from this source review.
