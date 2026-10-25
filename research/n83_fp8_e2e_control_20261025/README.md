# N83 K1107 fingerprint sidecar full-process control

The split four-byte root index supports an optional one-byte fingerprint
sidecar. This experiment compares the complete K1107 IC process with and
without that sidecar in ABBA order on the same existing public point. The
source is frozen at SHA-256
`05b64717b10e17c8cd09e8a6823e6efec600ac4b53c8a0e8e0476771c702cb7b`.
The only algorithm flag changed between arms is
`KIC_INDEXED4_FINGERPRINT8`. All four processes reconstruct their bases,
indexes and factor-base logs before solving the point.

The control checks exact base identity, final rank, independently replayed
scalar and the five exclusive online phases. The source's process and
precomputation phase timers provide supplementary cold-start data. Every
failure and cap remains in the result. This local shared-host control is an
algorithmic diagnostic; Tier 1 comparison requires Pod resource and noise
receipts, plus a matched same-point rho run.

From a complete checkout:

```sh
CARGO_TARGET_DIR=/absolute/path/to/build-cache rustup run 1.98 cargo build \
  --offline --locked --release --example koblitz_orbit_dlp_fast
python3 -m research.n83_fp8_e2e_control_20261025.run \
  --binary /absolute/path/to/build-cache/release/examples/koblitz_orbit_dlp_fast \
  --output /absolute/path/to/new-output
python3 -m research.n83_fp8_e2e_control_20261025.verify \
  /absolute/path/to/new-output
```

Each child has a 1,200-second process cap and a 16,374,562,816-byte sampled
RSS cap. The run stops at the first failed child and records its partial data.
The output directory must be new and outside the checkout.
