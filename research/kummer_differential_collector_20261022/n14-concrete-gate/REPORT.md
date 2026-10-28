# KUMMER-N14: concrete model extension

The [frozen protocol](PROTOCOL.md) sampled 1,029 signed states from each of
the 18 remaining odd-prime models. The native replay ran 18,522 sampled
states, called the actual cached theta xADD source three times per state,
and compared all outputs with independent Jacobian arithmetic. All 55,566
raw and repaired xADD outputs matched; no torsion repair was needed in this
sample. This is a finite-state result, not an all-state exception bound.

| Subgroup bits | Models | States | xADD calls | xADD null or mismatch | Correct signed lifts | Final no-divisor lifts | Lift M | Lift A | Lift I |
| ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| 20 | 3 | 3,087 | 9,261 | 0 | 3,070 | 17 | 1,073,281 | 866,417 | 58,586 |
| 24 | 3 | 3,087 | 9,261 | 0 | 3,078 | 9 | 1,073,664 | 866,784 | 58,608 |
| 28 | 3 | 3,087 | 9,261 | 0 | 3,078 | 9 | 1,073,592 | 866,700 | 58,608 |
| 32 | 3 | 3,087 | 9,261 | 0 | 3,078 | 9 | 1,073,592 | 866,700 | 58,608 |
| 36 | 3 | 3,087 | 9,261 | 0 | 3,078 | 9 | 1,073,592 | 866,700 | 58,608 |
| 40 | 3 | 3,087 | 9,261 | 0 | 3,078 | 9 | 1,073,592 | 866,700 | 58,608 |
| **total** | **18** | **18,522** | **55,566** | **0** | **18,460** | **62** | **6,441,313** | **5,200,001** | **351,626** |

All 18 selected theta tuples have nonzero `A,B,C,D`, and both `K(0)` and
`K(G)` have four nonzero coordinates. These are exact checks on the frozen
model constants and fixed differences. The sampled xADD outputs were all
nonzero, but the all-state xADD exceptional locus is not bounded. The 62
final no-divisor lifts comprise 56 `general_conversion_inapplicable`, five
`Delta_zero`, and one `b1_coefficient_zero`; no sampled fallback succeeded or
performed a Jacobian enumeration. Every model includes the scalar failures
`k=0,1,r-1`. Two 20-bit models have eight additional sampled failures in
total. Their exact scalar IDs are in the raw rows.

The lift M/A/I counts above are the signed-recovery module's field-operation
counters after one-time cached theta inverse setup. Jacobian fallback
counters were zero in this sample. Labels, setup, collector, relation
processing, final target verification, elapsed time, and rho were not
measured. No end-to-end or rho cost ratio is available from this packet.

The [per-state CSV](results/states.csv) retains each scalar, projection and
xADD status, repair root, signed-lift status and correctness, and all module
and Jacobian field counters. The [JSON summary](results/summary.json) keeps
the denominator guards, model counts, source/input SHA-256 digests and CSV
digest. The native validator independently recomputes the frozen sample set,
all counts, cost totals and digests. The [graph](status-graph.svg) and
[PDF](REPORT.pdf) present the nulls by model. N13's exhaustive 16-bit
packet remains [separate](../n13-concrete-gate/REPORT.md).

The CSV, PDF and graph PNG are in the repository artifact store under the
adjacent `.artifacts.json` manifests. Hydrate them from the root before
validation:

```sh
python3 tools/artifacts/artifacts.py pull research/kummer_differential_collector_20261022/n14-concrete-gate
cargo run --offline --release --manifest-path research/kummer_differential_collector_20261022/n14-concrete-gate/Cargo.toml --bin validate
```

Host: migrated Spot worker; local process; no timing isolation claim.
Source basis: `78c3189cc69` plus the frozen protocol and this packet.
This packet did not launch EKS, Diem, or a target cohort. An exact concrete
all-state bound, complete signed-lift fallback, and a matched 1,260-target
collector/rho cost comparison remain open.
