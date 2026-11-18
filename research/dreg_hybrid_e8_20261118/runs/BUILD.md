# Build provenance

- Source: origin/main 833a35d21ec2368ab081b2083d7762ed0bfd2bbc. That commit does not compile: `src/cryptanalysis/f4_fp.rs` has
  duplicate `VisitReport` fall-degree fields and a duplicate `recording_degree_falls`, from a merge.
- Applied only `build-f4fp-dedup.patch` (sha256 c73f69e6…d5f0), the `f4_fp.rs` hunk of open PR #4608. It deletes 18 lines in the
  F_p F4 module. `examples/dreg_closure.rs` and the GF(2) closure modules it uses are untouched.
- `cargo build --release --example dreg_closure --example dreg_ladder --example dreg_hybrid`, x86_64-unknown-linux-gnu, glibc 2.39,
  no target-cpu flags. Hashes in `binary.sha256`.
