# Raw primary-run files kept out of Git

`runs/n17.jsonl`, `runs/n19.jsonl` and `runs/n23.jsonl` are over the artifact-store policy
limit (64 KiB of data). The store push needs the `aws` CLI, which the producing container
lacked, so they are not committed. Their SHA-256s are in `SHA256SUMS`. They are deterministic:
`native/target/release/dspec <n> runs/n<n>.jsonl` (no DSPEC_* variables) regenerates them
byte-identically. This was checked for n = 17 on the producing host. Every scored value is in
`SCORE.tsv`, which is committed. A later session with store access should
`python3 tools/artifacts/artifacts.py push` them.

`reports/run-20261116/spectrum.png` is likewise not committed. The same figure is committed as
`spectrum.svg` and `spectrum.pdf`; `python3 plot.py` regenerates it.
