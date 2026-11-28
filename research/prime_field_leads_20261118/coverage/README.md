# Disjoint APS label-law check

The compact schedule [1,1,1,2] retained a nonzero recovery denominator on every tested one-per-slot tuple. The source-slice runner matched each baseline sample to the pinned production sampler and matched every complete Split4 expression root set to a separate affine group oracle. This is local correctness evidence. [pcc:unverified]

## Evidence

Both schedules used the same raw StdRng draws by index. A schedule changes the Q shift and can alter duplicate and infinity rejection, so actual coordinates, slot assignment by draw index, and recovery outcomes can differ. Each fixture used sizes 2 and 3 per slot and four seeds, giving 16 paired cells. The affine oracle checked 1,552 one-per-slot tuples and 12,416 signed cases modulo global negation. Every Split4 root on these fixtures was informative. [pcc:unverified]

| Schedule | Completed samples | Raw draws | Informative roots and group incidents | Samples with a verified scalar witness |
|---|---:|---:|---:|---:|
| [1,2,4,8] | 16 | 179 | 31 | 11 |
| [1,1,1,2] | 16 | 180 | 37 | 10 |

Seven pairs recovered on both arms, three on compact only, four on baseline only, and two on neither. All six cap controls exhausted, and all 32 deliberately corrupted labels were refused. The full draw trace, signed witnesses, and per-arm accounting are in [evidence.tsv](evidence.tsv). The same copied release executable produced byte-identical evidence on a fresh process replay. [pcc:unverified]

## What the valuation rule certifies

For one point from each fixed partition, canonicalization changes only the signs of the four weights. Both schedules satisfy the bridge rule. There are three minimum 2-adic valuations for [1,1,1,2], and one for [1,2,4,8]. Both counts are odd. Each signed weight sum is therefore odd as an integer, and the sums of the absolute magnitudes are at most 5 and 15 respectively. When the subgroup order exceeds the corresponding bound, every such sum is nonzero modulo the subgroup order. The compact schedule has the smallest possible positive integer total for this invariant with four slots. Total four forces all weights to be one, permitting two positive and two negative signs to cancel. [pcc:unverified]

Once a tuple can take multiple points from the same original slot, the invariant depends on its actual multiset. For example, four weight-one points can cancel, and two weight-one points and two weight-two points can cancel. The bridge rule is sufficient and treats the remaining tuples as uncertain. The audit compares it to exact 16-sign denominator checks on every four-element subset of the complete base. [pcc:unverified]

| Points per slot | All four-subsets | Baseline valuation certain / exactly safe | Compact valuation certain / exactly safe |
|---|---:|---:|---:|
| 2 | 70 | 48 / 56 | 40 / 40 |
| 3 | 495 | 333 / 387 | 261 / 261 |

The core sampler uses selected fixed weights and the core recovery always checks the actual denominator. The bridge valuation rule lives in a separate experiment and certifies both layouts on the one-per-slot domain. Recoloring, merging, and block-symmetric layouts must still test the actual multiset or preserve one point per original slot. Keep the production law as the baseline. Compact weights are a bounded disjoint-layout candidate. [pcc:unverified]

For independent uniform draws of u, translating uG by wQ is a permutation of the subgroup for every w. Conditioned on the previously accepted x-values, canonicalization therefore has the same uniform next-abscissa law under either schedule. The observed draw and recovery differences provide a check on specific coupled streams. A candidate cost benefit would require separately testing setup, such as reusing Q and 2Q across slots. This run intentionally counted four shift calls per arm and did not measure a cost difference. [pcc:unverified]

## Replay and boundaries

The [protocol](PROTOCOL.md) predates the main run. The first root-crate check stopped during dependency resolution because this sparse worktree omitted crates/certified-relations. The frozen [amendment](PROTOCOL_AMENDMENT.md) defines the successful isolated build. It incorporates exact source ranges for FastCurve, the APS sampler and recovery, modular helpers, and S3 expression construction. The build checks the three input hashes and omits unrelated stock modules. It evaluates the Split4 equations on the complete discrete domain with a finite-field auxiliary scan. Production F4, runtime measurements, and scaling are outside this result. [pcc:unverified]

From this directory, with the pinned source files available under the enclosing repo:

```sh
cargo build --release --locked
cp target/release/aps-disjoint-label-law-check /tmp/aps-disjoint-label-law-check
chmod a-w /tmp/aps-disjoint-label-law-check
(ulimit -v 1048576; timeout 120 /tmp/aps-disjoint-label-law-check) > /tmp/aps-label-evidence.tsv
cmp evidence.tsv /tmp/aps-label-evidence.tsv
```

Set APS_PROBE_SOURCE_ROOT to a separate checkout of the pinned source if necessary. The source hash check covers absolute line slices. The results require an unchanged arithmetic source anchor and the checked-in dependency lock.
