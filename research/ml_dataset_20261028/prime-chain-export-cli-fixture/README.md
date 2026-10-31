# Prime-chain graph CLI export fixture

This is a bounded software correctness check of the actual `crypto dataset export`
command. It reuses two immutable n=7, m=3 prime-field cells (curve and twist)
from the previously archived startup integration fixture. It generates no new
research samples, runs no training, and launches no cloud worker.

Freeze `run.py`, `protocol.json`, and this document with `fixture-manifest.json`
before execution. Supply an independently built binary, its SHA256, and its full
source commit; the build receipt separately attests their relationship.

```sh
python3 research/ml_dataset_20261028/prime-chain-export-cli-fixture/run.py \
  --binary /absolute/path/crypto --binary-sha256 SHA256 \
  --source-commit FULL_COMMIT \
  --source-fixture /absolute/path/startup-main-cli-fixture-run \
  --scratch /absolute/path/new-empty-output
```

The five commands export each twist twice and reject the existing rehashed
semantic-corruption copy. Each export must reproduce its immutable manifest.
The Python consumer reconstructs polynomial maps from graph edges and compares
them against the retained original equations, checks exact decimal field values,
leaf/auxiliary domains, signed points, public feature hashes, restricted outputs,
and artifact hashes. Rust tests separately cover n=61, m=15, graph mutation,
resource refusal, private metadata isolation, and independent S3 evaluation.

Acceptance requires all five expected exits, both graph checks, unchanged source
inventory and binary hash, and rejection without an output manifest. Any timeout,
signal, unexpected exit, or assertion is a failed fixture; retain its output.
Each command is bounded to 60 seconds, 512 MiB address space, and 16 MiB per file;
total scratch output is polled at 64 MiB. These are fixture limits, not a general
worker admission contract. Elapsed times are diagnostics, not benchmarks.

Only local software checks are established by a passing receipt. This does not
establish independent ECDLP recovery, a complete solver execution trace, exact
instruction counts, fall-degree theorems, cloud startup, or cloud corpus coverage.
