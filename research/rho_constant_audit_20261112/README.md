# Finite repository rho accounting audit

The actual repository benchmark completes768/768 known-answer executions:
576 measured and192 warmup rows, with no failure or interruption. Every
measured deterministic replay matches. All192 measured and64 warmup
same-seed A/A pairs agree on counts, phases, native counters and answers.
A separate one-counter corruption control is detected. [pcc:unverified]

These are cold counted group-addition equivalents, setup through recovery
and internal verification, divided by sqrt(r). Each cell contains16 targets
and three seeded rounds. The A/A method repeats the negation arm exactly;
it tests determinism, not independent-seed variance. [pcc:unverified]

| Registered curve | r | Negation S0 | Plain S0 | Negation S0.1 | Negation S1 |
|:--|--:|--:|--:|--:|--:|
| icv1-fp14-tm43-158b6914 | 16231 | 1.715 | 2.178 | 1.829 | 2.857 |
| icv1-fp16-t351-d433f24e | 65003 | 1.839 | 1.892 | 1.962 | 3.075 |
| icv1-fp18-t175-45770ebc | 261673 | 1.349 | 1.532 | 1.453 | 2.381 |
| icv1-fp20-t1293-3c2999ba | 1046999 | 1.245 | 1.631 | 1.353 | 2.328 |

S0/S0.1/S1 price the emitted unpriced native operations at0/0.1/1 GAE
each, on every arm. The negation arm emits canonicalisation counts;
plain rho lists none, so its displayed band is constant. This is the
emitted-counter band, not a calibrated complete bit-operation model.
The ordering reverses between weights0 and1 on all four curves: no ranking
holds throughout the declared band. The full native tables retain all
three arms, every counter and the recorded descriptive intervals.
[pcc:unverified]

The SafeCurves standard-negation reference is sqrt(pi/4), approximately0.886.
Every measured mean here is above it, with explicit fixed setup and all
sizes below the harness's2^25 calibration range. This finite cohort does
not refute or establish the qualified asymptotic average. The historical
native field `floor_s` is treated here as its birthday reference, not a
proved universal lower bound. No wall-clock speed or new security claim.
[pcc:unverified]

## Protocol, provenance and retained controls

- Main protocol045a59cc95f45314d5bf2b2c3cf9411aa8819b8e96ab17545fb3900676d014b6
  was frozen and committed01eb80f3 before data. SessionECBS1h99ad02ca65c0,
  fixed64 targets/3rounds/1warmup/3arms, original caps180s/session and15s/child.
- Separate preflight protocol3892337a84ebace912724965f9008904940611b1784ea987fee8d2921081712e
  was frozen and committed1214cbda before12calls. SessionECBS1h9cd21a0011c9
  verifies and replays12/12. Its targets/seeds are separate and never pooled.
- Exact release binary61030c23b4e95c0ed56b86a65b980c28e19ed09ba643719248734e13218d718d;
  all3,098 selected source/input files match baseline1d2ad3fd exactly.
  The actual release build succeeds after retained sparse-source errors and
  connection interruptions. Binary Git metadata is runtime-derived, not a
  compile-time revision receipt; actual main session Git commit is null and
  is not backfilled. Exact compiled-input and binary hashes are authoritative.
- Main export.json passes full file/contract validation; flow768planned,
  attempted and completed,0notcompleted/interrupted/resumed/excluded.
  Every scientific clock has wall_validfalse. L0 count results only.
- The pilot adapter draft export-preflight.json omitted analysis metadata and
  remains unchanged/unvalidated. export-preflight-complete.json supplies
  actual executed frozen-analysis hashes in a new write-once record and
  passes. Main adapter output was completed before its first disk write.

All items above remain unsigned producer evidence. No independent-machine
or signed-execution claim. No paid compute/storage. [pcc:unverified]

## Lean proof map

The separate reporting project proves exact arithmetic on the explicitly
recorded integer totals. It does not certify native execution authenticity,
compiler refinement, curve orders or an asymptotic collision law.
[pcc:unverified]

