# Genus-4 independent audit

Read [REPORT.md](REPORT.md), [REPORT.pdf](REPORT.pdf),
[out/CHECKS.md](out/CHECKS.md) and [PROTOCOL.md](PROTOCOL.md).
No paid compute. No leaderboard promotion. Signed PCC standing is unverified.

## Reproduce

Use a source checkout of `2ca7ece552ad276908db4ec211c2ab452283dd58`:

```sh
git worktree add --detach /tmp/g4-frozen 2ca7ece552ad276908db4ec211c2ab452283dd58
cd research/genus4_independent_audit_20261116/native
bash stage.sh /tmp/g4-frozen
cargo test --release --locked
cargo build --release --locked
target/release/g4audit recalc ORIGINAL_RUN_JSONL
target/release/g4audit recalc FOLLOWUP_RUN_JSONL
target/release/g4audit calibrated FOLLOWUP_RUN_JSONL
target/release/g4audit replay ORIGINAL_RUN_JSONL cvwR 3
target/release/g4audit certify REPLAY_JSONL
target/release/g4audit transfer ORIGINAL_RUN_JSONL
```

For the retained complete relation collection:

```sh
G4_AUDIT_DUMP=1 bash stage.sh /tmp/g4-frozen
cargo build --release --locked
G4_REL_DUMP=/tmp/g4-relations.jsonl target/release/g4audit replay ../out/relation-input.jsonl w 1 verify
target/release/g4audit relations /tmp/g4-relations.jsonl
```

The dump path must be new: append mode intentionally preserves previous
observations. The optional observation hook adds no random draws or arithmetic
and changes no solver counters; the replay records its matches. The original
full replay used the uninstrumented binary.

Window-free plain rho is the original Rust `RhoWalk::plain()` with
`cycle_window=0`; the driver denotes this audit arm Q. It preserves seeds,
jumps, targets, step cap and group arithmetic. Its input rows retain the
recorded plain-rho counts so the replay can check shared counters. The changed
window counter is emitted separately as `compares` and must be zero.

`render_report.py` performs document layout and plotting only. Every plotted
number and interval is read from Rust-generated JSON. It performs no
cryptographic calculation or benchmark analysis.

## Boundaries

`certify` checks finite genus-4 divisor/scalar witnesses. `relations` checks
one complete collection independently. The group-order deduction additionally
uses prime r, nonzero r-torsion and a unique multiple in a rational Hasse-Weil
interval. None is a Lean theorem or signed PCC promotion. Full per-matrix
operation certification, isolated calibrated costs, per-phase RSS, external
IC frontier comparison and a usable prime-field transfer remain separate
acceptance requirements.
