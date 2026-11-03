# Round09 command reference

All paths below are relative to the repository root. Raw execution logs
use numbered attempts; unsuccessful attempts are retained. No timing run.

```sh
study=research/koblitz_modular_ic_20261018
round="$study/rounds/09"
cargo build --release --offline --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
"$runner" --round09-controls "$round/runs"
"$runner" --round09-descent "$round/runs"
"$runner" --verify-round09-descent "$round/runs"
"$runner" --round09-invariants "$round/runs"
```

The runner refuses to overwrite accepted records. Replay in a fresh
output directory instead of rerunning the generation commands in runs.

```sh
"$round/REPLAY.sh" "$round/runs" /tmp/koblitz-round09-fresh
"$study/rounds/08/REPLAY.sh" "$study/rounds/08/runs" /tmp/koblitz-round09-prior08-fresh
cargo test --release --offline --locked --manifest-path "$study/native/Cargo.toml"
CARGO_TARGET_DIR=/home/ec2-user/crypto/target cargo test --release --lib --offline --locked
(cd "$study/lean" && lake build && lake env lean --run ../../../tools/lean/AuditAxioms.lean KoblitzRound09)
tools/lean/proof-gate.sh "$study/lean" --out /tmp/koblitz-round09-proof-gate
node tools/render_ic_dashboard.mjs --check
python3 -m unittest discover -s scripts/site -p 'test_build.py'
python3 -m unittest discover -s scripts -p 'test_workflows_parse.py'
```

Use an absolute AuditAxioms.lean path when executing from another working
directory. Source/raw hashes and IDC certificates are assembled natively
by --record-round09 after ordinary raw logs are closed. Assembly receipts
record-* are excluded from the ledger's raw binding to avoid circularity,
and included in runs.sha256. Archive/repack commands are in README.md.
The report renderer consumes only frozen ledger data and written text.
