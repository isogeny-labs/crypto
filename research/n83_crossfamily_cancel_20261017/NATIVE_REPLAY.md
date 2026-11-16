# Native arithmetic replay of the N83 crossfamily panel

The Rust [replayer](../../examples/n83_crossfamily_native_arithmetic_replay.rs)
independently verified all 24 fresh public N83 points, 96 recovered IC
scalars and four-point relations, and 96 paired same-point rho scalars from
the four raw archives of [run 39075872898](https://github.com/aburan28/crypto/actions/runs/39075872898).
Every ordered base dump matched its candidate manifest SHA-256 and base hash.
The replayer checked each IC online phase sum and paired target point. Its
[complete result](native_arithmetic_replay_39075872898.json) has status
`PASS_NATIVE_IC_AND_PAIRED_RHO_TARGET_REPLAY` and SHA-256
`ccceea9197e9da9fb8fab4667913bd54caa9e20c7d57be9c5dd841a9c398d414`.

The replayer source is SHA-256
`c2e3574651aefc786a5a6dfddfa48189ad8afe4172f3e5d397d2b0613d5bf09d`.
It requires the frozen public target and answer sidecars and the four raw
archive ZIPs with SHA-256, in stratum order:

| Stratum | Raw ZIP SHA-256 |
| ---: | --- |
| 0 | `036efe7a2e759524b7367544841e10a0ab3e0cf807580e03514ba4af5f3c968d` |
| 1 | `d42aa19364ac541810ac6b1ca0bf1aed4882c9006ab17b881142b59b3a3e87d1` |
| 2 | `cab308248b6abdc9ef6047bf905f6d0c648d749f1ac99289c25614eb49c2aa18` |
| 3 | `f72a1b2563cce8798271e316042eb7f016f54c8c7089ba99bc0f5f2ac60b33ec` |

Build with `cargo build --release --locked --example
n83_crossfamily_native_arithmetic_replay`, then invoke the binary with an
archive directory containing `stratum0.zip` through `stratum3.zip` and this
experiment directory as the fixture directory. The program streams each ZIP
hash, reads its exact members with `unzip -p`, and exits on the first failed
arithmetic or identity check.

The frozen four-arm selector over archived stratum analyses places K2816
first for supplementary cold-process cost among these four bases: 24/24
paired wins against each other arm, with mean cold times 334.291, 393.983,
577.128, and 586.219 seconds for K2816, K3072, K3450, and K3500. Its
primary online selector remains `BOUNDED_UNKNOWN`. This arithmetic replay
independently covers public points, IC relations, recovered scalars, and
paired rho target results. A separate
[native raw resource and calibration replay](NATIVE_SELECTION.md) now checks
all 768 archived events; the selector binds both replay digests. The
prospective batched-index and fresh small-intermediate panels remain separate
comparisons.
