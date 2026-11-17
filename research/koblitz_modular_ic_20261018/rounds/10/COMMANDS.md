# Round10 commands

All paths are relative to the repository root. Use fresh output directories:
the runner refuses to overwrite accepted records.

```sh
study=research/koblitz_modular_ic_20261018
round="$study/rounds/10"
cargo build --release --offline --locked --manifest-path "$study/native/Cargo.toml"
runner="$study/native/target/release/koblitz-modular-carriers"
"$runner" --round10-polynomials "$round/runs"
"$runner" --verify-round10-polynomials "$round/runs"
"$runner" --round10-expand "$round/runs"
"$runner" --round10-projective-control "$round/runs"
"$runner" --round10-divisors "$round/runs"
"$runner" --round10-collision "$round/runs"
"$runner" --round10-diagonal "$round/runs"
```

For routine verification,replay all13 outputs using REPLAY.sh rather than
regenerating into runs:

```sh
"$round/REPLAY.sh" "$round/runs" /tmp/koblitz-round10-fresh
"$study/rounds/09/REPLAY.sh" "$study/rounds/09/runs" /tmp/koblitz-round10-prior09-fresh
cargo test --release --offline --locked --manifest-path "$study/native/Cargo.toml"
CARGO_TARGET_DIR=/home/ec2-user/crypto/target cargo test --release --lib --offline --locked
(cd "$study/lean" && lake build && lake env lean --run ../../../tools/lean/AuditAxioms.lean KoblitzRound10)
tools/lean/proof-gate.sh "$study/lean" --out /tmp/koblitz-round10-proof-gate
node tools/render_ic_dashboard.mjs --check
python3 -m unittest discover -s scripts/site -p 'test_build.py'
python3 -m unittest discover -s scripts -p 'test_workflows_parse.py'
```

After every ordinary raw log is closed,native --record-round10 assembles
the ledger and two IDC certificates. Assembly receipts record-* avoid
circular raw bindings and are included in runs.sha256. Archive/repack
commands are in README.md. Report rendering reads frozen evidence only.
