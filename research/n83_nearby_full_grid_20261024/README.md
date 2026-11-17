# Exact current-source N83 nearby factor-base grid

The ordered K1120 producer dump fixes every complete signed-Frobenius orbit
prefix from K1102 through K1120. `derive_grid.py` reconstructs all 19 exact
factor-base candidate manifests from that dump. It checks the producer's
BLAKE3 update sequence, the sorted point-set digest, the compact raw dump,
and the root-slot sizing. The reconstructed K1104, K1107, and K1120 dumps
match their independently published producer dumps byte for byte, and their
candidate manifests match the published records.

The 16 intermediate bases have `DERIVED_VERIFIED_PREFIX` identity status in
`catalog.json`; direct process construction and performance measurements are
pending. `selected_best_total_runtime` stays null. This grid supplies exact
identities and reproducible inputs for a fresh same-point Tier 1 comparison,
including failed attempts and same-point rho controls. The existing five-base
16-point panel and its independent replay still govern the wider selection.

From the repository root, after installing `requirements.txt` into an isolated
Python environment:

```sh
python3 -m research.n83_nearby_full_grid_20261024.derive_grid
```

The command recomputes all manifests and fails on any changed artifact.
`--write` is used only when regenerating the versioned identities from the
pinned K1120 parent.
