# Backend report revision before the method-registry additions

The adjacent `reproduction.lock.json` preserves the report lock before the
`rho.plus_model_v1`, `rho.best2_model_v1`, and `ic.known_log_mitm2_v1` method
declarations entered current `methods.rs`. Its SHA-256 is
`5385e70d724341e07922412b007b42e0ce685fde2c3c68f462bc7dd6f8721278`.
The corresponding generated `index.html` remains in Git at commit
`144c813726add2b8a5d85e6e6e9a52fa6dd9a5e5`, blob
`62702995c16e7f3d34c6bd8b27fb932979d53f46`, SHA-256
`aa9964c2e7ba7e785c4aec3ede6748041106041a4d7ef404f5738153311981d0`.
Reproduce that historical report at the pinned commit with the lock there;
the current tree's method registry has different source bytes.

The current report was generated with Rust 1.99.1 using the native
`backend-leaderboard --seal` producer after the registry additions. A
structured comparison of the old and new embedded report data found one
changed leaf: `producer_input_hashes/src/cryptanalysis/ecbench/methods.rs`,
from `b35fc57c267210657521e40f80fdca7a651d800b8868c34f67c33633e378a5dc`
to `9fc4f7cd32bb520dcdaaf1af67aa82c734e53a36fbe9410addd9c78603b814cc`.
The session counts, entries, frontiers, and timing verdicts are unchanged.
The current lock SHA-256 is
`9a93e4224835be898394bd1284966db738282a85f431c26aea55731575600f9c`.
This revision records report provenance only; it contains no fresh solver run.

## Complete archived outputs restored (2026-10-28)

All five original report outputs now accompany this lock. Their bytes match
the unchanged lock and snapshot 91424ecf8cc7db04e7c4a2805c9001a02a6106ca.
The original leaderboard JSON uses its existing content-addressed object;
the other outputs reuse their original Git blobs. The historical checker
reads this complete archive while continuing to verify current scientific
session inputs against the old lock. Current report revisions keep their
own provenance. A native reproduction at the pinned snapshot passes without
a solver run or any timing-reproduction claim.
