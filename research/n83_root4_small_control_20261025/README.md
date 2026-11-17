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
python3 -m research.n83_root4_small_control_20261025.seal_candidate \
  /absolute/path/to/new-output
```

The producer and verifier pin the existing target and answer hashes, exact
source, root4 execution flags, base hash and ordered dump digest. A passing
control supports a later matched-index fresh-point bridge arm.
`seal_candidate.py` emits an exact candidate manifest only after independent
replay passes, combining the K1107 base identity with the K3500 root4
policy and changing the relation rank and column count to 1107. Any Tier 1
performance comparison requires a separate sealed dispatch.

## Recorded control

The source-frozen run at commit `5e99e42bc819832eb60e8f38f5c8ffea1572eac3`
completed on macOS ARM64. Its [archived raw files](runs/n83-k1107-root4-control-5e99e42-20261025)
replayed after relocation, including the compressed ordered base dump and
executed binary. The archive's seven raw files match
`archive_manifest.json`; `independent_replay.json` reports
`PASS_INDEPENDENT_REPLAY`.

| Control field | Result |
| --- | ---: |
| Actual subgroup-usable base points / folded columns | 183,762 / 1,107 |
| Distinct indexed roots / fixed root-table allocation | 101,708,823 / 8 GiB |
| Standard Bloom allocation | 1 GiB |
| Final relation rank / verified target count | 1,107 / 1 |
| Peak sampled child RSS / cap | 9,941,991,424 / 16,374,562,816 bytes |
| Full process / one-target online interval | 438,910.193 / 46.294 ms |
| Base hash / ordered-dump SHA-256 | `b0aea97c1c8b9b46e13df67e6af9c33ca73034d2ec3eff33a2392ceb1452b40e` / `08608f6ad2b8c66510246348933740c78d0937cae783d521514f467940edc3dc` |

The exact complete-pipeline ID is
`IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0h1a837bb63bd5`.
Its [manifest](candidate_IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0h1a837bb63bd5.json)
has SHA-256 `1eee8aa89fe17728a1424f8d657cc21014c0fb107b889bcc920e31cddbd1e16e`.
The fresh-point matched-index comparison remains to be dispatched.

Replay the committed archive directly:

```sh
python3 -m research.n83_root4_small_control_20261025.verify \
  research/n83_root4_small_control_20261025/runs/n83-k1107-root4-control-5e99e42-20261025
python3 -m research.n83_root4_small_control_20261025.seal_candidate \
  research/n83_root4_small_control_20261025/runs/n83-k1107-root4-control-5e99e42-20261025
```
