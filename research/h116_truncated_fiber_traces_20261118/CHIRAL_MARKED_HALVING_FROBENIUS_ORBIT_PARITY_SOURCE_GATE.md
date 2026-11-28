# H116 chiral marked-halving Frobenius-orbit parity source gate

Protocol `H116-CHIRAL-MARKED-HALVING-FROBENIUS-ORBIT-PARITY-001`, ordered
source/dual halving fibers, complete Frobenius cycle types, three frozen
predicates per fiber, all 600 single/pair-xor rules, binary recursion,
inherited branch/resultant/genus-one/marked-isogeny controls, independent
degree-four permutation and character classifications, shuffle statistic,
sources and launch manifest were frozen before evaluating any orbit predicate
against a scalar label.

```text
57787057949a9a89cf4bcb0f6b14013a502fecd7ea890253fb81ccfe7b080c42  CHIRAL_MARKED_HALVING_FROBENIUS_ORBIT_PARITY_PROTOCOL.md
4b8c34783642456765e308fcc9992b5c3d4f1cd8acc8161fdbb4b57263b28d11  CHIRAL_MARKED_HALVING_FROBENIUS_ORBIT_PARITY_LAUNCH.json
d69459fee56de959a2638024e27033b0062abeec68cb3a114616cd5de14f341a  h116_chiral_branch_residual_parity.rs
e5f508ffb20586f500b0c54c8f6529a6157893a339f9dc42cdd5356c689ecdc8  h116_producer.rs
577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82  h116_field.rs
2bafe271f1b2f6e6eb1a100f672f81e2b16bbe30b6db29ac579c0740dfe9eddf  ../../src/hash/sha256.rs
```

Pre-run build gate:

```text
h116 chiral branch residual parity selftest: pass rules=512 resultant_rules=812 trace_rules=243 invariant_rules=512 kummer_rules=512 orbit_rules=600
1ed3293e74f9c3d92b2b2744e899dd2b0456f649c3ecd95a72cf463970ab6dad  /tmp/h116_chiral_marked_halving_frobenius_orbit_parity
```

The source compiled with pinned Rust 1.100.0 under
`--edition=2021 -O -D warnings`.  No target orbit tuple, selected predicates
or shuffle score existed when this gate was written.  The first permissible
run is from the first pushed commit containing these exact bytes.