| Declaration in RhoConstantReportAudit | Exact checked scope |
|:--|:--|
| normalized_interval | Positive-denominator normalization bounds from exact squared inequalities |
| recorded_mean_intervals | Four rational intervals, each width0.0001, for the reported negation means |
| exact_phase_totals | Setup + search + internal verification equals each reported negation total |
| emitted_weight_order_flips | Each unpriced total is below plain; adding the recorded native counter at weight1 puts it above plain |
| recorded_means_above_reference | Each fixed reported mean exceeds sqrt(pi/4), without an expectation/asymptotic assertion |
| finite_report_arithmetic_certificate | Combined finite arithmetic certificate |

Full local gate11declarations/0failures/0trusted-native exceptions.
Lean4.35.0 / Mathlib56a00585. Only propext, Classical.choice and Quot.sound;
no holes, custom axioms, native_decide, waived linters or new elaboration
options. Exact source and packet evidence are recorded separately from
native measurement assurance. [pcc:unverified]

## Reproduction and files

Canonical session records must be hydrated as loose files for native replay
and the protocol's file hashes. The approved local checkpoint preserved
them in place. After publication, the existing artifact-autofix bot moved
four files to the artifact store and committed SHA-256/byte-count manifests:
the main export, both sessions' records and the main verify stdout.
Fresh checkouts must run the pull command below before replay or file checks.
Local retained copies match all four bot manifests byte-for-byte. Both exec/
directories contain no files. Original bytes, warmups and controls remain
retained; this publication move does not change the issued source plans,
exports, protocols or historical reports. The generated
main SQLite index is a rebuildable index, not canonical evidence; it remains
local and is regenerated by analyze-main.sh. No .db is committed.
[pcc:unverified]

```sh
python3 tools/artifacts/artifacts.py pull research/rho_constant_audit_20261112
python3 tools/artifacts/artifacts.py verify research/rho_constant_audit_20261112
cargo build --release --locked --bin ecbench
# Read-only replay; never overwrite or rerun either completed session.
target/release/ecbench verify --dir research/rho_constant_audit_20261112/sessions/main --replay-all
python3 ../crypto-autoresearcher-current/tools/experiment_protocol.py check-export \
  research/rho_constant_audit_20261112/export.json \
  --protocol research/rho_constant_audit_20261112/protocol.json --verify-files
cd research/rho_constant_audit_20261112/lean
lake build
```

Reports: [main](reports/run-20261112T2233Z/report.pdf) and
[separate preflight](reports/run-20261112T2225Z/report.pdf).
Plot source only lays out native-recorded means/intervals; no recomputed
statistics. The main report includes accounting and the finite proof map.
[pcc:unverified]

## Source reading and original requirement11

Read selected sections4,4.1–4.5,5intro,B.1–B.2 of
[Bernstein–Lange, Safe curves for elliptic-curve cryptography](https://pure.tue.nl/ws/portalfiles/portal/375666556/safecurves.pdf),
permanentID47f17793322fcaf3ef67f982e2c3ae86b34c17ce, dated2024-08-09,
portal2025 publication. The paper distinguishes batched-affine additions,
standard negation rho and free-memory models with better constants.
The rho web page returned502/eprint403/direct portal403; web PDF text
succeeded. No full69-page read or local PDF-byte hash claimed.
[pcc:unverified]

Pinned repository walk/counting/constructor/interval sources and KB fallback
are in evidence/source-review.json. KB unavailable (MCP absent/wrapper127),
bounded repository/primary/Mathlib fallback. This completes the requested
finite repository-rho measurement component. Item11's generic p-adic/
complexity/native-refinement and limiting-law obligations, and the remaining
fourteen-item matrix, stay open. No novelty or qualified-source refutation.
[pcc:unverified]

Local certificate replay: PCC-20261112-cb73f6 passes at2026-11-12T23:03:20Z,
bound to producer0c151089 and complete declaration13773bdc8a2afb0d915a6367179888e314290ba9b0638cf657958b7c194da8ea.
The separate native packet PCC-20261112-06eb58 is not_covered as unsigned
producer evidence. Both signed standings remain unverified. Exact receipts:
[sources.json](sources.json), [native-attestation.json](native-attestation.json),
[verification.json](verification.json). [pcc:unverified]
