# Amendment A1: accepted native-only execution path

Date: 2026-11-23. Parent: crypto PR #5049.

## Why a new attempt is required

Attempt-01 used Python to select source byte ranges, orchestrate the native
build/run, and summarize the Rust output. No field arithmetic or curve
calculation ran in Python, but that orchestration and research-result
summarization conflict with the repository's current no-Python instruction.
Its raw result log is also 97,937 bytes, above the 64 KiB publication limit.
Attempt-01 remains frozen and is not accepted as the publication execution
path. Do not overwrite, delete, relabel, or rename its evidence to conceal
either issue. Its source and result hashes remain in the custody note.

## Scope and retained checks

Use the same immutable production source revision
`bd20b95dce44a91d7f74a1367853e724f280a3b6` and the same unchanged 6,660-byte
selected `production.rs`, SHA-256
`c6dff02843fceab1cf221465056598e6cd94d3d29a350b72eed85bee4c02c56c`.
This amendment changes orchestration and prospective output volume, not the
selector or field arithmetic. The original source is not modified.

The accepted native qualifier keeps every predetermined check from the
original protocol: depths k=1..12 use the production constructor; k=0 is a
separate mathematical control while production's k=0 rejection is retained;
distinct root counts, terminal zero, nonzero derivative, norm-one lifts,
Frobenius conjugation, exact 2^(k+2) order, trace doubling, target curve
membership, and the standard odd-power recurrence are checked for every
root. Target: p=2147483647, a=p-3, b=21. Depth30, ramified fibre, unsplit
fibre and off-fibre x=2 controls remain in the native program.

Two additional predetermined deliberate corruptions use the same compiled
native binary and the ordinary checking path:

1. `--mutate-first-root` replaces the first production depth-one root with
   the off-fibre value2. The native terminal-zero assertion must reject it.
2. `--mutate-norm-one-lift` increments the first lift's real coordinate by
   one. The native trace or norm assertion must reject it.

Both corruptions must exit nonzero, and their original error output is
retained. They are expected failures, not alternate successful runs.

## Accepted orchestration and custody

`run.sh` is a thin shell script that invokes the explicit rustc, copies the
executable into an attempt-specific directory, invokes it, and checks the
two deliberate failures. It invokes no Python. Node is used only for
source byte-range comparison and custody metadata: reading/writing files,
hashing source/executable bytes, and preserving existing declared values.
It computes no selector, field, group, relation, research result, or count.

The Rust qualifier emits the compact per-depth counts directly as raw JSON;
the individual root arrays are omitted prospectively. Every root is still
checked. No script summarizes or transforms the research output. Every
captured stdout/stderr file must remain below 65,536 bytes. No output is
truncated to meet that limit.

Freeze protocol, selected source, qualifier, custody utility and shell-runner
hashes before compilation. After compilation, copy the native binary to a
read-only run location and hash it before executing it. Require source,
compiler and executable hashes to agree afterward. Build timeout60 seconds;
each invocation timeout30 seconds. Use a fresh directory; never overwrite an
attempt. Record exit status and raw stdout/stderr for every invocation.

## Acceptance and limits

The unmodified native run must exit zero and every expected corruption must
exit nonzero. Source/executable custody checks and source-range comparisons
must pass. No cost timing or benchmark is recorded. This remains a same-team
bounded correctness observation, `[pcc:unverified]`, with no independent
signed validation, complete ECDLP solve, group-order certification, or
full-root-library test claim. The parent owns publication.
