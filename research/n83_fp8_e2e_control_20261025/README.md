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
python3 -m research.n83_fp8_e2e_control_20261025.seal \
  /absolute/path/to/new-output
```

Each child has a 1,200-second process cap and a 16,374,562,816-byte sampled
RSS cap. The run stops at the first failed child and records its partial data.
The output directory must be new and outside the checkout.

## Recorded full-process control

The [archived ABBA output](runs/n83-fp8-e2e-abba-20261025-1) contains the
executed binary, all four launch records, stdout, stderr, target records,
resource receipts, independent replay, candidate/workload registration, and
a 19-file SHA-256 archive manifest. Replay after relocation reports
`PASS_EXACT_FULL_PROCESS_PARITY`; altering an archived stdout file fails its
hash gate. The executed binary SHA-256 is
`a768b71f8059f0cf9049213ebc5bd2ed7ff9eccc3ab4947cff822093b58ef706`.
All four runs use 183,762 subgroup-usable base points, 1,107 folded columns,
101,708,868 indexed roots, final rank 1,107, and independently replay the
same scalar `2948503450338408` on the same public point. The sidecar adds
167,772,160 bytes of index storage.

The two exact candidate IDs are
`IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0h3962da813c7d`
for the packed-fingerprint baseline and
`IC1N83Ckb1a1fb183762PDP4rootRCguidedLAgaussTDdirectISO0h384942920658`
for the byte sidecar. The frozen single-point workload ID is `1b2e5e958684`;
the [sealed result](runs/n83-fp8-e2e-abba-20261025-1/sealed_result.json)
lists each candidate, workload, and run ID.

| ABBA arm | Full process (ms) | Index build (ms) | Rank stage (ms) | One-target online (ms) | Sampled peak RSS (MiB) |
| --- | ---: | ---: | ---: | ---: | ---: |
| Baseline 1 | 171,521.491 | 46,406.082 | 124,605.899 | 164.765 | 1,244.8 |
| Fingerprint 1 | 91,371.190 | 29,634.597 | 61,283.081 | 61.233 | 1,414.0 |
| Fingerprint 2 | 97,009.890 | 28,128.960 | 68,484.108 | 62.029 | 1,401.6 |
| Baseline 2 | 145,300.836 | 43,622.289 | 101,235.381 | 107.266 | 1,244.7 |

The geometric-mean baseline/fingerprint ratios are 1.677 for full-process
time, 1.558 for index build, and 2.157 for one-target online time. These are
descriptive local shared-host ratios. The two adjacent-pair ratios span
1.498–1.877 for full process, 1.551–1.566 for index build, and
1.729–2.691 for one-target online time; two pairs do not support a narrow
confidence interval. The baseline process time fell 15.3%
between bookends, and its one-target interval fell 34.9%. The latter drift
prevents a stable online timing aggregate under the Tier 1 gate. The raw
target records preserve the exclusive query, PDP, relation-check, descent,
and scalar-replay phases; their sums agree with each charged online interval
within 1 ms. The rank stage attempted one guided relation per column in
each run. The sidecar's next measurement is a calibrated Tier 1 same-point
IC/rho pair with these complete candidate manifests.

Replay the committed output directly:

```sh
python3 -m research.n83_fp8_e2e_control_20261025.verify \
  research/n83_fp8_e2e_control_20261025/runs/n83-fp8-e2e-abba-20261025-1
python3 -m research.n83_fp8_e2e_control_20261025.seal \
  research/n83_fp8_e2e_control_20261025/runs/n83-fp8-e2e-abba-20261025-1
```

## Matched-index nearby factor-base control

The next frozen local control alternates K1111 and K1115 in ABBA order, with
the same public point, source, binary, split-at-512 four-byte index, byte
sidecar, compact4 Bloom policy, thread counts and 1,200-second/16.37-GB
child caps. The two exact base hashes are
`f0c8213daff19553f3a3b450e89b5b8e85fb932d134c004ef89aeff89745b705`
and `95e970dcb7cbc5c299498409cd062749c49c6320175e83e0db7e635d906a594f`.
The [producer](run_base_abba.py) preserves every outcome, while the
[independent replay](verify_base_abba.py) checks the 166-points-per-column
base, full rank, scalar, and exclusive target phases. This local pairing
tests the factor-base choice under one index policy; a fresh Tier 1 panel
will supply the selection gate.
