# Reproduction commands and evidence boundary

Run from the repository root. The code base at experiment start was
`1dd582cc7` on `codex/new-leads-20261019`. The new method source and
fixture programs are on this branch; use the committed revision to rebuild.
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
research/five_leads_e2e_20261019/native/target/release/mixed-fiber-dlp > research/five_leads_e2e_20261019/correctness/mixed-fiber-dlp-v2.jsonl
research/five_leads_e2e_20261019/native/target/release/mixed-fiber-dlp-analyze
```

The Smith holdout was selected from the earlier immutable compressed census
before odd-kernel testing. Its source revision and compressed-file SHA-256
are embedded in
[`smith-negative-holdout-v1.json`](fixtures/smith-negative-holdout-v1.json).

The Tian source-map coefficient fixture is reproducibly extracted from the
original [arXiv v1 source](https://arxiv.org/abs/2012.07173v1). The extractor
rejects a TeX SHA-256 mismatch; its six function arrays and the denominator
are then evaluated natively on 101 source points:

```sh
mkdir -p /tmp/tian2012-source
curl -L --fail https://export.arxiv.org/src/2012.07173 | tar -xz -C /tmp/tian2012-source
python3 research/five_leads_e2e_20261019/extract_tian_map.py /tmp/tian2012-source/Cover_attack.tex > research/five_leads_e2e_20261019/fixtures/tian-cubic-map-v1.json
research/five_leads_e2e_20261019/native/target/release/tian-cubic-map-gate > research/five_leads_e2e_20261019/correctness/tian-cubic-map-gate-v2.jsonl
```

The map gate checks a separable three-point cut on the quartic and a
deliberate bad-coefficient control. It does not construct the corresponding
Jacobian classes or trace them to the base field by itself. The following
commands run that trace, verify 101 mapped source points, solve the mapped
P/Q DLP, and price all three cold stages:

```sh
research/five_leads_e2e_20261019/native/target/release/tian-quartic-km --osculating9-gate > research/five_leads_e2e_20261019/correctness/tian-osculating9-gate-v1.json
research/five_leads_e2e_20261019/native/target/release/tian-quartic-km --trace-map-gate > research/five_leads_e2e_20261019/correctness/tian-cubic-trace-map-gate-v1.json
research/five_leads_e2e_20261019/native/target/release/tian-quartic-km --trace-map-pq > research/five_leads_e2e_20261019/correctness/tian-cubic-trace-map-pq-v1.json
research/five_leads_e2e_20261019/native/target/release/tian-quartic-km --line-sieve-mapped-dlp > research/five_leads_e2e_20261019/correctness/tian-quartic-line-sieve-mapped-dlp-v1.json
research/five_leads_e2e_20261019/native/target/release/tian-line-sieve-mapped-analyze > research/five_leads_e2e_20261019/correctness/tian-line-sieve-mapped-analysis-v1.json
```

The `--trace-map-pq` mode consumes the P and published Q coordinates from
the map receipt and stores the resulting Jacobian classes. It checks them
against Tian's S,T but does not use the planted scalar to form either class.
The mapped line sieve consumes those classes, not the published classes.
The price analysis adds P/Q formula and trace-constraint counts, the cold
trace-class run, and the mapped line-sieve run. Source rho uses the same
five seeds as the published-class control. Memory and control flow are
unpriced; the single published fixture gives no asymptotic claim.
Re-running its selection requires that archived census:

```sh
python3 research/five_leads_e2e_20261019/freeze_smith_holdout.py /home/ec2-user/crypto-w3b-smith-isogeny/research/smith_prop24_native_20261014/data/random_p2147483647_rows.csv.gz
```

The published \(\mathbf F_{257}\) one-infinity genus-3 rational divisor
was checked with the existing hyperelliptic CLI:

```sh
target/release/hyperelliptic-cover arithmetic --input research/five_leads_e2e_20261019/fixtures/tian-genus3-f257-t1.json > research/five_leads_e2e_20261019/correctness/tian-genus3-f257-t1-result.json
```

For the exact-order collector cohort, the seven specifications were frozen
from the original fixture specifications and then run once in ascending order.
The historical Python spec generator is retained for provenance; reproduction
uses the committed JSON specifications directly:

```sh
for bits in 16 20 24 28 32 36 40; do
  target/release/ecbench run --spec research/five_leads_e2e_20261019/specs/genus2-choice2-v1/order-genus-$bits.json --out research/five_leads_e2e_20261019/sessions/order-genus-$bits --cpus none --allow-busy --settle 0 --lock /tmp/crypto-five-leads-choice2-$bits.lock --quiet
