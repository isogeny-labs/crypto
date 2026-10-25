# N83 K1107 fixed-root4 control

This control runs the existing public nearby-grid screen point on the
K1107 base with the same fixed four-byte root-index policy used by the
current-source K3500/K3596 root4 candidates. The IC producer source is
SHA-256 `58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8`.
The run verifies that the root4 policy preserves the exact K1107 base,
recovers the public point's scalar, and fits the high-K panel's 15.25 GiB
child RSS cap. It uses a 1,200-second process cap.

This is a shared local host correctness and capacity control. The existing
point is intentionally outside the fresh 16-point confirmation and bridge
fixtures. Its wall times are retained as diagnostics; they do not enter the
Tier 1 selection or a same-point rho ratio.

Build the native producer in a complete checkout, then run and independently
replay it from the repository root. The output directory must be new and
outside the repository:

```sh
CARGO_TARGET_DIR=/absolute/path/to/build-cache \
  rustup run 1.98 cargo build --offline --locked --release \
  --example koblitz_orbit_dlp_fast
python3 -m research.n83_root4_small_control_20261025.run \
  --binary /absolute/path/to/build-cache/release/examples/koblitz_orbit_dlp_fast \
  --output /absolute/path/to/new-output
python3 -m research.n83_root4_small_control_20261025.verify \
  /absolute/path/to/new-output
```

The producer and verifier pin the existing target and answer hashes, exact
source, root4 execution flags, base hash and ordered dump digest. A passing
control supports a later matched-index fresh-point bridge arm. A candidate
manifest and any Tier 1 performance comparison require a separate sealed
dispatch.
