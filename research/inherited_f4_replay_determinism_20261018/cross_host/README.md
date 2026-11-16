# Cross-host inherited-F4 node dump (aarch64 vs x86-64)

Purpose: find the first solver node at which the inherited-F4 word-XOR count
(`relations.solver_ops`) of the Apple M4 Pro record (`ECBS1hfd5bf1885227`,
aarch64) departs from x86-64, so the cause of the drift in
`../README.md` can be localised. The x86-64 side is already done and
committed here; the owner runs the same script on the Mac.

## What is pinned, and why

- **Commit `2245b7596f3fe04586c33b53a7cac97185fadba2`** (the archived session's
  `git_commit`, `git_dirty: false`). `KIC_F4_NODE_DUMP` exists there
  (`dump_node_system`, called from `reduce_inherited`), so no other commit is
  needed. The spec is the committed
  `docs/ecbench/specs/claim-koblitz-rootfind-n13.json`, sha256
  `62fdb5ae...1d4`, byte-identical to the archived `spec.json`.
- **A small patch (about 40 added lines, two files) is applied on top** (`node-dump-compact.patch`, embedded in
  `mac_dump.sh`). Two reasons, both found by running it:
  1. The existing `KIC_F4_NODE_DUMP` writes the node *system* only, with no word
     count, so it cannot by itself show per-node word counts. The patch adds
     `KIC_F4_NODE_DUMP_COMPACT=1`: per node it writes the index, an FNV hash and
     size of the system, and the word-op counter (`w_total`, `w_thread`) at node
     entry. Node i's charge is `w[i+1]-w[i]`; the last node's is
     `solver_ops - w[last]`. With the flag unset the dump is unchanged.
  2. `pdp3-koblitz` refuses to run with any `KIC_F4_*` variable set (it would
     change the method identity), including `KIC_F4_NODE_DUMP`. The patch exempts
     `KIC_F4_NODE_DUMP*` (diagnostic file output only; the search is unchanged).
  The patch changes no counted quantity: on x86 the patched build reproduces the
  recorded numbers (below).
- **Runs**: `ecbench run` clears the child environment, so the dump variable
  cannot reach it. The script instead exports each F4 run's exact child input
  with `ecbench profile-input --seq N` and executes `ecbench exec` itself
  under the runner's child environment (`env -i`, `LC_ALL=C`,
  `RAYON_NUM_THREADS=1`, `OMP_NUM_THREADS=1`, `RUST_MIN_STACK=8388608`). Same
  inputs and algorithm seeds as the session. Subset: **all 64 runs of arms
  `ic-f4-m2` and `ic-f4-m3`** (4 rounds x 8 targets x 2 arms, warm-ups
  included; seq 58 is `ic-f4-m2`, target 0, round 1, 300,531 words on Apple).
  The whole run set takes about 13 s; the build dominates.
