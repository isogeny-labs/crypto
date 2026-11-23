# Rebuild of f13a690df016d0a457bd40579fbb9d054f486adf (2026-11-14), measured

Recipe: image/build_layer_r2.sh's archive paths plus docs, tests/fixtures, research/aps_measurement_20261029 (as R2_READY.md step 1);
rustc 1.97.0 (2d8144b78 2026-07-07), --locked --release --no-default-features --target x86_64-unknown-linux-musl, RUSTFLAGS=-C target-cpu=x86-64, CC=musl-gcc. Build exit 0.

Rebuilt SHA-256 (all four equal image/image.json study_binary_sha256, so the rebuild is bit-identical to the frozen image's binaries):
```
f78f39599d75859a693d3177272e95cae79559eebb10c1ac528e0cb13e4fcee7  pkm_tower_pilot
7e8a67030e8db88ab06be468ee3b01f7163eb6b5aa80275e690cc71102df9e7a  prime_fall_degree_tools
c36d324e8a60b0d8e827982baf7e07e0ce155664c67ce8c11b4debc410b95c6e  hky_fp_tower_check
962833611e13e78a3dc5fbb9ab6bfc498fde9cccab255c704ed6e2ff12a03354  pfd_grid_zeros
```
Static check: pkm_tower_pilot static; prime_fall_degree_tools static; hky_fp_tower_check static; pfd_grid_zeros static; 
