# Replay and custody

Use the committed `.artifacts.json` pointer and the official artifact tool to fetch `proof-review.tar.gz`; `custody.json` supplies its durable URI, SHA-256 and size. Verify every extracted regular file against `archive-members.json` after removing the `review/` member prefix. The archive retains all ten invocation sources, stdout/stderr, compiled outputs, private precursor snapshots and complete pre/post input inventories.

The exact bounded invocation was `timeout --signal=TERM --kill-after=5s 295s python3 -B <this packet>/runner.py`. The runner applies 8 GiB address space, a 10 MiB OS stack, Lean `-j1 -M3500 -s16384`, and one fresh workspace. Its source revision is fixed; shared toolchain/cache locations and hashes are recorded in the archive. Replay is a new attempt and must receive its own planning scope; do not overwrite this evidence.

The runner and sources were preserved as used. Archive readback and visualization do not repeat Lean compilation. The producer's three attempts, including both earlier failures, are unchanged in its own packet.
