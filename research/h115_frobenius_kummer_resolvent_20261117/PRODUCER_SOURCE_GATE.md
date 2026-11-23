# H115 producer source gate

Gate date: 2026-11-17

Status: **PASS** for strict compilation and the non-scoring structural
selftest. The candidate-scoring `--run` path has not been executed.

## Sources

| File | SHA-256 | Lines |
| --- | --- | ---: |
| `h115_sha256.rs` | `4d508fe8fed91c71e0150446c53f024ff97fa8a9519d812a672ea59ef2329786` | 167 |
| `h115_field.rs` | `577015b44fae9828b9bddf1b2afc0403e4f44d732b3a01325f42253f0982cc82` | 684 |
| `h115_producer.rs` | `f0644129c8d7c0161cbddac3283f94453983d25b6f6cdeaf327b170c1fc579bd` | 1,349 |

Compiler: `rustc 1.100.0 (0ef1bc386 2026-11-09)`.

Strict command:

```text
rustc --edition=2021 -D warnings h115_producer.rs -o h115_producer
./h115_producer --selftest
```

Both commands exited `0`. A source scan found no `TODO`, `FIXME`, `todo!` or
`unimplemented!` marker.

## Selftest receipt

```text
h115-fp7-a1-b1-r5-d4 modulus=[1,0,0,1,1] N=2475 cofactor=99 states=10 G=[0,1] T=[2009,1194] H=[2009,1194] torsion=25 kernel=5 moment_terms=5
h115-fp5-a2-b1-r7-d6 modulus=[1,0,0,0,1,1,1] N=15680 cofactor=320 states=6 G=[0,1] T=[14161,6938] H=[612,5192] torsion=49 kernel=7 moment_terms=7
h115-fp13-a2-b4-r17-d4 modulus=[1,0,0,1,1] N=28611 cofactor=99 states=14 G=[0,2] T=[7013,8633] H=[25744,15384] torsion=289 kernel=17 moment_terms=17
h115-fp31-a1-b19-r37-d4 modulus=[1,0,0,1,1] N=924075 cofactor=675 states=33 G=[0,9] T=[755239,33125] H=[384612,757240] torsion=1369 kernel=37 moment_terms=37
h115-fp47-a1-b38-r61-d3 modulus=[1,0,5,1] N=104188 cofactor=28 states=48 G=[2,1] T=[51403,71944] H=[65855,12311] torsion=3721 kernel=61 moment_terms=61
```

For every curve this checks the lexicographically first irreducible modulus,
field serialization/inversion/Frobenius, extension group order, corrected
`N/r^2` cofactor, deterministic transverse point, `r^2` distinct torsion
points, Frobenius characteristic equation, `(V-1)(pi-1)=[r]`, exact
image/kernel line, one nonzero-target fiber, base-field moments and all 648 ID
round-trips. It reads no scalar label and computes no score.

## Frozen run path present but unexecuted

The compiled `--run` path contains all 15 rows, 366 targets, 648 predicates,
random-line, rational-line, shuffled-label and coordinate-isomorphism controls,
raw/normalized ablations, the forbidden-oracle positive decoder, candidate
intersection and scalar-blind decoder. It refuses mismatched `CURVES.json` or
`PROTOCOL.json`, emits canonical JSONL, and hashes raw records before the
summary.

An independent verifier source and checkpoint/EKS gate remain required before
executing this path.
