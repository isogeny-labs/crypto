# Reproduction commands and evidence boundary

Run from the repository root. The code base at experiment start was
`1dd582cc7` on `codex/new-leads-20261019`. The new method source and
fixture programs are in this PR; use the committed PR revision to rebuild.
These runs are deterministic operation-count and mathematical checks, not
qualified isolated timing. The host had competing jobs, so wall time from
the ecbench sessions is not used for a speed claim.

```sh
cargo build --release --offline --locked --bin ecbench --bin tian_quartic_smoothness --bin smith_holdout_materialize
cargo build --release --offline --locked --manifest-path research/five_leads_e2e_20261019/native/Cargo.toml
research/five_leads_e2e_20261019/native/target/release/tian-cubic-kernel > research/five_leads_e2e_20261019/correctness/tian-cubic-kernel-v1.json
target/release/tian_quartic_smoothness > research/five_leads_e2e_20261019/correctness/tian-quartic-smoothness-v1.jsonl
target/release/smith_holdout_materialize > research/five_leads_e2e_20261019/correctness/smith-negative-materialized-v1.jsonl
research/five_leads_e2e_20261019/native/target/release/genus2-model-gate > research/five_leads_e2e_20261019/correctness/genus2-model-gate-v1.jsonl
research/five_leads_e2e_20261019/native/target/release/mixed-fiber-census > research/five_leads_e2e_20261019/correctness/mixed-fiber-census-v1.jsonl
python3 research/five_leads_e2e_20261019/analyze_mixed_fiber.py
```

The Smith holdout was selected from the earlier immutable compressed census
before odd-kernel testing. Its source revision and compressed-file SHA-256
are embedded in
[`smith-negative-holdout-v1.json`](fixtures/smith-negative-holdout-v1.json).
Re-running its selection requires that archived census:

```sh
python3 research/five_leads_e2e_20261019/freeze_smith_holdout.py /home/ec2-user/crypto-w3b-smith-isogeny/research/smith_prop24_native_20261014/data/random_p2147483647_rows.csv.gz
```

The published \(\mathbf F_{257}\) one-infinity genus-3 rational divisor
was checked with the existing hyperelliptic CLI:

```sh
target/release/hyperelliptic-cover arithmetic --input research/five_leads_e2e_20261019/fixtures/tian-genus3-f257-t1.json > research/five_leads_e2e_20261019/correctness/tian-genus3-f257-t1-result.json
```

For the exact-order collector cohort, the seven specifications were generated
from the original fixture specifications and then run once in ascending order:

```sh
python3 research/five_leads_e2e_20261019/make_genus2_choice2_specs.py
for bits in 16 20 24 28 32 36 40; do
  target/release/ecbench run --spec research/five_leads_e2e_20261019/specs/genus2-choice2-v1/order-genus-$bits.json --out research/five_leads_e2e_20261019/sessions/order-genus-$bits --cpus none --allow-busy --settle 0 --lock /tmp/crypto-five-leads-choice2-$bits.lock --quiet
done
for bits in 16 20 24 28 32 36 40; do
  target/release/ecbench verify --dir research/five_leads_e2e_20261019/sessions/order-genus-$bits --replay-all > research/five_leads_e2e_20261019/replay/order-genus-$bits.json
done
python3 research/five_leads_e2e_20261019/analyze_genus2_choice2.py
```

The complete session trees are packed in `sessions.tar.zst` and can be
extracted by `tar --zstd -xf sessions.tar.zst` from this directory. Its
SHA-256 and the compact outputs' digests are in `SHA256SUMS`. A replay
receipt binds each session's spec, plan, host, records, session summary
and audit binary digests. The source rho and destination rho records are
paired with each collector record by workload ID. The collector's source
scalar was checked during execution and again by `verify --replay-all`.

Final verification commands:

```sh
cargo test --release --offline --locked --lib
cargo test --release --offline --locked --bin tian_quartic_smoothness --bin smith_holdout_materialize
cargo test --release --offline --locked --manifest-path research/five_leads_e2e_20261019/native/Cargo.toml
python3 -m py_compile research/five_leads_e2e_20261019/*.py
rustfmt --check --edition 2021 src/cryptanalysis/three_leads_followup.rs src/bin/smith_holdout_materialize.rs src/bin/tian_quartic_smoothness.rs research/five_leads_e2e_20261019/native/*.rs
```