- **Toolchain**: rustc 1.93.1 (the Apple record's) via `rustup run`; pass
  `--install-toolchain` to let the script install it. Without it the script
  uses the default toolchain and records `NOT PINNED` in the output.

## What the owner runs (macOS, Apple silicon)

```bash
# once: Xcode command line tools and rustup
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. "$HOME/.cargo/env"

# get the script (any checkout of this branch has it; or `curl` the raw file)
cd /path/to/crypto
bash research/inherited_f4_replay_determinism_20261018/cross_host/mac_dump.sh \
     --work "$HOME/f4-xhost-work" --out "$HOME/f4-xhost-out" --install-toolchain

# copy the two files back and commit
cp "$HOME"/f4-xhost-out/node-dump-aarch64-macos-2245b75.jsonl.gz \
   "$HOME"/f4-xhost-out/host-aarch64-macos.json \
   research/inherited_f4_replay_determinism_20261018/cross_host/

# compare against the x86-64 dump
cd research/inherited_f4_replay_determinism_20261018/cross_host
python3 compare.py node-dump-x86_64-linux-2245b75.jsonl.gz \
                   node-dump-aarch64-macos-2245b75.jsonl.gz --json compare.json
```

The script clones nothing from your working tree: it fetches exactly the pinned
commit from `https://github.com/aburan28/crypto` into `--work` (needs network),
builds `ecbench` in release mode there, and deletes the build dir at the end
(`--keep-build` keeps it). Re-running is safe: the checkout is reset to the
pinned commit and the patch re-applied. It fails loudly on a missing tool, a
fetch or patch failure, a spec hash mismatch, or a missing `solver_ops`.

Expected cost: a cold release build of the crate is about 5 to 9 minutes on 4
x86 vCPUs (an M4 Pro should be faster), about 7 GB of disk for the build
(freed at the end), under 1 minute for the runs; the output is about 1.5 MB.
Apple's `rustc` here is the Homebrew 1.93.1 in the record; the pinned rustup
1.93.1 is the same release (not necessarily the same build).

Files that come back: `node-dump-aarch64-macos-2245b75.jsonl.gz` (one gzip of
JSON lines: one `meta`, 64 `run`, one `node` per solver reduction) and
`host-aarch64-macos.json` (uname, CPU, rustc, cargo, commit, patch and spec
hashes, dump sha256 and line count). `compare.py` is stdlib-only plumbing that
diffs two dumps; it computes nothing that is counted. Exit status 1 means a
divergence was found.

Optional second check on the Mac: `solver_ops` in the dump's `run` lines should
equal the archived record for the same seq (e.g. seq 58 = 300,531). If they do,
the dump reproduces the Apple record, and the comparison is like for like.

## What the comparison decides

For each run, `compare.py` reports (a) the first node whose system hash or size
differs, (b) the first node whose entry word counter differs, and (c) the
culprit node: the one whose own reduction charged a different number of words,
with its size.

- **(a) fires**: the splitting trees are not identical after all (contradicts
  the equal `splits`/`propagations` counters); the divergence is in node logic
  or in its inputs, and the node is named.
- **(b) fires, (a) does not**: same node systems, different charge on one node.
  That node's system is then dumped in full (rerun `ecbench exec` by hand for
  that seq with `KIC_F4_NODE_DUMP` set and `KIC_F4_NODE_DUMP_COMPACT` unset,
  on both hosts) and reduced under each host's primitives; the first
  differing word count localises the cause to a primitive (specialisation,
  completion, a sort tie-break, `pclmulqdq` fold vs table).
- **Nothing differs**: the Mac at this commit reproduces x86; the Apple record
  then comes from a dirty or different binary (the record's `host.json` says
  `dirty: true`) or from rustc, and the Mac rebuild of `2245b75` is the
  control that separates them.

## x86-64 side (done, committed here)

| | |
|---|---|
| File | `node-dump-x86_64-linux-2245b75.jsonl.gz` |
| Commit | `2245b7596f3fe04586c33b53a7cac97185fadba2` plus `node-dump-compact.patch` (sha256 `d5496214a318a23f0e30d34a47011d1ddaf4d02be4c634b226a371e9a84425e7`) |
| Toolchain | rustc 1.93.1 (01f6ddf75 2026-02-11), cargo 1.93.1, pinned via rustup |
| Host | Intel Xeon 2.10 GHz VM, Linux 6.18.44, 4 vCPU (`host-x86_64-linux.json`) |
| Runs | 64 (`ic-f4-m2`, `ic-f4-m3`, all seqs, warm-ups included) |
| Lines / size | 58,485 lines (1 meta, 64 run, 58,420 node) / 1,485,344 bytes |
| sha256 | `718b32ef11ee79d4c3dc2f55c96a14316bcee4635bd7678b4c440cc016614837` |
| Wall | 13 s for the runs; the build took 4 min 46 s (second, warm-cache-free rebuild after a patch re-apply; the first build was 8 min 44 s) |

Produced by exactly `mac_dump.sh --install-toolchain` (the same script, on
Linux). Note the host differs from the one in `../README.md` (AMD EPYC 9V74
there); counts do not depend on the CPU model.

Sanity checks against recorded numbers, 2026-10-18:
- Seq 58 (`ic-f4-m2`, target 0, round 1): 301,961 words, 114 nodes; equals
  the x86 value in `../counters.json` (`t1`) and the example in `../README.md`;
  the last node's entry counter equals the run total.
- All 24 non-warm-up `ic-f4-m2` runs equal `counters.json` `t1` (24/24, a
  different binary: current main, rustc 1.97.0), so the patched pinned build
  reproduces the x86 numbers.
- Warm-up target 0: `ic-f4-m2` 419,828 and `ic-f4-m3` 10,305,435, equal to the
  `2245b75`-on-x86 figures quoted in `../README.md`. The non-warm-up `ic-f4-m3`
  runs are 0/24 equal to `t1`, as expected: m3 differs between `2245b75` and
  main (`../README.md`).
- `compare.py` run on the x86 dump against itself: 64/64 equal; against a copy
  perturbed by 7 words from node 40 of seq 58, it reports culprit node 39 with
  27 versus 20 words.
