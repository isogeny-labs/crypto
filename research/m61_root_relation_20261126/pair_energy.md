# Nontrivial root-base pair-sum energy

Run from the repository root:

`cargo run --release --offline --manifest-path research/m61_root_relation_20261126/native/Cargo.toml --example root_pair_energy`

The standalone Rust code uses `E:y²=x³−3x+40` over three small Mersenne primes. For each root-of-unity x basis and 16 deterministic equal-size random-x bases, it forms all `4*C(B,2)=2B(B−1)` signed sums using **distinct base indices**. A nontrivial collision is an equality between signed pair sums whose four base indices are all different. Thus pair order, duplicate input pairs, and antipodal cancellations are excluded. Every tested pair costs one curve group addition; base generation and square roots are separate preprocessing and are not included in the addition count.

| p | root order | B | pair additions per base | root pair collisions | root disjoint collisions | random mean disjoint (16 bases) | root distinct sums | random mean distinct |
|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 8,191 | 42 | 20 | 760 | 28 | 24 | 28.12 | 732 | 729.69 |
| 131,071 | 102 | 54 | 5,724 | 120 | 114 | 112.12 | 5,604 | 5,608.25 |
| 524,287 | 171 | 79 | 12,324 | 150 | 150 | 135.75 | 12,174 | 12,185.62 |

Maximum bucket size is two in each root case. Across toy fields, root-indexed bases show no conspicuous enrichment of disjoint pair-sum collisions over controls. This probes one simple algebraic compression mechanism; it cannot exclude other structure or an implicit algorithm.

For the selected M61 root basis, B=2,447. A deterministic sample of 100,000 signed pair draws in each of root and random bases used 100,000 group additions. Each had 99,601 unique canonical input pairs, 399 repeated draws, and **zero collisions among distinct inputs**. Under uniform sums, the expected collision count for the sample is about `2.2×10⁻⁹`; zero is therefore almost certain and is not evidence against rare structure. Even the complete `2B(B−1)=11,970,724` distinct-index signed pair inventory would have only about `3.1×10⁻⁵` pair-sum collisions under the uniform model. Obtaining even one four-index relation from pair collisions would require a substantial structural enrichment absent from these toy measurements.

The toy B values exceed their p^(1/6) scale, so their absolute collision counts must not be extrapolated directly to M61. The M61 sample cannot measure a collision rate this low; its value is confirming the implementation's no-duplicate accounting and the exact addition cost.
