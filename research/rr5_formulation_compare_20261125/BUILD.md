# Building `rr5-formulation-compare`

The RR5 native crate (`research/prime31_four_summand_20261122/native`) builds only against
the exact production sources pinned by its `f4-source/build.rs`. On the current branch,
the merge of main moved `src/cryptanalysis/ic_boundary.rs` and the pin refuses the build.
Build in a scratch tree from #5119's qualified head `e4c64edcf542b1551e28fd2f91bd12386144d3cc`:

```
C=e4c64edcf542b1551e28fd2f91bd12386144d3cc
git fetch --depth=1 origin $C
mkdir -p /tmp/q5119
git show $C:research/prime31_rr5_coprime_sectors_20261123/source-paths.txt > /tmp/q5119/paths.txt
git archive $C $(cat /tmp/q5119/paths.txt) crates/certified-relations crates/relation-replay-checker | tar -x -C /tmp/q5119
N=/tmp/q5119/research/prime31_four_summand_20261122/native
cp research/rr5_formulation_compare_20261125/native/{formulation_compare.rs,graeffe_jets.rs} $N/
# Add to $N/Cargo.toml, before the [[example]] block:
#   [[bin]]
#   name = "rr5-formulation-compare"
#   path = "formulation_compare.rs"
(cd $N && CARGO_BUILD_JOBS=3 cargo build --release --bin rr5-formulation-compare)
```

`source-paths.txt` omits three `certified-relations` files (`flat_map`,
`flat_constructor`, `packed_body.rs`), so the two crates are added whole.

The binary used for every run in `evidence/` had SHA-256
`62a5a43b6270dd275da3cd3e2314694bccacf650901de6bc0c5cf6466e033c66` (rustc 1.97.0).
