# K2304 current-source full-process control

This control exercises the exact a1 K2304 bridge contender on fixture zero
from the earlier public N83 target set. Its candidate is
`IC1N83Ckb1a1fb382464PDP4rootRCguidedLAgaussTDdirectISO0h06c953ec903b`:
382,464 usable points in 2,304 signed-Frobenius columns. The producer uses
the candidate's dynamic five-byte root index at 75% load, with no Bloom
filter. The sealed cross-family bridge points are reserved for the later
Tier 1 paired comparison.

The runner pins the current producer source, local executable, candidate
manifest, public target, answer sidecar and all algorithm flags. It stops
after 1,200 seconds, at 12 GiB sampled child RSS, below 10% host free
memory, or after more than 1 GiB additional host swap. Its receipt retains
all attempts and hashes. This local run is a correctness control; its
process wall and target-stage timing are not a Tier 1 factor-base selection.

From the repository root, use the pinned locally built executable:

```sh
python3 -m research.n83_k2304_current_source_control_20261026.run \
  --binary /Volumes/SSD990-2/.codex/n83-fresh-cold-target/release/examples/koblitz_orbit_dlp_fast \
  --output /absolute/path/outside/repository/k2304-control --preflight
python3 -m research.n83_k2304_current_source_control_20261026.run \
  --binary /Volumes/SSD990-2/.codex/n83-fresh-cold-target/release/examples/koblitz_orbit_dlp_fast \
  --output /absolute/path/outside/repository/k2304-control
python3 -m research.n83_k2304_current_source_control_20261026.verify \
  /absolute/path/outside/repository/k2304-control \
  --binary /Volumes/SSD990-2/.codex/n83-fresh-cold-target/release/examples/koblitz_orbit_dlp_fast
```

The verifier checks exact file hashes, the published ordered-base prefix,
full column rank, every point in the four-summand witness, independent
scalar replay in the N83 group model, online phase arithmetic and the
sampled host envelope. Only a completed producer and passing independent
replay establish this control.

## Completed control

The bounded local run finished with `PASS_PROCESS_EXIT` in 438.548 seconds
of wrapper wall time and 5,926,551,552 bytes peak sampled child RSS. The
producer built the 2,304-column index, inserted 2,304 rank-gaining
relations, and completed the one public target. Independent replay returned
`PASS_INDEPENDENT_REPLAY`: the ordered base SHA-256 is
`f51e436c399688ebd710075f16a57cd631a6799ce79891d53250b46bfd6c4534`,
the recovered scalar is `3640978018804724`, and the verified four-point
target stage is 420.190916 ms. The producer reported 269,429.473125 ms
setup and 164,763.22625 ms relation-rank work. These intervals share a
single local process and are diagnostics for this control.

`evidence-01/` retains config, samples, process output, target certificate,
the checked macOS executable, receipt, independent replay and their hashes.
The [executed producer source](koblitz_orbit_dlp_fast.executed-20261026.rs)
is retained with SHA-256
`58e3ee492093de5f6b760b96e3aba819fa479cdecd22b1adee94f0ecb26026d8`
from commit `c1ed89ba343`. Independent replay checks this versioned source:
later changes to `examples/koblitz_orbit_dlp_fast.rs` do not alter the
recorded run. The runner still requires its frozen source hash when starting
a new process. The replay also compares the recorded algorithm flags with
the executed flag map in `verify.py`, so later changes to the cross-family
runner's flag defaults do not reinterpret this receipt.
The raw ordered base is
31,156,577 bytes; its deterministic gzip is 9,135,776 bytes with SHA-256
`3e0b7c9bad6a2abe2588a9f2b84d36f5fbe5991f3a6e0ef84f59135e3e40ec46`.
The gzip is indexed as
`store:evidence/n83-k2304-current-source-control-20261026/base_dump.jsonl.gz`;
`crypto store verify --remote` passed for its digest and size. Restore it as
`base_dump.jsonl` beside the compact evidence before rerunning the verifier.
A relocated package with the restored gzip and retained executable also
passed the independent replay. A forged scalar with a matching revised
artifact hash was rejected by the independent group replay. The original output is retained at
`/Volumes/data/cryptanalysis/n83_k2304_current_source_control_20261026_01`.

For a portable replay, copy `evidence-01/` to an output directory, then run:

```sh
crypto store pull \
  evidence/n83-k2304-current-source-control-20261026/base_dump.jsonl.gz \
  --out /absolute/path/copied-evidence/base_dump.jsonl.gz
gzip -dc /absolute/path/copied-evidence/base_dump.jsonl.gz \
  > /absolute/path/copied-evidence/base_dump.jsonl
python3 -m research.n83_k2304_current_source_control_20261026.verify \
  /absolute/path/copied-evidence \
  --binary /absolute/path/copied-evidence/koblitz_orbit_dlp_fast.macos-arm64
```