done
for bits in 16 20 24 28 32 36 40; do
  target/release/ecbench verify --dir research/five_leads_e2e_20261019/sessions/order-genus-$bits --replay-all > research/five_leads_e2e_20261019/replay/order-genus-$bits.json
done
research/five_leads_e2e_20261019/native/target/release/genus2-choice2-analyze
```

The complete session trees are packed in `sessions.tar.zst` and can be
extracted by `tar --zstd -xf sessions.tar.zst` from this directory. Its
SHA-256 and the compact outputs' digests are in `SHA256SUMS`. A replay
receipt binds each session's spec, plan, host, records, session summary
and audit binary digests. The source rho and destination rho records are
paired with each collector record by workload ID. The collector's source
scalar was checked during execution and again by `verify --replay-all`.
The historical Python price grid remains retained; the native reanalysis
above independently checked all 6,300 records and their replays and reproduced
its representative ratios and processed-divisor counts.

The projective chart cohort uses the same 1,260 exact-order workloads as the
two-choice cohort. The corrected second run is the counted archive; the first
developmental run changed only zero-valued counters and one detail label on
the unchanged affine arm and is excluded from comparison. Reproduce the
corrected run and native analysis with:

```sh
cd research/five_leads_e2e_20261019
tar --zstd -xf sessions_projective_chart_v2.tar.zst
tar --zstd -xf sessions_projective_chart_unsealed_v1.tar.zst
cd ../..
```

The archives preserve the raw runs. To rerun rather than audit their recorded
outputs, choose new output directories; ecbench refuses to overwrite a sealed
session. The original commands were:

```sh
sh research/five_leads_e2e_20261019/make_genus2_projective_specs.sh
for bits in 16 20 24 28 32 36 40; do
  target/release/ecbench run --spec research/five_leads_e2e_20261019/specs/genus2-projective-chart-v1/order-genus-$bits.json --out research/five_leads_e2e_20261019/sessions_projective_chart_v2/order-genus-$bits --cpus none --allow-busy --settle 0 --lock /tmp/crypto-five-leads-projective-chart-v2-$bits.lock --quiet
done
for bits in 16 20 24 28 32 36 40; do
  target/release/ecbench verify --dir research/five_leads_e2e_20261019/sessions_projective_chart_v2/order-genus-$bits --replay-all > research/five_leads_e2e_20261019/replay_projective_chart_v2/order-genus-$bits.json
done
research/five_leads_e2e_20261019/native/target/release/genus2-projective-analyze
```

The native mixed-fiber analysis can be regenerated from the retained v2 DLP
records with `mixed-fiber-dlp-analyze`.
The new exhaustive summation-polynomial arm uses the same 180 target points
and three fiber choices, with all roots and sign lifts charged:

```sh
research/five_leads_e2e_20261019/native/target/release/mixed-fiber-polynomial-dlp > research/five_leads_e2e_20261019/correctness/mixed-fiber-polynomial-dlp-v1.jsonl
research/five_leads_e2e_20261019/native/target/release/mixed-fiber-polynomial-analyze
```

The analyzer requires all 540 candidate and matched rho records to verify,
matches every target and fiber to the pair-table cohort, and writes the
36-price per-curve grid. This is complete exhaustive Semaev-S4 rational-root
evaluation, not F4/msolve; an external Gröbner solver is not installed here.

The native F4 route uses the repository's degree-bounded solver on the
same Semaev-S4 resultant and three fiber equations. Reproduction requires
the exhaustive-polynomial receipt above because it freezes every actual
relation-query x-coordinate. Run from repository root:

```sh
cargo build --release --offline --locked --bin five_leads_mixed_f4
target/release/five_leads_mixed_f4 --cohort > research/five_leads_e2e_20261019/correctness/mixed-fiber-f4-target-cohort-v1.jsonl
target/release/five_leads_mixed_f4 --dlp-cohort > research/five_leads_e2e_20261019/correctness/mixed-fiber-f4-dlp-oracle-v1.jsonl
research/five_leads_e2e_20261019/native/target/release/mixed-fiber-polynomial-dlp --f4-replay > research/five_leads_e2e_20261019/correctness/mixed-fiber-f4-dlp-v1.jsonl
research/five_leads_e2e_20261019/native/target/release/mixed-fiber-f4-analyze
```

The first F4 run checks 540 target-coordinate root sets; the second checks
2,290 polynomial systems among 2,316 actual DLP relation trials (26 have an
identity right-hand side). Each F4 result is independently compared
with exhaustive rational-root enumeration before being saved. The replay
consumes those ordered roots, sign-lifts them, and source-verifies every
scalar. The analyzer requires all 540 targets, rho controls, query lists,
relation counts and answers to match, then writes a 36-price grid. Its
F4 `field_ops` counter counts row-reduction multiplications but omits
symbolic preprocessing and substitution arithmetic, so the computed ratios
are *lower bounds*, not complete F4 costs. F4 internal wall times are
unqualified on this busy host; no external msolve run was available.

Tian's published quartic group-law
checks, optimized cubic source rho, and the explicitly non-DLP arithmetic
cost gate use `tian-quartic-km`, `tian-cubic-rho`, and
`tian-km-gate-analyze` from the same native crate.
The published-class destination DLP control runs with
`tian-quartic-km --dlp-linear`, writes
`correctness/tian-km-linear-dlp-v1.json`, and is priced by
`tian-km-linear-analyze`. This bounded linear search receives only the
published destination classes, has a cap of 10,000, and source-verifies the
recovered scalar. It does not execute the source-to-Jacobian map or the
separate quartic line-sieve index calculus described below.
The initial `tian-cubic-rho-v1.jsonl` receipt expanded every cubic-field
inversion into exponentiation and is excluded from cost comparisons; v2 uses
the norm inversion and charges its base-field M/S/I vector. The initial
`tian-quartic-km-v1.json` receipt lacked phase snapshots and group-law
controls; v2 is the priced arithmetic gate. Both earlier receipts are
retained as developmental evidence.

The published-class quartic line-sieve DLP can be reproduced with:

```sh
research/five_leads_e2e_20261019/native/target/release/tian-quartic-km --line-sieve-dlp > research/five_leads_e2e_20261019/correctness/tian-quartic-line-sieve-dlp-v1.json
research/five_leads_e2e_20261019/native/target/release/tian-line-sieve-analyze
```

The first command receives the published destination classes only. It
enumerates every rational line relation, performs modular elimination,
attempts rational-divisor descent and independently checks each successful
divisor equality in the Jacobian before source and destination DLP checks.
Its output records all failed attempts. The analyzer verifies the rank,
answer and checks, prices the complete cold field-operation vectors across
36 M/S/I/addition cells, and retains every loss to rho. The source map is
not executed, and memory/control-flow work remains unpriced.

Final verification commands:

```sh
cargo test --release --offline --locked --lib
cargo test --release --offline --locked --bin tian_quartic_smoothness --bin smith_holdout_materialize --bin five_leads_mixed_f4
cargo test --release --offline --locked --manifest-path research/five_leads_e2e_20261019/native/Cargo.toml
python3 -m py_compile research/five_leads_e2e_20261019/*.py
rustfmt --check --edition 2021 src/cryptanalysis/three_leads.rs src/cryptanalysis/three_leads_followup.rs src/bin/smith_holdout_materialize.rs src/bin/tian_quartic_smoothness.rs research/five_leads_e2e_20261019/native/*.rs
```
